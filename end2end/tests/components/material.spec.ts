import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle } from "../lib/assertions/style";
function materialRoot(page: import("@playwright/test").Page, testId: string) {
  return page.getByTestId(testId).locator(".orbital-material").first();
}

test.describe("material surface preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "material");
    await expect(page.getByTestId("material-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "material");
    await expectPreviewVariants(page, [
      "material-preview",
      "material-raised-preview",
      "material-variant-matrix",
      "material-elevation-matrix",
      "material-frost-preview",
      "material-var-override",
      "material-theme-elevation",
      "material-in-card",
      "material-outlined-preview",
      "material-square-preview",
      "material-appearance-matrix",
    ]);
  });

  test("M-01: solid variant data attribute", async ({ page }) => {
    await openComponentPreview(page, "material");
    const root = materialRoot(page, "material-preview");
    await expect(root).toHaveAttribute("data-material-variant", "solid");
    await expect(root).toHaveAttribute("data-material-elevation", "resting");
  });

  test("M-02: resting elevation applies shadow", async ({ page }) => {
    await openComponentPreview(page, "material");
    const root = materialRoot(page, "material-preview");
    const shadow = await root.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(shadow).not.toBe("none");
  });

  test("M-03: raised shadow differs from resting", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-raised-preview").scrollIntoViewIfNeeded();
    const resting = materialRoot(page, "material-preview");
    const raised = materialRoot(page, "material-raised-preview");
    const restingShadow = await resting.evaluate((el) => getComputedStyle(el).boxShadow);
    const raisedShadow = await raised.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(raisedShadow).not.toEqual(restingShadow);
  });

  test("M-04: flat elevation has no shadow", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-elevation-matrix").scrollIntoViewIfNeeded();
    await expectComputedStyle(page, "material-elevation-matrix", { "box-shadow": "none" }, {
      childSelector: '.orbital-material[data-material-elevation="flat"]',
    });
  });

  test("M-05: theme elevation scale affects resting shadow", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-theme-elevation").scrollIntoViewIfNeeded();
    const themed = materialRoot(page, "material-theme-elevation");
    const defaultRoot = materialRoot(page, "material-preview");
    const themedShadow = await themed.evaluate((el) => getComputedStyle(el).boxShadow);
    const defaultShadow = await defaultRoot.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(themedShadow).not.toEqual(defaultShadow);
  });

  test("M-06: outlined variant has border and no shadow", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-outlined-preview").scrollIntoViewIfNeeded();
    const root = materialRoot(page, "material-outlined-preview");
    await expect(root).toHaveAttribute("data-material-variant", "outlined");
    const styles = await root.evaluate((el) => {
      const cs = getComputedStyle(el);
      return { boxShadow: cs.boxShadow, borderWidth: cs.borderTopWidth };
    });
    expect(styles.boxShadow).toBe("none");
    expect(parseFloat(styles.borderWidth)).toBeGreaterThan(0);
  });

  test("M-07: square corners remove border radius", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-square-preview").scrollIntoViewIfNeeded();
    const root = materialRoot(page, "material-square-preview");
    await expect(root).toHaveAttribute("data-material-corners", "square");
    const radius = await root.evaluate((el) => getComputedStyle(el).borderRadius);
    expect(radius).toBe("0px");
  });

  test("card composition uses material surface", async ({ page }) => {
    await openComponentPreview(page, "material");
    await page.getByTestId("material-in-card").scrollIntoViewIfNeeded();
    const cardRoot = page.getByTestId("material-in-card").locator(".orbital-card.orbital-material").first();
    await expect(cardRoot).toBeVisible();
    await expect(cardRoot).toHaveAttribute("data-material-variant", "outlined");
    await expect(cardRoot).toHaveAttribute("data-material-elevation", "flat");
  });
});
