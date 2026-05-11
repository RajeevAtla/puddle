use serde::Deserialize;

use crate::models::{
    location::Location,
    weather::{CurrentWeather, DailyForecastItem},
};

#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
}

#[derive(Debug, Deserialize)]
pub struct GeocodingResult {
    pub id: i64,
    pub name: String,
    pub admin1: Option<String>,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

impl From<GeocodingResult> for Location {
    fn from(value: GeocodingResult) -> Self {
        Self {
            id: value.id,
            name: value.name,
            admin1: value.admin1,
            country: value.country,
            latitude: value.latitude,
            longitude: value.longitude,
            timezone: value.timezone,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub current: ForecastCurrent,
    pub daily: ForecastDaily,
}

#[derive(Debug, Deserialize)]
pub struct ForecastCurrent {
    pub time: String,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub weather_code: i32,
    pub is_day: i32,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: i32,
    pub precipitation: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastDaily {
    pub time: Vec<String>,
    pub weather_code: Vec<i32>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_probability_max: Vec<i32>,
    pub wind_speed_10m_max: Vec<f64>,
}

impl From<ForecastCurrent> for CurrentWeather {
    fn from(value: ForecastCurrent) -> Self {
        Self {
            time: value.time,
            temperature: value.temperature_2m,
            apparent_temperature: value.apparent_temperature,
            weather_code: value.weather_code,
            is_day: value.is_day,
            wind_speed: value.wind_speed_10m,
            wind_direction: value.wind_direction_10m,
            precipitation: value.precipitation,
        }
    }
}

impl ForecastDaily {
    pub fn into_items(self) -> Vec<DailyForecastItem> {
        self.time
            .into_iter()
            .zip(self.weather_code)
            .zip(self.temperature_2m_max)
            .zip(self.temperature_2m_min)
            .zip(self.precipitation_sum)
            .zip(self.precipitation_probability_max)
            .zip(self.wind_speed_10m_max)
            .map(|((((((date, weather_code), temp_max), temp_min), precipitation_sum), precipitation_probability_max), wind_speed_max)| DailyForecastItem {
                date,
                weather_code,
                temp_max,
                temp_min,
                precipitation_sum,
                precipitation_probability_max,
                wind_speed_max,
            })
            .collect()
    }
}
