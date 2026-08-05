pub fn weather_label(code: i32) -> &'static str {
    match code {
        0 => "Clear",
        1..=3 => "Cloudy",
        45 | 48 => "Fog",
        51..=67 => "Drizzle/Rain",
        71..=77 => "Snow",
        80..=82 => "Rain showers",
        85..=86 => "Snow showers",
        95..=99 => "Thunderstorm",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::weather_label;

    #[test]
    fn labels_cover_open_meteo_weather_groups() {
        assert_eq!(weather_label(0), "Clear");
        assert_eq!(weather_label(3), "Cloudy");
        assert_eq!(weather_label(45), "Fog");
        assert_eq!(weather_label(61), "Drizzle/Rain");
        assert_eq!(weather_label(75), "Snow");
        assert_eq!(weather_label(80), "Rain showers");
        assert_eq!(weather_label(85), "Snow showers");
        assert_eq!(weather_label(95), "Thunderstorm");
    }

    #[test]
    fn unknown_weather_codes_are_labeled_unknown() {
        assert_eq!(weather_label(-1), "Unknown");
        assert_eq!(weather_label(100), "Unknown");
    }
}
