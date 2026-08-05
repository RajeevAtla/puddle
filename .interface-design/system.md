# Puddle Interface System

## Direction

Puddle is an atmospheric instrument for weather enthusiasts: a field-station
logbook with a quiet, observational tone. The visual signature is the current
reading paired with a thin temperature trace, not a grid of generic metric
cards. The mast/search, station rail, reading, trace, and horizon form one
instrument panel.

## Intent

- Human: someone checking a place before a walk, commute, or outdoor session.
- Job: find a station, read the current atmosphere, then scan its near horizon.
- Feel: calm, tactile, precise, and slightly analog.

## Tokens and foundations

- Base spacing: 4px; major rhythm uses 8, 16, 24, and 40px.
- Typography: `Baskerville`/`Palatino`/`Georgia` for observation titles;
  `Avenir Next`/`Segoe UI` for controls; `Consolas`/`SFMono-Regular` for
  tabular readings.
- Depth: border-led layering and tonal shifts; no floating dashboard-card
  treatment and no gradients.
- Light theme is `data-theme="daybook"`; dark theme is
  `data-theme="nightwatch"`.
- Natural palette: lichen canvas, paper surfaces, graphite ink, wet-slate
  lines, oxidized-teal trace, and one copper action accent.

## Reusable patterns

- `WeatherInstrument`: accepts existing `Location` and `WeatherData` values;
  its desktop two-column station rail plus reading field collapses to one
  column below 880px.
- `CurrentWeatherCard`: one focal temperature, condition, observation time,
  and three supporting readings.
- `WeatherTrace`: SVG temperature trace with a native `<details>` table
  alternative. Accepts at most 24 existing `HourlyForecastItem` values.
- `SevenDayHorizon`: ordered forecast rows, not repeated cards.
- `WeatherControls`: native selects for units/theme, native buttons for saving
  and selecting favorite stations.
- `WeatherStatePanel`: loading, empty, and error states share an announced
  status surface; errors include a retry button.
