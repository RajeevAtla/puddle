use dioxus::prelude::*;

use crate::models::weather::CurrentWeather;
use crate::utils::weather_code::weather_label;

fn temperature_unit(unit: &str) -> &'static str {
    match unit.to_ascii_uppercase().as_str() {
        "C" | "CELSIUS" => "C",
        _ => "F",
    }
}

fn wind_unit(unit: &str) -> &'static str {
    match temperature_unit(unit) {
        "C" => "km/h",
        _ => "mph",
    }
}

fn compass_direction(degrees: i32) -> &'static str {
    match (degrees.rem_euclid(360) + 22) / 45 {
        0 => "N",
        1 => "NE",
        2 => "E",
        3 => "SE",
        4 => "S",
        5 => "SW",
        6 => "W",
        _ => "NW",
    }
}

#[component]
pub fn CurrentWeatherCard(weather: CurrentWeather, unit: String) -> Element {
    let unit = temperature_unit(&unit);
    let wind_unit = wind_unit(unit);
    let temperature = format!("{:.0}", weather.temperature);
    let apparent_temperature = format!("{:.0}", weather.apparent_temperature);
    let wind_speed = format!("{:.0}", weather.wind_speed);
    let precipitation_unit = if unit == "C" { "mm" } else { "in" };
    let precipitation = format!("{:.1} {precipitation_unit}", weather.precipitation);
    let condition = weather_label(weather.weather_code);
    let light = if weather.is_day == 1 {
        "Daylight"
    } else {
        "Night"
    };
    let wind_direction = compass_direction(weather.wind_direction);

    rsx! {
        section { class: "instrument-section reading-panel", aria_labelledby: "current-reading-heading",
            header { class: "section-heading reading-heading",
                div {
                    p { class: "section-kicker", "Current reading" }
                    h2 { id: "current-reading-heading", class: "section-title", "Atmosphere at a glance" }
                }
                time { class: "reading-time", datetime: weather.time.clone(), "{weather.time}" }
            }
            div { class: "reading-main",
                div { class: "instrument-mark", aria_hidden: "true",
                    span { class: "instrument-mark-ring" }
                    span { class: "instrument-mark-needle" }
                }
                div { class: "reading-value-block",
                    p { class: "reading-value", aria_label: "Current temperature {temperature} degrees {unit}",
                        strong { "{temperature}" }
                        span { "°{unit}" }
                    }
                    p { class: "reading-condition", "{condition}" }
                    p { class: "reading-light", "{light} observation" }
                }
            }
            dl { class: "reading-metrics",
                div { class: "reading-metric",
                    dt { "Feels like" }
                    dd { "{apparent_temperature}°{unit}" }
                }
                div { class: "reading-metric",
                    dt { "Wind" }
                    dd { "{wind_direction} {wind_speed} {wind_unit}" }
                }
                div { class: "reading-metric",
                    dt { "Precipitation" }
                    dd { "{precipitation}" }
                }
            }
        }
    }
}
