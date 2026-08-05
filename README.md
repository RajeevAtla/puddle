# Puddle

Rust/Dioxus weather instrument for worldwide station search, local forecasts,
and a 24-hour temperature trace.

## Local development

```sh
npm ci
dx serve
```

Build the production web artifact with `npm run build:web`.

## Verification

```sh
cargo test
npm run test:e2e
```

The browser tests mock Open-Meteo and cover Chromium, Firefox, WebKit, and
mobile Chromium. Pushes to `main` build and deploy the static bundle to
GitHub Pages.
