use dioxus::prelude::*;

use crate::models::weather::DailyForecastItem;
use crate::utils::weather_code::weather_label;

fn temperature_unit(unit: &str) -> &'static str {
    match unit.to_ascii_uppercase().as_str() {
        "C" | "CELSIUS" => "C",
        _ => "F",
    }
}

fn day_label(date: &str, index: usize) -> String {
    if index == 0 {
        return "Today".to_string();
    }

    date.split('-')
        .nth(2)
        .map(|day| format!("Day {day}"))
        .unwrap_or_else(|| date.to_string())
}

#[component]
pub fn ForecastList(items: Vec<DailyForecastItem>, unit: String) -> Element {
    rsx! { SevenDayHorizon { items, unit } }
}

#[component]
pub fn SevenDayHorizon(items: Vec<DailyForecastItem>, unit: String) -> Element {
    let unit = temperature_unit(&unit);
    let count = items.len().min(7);

    rsx! {
        section { class: "instrument-section horizon-panel", aria_labelledby: "seven-day-heading",
            header { class: "section-heading",
                div {
                    p { class: "section-kicker", "Outlook" }
                    h2 { id: "seven-day-heading", class: "section-title", "Seven-day horizon" }
                }
                p { class: "section-meta", "{count} observations" }
            }
            if items.is_empty() {
                div { class: "section-empty", role: "status",
                    p { class: "empty-title", "The horizon is blank" }
                    p { "A daily outlook will appear when the station returns a forecast." }
                }
            } else {
                ol { class: "horizon-list",
                    for (index, item) in items.into_iter().take(7).enumerate() {
                        li { class: "horizon-row",
                            span { class: "horizon-day", "{day_label(&item.date, index)}" }
                            span { class: "horizon-condition", "{weather_label(item.weather_code)}" }
                            span { class: "horizon-temperature", "{item.temp_max:.0}° / {item.temp_min:.0}°{unit}" }
                            span {
                                class: "horizon-rain",
                                aria_label: format!(
                                    "{} percent precipitation chance",
                                    item.precipitation_probability_max
                                ),
                                "{item.precipitation_probability_max}% rain"
                            }
                        }
                    }
                }
            }
        }
    }
}
