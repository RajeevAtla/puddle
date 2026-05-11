use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Network request failed")]
    Network(#[from] reqwest::Error),
    #[error("No matching location found")]
    EmptyResult,
}
