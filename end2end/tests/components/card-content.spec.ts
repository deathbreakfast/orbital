import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-content preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-content");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    await expectPreviewVariants(page, ["card-content-default", "card-content-stacked"]);
  });

  test("content that opens a card is padded on every side", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    const content = page.getByTestId("card-content-default").locator(".orbital-card-content");
    await expect(content).toBeVisible();
    const pad = await content.evaluate((el) => {
      const cs = getComputedStyle(el);
      return [cs.paddingTop, cs.paddingRight, cs.paddingBottom, cs.paddingLeft].map(parseFloat);
    });
    expect(pad).toEqual([16, 16, 16, 16]);
  });

  test("stacked content below media", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    await page.getByTestId("card-content-stacked").scrollIntoViewIfNeeded();
    const stacked = page.getByTestId("card-content-stacked");
    await expect(stacked.locator("img.orbital-card-media")).toBeVisible();
    const content = stacked.locator(".orbital-card-content");
    await expect(content).toBeVisible();
    await expect(stacked.locator(".orbital-card-footer")).toBeVisible();
    // Below media the gap spaces the two slots, so the body drops its top padding.
    const top = await content.evaluate((el) => parseFloat(getComputedStyle(el).paddingTop));
    expect(top).toBe(0);
  });
});
