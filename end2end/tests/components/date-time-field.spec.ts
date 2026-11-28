import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("date-time-field preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-time-field");
    await expect(page.getByTestId("date-time-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-time-field");
    await expectPreviewVariants(page, ["date-time-field-preview"]);
  });

  test("typing segments updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-time-field", "DTF-02");
    const wrapper = page.getByTestId("DTF-02");
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("DTF-02-VALUE");
    await expect(value).toHaveText("none");

    const dateSegments = wrapper.locator(".orb-date-field__segment");
    const timeSegments = wrapper.locator(".orb-time-field__segment");
    await expect(dateSegments).toHaveCount(3, { timeout: 30_000 });
    await expect(timeSegments).toHaveCount(3);
    await scrollIntoPreviewView(dateSegments.first());
    await dateSegments.nth(0).fill("06");
    await dateSegments.nth(1).fill("14");
    await dateSegments.nth(2).fill("2025");
    await timeSegments.nth(0).fill("09");
    await timeSegments.nth(1).fill("30");
    await timeSegments.nth(2).fill("AM");
    await timeSegments.nth(2).blur();

    await expect(value).not.toHaveText("none");
  });
});
