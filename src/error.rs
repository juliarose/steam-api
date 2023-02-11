
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    ParameterError(&'static str),
    #[error("{}", .0)]
    ResponseError(String),
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", .0)]
    ReqwestMiddlewareError(anyhow::Error),
    #[error("{}", .0)]
    QueryParameterError(#[from] serde_qs::Error),
    #[error("{}", .0)]
    ParseError(#[from] serde_json::Error),
    #[error("Request failed with status code: {}", .0)]
    HttpError(reqwest::StatusCode),
    #[error("Not logged in")]
    NotLoggedIn,
}


impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Error {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                Error::ReqwestError(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                Error::ReqwestMiddlewareError(e)
            },
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(_error: std::fmt::Error) -> Error {
        Error::ParameterError("Formatting error")
    }
}