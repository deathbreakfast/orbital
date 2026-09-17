import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNoOverlappingLabels } from "../lib/assertions/layout";
test.describe("charts-axis preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    await expect(page.getByTestId("charts-axis-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    await expectPreviewVariants(page, ["charts-axis-preview", "charts-axis-dense-preview"]);
  });

  test("renders axis ticks and grid lines", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    const preview = page.getByTestId("charts-axis-preview");
    await expect(preview.locator(".orb-axis-tick-label").first()).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orb-grid-line")).not.toHaveCount(0, { timeout: 10_000 });
  });

  test("pw-orbital-charts-axis-sparse-no-overlap-happy: 4-category axis stays unrotated", async ({ page }) => {
    // Regression guard: plenty of bandwidth for 4 quarters should never trigger rotation —
    // over-rotating a sparse axis would itself be a bug the dense-axis fix could introduce.
    await openComponentPreview(page, "charts-axis");
    const preview = page.getByTestId("charts-axis-preview");
    const labels = preview.locator(".orb-axis-tick-label");
    await expect(labels.first()).toBeVisible({ timeout: 30_000 });
    const count = await labels.count();
    for (let i = 0; i < count; i++) {
      await expect(labels.nth(i)).not.toHaveAttribute("transform", /rotate/);
    }
    await expectNoOverlappingLabels(preview);
  });

  test("pw-orbital-charts-axis-dense-no-overlap-happy: 24-bucket axis has no overlapping labels", async ({ page }) => {
    // The bug this plan fixes: 24 hourly buckets in a 320px chart used to draw every label
    // horizontal and full-width, overlapping into an unreadable smear.
    await openComponentPreview(page, "charts-axis");
    const preview = page.getByTestId("charts-axis-dense-preview");
    await expect(preview.locator(".orb-axis-tick-label").first()).toBeVisible({ timeout: 30_000 });
    await expectNoOverlappingLabels(preview);
  });
});
