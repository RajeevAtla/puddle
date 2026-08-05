use serde::Deserialize;

use crate::error::WeatherError;
use crate::models::{
    location::Location,
    weather::{CurrentWeather, DailyForecastItem, HourlyForecastItem, WeatherData},
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
    pub timezone: String,
    #[serde(default)]
    pub timezone_abbreviation: String,
    #[serde(default)]
    pub utc_offset_seconds: i32,
    pub current: ForecastCurrent,
    pub hourly: ForecastHourly,
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
pub struct ForecastHourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub relative_humidity_2m: Vec<i32>,
    pub apparent_temperature: Vec<f64>,
    pub precipitation_probability: Vec<i32>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<i32>,
    pub is_day: Vec<i32>,
    pub wind_speed_10m: Vec<f64>,
    pub wind_direction_10m: Vec<i32>,
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
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub daylight_duration: Vec<f64>,
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
    pub fn into_items(self) -> Result<Vec<DailyForecastItem>, WeatherError> {
        let length = parallel_array_length(
            "daily",
            &[
                ("time", self.time.len()),
                ("weather_code", self.weather_code.len()),
                ("temperature_2m_max", self.temperature_2m_max.len()),
                ("temperature_2m_min", self.temperature_2m_min.len()),
                ("precipitation_sum", self.precipitation_sum.len()),
                (
                    "precipitation_probability_max",
                    self.precipitation_probability_max.len(),
                ),
                ("wind_speed_10m_max", self.wind_speed_10m_max.len()),
                ("sunrise", self.sunrise.len()),
                ("sunset", self.sunset.len()),
                ("daylight_duration", self.daylight_duration.len()),
            ],
        )?;

        let Self {
            time,
            weather_code,
            temperature_2m_max,
            temperature_2m_min,
            precipitation_sum,
            precipitation_probability_max,
            wind_speed_10m_max,
            sunrise,
            sunset,
            daylight_duration,
        } = self;

        Ok((0..length)
            .map(|index| DailyForecastItem {
                date: time[index].clone(),
                weather_code: weather_code[index],
                temp_max: temperature_2m_max[index],
                temp_min: temperature_2m_min[index],
                precipitation_sum: precipitation_sum[index],
                precipitation_probability_max: precipitation_probability_max[index],
                wind_speed_max: wind_speed_10m_max[index],
                sunrise: sunrise[index].clone(),
                sunset: sunset[index].clone(),
                daylight_duration: daylight_duration[index],
            })
            .collect())
    }
}

impl ForecastHourly {
    pub fn into_items(self) -> Result<Vec<HourlyForecastItem>, WeatherError> {
        let length = parallel_array_length(
            "hourly",
            &[
                ("time", self.time.len()),
                ("temperature_2m", self.temperature_2m.len()),
                ("relative_humidity_2m", self.relative_humidity_2m.len()),
                ("apparent_temperature", self.apparent_temperature.len()),
                (
                    "precipitation_probability",
                    self.precipitation_probability.len(),
                ),
                ("precipitation", self.precipitation.len()),
                ("weather_code", self.weather_code.len()),
                ("is_day", self.is_day.len()),
                ("wind_speed_10m", self.wind_speed_10m.len()),
                ("wind_direction_10m", self.wind_direction_10m.len()),
            ],
        )?;

        let Self {
            time,
            temperature_2m,
            relative_humidity_2m,
            apparent_temperature,
            precipitation_probability,
            precipitation,
            weather_code,
            is_day,
            wind_speed_10m,
            wind_direction_10m,
        } = self;

        Ok((0..length)
            .map(|index| HourlyForecastItem {
                time: time[index].clone(),
                temperature: temperature_2m[index],
                relative_humidity: relative_humidity_2m[index],
                apparent_temperature: apparent_temperature[index],
                precipitation_probability: precipitation_probability[index],
                precipitation: precipitation[index],
                weather_code: weather_code[index],
                is_day: is_day[index],
                wind_speed: wind_speed_10m[index],
                wind_direction: wind_direction_10m[index],
            })
            .collect())
    }
}

impl TryFrom<ForecastResponse> for WeatherData {
    type Error = WeatherError;

    fn try_from(value: ForecastResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            current: value.current.into(),
            hourly: value.hourly.into_items()?,
            daily: value.daily.into_items()?,
            timezone: value.timezone,
            timezone_abbreviation: value.timezone_abbreviation,
            utc_offset_seconds: value.utc_offset_seconds,
        })
    }
}

fn parallel_array_length(
    dataset: &'static str,
    fields: &[(&'static str, usize)],
) -> Result<usize, WeatherError> {
    let expected = fields[0].1;
    for &(field, actual) in &fields[1..] {
        if actual != expected {
            return Err(WeatherError::MismatchedArrayLengths {
                dataset,
                field,
                expected,
                actual,
            });
        }
    }
    Ok(expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hourly() -> ForecastHourly {
        ForecastHourly {
            time: vec!["2026-08-04T12:00".to_string()],
            temperature_2m: vec![21.5],
            relative_humidity_2m: vec![62],
            apparent_temperature: vec![21.0],
            precipitation_probability: vec![10],
            precipitation: vec![0.0],
            weather_code: vec![1],
            is_day: vec![1],
            wind_speed_10m: vec![12.0],
            wind_direction_10m: vec![180],
        }
    }

    fn daily() -> ForecastDaily {
        ForecastDaily {
            time: vec!["2026-08-04".to_string()],
            weather_code: vec![1],
            temperature_2m_max: vec![25.0],
            temperature_2m_min: vec![15.0],
            precipitation_sum: vec![0.2],
            precipitation_probability_max: vec![20],
            wind_speed_10m_max: vec![18.0],
            sunrise: vec!["2026-08-04T05:30".to_string()],
            sunset: vec!["2026-08-04T20:30".to_string()],
            daylight_duration: vec![54000.0],
        }
    }

    #[test]
    fn dto_conversion_preserves_forecast_data() {
        let response = ForecastResponse {
            timezone: "Europe/Paris".to_string(),
            timezone_abbreviation: "CEST".to_string(),
            utc_offset_seconds: 7200,
            current: ForecastCurrent {
                time: "2026-08-04T12:00".to_string(),
                temperature_2m: 21.5,
                apparent_temperature: 21.0,
                weather_code: 1,
                is_day: 1,
                wind_speed_10m: 12.0,
                wind_direction_10m: 180,
                precipitation: 0.0,
            },
            hourly: hourly(),
            daily: daily(),
        };

        let weather = WeatherData::try_from(response).expect("valid forecast");
        assert_eq!(weather.timezone, "Europe/Paris");
        assert_eq!(weather.hourly.len(), 1);
        assert_eq!(weather.hourly[0].relative_humidity, 62);
        assert_eq!(weather.daily.len(), 1);
        assert_eq!(weather.daily[0].sunrise, "2026-08-04T05:30");
    }

    #[test]
    fn daily_conversion_rejects_mismatched_parallel_arrays() {
        let mut forecast = daily();
        forecast.sunset.clear();

        let error = forecast.into_items().expect_err("mismatched arrays");
        assert!(matches!(
            error,
            WeatherError::MismatchedArrayLengths {
                dataset: "daily",
                field: "sunset",
                expected: 1,
                actual: 0,
            }
        ));
    }

    #[test]
    fn hourly_conversion_rejects_mismatched_parallel_arrays() {
        let mut forecast = hourly();
        forecast.wind_direction_10m.clear();

        let error = forecast.into_items().expect_err("mismatched arrays");
        assert!(matches!(
            error,
            WeatherError::MismatchedArrayLengths {
                dataset: "hourly",
                field: "wind_direction_10m",
                expected: 1,
                actual: 0,
            }
        ));
    }
}
