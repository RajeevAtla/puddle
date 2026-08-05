use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Network request failed")]
    Network(#[from] reqwest::Error),
    #[error("Search query cannot be empty")]
    EmptyQuery,
    #[error("No matching location found")]
    EmptyResult,
    #[error(
        "Malformed {dataset} forecast: field '{field}' has length {actual}, expected {expected}"
    )]
    MismatchedArrayLengths {
        dataset: &'static str,
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}
