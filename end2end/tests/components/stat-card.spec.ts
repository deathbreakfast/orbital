import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("stat-card preview", () => {
  test("SC-01: default stat card shows label and value", async ({ page }) => {
    await openComponentPreview(page, "stat-card");
    const preview = page.getByTestId("stat-card-preview");
    await expect(preview.getByText("Total Users")).toBeVisible();
    await expect(preview.getByText("1,234")).toBeVisible();
  });

  test("SC-02: dashboard example shows multiple metrics", async ({ page }) => {
    await openComponentPreview(page, "stat-card");
    await expect(page.getByText("Total Orders")).toBeVisible();
    await expect(page.getByText("$124.5K")).toBeVisible();
  });

  test("SC-03: children render as a footer under the value", async ({ page }) => {
    await openComponentPreview(page, "stat-card");
    const preview = page.getByTestId("stat-card-footer");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByText("Launch goal")).toBeVisible();
    const [value, footer] = await Promise.all([
      preview.getByText("68%").boundingBox(),
      preview.getByText("12 of 18 tasks done").boundingBox(),
    ]);
    expect(footer!.y).toBeGreaterThan(value!.y);
  });
});
