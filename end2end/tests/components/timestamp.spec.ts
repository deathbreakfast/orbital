import { test, expect, type Page } from "@playwright/test";
import { openComponentPreview, scrollIntoPreviewView } from "../lib/preview/navigation";

const BASIC_ISO = "2026-10-27T08:42:17.204913+00:00";

async function openTimestamps(page: Page) {
  await openComponentPreview(page, "timestamp", "timestamp-basic");
}

function trigger(page: Page, testId: string) {
  return page.getByTestId(testId).locator(".orbital-timestamp__trigger").first();
}

function openCard(page: Page) {
  return page.locator("[data-testid='timestamp-card']:visible");
}

test.describe("timestamp in America/Los_Angeles", () => {
  test.use({ timezoneId: "America/Los_Angeles" });

  test("TS-01: time element carries the UTC instant and the label shows PDT", async ({ page }) => {
    await openTimestamps(page);
    const time = page.getByTestId("timestamp-basic").locator("time");
    await expect(time).toHaveAttribute("datetime", "2026-10-27T08:42:17.204Z");
    await expect(time).toHaveText("Oct 27, 2026, 1:42 AM PDT");
  });

  test("TS-02: hover opens the card with Local and UTC rows", async ({ page }) => {
    await openTimestamps(page);
    const target = trigger(page, "timestamp-basic");
    await expect(target).toHaveAttribute("tabindex", "0");
    await target.hover();
    const card = openCard(page);
    await expect(card).toBeVisible();
    await expect(card.getByTestId("timestamp-card-local")).toContainText(
      "Tue, Oct 27, 2026, 1:42:17.204 AM PDT",
    );
    await expect(card.getByTestId("timestamp-card-local")).toContainText("America/Los_Angeles");
    await expect(card.getByTestId("timestamp-card-local")).toContainText("UTC−07:00");
    await expect(card.getByTestId("timestamp-card-utc")).toContainText("2026-10-27 08:42:17.204 UTC");
    await expect(card.getByTestId("timestamp-card-relative")).toBeVisible();
    await expect(target).toHaveAttribute("aria-describedby", (await card.getAttribute("id"))!);
  });

  test("TS-03: keyboard focus opens the card", async ({ page }) => {
    await openTimestamps(page);
    const target = trigger(page, "timestamp-basic");
    await target.focus();
    await expect(target).toBeFocused();
    await expect(openCard(page).getByTestId("timestamp-card-utc")).toBeVisible();
  });

  test("TS-04: Expand shows the original ISO string and unix ms", async ({ page }) => {
    await openTimestamps(page);
    await trigger(page, "timestamp-basic").hover();
    const card = openCard(page);
    await expect(card).toBeVisible();
    await card.getByRole("button", { name: "Expand" }).click();
    await expect(card.getByTestId("timestamp-card-iso")).toContainText(BASIC_ISO);
    await expect(card.getByTestId("timestamp-card-unix-ms")).toContainText("1793090537204");
  });

  test("TS-05: styles and pinned zones", async ({ page }) => {
    await openTimestamps(page);
    await scrollIntoPreviewView(page.getByTestId("timestamp-style-date"));
    await expect(page.getByTestId("timestamp-style-date").locator("time")).toHaveText("Oct 27, 2026");
    await expect(page.getByTestId("timestamp-style-time").locator("time")).toHaveText("1:42 AM PDT");
    await expect(page.getByTestId("timestamp-style-relative").locator("time")).toHaveText("3h ago");
    await expect(page.getByTestId("timestamp-tz-utc").locator("time")).toHaveText(
      "Oct 27, 2026, 8:42 AM UTC",
    );
    await expect(page.getByTestId("timestamp-tz-fixed").locator("time")).toHaveText(
      "Oct 27, 2026, 10:42 AM GMT+2",
    );
  });

  test("TS-06: unix seconds show no milliseconds", async ({ page }) => {
    await openTimestamps(page);
    const target = trigger(page, "timestamp-unix-seconds");
    await scrollIntoPreviewView(target);
    await target.hover();
    await expect(openCard(page).getByTestId("timestamp-card-utc")).toContainText(
      "2026-10-27 08:42:17 UTC",
    );
    await expect(openCard(page).getByTestId("timestamp-card-utc")).not.toContainText(".");
  });

  test("TS-07: unparseable text renders unchanged without a card", async ({ page }) => {
    await openTimestamps(page);
    const raw = page.getByTestId("timestamp-unparseable");
    await scrollIntoPreviewView(raw);
    await expect(raw).toHaveText("last Tuesday");
    await expect(raw.locator("time")).toHaveCount(0);
    await expect(raw.locator(".orbital-timestamp__trigger")).toHaveCount(0);
  });
});

test.describe("timestamp in UTC", () => {
  test.use({ timezoneId: "UTC" });

  test("TS-08: label shows UTC", async ({ page }) => {
    await openTimestamps(page);
    await expect(page.getByTestId("timestamp-basic").locator("time")).toHaveText(
      "Oct 27, 2026, 8:42 AM UTC",
    );
  });
});
