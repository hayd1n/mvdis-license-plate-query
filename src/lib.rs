use std::{sync::Arc, time::Duration};

use ocr::Ocr;
use options::{PlateType, PlateVer, Region, Station, WindowNo};
use reqwest::redirect;
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

pub mod ocr;
pub mod options;
pub mod pool;

pub const DEFAULT_BASE_URL: &str = "https://www.mvdis.gov.tw";
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub fn default_reqwest_builder() -> reqwest::ClientBuilder {
    let cookie_store = reqwest_cookie_store::CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .timeout(Duration::from_secs(10))
        .redirect(redirect::Policy::limited(10))
        .cookie_provider(Arc::clone(&cookie_store))
}

pub struct ClientBuilder {
    client: reqwest::Client,
    base_url: String,
}

impl ClientBuilder {
    pub fn new() -> Self {
        let client = default_reqwest_builder().build().unwrap();

        Self {
            client,
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn new_with_client(client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    pub fn build(self) -> Client {
        Client {
            http_client: self.client,
            base_url: self.base_url,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QueryOptions {
    pub plate_ver: PlateVer,
    pub plate_type: PlateType,
    pub region: Region,
    pub station: Station,
    pub window_no: WindowNo,
}

#[derive(Debug)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        ClientBuilder::new().build()
    }

    pub async fn get_csrf_token(&self) -> Result<String, Error> {
        let url = format!("{}/m3-emv-plate/webpickno/queryPickNo", self.base_url);
        let res = self.http_client.get(&url).send().await?;
        let document = Html::parse_document(res.text().await?.as_str());
        let csrf_selector = Selector::parse("input[name='CSRFToken']").unwrap();
        let csrf_token = document
            .select(&csrf_selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap();

        Ok(csrf_token.to_string())
    }

    pub async fn get_captcha_img(&self) -> Result<Vec<u8>, Error> {
        let url = format!("{}/m3-emv-plate/captchaImg.jpg", self.base_url);
        let res = self.http_client.get(&url).send().await?;

        Ok(res.bytes().await?.to_vec())
    }

    pub async fn post_query(
        &self,
        csrf_token: &str,
        captcha_text: &str,
        options: &QueryOptions,
        query_no: Option<&str>,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/m3-emv-plate/webpickno/queryPickNo", self.base_url);

        let payload = [
            ("method", "qryPickNo"),
            ("selDeptCode", options.region.as_str()),
            ("selStationCode", options.station.as_str()),
            ("selWindowNo", options.window_no.as_str()),
            ("selPlateType", options.plate_type.as_str()),
            ("plateVer", options.plate_ver.as_str()),
            ("validateStr", captcha_text),
            ("queryType", "0"),
            ("queryNo", query_no.unwrap_or("*")),
            ("CSRFToken", csrf_token),
        ];

        let res = self.http_client.post(&url).form(&payload).send().await?;
        res.text().await
    }
}

pub struct QueryBuilder {
    client: Arc<Client>,
    options: QueryOptions,
    query_no: Option<String>,
}

impl QueryBuilder {
    pub fn new(client: Arc<Client>, options: QueryOptions) -> Self {
        Self {
            client,
            options,
            query_no: None,
        }
    }

    pub fn query_no(mut self, query_no: &str) -> Self {
        self.query_no = Some(query_no.to_string());
        self
    }

    pub async fn send_with_captcha_text(self, captcha_text: &str) -> Result<Query, Error> {
        let csrf_token = self.client.get_csrf_token().await?;

        let res = self
            .client
            .post_query(
                &csrf_token,
                captcha_text,
                &self.options,
                self.query_no.as_deref(),
            )
            .await?;

        let document = Html::parse_document(res.as_str());

        let message_selector = Selector::parse("td#headerMessage").unwrap();

        if let Some(message_dom) = document.select(&message_selector).next() {
            let message = message_dom.text().collect::<String>();

            if message.contains("驗證數字輸入錯誤") {
                return Err(Error::CaptchaError);
            }
        }

        Ok(Query {
            client: self.client,
            document: Some(document),
        })
    }

    pub async fn send(self, ocr: Arc<Ocr>) -> Result<Query, Error> {
        let captcha_img = self.client.get_captcha_img().await?;
        let captcha_text = ocr.classification_captcha_text(&captcha_img)?;

        self.send_with_captcha_text(captcha_text.as_str()).await
    }
}

pub struct Query {
    client: Arc<Client>,
    document: Option<Html>,
}

impl Query {
    pub async fn next_page(&mut self) -> Result<bool, Error> {
        let next_page_url = {
            let document = self
                .document
                .as_ref()
                .ok_or_else(|| Error::NoDocumentFound)?;

            let next_page_selector = Selector::parse("a#next").unwrap();
            document
                .select(&next_page_selector)
                .next()
                .and_then(|dom| dom.value().attr("href").map(|s| s.to_string()))
        };

        if let Some(next_page_url) = next_page_url {
            let url = format!("{}{}", self.client.base_url, next_page_url);
            let res = self.client.http_client.get(&url).send().await?;
            self.document = Some(Html::parse_document(res.text().await?.as_str()));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn results(&self) -> Result<Option<Vec<PlateInfo>>, Error> {
        if let Some(document) = &self.document {
            let cell_selector =
                Selector::parse("table#countList tbody td div.number_cell").unwrap();
            let plate_no_selector = Selector::parse("a.number").unwrap();
            let price_selector = Selector::parse("div.price").unwrap();

            let mut plate_infos = vec![];

            for cell in document.select(&cell_selector) {
                // Find plate number
                if let Some(plate_no_dom) = cell.select(&plate_no_selector).next() {
                    let plate_no = plate_no_dom
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_string()
                        .to_owned();

                    // Find price
                    if let Some(price_dom) = cell.select(&price_selector).next() {
                        let price = price_dom.text().collect::<String>().to_owned();

                        let price = price
                            .trim()
                            .replace("元", "")
                            .replace(",", "")
                            .parse::<i32>()?;

                        plate_infos.push(PlateInfo { plate_no, price });
                    }
                }
            }

            Ok(Some(plate_infos))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlateInfo {
    pub plate_no: String,
    pub price: i32,
}

#[derive(Debug)]
pub enum Error {
    CaptchaError,
    ReqwestError(reqwest::Error),
    ParseError(String),
    NoDocumentFound,
    OcrError(ocr::OcrError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CaptchaError => write!(f, "Captcha error"),
            Error::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            Error::ParseError(err) => write!(f, "Parse error: {}", err),
            Error::NoDocumentFound => write!(f, "No document found"),
            Error::OcrError(err) => write!(f, "OCR error: {}", err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError(err.to_string())
    }
}

impl From<ocr::OcrError> for Error {
    fn from(err: ocr::OcrError) -> Self {
        Error::OcrError(err)
    }
}
