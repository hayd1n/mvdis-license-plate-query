use std::collections::HashMap;
use std::sync::Arc;

use strum::IntoEnumIterator;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::{
    ocr::Ocr,
    options::{self, PlateType, PlateVer, Region, Station, WindowNo},
    Client, Error, PlateInfo, QueryBuilder, QueryOptions,
};

#[derive(Debug, Clone)]
pub struct PoolClientOptions {
    pub plate_ver: PlateVer,
    pub plate_type: PlateType,
    /// If None, query all regions. If Some, query only the specified regions.
    pub regions: Option<Vec<Region>>,
    /// Number of times to retry when encountering a captcha error.
    pub retry_times: usize,
    /// Number of concurrent tasks to run.
    pub concurrency: usize,
}

impl Default for PoolClientOptions {
    fn default() -> Self {
        Self {
            plate_ver: PlateVer::New,
            plate_type: PlateType::MotorcycleElectricNormalHeavy,
            regions: None,
            retry_times: 3,
            concurrency: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
        }
    }
}

pub struct PoolClient {
    options: PoolClientOptions,
    ocr: Arc<Ocr>,
}

pub type PoolResult = HashMap<Region, HashMap<Station, Vec<PlateInfo>>>;

impl PoolClient {
    pub fn new(options: PoolClientOptions) -> Result<Self, Error> {
        let ocr = Arc::new(Ocr::new()?);
        Ok(Self { options, ocr })
    }

    pub fn with_ocr(options: PoolClientOptions, ocr: Arc<Ocr>) -> Self {
        Self { options, ocr }
    }

    /// Executes the pool queries and returns a deeply nested map of results.
    ///
    /// The returned structure maps `Region` -> `Station` -> `Vec<PlateInfo>`.
    pub async fn execute(&self) -> Result<PoolResult, Error> {
        let concurrency = self.options.concurrency.max(1);
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let mut join_set = JoinSet::new();

        let regions = self
            .options
            .regions
            .clone()
            .unwrap_or_else(|| Region::iter().collect());

        for region in regions {
            let stations = options::get_station(region);
            for station in stations {
                for window_no in WindowNo::iter() {
                    let semaphore = Arc::clone(&semaphore);
                    let ocr = Arc::clone(&self.ocr);
                    let retry_times = self.options.retry_times;

                    let query_options = QueryOptions {
                        region,
                        station,
                        window_no,
                        plate_ver: self.options.plate_ver,
                        plate_type: self.options.plate_type,
                    };

                    join_set.spawn(async move {
                        let _permit = semaphore.acquire_owned().await.unwrap();
                        let client = Arc::new(Client::new());
                        let mut query_results = Vec::new();
                        let mut attempts = 0;

                        let mut query = loop {
                            match QueryBuilder::new(Arc::clone(&client), query_options)
                                .send(Arc::clone(&ocr))
                                .await
                            {
                                Ok(q) => break q,
                                Err(Error::CaptchaError) if attempts < retry_times => {
                                    attempts += 1;
                                    continue;
                                }
                                Err(e) => return Err(e),
                            }
                        };

                        loop {
                            match query.results() {
                                Ok(Some(res)) => query_results.extend(res),
                                Ok(None) => {}
                                Err(e) => return Err(e),
                            }

                            match query.next_page().await {
                                Ok(true) => continue,
                                Ok(false) => break,
                                Err(e) => return Err(e),
                            }
                        }

                        Ok::<(QueryOptions, Vec<PlateInfo>), Error>((query_options, query_results))
                    });
                }
            }
        }

        let mut results_map: PoolResult = HashMap::new();

        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(Ok((query_options, plates))) => {
                    let region_map = results_map.entry(query_options.region).or_default();
                    let station_plates = region_map.entry(query_options.station).or_default();
                    station_plates.extend(plates);
                }
                Ok(Err(e)) => {
                    return Err(e);
                }
                Err(e) => {
                    if e.is_panic() {
                        std::panic::resume_unwind(e.into_panic());
                    }
                }
            }
        }

        Ok(results_map)
    }
}

impl PoolClient {
    pub fn options(&self) -> &PoolClientOptions {
        &self.options
    }
}
