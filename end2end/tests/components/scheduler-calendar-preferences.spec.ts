import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scheduler-calendar-preferences preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-preferences");
    await expect(page.getByTestId("scheduler-calendar-preferences-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-preferences");
    await expectPreviewVariants(page, ["scheduler-calendar-preferences-preview"]);
  });

  test("hides weekend columns when show weekends is off", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-preferences");
    const preview = page.getByTestId("scheduler-calendar-preferences-preview");

    const menuTrigger = preview.getByTestId("scheduler-preferences-menu-trigger").getByRole("button");
    await scrollIntoPreviewView(menuTrigger);
    await menuTrigger.click({ force: true });
    await expect(page.getByTestId("scheduler-preferences-panel")).toBeVisible();
    const showWeekends = page.getByTestId("scheduler-pref-show-weekends");
    await scrollIntoPreviewView(showWeekends);
    await showWeekends.getByRole("switch").click();

    const headers = preview.locator(".orb-scheduler-view__day-header");
    await expect(async () => {
      await expect(headers).toHaveCount(5);
    }).toPass({ timeout: 15_000 });
  });
});
