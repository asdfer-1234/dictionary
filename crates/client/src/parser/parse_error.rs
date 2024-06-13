
#[derive(Debug)]
pub enum ParseError {
    File(std::io::Error),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::File(value)
    }
}
