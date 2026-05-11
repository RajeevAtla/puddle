use dioxus::prelude::*;

#[component]
pub fn SearchBar(
    query: String,
    on_input: EventHandler<FormEvent>,
    on_submit: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex gap-2",
            input {
                class: "w-full rounded border px-3 py-2",
                value: query,
                oninput: move |e| on_input.call(e),
                placeholder: "Search city",
            }
            button {
                class: "rounded bg-sky-600 px-4 py-2 text-white",
                onclick: move |e| on_submit.call(e),
                "Search"
            }
        }
    }
}
