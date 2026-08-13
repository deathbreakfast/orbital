import { defineConfig, devices } from "@playwright/test";

const desktopChrome = devices["Desktop Chrome"];
const previewE2e = !!process.env.COMPONENT_PREVIEW_E2E;

const mobileSpecs = [
  "**/mobile-overlays.spec.ts",
  "**/layout-responsive.spec.ts",
];

export default defineConfig({
  testDir: "./tests",
  timeout: 120 * 1000,
  expect: { timeout: 10_000 },
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: previewE2e
    ? Number(process.env.PLAYWRIGHT_WORKERS ?? (process.env.CI ? 4 : 1))
    : undefined,
  reporter: [["list"], ["html", { open: "never" }]],
  use: {
    baseURL: process.env.COMPONENT_PREVIEW_BASE_URL ?? "http://localhost:3010",
    actionTimeout: 30_000,
    navigationTimeout: 60_000,
    trace: "on-first-retry",
  },
  projects: previewE2e
    ? [
        {
          name: "component-preview",
          testMatch: "components/**/*.spec.ts",
          use: desktopChrome,
        },
        {
          name: "component-preview-mobile",
          testMatch: mobileSpecs,
          use: {
            ...devices["iPhone 13"],
            // CI installs Chromium; keep iPhone metrics without requiring WebKit.
            browserName: "chromium",
          },
        },
        {
          name: "component-preview-mobile-android",
          testMatch: mobileSpecs,
          use: {
            ...devices["Pixel 7"],
            browserName: "chromium",
          },
        },
        {
          name: "smoke",
          testMatch: "smoke/**/*.spec.ts",
          use: desktopChrome,
        },
      ]
    : [],
});
