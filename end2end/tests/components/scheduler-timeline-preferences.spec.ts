import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scheduler-timeline-preferences preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-preferences");
    await expect(page.getByTestId("scheduler-timeline-preferences-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-preferences");
    await expectPreviewVariants(page, ["scheduler-timeline-preferences-preview"]);
  });

  test("toggles 12-hour clock and updates time column labels", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-preferences");
    const preview = page.getByTestId("scheduler-timeline-preferences-preview");

    const header = preview.locator(".orb-scheduler-timeline__time-column").first();
    await expect(header).toBeVisible({ timeout: 30_000 });

    await scrollIntoPreviewView(preview);
    const menuTrigger = preview.getByTestId("scheduler-preferences-menu-trigger").getByRole("button");
    await scrollIntoPreviewView(menuTrigger);
    await menuTrigger.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByTestId("scheduler-preferences-panel")).toBeVisible();
    const ampmSwitch = page.getByTestId("scheduler-pref-ampm").getByRole("switch");
    await ampmSwitch.focus();
    await page.keyboard.press("Space");

    await expect(header).toBeVisible();
    const text = (await header.textContent()) ?? "";
    expect(text).toMatch(/^\d{2}:\d{2}$/);
  });
});
