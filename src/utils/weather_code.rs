pub fn weather_label(code: i32) -> &'static str {
    match code {
        0 => "Clear",
        1..=3 => "Cloudy",
        45 | 48 => "Fog",
        51..=67 => "Drizzle/Rain",
        71..=77 => "Snow",
        80..=82 => "Rain showers",
        95..=99 => "Thunderstorm",
        _ => "Unknown",
    }
}
