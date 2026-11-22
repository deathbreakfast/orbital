import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("flex primitive preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "flex");
    await expect(page.getByTestId("flex-item-a")).toBeVisible();
    await expect(page.getByTestId("flex-item-b")).toBeVisible();
  });

  test("shows variant examples", async ({ page }) => {
    await openComponentPreview(page, "flex");
    await expectPreviewVariants(page, ["flex-vertical", "flex-align", "flex-justify"]);
  });

  test("vertical prop sets flex-direction column", async ({ page }) => {
    await openComponentPreview(page, "flex");
    const container = page.getByTestId("flex-vertical").locator(".orbital-flex");
    await expect(container).toHaveCSS("flex-direction", "column");
  });

  test("gap prop changes spacing", async ({ page }) => {
    await openComponentPreview(page, "flex");
    const rows = page.getByTestId("flex-gap-matrix").locator(".orbital-flex");
    const smallGap = await rows.first().evaluate((el) => getComputedStyle(el).gap);
    const largeGap = await rows.last().evaluate((el) => getComputedStyle(el).gap);
    expect(smallGap).not.toEqual(largeGap);
  });

  test("align prop affects cross-axis alignment", async ({ page }) => {
    await openComponentPreview(page, "flex");
    await page.getByTestId("flex-align").scrollIntoViewIfNeeded();
    const centerRow = page.getByTestId("flex-align").locator(".orbital-flex").nth(2);
    await expect(centerRow).toHaveCSS("align-items", "center");
  });

  test("FlexItem grow and basis split the row equally", async ({ page }) => {
    await openComponentPreview(page, "flex");
    const preview = page.getByTestId("flex-item-sizing");
    await preview.scrollIntoViewIfNeeded();
    const items = preview.locator(".orbital-flex-item");
    await expect(items).toHaveCount(3);
    await expect(items.nth(0)).toHaveCSS("flex-grow", "1");
    await expect(items.nth(0)).toHaveCSS("flex-basis", "0px");
    await expect(items.nth(2)).toHaveCSS("flex-shrink", "0");
    const [a, b] = await Promise.all([items.nth(0).boundingBox(), items.nth(1).boundingBox()]);
    expect(Math.abs(a!.width - b!.width)).toBeLessThan(1);
  });
});
