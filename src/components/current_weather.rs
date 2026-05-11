use dioxus::prelude::*;

use crate::models::weather::CurrentWeather;
use crate::utils::weather_code::weather_label;

#[component]
pub fn CurrentWeatherCard(weather: CurrentWeather, unit: String) -> Element {
    rsx! {
        section { class: "rounded border p-4 shadow-sm",
            h2 { class: "text-xl font-semibold", "{weather.temperature}°{unit}" }
            p { class: "text-sm text-zinc-600", "{weather_label(weather.weather_code)}" }
            p { class: "text-sm", "Feels like {weather.apparent_temperature}°{unit}" }
            p { class: "text-sm", "Wind {weather.wind_speed} / Precip {weather.precipitation}" }
        }
    }
}
