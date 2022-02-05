use std::fmt;
use reqwest::{
    self,
    StatusCode
};
use reqwest_middleware;
use serde_qs;

#[derive(Debug)]
pub enum APIError {
    ParameterError(&'static str),
    ResponseError(String),
    ReqwestError(reqwest::Error),
    ReqwestMiddlewareError(anyhow::Error),
    QueryParameterError(serde_qs::Error),
    ParseError(serde_json::Error),
    HttpError(StatusCode),
    NotLoggedIn,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::ParameterError(s) => write!(f, "{}", s),
            APIError::ResponseError(s) => write!(f, "{}", s),
            APIError::ReqwestError(e) => write!(f, "{}", e),
            APIError::ReqwestMiddlewareError(e) => write!(f, "{}", e),
            APIError::QueryParameterError(e) => write!(f, "{}", e),
            APIError::ParseError(e) => write!(f, "{}", e),
            APIError::HttpError(e) => write!(f, "{}", e),
            APIError::NotLoggedIn => write!(f, "Not logged in"),
        }
    }
}

impl From<reqwest_middleware::Error> for APIError {
    fn from(error: reqwest_middleware::Error) -> APIError {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                APIError::ReqwestError(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                APIError::ReqwestMiddlewareError(e)
            },
        }
    }
}

impl From<std::fmt::Error> for APIError {
    fn from(_error: std::fmt::Error) -> APIError {
        APIError::ParameterError("Formatting error")
    }
}

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> APIError {
        APIError::ParseError(error)
    }
}

impl From<serde_qs::Error> for APIError {
    fn from(error: serde_qs::Error) -> APIError {
        APIError::QueryParameterError(error)
    }
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> APIError {
        APIError::ReqwestError(error)
    }
}