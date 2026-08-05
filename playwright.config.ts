import { defineConfig, devices } from "@playwright/test";

const port = Number(process.env.PREVIEW_PORT ?? 4173);
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${port}/puddle/`;

export default defineConfig({
    testDir: "./tests/e2e",
    timeout: 30_000,
    expect: {
        timeout: 10_000,
    },
    fullyParallel: true,
    forbidOnly: !!process.env.CI,
    retries: process.env.CI ? 2 : 0,
    reporter: process.env.CI ? "line" : "list",
    use: {
        baseURL,
        trace: "retain-on-failure",
        screenshot: "only-on-failure",
        video: "retain-on-failure",
        actionTimeout: 10_000,
    },
    projects: [
        {
            name: "chromium",
            use: { ...devices["Desktop Chrome"] },
        },
        {
            name: "firefox",
            use: { ...devices["Desktop Firefox"] },
        },
        {
            name: "webkit",
            use: { ...devices["Desktop Safari"] },
        },
        {
            name: "mobile-chromium",
            use: { ...devices["Pixel 7"] },
        },
    ],
    webServer: {
        command: "node tests/e2e/preview-server.mjs",
        url: `http://127.0.0.1:${port}/puddle/__health`,
        timeout: 30_000,
        reuseExistingServer: false,
    },
});
