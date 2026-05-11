use dioxus::prelude::*;

use crate::components::current_weather::CurrentWeatherCard;
use crate::components::forecast_list::ForecastList;
use crate::components::search_bar::SearchBar;
use crate::components::status_banner::StatusBanner;
use crate::models::location::Location;
use crate::models::weather::UnitSystem;
use crate::services::open_meteo::{fetch_weather, search_locations};

#[component]
pub fn WeatherPage() -> Element {
    let mut query = use_signal(String::new);
    let mut selected = use_signal(|| None::<Location>);
    let units = use_signal(|| UnitSystem::Imperial);

    let weather = use_resource(move || {
        let location = selected();
        let unit_system = units();
        async move {
            match location {
                Some(item) => fetch_weather(&item, unit_system).await.ok(),
                None => None,
            }
        }
    });

    let on_submit = move |_| {
        let city = query();
        spawn(async move {
            if let Ok(locations) = search_locations(&city).await {
                if let Some(found) = locations.into_iter().next() {
                    selected.set(Some(found));
                }
            }
        });
    };

    rsx! {
        main { class: "mx-auto max-w-3xl space-y-4 p-4",
            h1 { class: "text-2xl font-bold", "Weather" }
            SearchBar {
                query: query(),
                on_input: move |e: FormEvent| query.set(e.value()),
                on_submit,
            }
            {
                match selected() {
                    Some(location) => rsx! { p { class: "text-sm text-zinc-600", "{location.name}, {location.country}" } },
                    None => rsx! { StatusBanner { message: "Search for a US city to begin".to_string(), error: false } },
                }
            }
            {
                match weather() {
                    Some(Some(data)) => rsx! {
                        CurrentWeatherCard { weather: data.current, unit: "F".to_string() }
                        ForecastList { items: data.daily, unit: "F".to_string() }
                    },
                    None => rsx! { StatusBanner { message: "Loading weather...".to_string(), error: false } },
                    Some(None) => rsx! { StatusBanner { message: "No weather data available".to_string(), error: true } },
                }
            }
        }
    }
}
