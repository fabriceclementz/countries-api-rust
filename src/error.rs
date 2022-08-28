use thiserror::Error;

#[derive(Error, Debug)]
pub enum CountryError {
    #[error("Error loading csv data")]
    LoadingData(#[from] csv::Error),
    #[error("Error serializing data")]
    Serialize(#[from] serde_json::Error),
    #[error("Error HTTP")]
    Http(#[from] hyper::http::Error),
    #[error("Country not found")]
    CountryNotFound,
    #[error("Country not found")]
    Infallible(#[from] std::convert::Infallible),
}
