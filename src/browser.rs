use crate::components::controls::ThemeMode;
use crate::models::location::Location;

#[cfg(target_arch = "wasm32")]
pub const FAVORITES_STORAGE_KEY: &str = "puddle.favorites";
#[cfg(target_arch = "wasm32")]
pub const THEME_STORAGE_KEY: &str = "puddle.theme";

#[cfg(target_arch = "wasm32")]
const URL_LOCATION_KEYS: &[&str] = &[
    "id",
    "lat",
    "lon",
    "latitude",
    "longitude",
    "name",
    "country",
    "admin1",
    "timezone",
];

pub fn location_from_url() -> Option<Location> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window()?;
        let search = window.location().search().ok()?;
        let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
        return location_from_params(&params);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn set_url_location(location: Option<&Location>) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(href) = window.location().href() else {
            return;
        };
        let Ok(url) = web_sys::Url::new(&href) else {
            return;
        };
        let params = url.search_params();

        for key in URL_LOCATION_KEYS {
            params.delete(key);
        }

        if let Some(location) = location.filter(|location| is_valid_location(location)) {
            params.set("id", &location.id.to_string());
            params.set("lat", &location.latitude.to_string());
            params.set("lon", &location.longitude.to_string());
            params.set("name", &location.name);
            params.set("country", &location.country);
            if let Some(admin1) = &location.admin1 {
                params.set("admin1", admin1);
            }
            params.set("timezone", &location.timezone);
        }

        let updated_href = url.href();
        if let Ok(history) = window.history() {
            let _ = history.replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&updated_href),
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = location;
    }
}

pub fn load_favorites() -> Vec<Location> {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(storage) = storage() else {
            return Vec::new();
        };
        let Ok(Some(raw)) = storage.get(FAVORITES_STORAGE_KEY) else {
            return Vec::new();
        };
        return serde_json::from_str::<Vec<Location>>(&raw)
            .map(|locations| locations.into_iter().filter(is_valid_location).collect())
            .unwrap_or_default();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Vec::new()
    }
}

pub fn save_favorites(favorites: &[Location]) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(storage) = storage() else {
            return;
        };
        let valid_favorites: Vec<_> = favorites
            .iter()
            .filter(|location| is_valid_location(location))
            .cloned()
            .collect();
        let Ok(raw) = serde_json::to_string(&valid_favorites) else {
            return;
        };
        let _ = storage.set(FAVORITES_STORAGE_KEY, &raw);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = favorites;
    }
}

pub fn load_theme() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(storage) = storage() else {
            return ThemeMode::Light;
        };
        return match storage.get(THEME_STORAGE_KEY).ok().flatten().as_deref() {
            Some("dark") => ThemeMode::Dark,
            _ => ThemeMode::Light,
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        ThemeMode::Light
    }
}

pub fn save_theme(theme: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(storage) = storage() else {
            return;
        };
        let value = match theme {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        let _ = storage.set(THEME_STORAGE_KEY, value);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = theme;
    }
}

#[cfg(any(target_arch = "wasm32", test))]
pub fn is_valid_location(location: &Location) -> bool {
    location.id >= 0
        && location.latitude.is_finite()
        && (-90.0..=90.0).contains(&location.latitude)
        && location.longitude.is_finite()
        && (-180.0..=180.0).contains(&location.longitude)
        && valid_text(&location.name)
        && valid_text(&location.country)
        && valid_text(&location.timezone)
        && location.admin1.as_deref().map(valid_text).unwrap_or(true)
}

#[cfg(any(target_arch = "wasm32", test))]
fn valid_text(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty() && value.chars().count() <= 256 && !value.chars().any(char::is_control)
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[cfg(target_arch = "wasm32")]
fn location_from_params(params: &web_sys::UrlSearchParams) -> Option<Location> {
    let id = match params.get("id") {
        Some(value) => value.parse::<i64>().ok()?,
        None => 0,
    };
    if id < 0 {
        return None;
    }

    let latitude = coordinate(params, &["lat", "latitude"], -90.0, 90.0)?;
    let longitude = coordinate(params, &["lon", "longitude"], -180.0, 180.0)?;
    let name = query_text(params, &["name"])?;
    let country = query_text(params, &["country"])?;
    let timezone = query_text(params, &["timezone"])?;
    let admin1 = match params.get("admin1") {
        Some(value) => Some(validated_text(&value)?),
        None => None,
    };

    let location = Location {
        id,
        name,
        admin1,
        country,
        latitude,
        longitude,
        timezone,
    };
    is_valid_location(&location).then_some(location)
}

#[cfg(target_arch = "wasm32")]
fn coordinate(
    params: &web_sys::UrlSearchParams,
    keys: &[&str],
    minimum: f64,
    maximum: f64,
) -> Option<f64> {
    let raw = keys.iter().find_map(|key| params.get(key))?;
    let value = raw.parse::<f64>().ok()?;
    (value.is_finite() && (minimum..=maximum).contains(&value)).then_some(value)
}

#[cfg(target_arch = "wasm32")]
fn query_text(params: &web_sys::UrlSearchParams, keys: &[&str]) -> Option<String> {
    let value = keys.iter().find_map(|key| params.get(key))?;
    validated_text(&value)
}

#[cfg(target_arch = "wasm32")]
fn validated_text(value: &str) -> Option<String> {
    let value = value.trim();
    valid_text(value).then_some(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn location() -> Location {
        Location {
            id: 1,
            name: "Paris".to_string(),
            admin1: Some("Ile-de-France".to_string()),
            country: "France".to_string(),
            latitude: 48.8566,
            longitude: 2.3522,
            timezone: "Europe/Paris".to_string(),
        }
    }

    #[test]
    fn location_validation_rejects_bad_coordinates_and_strings() {
        let mut invalid = location();
        invalid.latitude = 91.0;
        assert!(!is_valid_location(&invalid));

        let mut invalid = location();
        invalid.name = "\n".to_string();
        assert!(!is_valid_location(&invalid));
    }
}
