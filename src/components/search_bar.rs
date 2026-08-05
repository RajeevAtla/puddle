use dioxus::prelude::*;

use crate::models::location::Location;

#[component]
pub fn SearchBar(
    query: String,
    on_input: EventHandler<FormEvent>,
    on_submit: EventHandler<FormEvent>,
    #[props(default)] results: Vec<Location>,
    #[props(default)] is_loading: bool,
    #[props(default)] empty: bool,
    #[props(default)] error: Option<String>,
    #[props(default)] on_select: EventHandler<Location>,
) -> Element {
    rsx! {
        section { class: "search-station", aria_labelledby: "weather-search-label",
            form {
                class: "search-form",
                role: "search",
                onsubmit: move |event| {
                    event.prevent_default();
                    on_submit.call(event);
                },
                label { id: "weather-search-label", class: "field-label", r#for: "weather-location-search",
                    "Find a station"
                }
                div { class: "search-control",
                    input {
                        id: "weather-location-search",
                        class: "station-input",
                        r#type: "search",
                        name: "location",
                        value: query,
                        oninput: move |event| on_input.call(event),
                        placeholder: "City, region, or country",
                        autocomplete: "off",
                        spellcheck: "false",
                        aria_describedby: "weather-search-hint",
                    }
                    button {
                        class: "station-button station-button-primary",
                        r#type: "submit",
                        "Measure"
                    }
                }
                p { id: "weather-search-hint", class: "field-hint",
                    "Search worldwide by city, region, or country."
                }
            }
            SearchResults {
                results,
                is_loading,
                empty,
                error,
                on_select,
            }
        }
    }
}

#[component]
pub fn SearchResults(
    results: Vec<Location>,
    is_loading: bool,
    empty: bool,
    error: Option<String>,
    on_select: EventHandler<Location>,
) -> Element {
    rsx! {
        div { class: "search-results", aria_live: "polite", aria_atomic: "true",
            if is_loading {
                div { class: "search-result-status", role: "status",
                    span { class: "loading-dot", aria_hidden: "true" }
                    "Looking across the world..."
                }
            } else if let Some(error) = error {
                div { class: "search-result-status search-result-status-error", role: "alert",
                    "{error}"
                }
            } else if empty {
                div { class: "search-result-status", role: "status",
                    "No matching station found. Try a larger place name."
                }
            } else if !results.is_empty() {
                ul { class: "search-result-list", aria_label: "Location results",
                    for location in results {
                        li { class: "search-result-item",
                            button {
                                class: "search-result-button",
                                r#type: "button",
                                aria_label: format!(
                                    "Select {}, {}",
                                    location.name,
                                    location.country
                                ),
                                onclick: move |_| on_select.call(location.clone()),
                                span { class: "search-result-name", "{location.name}" }
                                span { class: "search-result-detail",
                                    if let Some(admin1) = &location.admin1 {
                                        "{admin1}, "
                                    }
                                    "{location.country}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
