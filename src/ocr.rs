use std::sync::{Arc, Mutex};

pub struct Ocr {
    ddddocr: Arc<Mutex<ddddocr::Ddddocr<'static>>>,
}

impl Ocr {
    pub fn new() -> Result<Self, OcrError> {
        let ddddocr = ddddocr::ddddocr_classification().map_err(OcrError::DdddocrError)?;
        Ok(Self {
            ddddocr: Arc::new(Mutex::new(ddddocr)),
        })
    }

    pub fn classification_captcha_text(&self, image: &[u8]) -> Result<String, OcrError> {
        let mut ddddocr = self.ddddocr.lock().map_err(|_| {
            OcrError::DdddocrError(anyhow::anyhow!("Failed to acquire lock on OCR"))
        })?;
        let result = ddddocr.classification(image.to_vec(), false)?;
        Ok(result)
    }
}

#[derive(Debug)]
pub enum OcrError {
    DdddocrError(anyhow::Error),
}

impl std::error::Error for OcrError {}

impl std::fmt::Display for OcrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DdddocrError(err) => write!(f, "Ddddocr error: {}", err),
        }
    }
}

impl From<anyhow::Error> for OcrError {
    fn from(err: anyhow::Error) -> Self {
        Self::DdddocrError(err)
    }
}
