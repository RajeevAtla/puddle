use crate::error::WeatherError;
use crate::models::location::Location;
use crate::models::open_meteo_dto::{ForecastResponse, GeocodingResponse};
use crate::models::weather::{UnitSystem, WeatherData};
use crate::services::cache;
use std::time::Duration;

const GEO_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";

pub async fn search_locations(query: &str) -> Result<Vec<Location>, WeatherError> {
    let url = format!("{GEO_URL}?name={query}&count=5&language=en&countryCode=US");
    let payload: GeocodingResponse = reqwest::get(url).await?.json().await?;
    let results = payload.results.unwrap_or_default();
    if results.is_empty() {
        return Err(WeatherError::EmptyResult);
    }
    Ok(results.into_iter().map(Into::into).collect())
}

pub async fn fetch_weather(
    location: &Location,
    unit_system: UnitSystem,
) -> Result<WeatherData, WeatherError> {
    let cache_key = format!(
        "forecast:{}:{}:{:?}",
        location.latitude, location.longitude, unit_system
    );
    if let Some(raw) = cache::get(&cache_key, Duration::from_secs(900)) {
        if let Ok(cached) = serde_json::from_str::<WeatherData>(&raw) {
            return Ok(cached);
        }
    }
    let (temperature_unit, wind_speed_unit, precipitation_unit) = match unit_system {
        UnitSystem::Imperial => ("fahrenheit", "mph", "inch"),
        UnitSystem::Metric => ("celsius", "kmh", "mm"),
    };
    let url = format!(
        "{FORECAST_URL}?latitude={}&longitude={}&timezone=auto&forecast_days=3&temperature_unit={temperature_unit}&wind_speed_unit={wind_speed_unit}&precipitation_unit={precipitation_unit}&current=temperature_2m,apparent_temperature,is_day,precipitation,weather_code,wind_speed_10m,wind_direction_10m&daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum,precipitation_probability_max,wind_speed_10m_max",
        location.latitude, location.longitude
    );
    let payload: ForecastResponse = reqwest::get(url).await?.json().await?;
    let data = WeatherData {
        current: payload.current.into(),
        daily: payload.daily.into_items(),
    };
    if let Ok(raw) = serde_json::to_string(&data) {
        cache::set(cache_key, raw);
    }
    Ok(data)
}
