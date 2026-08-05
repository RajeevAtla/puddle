use dioxus::prelude::*;

use crate::browser;
use crate::components::controls::ThemeMode;
use crate::components::status_banner::WeatherState;
use crate::components::weather_instrument::WeatherInstrument;
use crate::error::WeatherError;
use crate::models::location::Location;
use crate::models::weather::{CurrentWeather, UnitSystem, WeatherData};
use crate::services::open_meteo::{fetch_weather, search_locations};

#[component]
pub fn WeatherPage() -> Element {
    let initial_location = browser::location_from_url();
    let initial_query = initial_location
        .as_ref()
        .map(|location| location.name.clone())
        .unwrap_or_default();

    let mut query = use_signal(move || initial_query);
    let selected = use_signal(move || initial_location);
    let mut units = use_signal(|| UnitSystem::Imperial);
    let mut theme = use_signal(browser::load_theme);
    let mut favorites = use_signal(browser::load_favorites);
    let mut search_results = use_signal(Vec::<Location>::new);
    let mut search_loading = use_signal(|| false);
    let mut search_empty = use_signal(|| false);
    let mut search_error = use_signal(|| None::<String>);
    let mut search_generation = use_signal(|| 0_u64);
    let mut weather_retry = use_signal(|| 0_u64);

    let weather = use_resource(move || {
        let location = selected();
        let unit_system = units();
        let _retry = weather_retry();

        async move {
            match location {
                Some(location) => fetch_weather(&location, unit_system)
                    .await
                    .map(Some)
                    .map_err(|error| error.to_string()),
                None => Ok(None),
            }
        }
    });

    let on_search_input = move |event: FormEvent| {
        query.set(event.value());
        search_generation.set(search_generation() + 1);
        search_results.set(Vec::new());
        search_loading.set(false);
        search_empty.set(false);
        search_error.set(None);
    };

    let on_search_submit = move |_: FormEvent| {
        let search_query = query().trim().to_string();
        query.set(search_query.clone());
        search_generation.set(search_generation() + 1);
        let request_id = search_generation();
        search_results.set(Vec::new());
        search_empty.set(false);
        search_error.set(None);

        if search_query.is_empty() {
            search_loading.set(false);
            search_error.set(Some(WeatherError::EmptyQuery.to_string()));
            return;
        }

        search_loading.set(true);
        spawn(async move {
            let result = search_locations(&search_query).await;
            if search_generation() != request_id {
                return;
            }

            search_loading.set(false);
            match result {
                Ok(locations) => {
                    search_results.set(locations);
                    search_empty.set(false);
                }
                Err(WeatherError::EmptyResult) => {
                    search_results.set(Vec::new());
                    search_empty.set(true);
                }
                Err(error) => {
                    search_results.set(Vec::new());
                    search_empty.set(false);
                    search_error.set(Some(error.to_string()));
                }
            }
        });
    };

    let on_location_select = move |location: Location| {
        select_location(
            selected,
            query,
            search_results,
            search_loading,
            search_empty,
            search_error,
            search_generation,
            location,
        );
    };

    let on_favorite_select = move |location: Location| {
        select_location(
            selected,
            query,
            search_results,
            search_loading,
            search_empty,
            search_error,
            search_generation,
            location,
        );
    };

    let on_unit_change = move |event: FormEvent| {
        units.set(if event.value().eq_ignore_ascii_case("C") {
            UnitSystem::Metric
        } else {
            UnitSystem::Imperial
        });
    };

    let on_theme_change = move |event: FormEvent| {
        let next_theme = if event.value().eq_ignore_ascii_case("dark") {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        theme.set(next_theme);
        browser::save_theme(next_theme);
    };

    let on_favorite_toggle = move |_| {
        let Some(location) = selected() else {
            return;
        };

        favorites.with_mut(|saved| {
            if let Some(index) = saved.iter().position(|item| same_location(item, &location)) {
                saved.remove(index);
            } else {
                saved.push(location);
            }
        });
        browser::save_favorites(&favorites());
    };

    let on_retry = move |_| {
        weather_retry.set(weather_retry() + 1);
    };

    let selected_location = selected();
    let unit_system = units();
    let theme_mode = theme();
    let saved_favorites = favorites();
    let state = if selected_location.is_none() {
        Some((
            WeatherState::Empty,
            "Search for a place to begin a new weather log.".to_string(),
        ))
    } else if weather.pending() {
        Some((
            WeatherState::Loading,
            "Fetching readings in the station's local time.".to_string(),
        ))
    } else {
        match weather() {
            Some(Ok(Some(_))) => None,
            Some(Ok(None)) => Some((
                WeatherState::Empty,
                "Search for a place to begin a new weather log.".to_string(),
            )),
            Some(Err(error)) => Some((WeatherState::Error, error)),
            None => Some((
                WeatherState::Loading,
                "Fetching readings in the station's local time.".to_string(),
            )),
        }
    };

    let weather_data = match weather() {
        Some(Ok(Some(data))) if state.is_none() => data,
        _ => empty_weather_data(),
    };
    let location = selected_location.unwrap_or_else(empty_location);
    let unit = match unit_system {
        UnitSystem::Imperial => "F",
        UnitSystem::Metric => "C",
    }
    .to_string();
    let is_favorite = saved_favorites
        .iter()
        .any(|favorite| same_location(favorite, &location));
    let (state, state_message) = state
        .map(|(state, message)| (Some(state), message))
        .unwrap_or((None, String::new()));

    rsx! {
        WeatherInstrument {
            location,
            weather: weather_data,
            unit,
            theme: theme_mode,
            is_favorite,
            favorites: saved_favorites,
            search_query: query(),
            search_results: search_results(),
            search_loading: search_loading(),
            search_empty: search_empty(),
            search_error: search_error(),
            state,
            state_message,
            on_search_input,
            on_search_submit,
            on_location_select,
            on_unit_change,
            on_theme_change,
            on_favorite_toggle,
            on_favorite_select,
            on_retry,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select_location(
    mut selected: Signal<Option<Location>>,
    mut query: Signal<String>,
    mut search_results: Signal<Vec<Location>>,
    mut search_loading: Signal<bool>,
    mut search_empty: Signal<bool>,
    mut search_error: Signal<Option<String>>,
    mut search_generation: Signal<u64>,
    location: Location,
) {
    selected.set(Some(location.clone()));
    query.set(location.name.clone());
    search_generation.set(search_generation() + 1);
    search_results.set(Vec::new());
    search_loading.set(false);
    search_empty.set(false);
    search_error.set(None);
    browser::set_url_location(Some(&location));
}

fn same_location(left: &Location, right: &Location) -> bool {
    (left.id != 0 && left.id == right.id)
        || (left.latitude == right.latitude && left.longitude == right.longitude)
}

fn empty_location() -> Location {
    Location {
        id: 0,
        name: "No station selected".to_string(),
        admin1: None,
        country: String::new(),
        latitude: 0.0,
        longitude: 0.0,
        timezone: "Local time unavailable".to_string(),
    }
}

fn empty_weather_data() -> WeatherData {
    WeatherData {
        current: CurrentWeather {
            time: String::new(),
            temperature: 0.0,
            apparent_temperature: 0.0,
            weather_code: -1,
            is_day: 1,
            wind_speed: 0.0,
            wind_direction: 0,
            precipitation: 0.0,
        },
        hourly: Vec::new(),
        daily: Vec::new(),
        timezone: String::new(),
        timezone_abbreviation: String::new(),
        utc_offset_seconds: 0,
    }
}
