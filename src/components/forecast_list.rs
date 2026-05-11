use dioxus::prelude::*;

use crate::models::weather::DailyForecastItem;

#[component]
pub fn ForecastList(items: Vec<DailyForecastItem>, unit: String) -> Element {
    rsx! {
        section { class: "grid gap-2 md:grid-cols-3",
            for item in items {
                article { class: "rounded border p-3",
                    p { class: "font-medium", "{item.date}" }
                    p { class: "text-sm", "{item.temp_max}°{unit} / {item.temp_min}°{unit}" }
                    p { class: "text-sm text-zinc-600", "Precip {item.precipitation_probability_max}%" }
                }
            }
        }
    }
}
