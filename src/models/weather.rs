use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum UnitSystem {
    Imperial,
    Metric,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub weather_code: i32,
    pub is_day: i32,
    pub wind_speed: f64,
    pub wind_direction: i32,
    pub precipitation: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HourlyForecastItem {
    pub time: String,
    pub temperature: f64,
    pub relative_humidity: i32,
    pub apparent_temperature: f64,
    pub precipitation_probability: i32,
    pub precipitation: f64,
    pub weather_code: i32,
    pub is_day: i32,
    pub wind_speed: f64,
    pub wind_direction: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DailyForecastItem {
    pub date: String,
    pub weather_code: i32,
    pub temp_max: f64,
    pub temp_min: f64,
    pub precipitation_sum: f64,
    pub precipitation_probability_max: i32,
    pub wind_speed_max: f64,
    pub sunrise: String,
    pub sunset: String,
    pub daylight_duration: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub hourly: Vec<HourlyForecastItem>,
    pub daily: Vec<DailyForecastItem>,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i32,
}
