use dioxus::prelude::*;

use crate::components::controls::{ThemeMode, WeatherControls};
use crate::components::current_weather::CurrentWeatherCard;
use crate::components::forecast_list::SevenDayHorizon;
use crate::components::search_bar::SearchBar;
use crate::components::status_banner::{WeatherState, WeatherStatePanel};
use crate::components::weather_trace::WeatherTrace;
use crate::models::location::Location;
use crate::models::weather::WeatherData;

fn location_coordinates(location: &Location) -> String {
    format!("{:.2}°, {:.2}°", location.latitude, location.longitude)
}

#[component]
pub fn WeatherInstrument(
    location: Location,
    weather: WeatherData,
    #[props(default)] unit: String,
    #[props(default)] theme: ThemeMode,
    #[props(default)] is_favorite: bool,
    #[props(default)] favorites: Vec<Location>,
    #[props(default)] search_query: String,
    #[props(default)] search_results: Vec<Location>,
    #[props(default)] search_loading: bool,
    #[props(default)] search_empty: bool,
    #[props(default)] search_error: Option<String>,
    #[props(default)] state: Option<WeatherState>,
    #[props(default)] state_message: String,
    #[props(default)] on_search_input: EventHandler<FormEvent>,
    #[props(default)] on_search_submit: EventHandler<FormEvent>,
    #[props(default)] on_location_select: EventHandler<Location>,
    #[props(default)] on_unit_change: EventHandler<FormEvent>,
    #[props(default)] on_theme_change: EventHandler<FormEvent>,
    #[props(default)] on_favorite_toggle: EventHandler<MouseEvent>,
    #[props(default)] on_favorite_select: EventHandler<Location>,
    #[props(default)] on_retry: EventHandler<MouseEvent>,
) -> Element {
    let unit = if unit.is_empty() {
        "F".to_string()
    } else {
        unit
    };

    rsx! {
        a { class: "skip-link", href: "#weather-main", "Skip to readings" }
        div { class: "weather-instrument", "data-theme": theme.attribute(),
            header { class: "weather-mast",
                div { class: "mast-identity",
                    p { class: "mast-overline", "Puddle / field station" }
                    p { class: "mast-title", "Atmospheric instrument" }
                }
                SearchBar {
                    query: search_query,
                    on_input: on_search_input,
                    on_submit: on_search_submit,
                    results: search_results,
                    is_loading: search_loading,
                    empty: search_empty,
                    error: search_error,
                    on_select: on_location_select,
                }
            }
            div { class: "weather-frame",
                aside { class: "station-rail", aria_labelledby: "station-heading",
                    div { class: "station-rail-top",
                        p { class: "section-kicker", "Observed station" }
                        h1 { id: "station-heading", class: "station-name", "{location.name}" }
                        p { class: "station-region", "{location.admin1.as_deref().unwrap_or(\"\")}" }
                        p { class: "station-country", "{location.country}" }
                    }
                    div { class: "station-coordinate-block",
                        p { class: "field-label", "Coordinates" }
                        p { class: "station-coordinates", "{location_coordinates(&location)}" }
                        p { class: "station-timezone", "{location.timezone}" }
                    }
                    WeatherControls {
                        unit: unit.clone(),
                        theme,
                        is_favorite,
                        favorites: favorites.clone(),
                        on_unit_change,
                        on_theme_change,
                        on_favorite_toggle,
                        on_favorite_select,
                    }
                }
                main { id: "weather-main", class: "weather-main",
                    if let Some(state) = state {
                        WeatherStatePanel {
                            state,
                            message: state_message,
                            on_retry,
                        }
                    } else {
                        CurrentWeatherCard { weather: weather.current, unit: unit.clone() }
                        div { class: "weather-content-grid",
                            WeatherTrace { readings: weather.hourly, unit: unit.clone() }
                            SevenDayHorizon { items: weather.daily, unit: unit.clone() }
                        }
                    }
                }
            }
            footer { class: "weather-footer",
                "Readings are presented in local station time."
            }
        }
    }
}
