import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
import { fillSegment } from "../lib/preview/forms";
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
    await fillSegment(dateSegments.nth(0), "06");
    await fillSegment(dateSegments.nth(1), "14");
    await fillSegment(dateSegments.nth(2), "2025");
    await fillSegment(timeSegments.nth(0), "09");
    await fillSegment(timeSegments.nth(1), "30");
    await fillSegment(timeSegments.nth(2), "AM");
    await timeSegments.nth(2).blur();

    await expect(value).not.toHaveText("none");
  });
});
