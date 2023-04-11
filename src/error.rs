use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Upstream(UpstreamError),
    Request(reqwest::Error),
    Internal(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Upstream(e) => write!(f, "{e}"),
            Error::Request(e) => write!(f, "{e}"),
            Error::Internal(e) => write!(f, "{e}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(v: reqwest::Error) -> Self {
        Error::Request(v)
    }
}

impl From<url::ParseError> for Error {
    fn from(v: url::ParseError) -> Self {
        Error::Internal(v.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct UpstreamError {
    pub code: i32,
    pub message: String,
    pub details: Option<Vec<serde_json::Value>>,
}

impl std::error::Error for UpstreamError {}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(details) = &self.details.as_ref().filter(|d| !d.is_empty()) {
            write!(f, "\nDetails:")?;
            for error in details.iter() {
                write!(f, "\n- {error}")?;
            }
        }
        Ok(())
    }
}
