use dioxus::prelude::*;

use crate::models::location::Location;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn attribute(self) -> &'static str {
        match self {
            Self::Light => "daybook",
            Self::Dark => "nightwatch",
        }
    }

    fn value(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

#[component]
pub fn WeatherControls(
    unit: String,
    theme: ThemeMode,
    is_favorite: bool,
    #[props(default)] favorites: Vec<Location>,
    #[props(default)] on_unit_change: EventHandler<FormEvent>,
    #[props(default)] on_theme_change: EventHandler<FormEvent>,
    #[props(default)] on_favorite_toggle: EventHandler<MouseEvent>,
    #[props(default)] on_favorite_select: EventHandler<Location>,
) -> Element {
    let unit = if unit.eq_ignore_ascii_case("C") {
        "C"
    } else {
        "F"
    };
    let favorite_label = if is_favorite {
        "Saved to favorites"
    } else {
        "Save this station"
    };

    rsx! {
        section { class: "control-panel", aria_labelledby: "station-controls-heading",
            header { class: "section-heading",
                div {
                    p { class: "section-kicker", "Station controls" }
                    h2 { id: "station-controls-heading", class: "section-title", "Tune the instrument" }
                }
            }
            div { class: "control-grid",
                label { class: "control-field",
                    span { class: "field-label", "Temperature" }
                    select {
                        class: "station-select",
                        value: unit,
                        aria_label: "Temperature unit",
                        onchange: move |event| on_unit_change.call(event),
                        option { value: "F", "Fahrenheit (°F)" }
                        option { value: "C", "Celsius (°C)" }
                    }
                }
                label { class: "control-field",
                    span { class: "field-label", "Appearance" }
                    select {
                        class: "station-select",
                        value: theme.value(),
                        aria_label: "Color theme",
                        onchange: move |event| on_theme_change.call(event),
                        option { value: "light", "Daybook" }
                        option { value: "dark", "Nightwatch" }
                    }
                }
            }
            button {
                class: "favorite-toggle",
                r#type: "button",
                aria_pressed: is_favorite,
                onclick: move |event| on_favorite_toggle.call(event),
                span { class: "favorite-toggle-mark", aria_hidden: "true", if is_favorite { "Saved" } else { "Save" } }
                "{favorite_label}"
            }
            nav { class: "favorites-nav", aria_labelledby: "favorites-heading",
                h3 { id: "favorites-heading", class: "subsection-title", "Favorites" }
                if favorites.is_empty() {
                    p { class: "favorites-empty", "No saved stations yet." }
                } else {
                    ul { class: "favorites-list",
                        for location in favorites {
                            li {
                                button {
                                    class: "favorite-location",
                                    r#type: "button",
                                    onclick: move |_| on_favorite_select.call(location.clone()),
                                    span { "{location.name}" }
                                    span { class: "favorite-location-country", "{location.country}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
