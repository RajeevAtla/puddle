use dioxus::prelude::*;

use crate::models::weather::HourlyForecastItem;
use crate::utils::weather_code::weather_label;

fn temperature_unit(unit: &str) -> &'static str {
    match unit.to_ascii_uppercase().as_str() {
        "C" | "CELSIUS" => "C",
        _ => "F",
    }
}

fn trace_coordinates(readings: &[HourlyForecastItem], index: usize) -> (f64, f64) {
    let minimum = readings
        .iter()
        .map(|reading| reading.temperature)
        .fold(f64::INFINITY, f64::min);
    let maximum = readings
        .iter()
        .map(|reading| reading.temperature)
        .fold(f64::NEG_INFINITY, f64::max);
    let span = (maximum - minimum).max(1.0);
    let denominator = readings.len().saturating_sub(1).max(1) as f64;
    let reading = &readings[index];
    let x = index as f64 / denominator * 100.0;
    let y = 88.0 - ((reading.temperature - minimum) / span * 72.0);

    (x, y)
}

fn trace_points(readings: &[HourlyForecastItem]) -> String {
    readings
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let (x, y) = trace_coordinates(readings, index);
            format!("{x:.2},{y:.2}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn hour_x(readings: &[HourlyForecastItem], index: usize) -> f64 {
    let denominator = readings.len().saturating_sub(1).max(1) as f64;
    index as f64 / denominator * 100.0
}

#[component]
pub fn WeatherTrace(readings: Vec<HourlyForecastItem>, unit: String) -> Element {
    let readings: Vec<_> = readings.into_iter().take(24).collect();
    let unit = temperature_unit(&unit);
    let points = trace_points(&readings);
    let count = readings.len();

    rsx! {
        section { class: "instrument-section trace-panel", aria_labelledby: "trace-heading",
            header { class: "section-heading",
                div {
                    p { class: "section-kicker", "Instrument trace" }
                    h2 { id: "trace-heading", class: "section-title", "Next 24 hours" }
                }
                p { class: "section-meta", "{count} hourly points" }
            }
            if readings.is_empty() {
                div { class: "section-empty trace-empty", role: "status",
                    div { class: "trace-empty-line", aria_hidden: "true" }
                    p { class: "empty-title", "The trace has not started" }
                    p { "Hourly observations will be plotted here when available." }
                }
            } else {
                div { class: "trace-plot-wrap",
                    svg {
                        class: "trace-plot",
                        view_box: "0 0 100 100",
                        role: "img",
                        "aria-label": "Temperature trace for the next 24 hours",
                        preserve_aspect_ratio: "none",
                        title { "Temperature trace for the next 24 hours" }
                        for (index, reading) in readings.iter().enumerate() {
                            rect {
                                class: if reading.is_day == 1 { "trace-daylight" } else { "trace-night" },
                                x: format!("{:.2}", hour_x(&readings, index)),
                                y: "0",
                                width: format!("{:.2}", 100.0 / readings.len().max(1) as f64),
                                height: "100",
                            }
                        }
                        line { class: "trace-grid-line", x1: "0", y1: "16", x2: "100", y2: "16" }
                        line { class: "trace-grid-line", x1: "0", y1: "52", x2: "100", y2: "52" }
                        line { class: "trace-grid-line", x1: "0", y1: "88", x2: "100", y2: "88" }
                        polyline { class: "trace-line", points, fill: "none" }
                        for (index, reading) in readings.iter().enumerate() {
                            if reading.precipitation_probability > 0 {
                                line {
                                    class: "trace-rain-pulse",
                                    x1: format!("{:.2}", hour_x(&readings, index)),
                                    x2: format!("{:.2}", hour_x(&readings, index)),
                                    y1: "94",
                                    y2: format!("{:.2}", 94.0 - reading.precipitation_probability as f64 * 0.24),
                                }
                            }
                            line {
                                class: "trace-wind",
                                x1: format!("{:.2}", hour_x(&readings, index)),
                                x2: format!("{:.2}", hour_x(&readings, index)),
                                y1: "7",
                                y2: format!("{:.2}", 7.0 + reading.wind_speed.min(20.0) * 0.25),
                                transform: format!("rotate({} {:.2} 7)", reading.wind_direction, hour_x(&readings, index)),
                            }
                            circle {
                                class: "trace-point",
                                cx: format!("{:.2}", hour_x(&readings, index)),
                                cy: format!("{:.2}", trace_coordinates(&readings, index).1),
                                r: "1.8",
                            }
                        }
                    }
                    div { class: "trace-axis", aria_hidden: "true",
                        span { "{readings.first().unwrap().time}" }
                        span { "{readings.last().unwrap().time}" }
                    }
                }
                details { class: "trace-alternative",
                    summary { "Read the trace as a table" }
                    div { class: "trace-table-wrap",
                        table {
                            caption { "Hourly weather trace" }
                            thead {
                                tr {
                                    th { scope: "col", "Time" }
                                    th { scope: "col", "Temperature" }
                                    th { scope: "col", "Condition" }
                                    th { scope: "col", "Rain chance" }
                                }
                            }
                            tbody {
                                for reading in readings {
                                    tr {
                                        th { scope: "row", "{reading.time}" }
                                        td { "{reading.temperature:.0}°{unit}" }
                                        td { "{weather_label(reading.weather_code)}" }
                                        td { "{reading.precipitation_probability}%" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
