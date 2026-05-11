use dioxus::prelude::*;

#[component]
pub fn StatusBanner(message: String, error: bool) -> Element {
    let style = if error {
        "rounded border border-red-300 bg-red-50 p-3 text-red-700"
    } else {
        "rounded border border-sky-300 bg-sky-50 p-3 text-sky-700"
    };
    rsx! { div { class: style, aria_live: "polite", "{message}" } }
}
