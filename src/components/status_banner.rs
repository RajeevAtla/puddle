use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WeatherState {
    Loading,
    Empty,
    Error,
}

#[component]
pub fn StatusBanner(message: String, error: bool) -> Element {
    let class = if error {
        "status-panel status-panel-error"
    } else {
        "status-panel status-panel-neutral"
    };
    let role = if error { "alert" } else { "status" };

    rsx! {
        aside { class, role, aria_live: "polite",
            p { class: "status-kicker", if error { "Station notice" } else { "Station note" } }
            p { class: "status-message", "{message}" }
        }
    }
}

#[component]
pub fn WeatherStatePanel(
    state: WeatherState,
    #[props(default)] message: String,
    #[props(default)] on_retry: EventHandler<MouseEvent>,
) -> Element {
    let (heading, fallback, class, role) = match state {
        WeatherState::Loading => (
            "Reading the atmosphere",
            "The instrument is waiting for a fresh observation.",
            "status-panel status-panel-loading",
            "status",
        ),
        WeatherState::Empty => (
            "No station selected",
            "Search for a place to begin a new weather log.",
            "status-panel status-panel-neutral",
            "status",
        ),
        WeatherState::Error => (
            "The station went quiet",
            "We could not load this observation right now.",
            "status-panel status-panel-error",
            "alert",
        ),
    };
    let message = if message.is_empty() {
        fallback
    } else {
        &message
    };

    rsx! {
        aside {
            class,
            role,
            aria_live: "polite",
            aria_busy: state == WeatherState::Loading,
            div { class: "status-panel-marker", aria_hidden: "true" }
            div { class: "status-panel-copy",
                p { class: "status-kicker", "Instrument status" }
                h2 { class: "status-heading", "{heading}" }
                p { class: "status-message", "{message}" }
            }
            if state == WeatherState::Error {
                button {
                    class: "station-button station-button-secondary",
                    r#type: "button",
                    onclick: move |event| on_retry.call(event),
                    "Try again"
                }
            }
        }
    }
}
