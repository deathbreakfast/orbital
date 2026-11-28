import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-field preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    await expect(page.getByTestId("date-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    await expectPreviewVariants(page, ["date-field-preview"]);
  });

  test("typing segments updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    const wrapper = page.getByTestId("date-field-preview").first();
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("date-field-preview-VALUE");
    await expect(value).toHaveText("none");

    const month = wrapper.getByTestId("date-field-segment-month");
    const day = wrapper.getByTestId("date-field-segment-day");
    const year = wrapper.getByTestId("date-field-segment-year");

    // A complete segment moves focus to the next one by itself (now and again 16ms
    // later), so wait for that instead of pressing Tab on top of it.
    await month.click();
    await month.fill("06");
    await expect(month).toHaveValue("06");
    await expect(day).toBeFocused();
    await day.fill("14");
    await expect(day).toHaveValue("14");
    await expect(year).toBeFocused();
    await year.fill("2025");
    await expect(year).toHaveValue("2025");
    await year.blur();

    await expect(value).not.toHaveText("none");
  });
});
