#[derive(Debug, Clone, PartialEq)]
pub enum UnitSystem {
    Imperial,
    Metric,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct DailyForecastItem {
    pub date: String,
    pub weather_code: i32,
    pub temp_max: f64,
    pub temp_min: f64,
    pub precipitation_sum: f64,
    pub precipitation_probability_max: i32,
    pub wind_speed_max: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub daily: Vec<DailyForecastItem>,
}
