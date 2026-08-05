import { expect, test as base } from "@playwright/test";
import type { Page, Route } from "@playwright/test";

const TOKYO = {
    id: 1850147,
    name: "Tokyo",
    admin1: "Tokyo",
    country: "Japan",
    latitude: 35.6762,
    longitude: 139.6503,
    timezone: "Asia/Tokyo",
};

const SEARCH_RESULTS = [
    TOKYO,
    {
        id: 2988507,
        name: "Paris",
        admin1: "Ile-de-France",
        country: "France",
        latitude: 48.8566,
        longitude: 2.3522,
        timezone: "Europe/Paris",
    },
    {
        id: 360630,
        name: "Cairo",
        admin1: "Cairo",
        country: "Egypt",
        latitude: 30.0444,
        longitude: 31.2357,
        timezone: "Africa/Cairo",
    },
];

type WeatherMocks = {
    apiRequests: string[];
    consoleErrors: string[];
    forecastRequests: string[];
    geocodingRequests: string[];
    pageErrors: string[];
    unexpectedApiRequests: string[];
    failNextForecasts: (count: number) => void;
};

const test = base.extend<{ mocks: WeatherMocks }>({
    mocks: async ({ page }, use) => {
        const mocks = await installWeatherMocks(page);
        await use(mocks);
        expect(mocks.unexpectedApiRequests, "all Open-Meteo requests must match a mocked endpoint").toEqual([]);
    },
});

function corsHeaders(contentType = "application/json; charset=utf-8") {
    return {
        "Access-Control-Allow-Headers": "*",
        "Access-Control-Allow-Methods": "GET, OPTIONS",
        "Access-Control-Allow-Origin": "*",
        "Cache-Control": "no-store",
        "Content-Type": contentType,
    };
}

function makeForecastResponse(metric: boolean) {
    const currentTemperature = metric ? 22 : 72;
    const hourlyTemperatures = Array.from({ length: 24 }, (_, index) => currentTemperature + ((index % 6) - 2));
    const hourlyTimes = Array.from({ length: 24 }, (_, index) => {
        const hour = 12 + index;
        const day = 4 + Math.floor(hour / 24);
        const normalizedHour = String(hour % 24).padStart(2, "0");
        return `2026-08-${String(day).padStart(2, "0")}T${normalizedHour}:00`;
    });
    const dailyTimes = Array.from({ length: 7 }, (_, index) => `2026-08-${String(4 + index).padStart(2, "0")}`);

    return {
        timezone: "Asia/Tokyo",
        timezone_abbreviation: "JST",
        utc_offset_seconds: 32400,
        current: {
            time: "2026-08-04T12:00",
            temperature_2m: currentTemperature,
            apparent_temperature: currentTemperature - 1,
            weather_code: 0,
            is_day: 1,
            wind_speed_10m: metric ? 18 : 11,
            wind_direction_10m: 180,
            precipitation: metric ? 0.4 : 0.02,
        },
        hourly: {
            time: hourlyTimes,
            temperature_2m: hourlyTemperatures,
            relative_humidity_2m: Array(24).fill(60),
            apparent_temperature: hourlyTemperatures.map((temperature) => temperature - 1),
            precipitation_probability: Array.from({ length: 24 }, (_, index) => index * 3),
            precipitation: Array(24).fill(metric ? 0.1 : 0.01),
            weather_code: Array(24).fill(0),
            is_day: Array(24).fill(1),
            wind_speed_10m: Array(24).fill(metric ? 18 : 11),
            wind_direction_10m: Array(24).fill(180),
        },
        daily: {
            time: dailyTimes,
            weather_code: Array(7).fill(0),
            temperature_2m_max: Array.from({ length: 7 }, (_, index) => currentTemperature + 5 + index),
            temperature_2m_min: Array.from({ length: 7 }, (_, index) => currentTemperature - 4 + index),
            precipitation_sum: Array(7).fill(metric ? 1.2 : 0.05),
            precipitation_probability_max: [10, 20, 30, 40, 50, 60, 70],
            wind_speed_10m_max: Array(7).fill(metric ? 24 : 15),
            sunrise: Array(7).fill("2026-08-04T05:00"),
            sunset: Array(7).fill("2026-08-04T18:30"),
            daylight_duration: Array(7).fill(48600),
        },
    };
}

async function installWeatherMocks(page: Page): Promise<WeatherMocks> {
    const apiRequests: string[] = [];
    const consoleErrors: string[] = [];
    const forecastRequests: string[] = [];
    const geocodingRequests: string[] = [];
    const pageErrors: string[] = [];
    const unexpectedApiRequests: string[] = [];
    let forecastFailures = 0;

    page.on("console", (message) => {
        if (message.type() === "error" && !message.location().url.endsWith("/favicon.ico")) {
            consoleErrors.push(`${message.text()} @ ${message.location().url}`);
        }
    });
    page.on("pageerror", (error) => pageErrors.push(error.message));

    const routeState = {
        apiRequests,
        forecastRequests,
        geocodingRequests,
        unexpectedApiRequests,
        failForecast: () => {
            if (forecastFailures === 0) {
                return false;
            }
            forecastFailures -= 1;
            return true;
        },
    };
    await page.route("https://geocoding-api.open-meteo.com/**", (route) => handleApiRoute(route, routeState));
    await page.route("https://api.open-meteo.com/**", (route) => handleApiRoute(route, routeState));

    return {
        apiRequests,
        consoleErrors,
        forecastRequests,
        geocodingRequests,
        pageErrors,
        unexpectedApiRequests,
        failNextForecasts: (count) => {
            forecastFailures = Math.max(0, count);
        },
    };
}

async function handleApiRoute(
    route: Route,
    state: {
        apiRequests: string[];
        forecastRequests: string[];
        geocodingRequests: string[];
        unexpectedApiRequests: string[];
        failForecast: () => boolean;
    },
) {
    const request = route.request();
    const url = new URL(request.url());
    if (!url.hostname.endsWith("open-meteo.com")) {
        await route.continue();
        return;
    }

    state.apiRequests.push(request.url());
    if (request.method() === "OPTIONS") {
        await route.fulfill({ status: 204, headers: corsHeaders() });
        return;
    }

    if (url.hostname === "geocoding-api.open-meteo.com" && url.pathname === "/v1/search") {
        state.geocodingRequests.push(request.url());
        await route.fulfill({
            status: 200,
            headers: corsHeaders(),
            body: JSON.stringify({ results: SEARCH_RESULTS }),
        });
        return;
    }

    if (url.hostname === "api.open-meteo.com" && url.pathname === "/v1/forecast") {
        state.forecastRequests.push(request.url());
        if (state.failForecast()) {
            await route.fulfill({
                status: 503,
                headers: corsHeaders(),
                body: JSON.stringify({ error: true, reason: "synthetic outage" }),
            });
            return;
        }

        const metric = url.searchParams.get("temperature_unit") === "celsius";
        await route.fulfill({
            status: 200,
            headers: corsHeaders(),
            body: JSON.stringify(makeForecastResponse(metric)),
        });
        return;
    }

    state.unexpectedApiRequests.push(request.url());
    await route.abort("blockedbyclient");
}

async function openApp(page: Page) {
    const response = await page.goto("./");
    expect(response?.status()).toBe(200);
}

async function searchForTokyo(page: Page) {
    await page.locator("#weather-location-search").fill("Tokyo");
    await page.getByRole("button", { name: "Measure", exact: true }).click();
    await expect(page.getByRole("button", { name: "Select Tokyo, Japan", exact: true })).toBeVisible();
}

async function selectTokyo(page: Page) {
    await searchForTokyo(page);
    await page.getByRole("button", { name: "Select Tokyo, Japan", exact: true }).click();
    await expect(page.getByRole("heading", { name: "Atmosphere at a glance", exact: true })).toBeVisible();
}

test.describe("Puddle web app", () => {
    test("shows an empty first-visit station state", async ({ page, mocks }) => {
        await openApp(page);

        await expect(page.locator("#weather-main").getByRole("heading", { name: "No station selected", exact: true })).toBeVisible();
        await expect(page.getByText("Search for a place to begin a new weather log.", { exact: true })).toBeVisible();
        expect(mocks.geocodingRequests).toEqual([]);
        expect(mocks.forecastRequests).toEqual([]);
    });

    test("searches worldwide and selects a result", async ({ page, mocks }) => {
        await openApp(page);
        await searchForTokyo(page);

        await expect(page.locator(".search-result-list > li")).toHaveCount(3);
        await expect(page.getByRole("button", { name: "Select Paris, France", exact: true })).toBeVisible();
        await page.getByRole("button", { name: "Select Tokyo, Japan", exact: true }).click();

        await expect(page.getByRole("heading", { name: "Tokyo", exact: true })).toBeVisible();
        await expect(page.locator(".station-country")).toHaveText("Japan");
        expect(new URL(page.url()).searchParams.get("name")).toBe("Tokyo");
        expect(mocks.geocodingRequests[0]).toContain("name=Tokyo");
    });

    test("renders the current reading, local timezone, trace, table, and seven-day horizon", async ({ page, mocks }) => {
        await openApp(page);
        await selectTokyo(page);

        await expect(page.locator('[aria-label="Current temperature 72 degrees F"]')).toBeVisible();
        await expect(page.getByText("Clear", { exact: true }).first()).toBeVisible();
        await expect(page.locator(".reading-time")).toHaveText("2026-08-04T12:00");
        await expect(page.locator(".station-timezone")).toHaveText("Asia/Tokyo");

        await expect(page.getByRole("heading", { name: "Next 24 hours", exact: true })).toBeVisible();
        await expect(page.locator(".trace-panel .section-meta")).toHaveText("24 hourly points");
        await expect(page.locator(".trace-point")).toHaveCount(24);
        await page.getByText("Read the trace as a table", { exact: true }).click();
        await expect(page.locator(".trace-table-wrap table")).toBeVisible();
        await expect(page.locator(".trace-table-wrap tbody tr")).toHaveCount(24);

        await expect(page.getByRole("heading", { name: "Seven-day horizon", exact: true })).toBeVisible();
        await expect(page.locator(".horizon-list > li")).toHaveCount(7);
        await expect(page.locator(".horizon-list > li").first()).toContainText("Today");
    });

    test("changes between imperial and metric readings", async ({ page, mocks }) => {
        await openApp(page);
        await selectTokyo(page);

        const unit = page.getByLabel("Temperature unit", { exact: true });
        await expect(unit).toHaveValue("F");
        await unit.selectOption("C");
        await expect(page.locator('[aria-label="Current temperature 22 degrees C"]')).toBeVisible();
        await expect(unit).toHaveValue("C");
        await expect(page.locator(".horizon-list").first()).toContainText("°C");

        await unit.selectOption("F");
        await expect(page.locator('[aria-label="Current temperature 72 degrees F"]')).toBeVisible();
        expect(mocks.forecastRequests.map((request) => new URL(request).searchParams.get("temperature_unit"))).toEqual([
            "fahrenheit",
            "celsius",
        ]);
    });

    test("keeps selected location in URL state across reload", async ({ page, mocks }) => {
        await openApp(page);
        await selectTokyo(page);

        const selectedUrl = new URL(page.url());
        expect(selectedUrl.pathname).toBe("/puddle/");
        expect(selectedUrl.searchParams.get("id")).toBe(String(TOKYO.id));
        expect(selectedUrl.searchParams.get("lat")).toBe(String(TOKYO.latitude));
        expect(selectedUrl.searchParams.get("lon")).toBe(String(TOKYO.longitude));
        expect(selectedUrl.searchParams.get("timezone")).toBe(TOKYO.timezone);
        const forecastCountBeforeReload = mocks.forecastRequests.length;

        await page.reload();
        await expect(page.getByRole("heading", { name: "Tokyo", exact: true })).toBeVisible();
        await expect(page.getByRole("heading", { name: "Atmosphere at a glance", exact: true })).toBeVisible();
        await expect(page.locator("#weather-location-search")).toHaveValue("Tokyo");
        expect(new URL(page.url()).search).toBe(selectedUrl.search);
        expect(mocks.geocodingRequests).toHaveLength(1);
        expect(mocks.forecastRequests.length).toBeGreaterThan(forecastCountBeforeReload);
    });

    test("persists favorites in localStorage and restores a saved station", async ({ page }) => {
        await openApp(page);
        await selectTokyo(page);

        await page.getByRole("button", { name: "Save this station", exact: true }).click();
        await expect(page.getByRole("button", { name: "Saved to favorites", exact: true })).toBeVisible();
        const saved = await page.evaluate(() => JSON.parse(window.localStorage.getItem("puddle.favorites") ?? "null"));
        expect(saved).toHaveLength(1);
        expect(saved[0]).toMatchObject({ name: "Tokyo", country: "Japan", timezone: "Asia/Tokyo" });

        await page.goto("./");
        await expect(page.locator("#weather-main").getByRole("heading", { name: "No station selected", exact: true })).toBeVisible();
        const favorite = page.locator(".favorite-location").filter({ hasText: "Tokyo" });
        await expect(favorite).toBeVisible();
        await favorite.click();
        await expect(page.getByRole("heading", { name: "Atmosphere at a glance", exact: true })).toBeVisible();
    });

    test("switches themes and persists the selected theme", async ({ page }) => {
        await openApp(page);
        const theme = page.getByLabel("Color theme", { exact: true });
        const instrument = page.locator(".weather-instrument");

        await expect(theme).toHaveValue("light");
        await theme.selectOption("dark");
        await expect(instrument).toHaveAttribute("data-theme", "nightwatch");
        await expect.poll(() => page.evaluate(() => window.localStorage.getItem("puddle.theme"))).toBe("dark");

        await page.reload();
        await expect(page.getByLabel("Color theme", { exact: true })).toHaveValue("dark");
        await expect(instrument).toHaveAttribute("data-theme", "nightwatch");
        await page.getByLabel("Color theme", { exact: true }).selectOption("light");
        await expect(instrument).toHaveAttribute("data-theme", "daybook");
    });

    test("shows a network error and recovers on retry", async ({ page, mocks }) => {
        mocks.failNextForecasts(1);
        await openApp(page);
        await searchForTokyo(page);
        await page.getByRole("button", { name: "Select Tokyo, Japan", exact: true }).click();

        await expect(page.getByRole("heading", { name: "The station went quiet", exact: true })).toBeVisible();
        await expect(page.getByRole("button", { name: "Try again", exact: true })).toBeVisible();
        await page.getByRole("button", { name: "Try again", exact: true }).click();
        await expect(page.locator('[aria-label="Current temperature 72 degrees F"]')).toBeVisible();
        expect(mocks.forecastRequests).toHaveLength(2);
    });

    test("reports an empty search without making a geocoding request", async ({ page, mocks }) => {
        await openApp(page);
        await page.locator("#weather-location-search").fill("   ");
        await page.getByRole("button", { name: "Measure", exact: true }).click();

        await expect(page.getByRole("alert")).toContainText("Search query cannot be empty");
        expect(mocks.geocodingRequests).toEqual([]);
        expect(mocks.forecastRequests).toEqual([]);
    });

    test("keeps weather traffic entirely at the mocked Open-Meteo boundary", async ({ page, mocks }) => {
        await openApp(page);
        await selectTokyo(page);
        await page.getByLabel("Temperature unit", { exact: true }).selectOption("C");
        await expect(page.locator('[aria-label="Current temperature 22 degrees C"]')).toBeVisible();

        expect(mocks.unexpectedApiRequests).toEqual([]);
        expect(mocks.apiRequests.every((request) => request.includes("open-meteo.com"))).toBe(true);
        expect(mocks.apiRequests.some((request) => request.includes("geocoding-api.open-meteo.com/v1/search"))).toBe(true);
        expect(mocks.apiRequests.some((request) => request.includes("api.open-meteo.com/v1/forecast"))).toBe(true);
    });

    test("has no unexpected browser console or page errors", async ({ page, mocks }) => {
        await openApp(page);
        await expect(page.locator("#weather-main").getByRole("heading", { name: "No station selected", exact: true })).toBeVisible();
        await page.waitForLoadState("networkidle");

        expect(mocks.consoleErrors).toEqual([]);
        expect(mocks.pageErrors).toEqual([]);
    });
});
