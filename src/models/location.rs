use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub admin1: Option<String>,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}
