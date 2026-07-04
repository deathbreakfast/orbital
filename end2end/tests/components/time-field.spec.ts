import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("time-field preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "time-field");
    await expect(page.getByTestId("time-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "time-field");
    await expectPreviewVariants(page, ["time-field-preview"]);
  });

  test("typing segments updates bound value", async ({ page }) => {
    await openComponentPreview(page, "time-field", "TF-03");
    const wrapper = page.getByTestId("TF-03");
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("TF-03-VALUE");
    await expect(value).toHaveText("none");

    const segments = wrapper.locator(".orb-time-field__segment");
    await expect(segments).toHaveCount(3, { timeout: 30_000 });
    await scrollIntoPreviewView(segments.first());

    await expect(async () => {
      await segments.nth(0).click();
      await segments.nth(0).fill("02");
      await segments.nth(1).click();
      await segments.nth(1).fill("30");
      await segments.nth(2).click();
      await segments.nth(2).fill("PM");
      await segments.nth(2).blur();
      await expect(value).not.toHaveText("none");
    }).toPass({ timeout: 30_000 });
  });
});
