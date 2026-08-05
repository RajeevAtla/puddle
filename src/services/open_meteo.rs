use crate::error::WeatherError;
use crate::models::location::Location;
use crate::models::open_meteo_dto::{ForecastResponse, GeocodingResponse};
use crate::models::weather::{UnitSystem, WeatherData};
use crate::services::cache;
use std::time::Duration;

const GEO_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
pub const HOURLY_FORECAST_HOURS: usize = 24;
pub const DAILY_FORECAST_DAYS: usize = 7;

pub async fn search_locations(query: &str) -> Result<Vec<Location>, WeatherError> {
    let query = query.trim();
    if query.is_empty() {
        return Err(WeatherError::EmptyQuery);
    }

    let payload: GeocodingResponse = reqwest::get(build_geocoding_url(query))
        .await?
        .error_for_status()?
        .json()
        .await?;
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
    let payload: ForecastResponse = reqwest::get(build_forecast_url(location, unit_system))
        .await?
        .error_for_status()?
        .json()
        .await?;
    let data = WeatherData::try_from(payload)?;
    if let Ok(raw) = serde_json::to_string(&data) {
        cache::set(cache_key, raw);
    }
    Ok(data)
}

pub fn build_geocoding_url(query: &str) -> reqwest::Url {
    let mut url = reqwest::Url::parse(GEO_URL).expect("valid Open-Meteo geocoding URL");
    url.query_pairs_mut()
        .append_pair("name", query)
        .append_pair("count", "5")
        .append_pair("language", "en");
    url
}

pub fn build_forecast_url(location: &Location, unit_system: UnitSystem) -> reqwest::Url {
    let (temperature_unit, wind_speed_unit, precipitation_unit) = match unit_system {
        UnitSystem::Imperial => ("fahrenheit", "mph", "inch"),
        UnitSystem::Metric => ("celsius", "kmh", "mm"),
    };
    let mut url = reqwest::Url::parse(FORECAST_URL).expect("valid Open-Meteo forecast URL");
    url.query_pairs_mut()
        .append_pair("latitude", &location.latitude.to_string())
        .append_pair("longitude", &location.longitude.to_string())
        .append_pair("timezone", "auto")
        .append_pair("forecast_hours", &HOURLY_FORECAST_HOURS.to_string())
        .append_pair("forecast_days", &DAILY_FORECAST_DAYS.to_string())
        .append_pair("temperature_unit", temperature_unit)
        .append_pair("wind_speed_unit", wind_speed_unit)
        .append_pair("precipitation_unit", precipitation_unit)
        .append_pair(
            "current",
            "temperature_2m,apparent_temperature,is_day,precipitation,weather_code,wind_speed_10m,wind_direction_10m",
        )
        .append_pair(
            "hourly",
            "temperature_2m,relative_humidity_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,is_day,wind_speed_10m,wind_direction_10m",
        )
        .append_pair(
            "daily",
            "weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum,precipitation_probability_max,wind_speed_10m_max,sunrise,sunset,daylight_duration",
        );
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    fn location() -> Location {
        Location {
            id: 1,
            name: "Paris".to_string(),
            admin1: Some("Ile-de-France".to_string()),
            country: "France".to_string(),
            latitude: 48.8566,
            longitude: 2.3522,
            timezone: "Europe/Paris".to_string(),
        }
    }

    fn query_value(url: &reqwest::Url, key: &str) -> Option<String> {
        url.query_pairs()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.into_owned())
    }

    #[test]
    fn geocoding_query_is_encoded_and_global() {
        let url = build_geocoding_url("München & 東京");

        assert_eq!(query_value(&url, "name").as_deref(), Some("München & 東京"));
        assert_eq!(query_value(&url, "count").as_deref(), Some("5"));
        assert!(query_value(&url, "countryCode").is_none());
        assert!(url.as_str().contains("%26"));
    }

    #[test]
    fn forecast_query_contains_requested_counts_and_units() {
        let url = build_forecast_url(&location(), UnitSystem::Metric);

        assert_eq!(query_value(&url, "timezone").as_deref(), Some("auto"));
        assert_eq!(query_value(&url, "forecast_hours").as_deref(), Some("24"));
        assert_eq!(query_value(&url, "forecast_days").as_deref(), Some("7"));
        assert_eq!(
            query_value(&url, "temperature_unit").as_deref(),
            Some("celsius")
        );
        assert_eq!(query_value(&url, "wind_speed_unit").as_deref(), Some("kmh"));
        assert_eq!(
            query_value(&url, "precipitation_unit").as_deref(),
            Some("mm")
        );
        assert!(query_value(&url, "sunrise").is_none());
        assert!(query_value(&url, "daily")
            .expect("daily variables")
            .contains("sunrise"));
    }

    #[test]
    fn forecast_query_uses_imperial_units() {
        let url = build_forecast_url(&location(), UnitSystem::Imperial);

        assert_eq!(
            query_value(&url, "temperature_unit").as_deref(),
            Some("fahrenheit")
        );
        assert_eq!(query_value(&url, "wind_speed_unit").as_deref(), Some("mph"));
        assert_eq!(
            query_value(&url, "precipitation_unit").as_deref(),
            Some("inch")
        );
    }
}
