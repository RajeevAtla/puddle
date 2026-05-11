use crate::error::WeatherError;
use crate::models::location::Location;
use crate::models::open_meteo_dto::{ForecastResponse, GeocodingResponse};
use crate::models::weather::{CurrentWeather, UnitSystem};

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

pub async fn fetch_current_weather(
    location: &Location,
    unit_system: UnitSystem,
) -> Result<CurrentWeather, WeatherError> {
    let (temperature_unit, wind_speed_unit, precipitation_unit) = match unit_system {
        UnitSystem::Imperial => ("fahrenheit", "mph", "inch"),
        UnitSystem::Metric => ("celsius", "kmh", "mm"),
    };
    let url = format!(
        "{FORECAST_URL}?latitude={}&longitude={}&timezone=auto&temperature_unit={temperature_unit}&wind_speed_unit={wind_speed_unit}&precipitation_unit={precipitation_unit}&current=temperature_2m,apparent_temperature,is_day,precipitation,weather_code,wind_speed_10m,wind_direction_10m",
        location.latitude, location.longitude
    );
    let payload: ForecastResponse = reqwest::get(url).await?.json().await?;
    Ok(payload.current.into())
}
