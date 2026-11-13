import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle } from "../lib/assertions/style";
test.describe("card surface preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card");
    await expect(page.getByTestId("card-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card");
    await expectPreviewVariants(page, [
      "card-preview",
      "card-compound-preview",
      "card-with-header",
      "card-media-preview",
      "card-button-area-preview",
      "card-custom-hero-preview",
      "card-raised",
      "card-solid",
    ]);
  });

  test("default card is outlined and flat", async ({ page }) => {
    await openComponentPreview(page, "card");
    const root = page.getByTestId("card-preview").locator(".orbital-card.orbital-material").first();
    await expect(root).toBeVisible();
    await expect(root).toHaveAttribute("data-material-variant", "outlined");
    await expect(root).toHaveAttribute("data-material-elevation", "flat");
    const { shadow, border } = await root.evaluate((el) => {
      const cs = getComputedStyle(el);
      return { shadow: cs.boxShadow, border: cs.borderTopWidth };
    });
    expect(shadow).toBe("none");
    expect(parseFloat(border)).toBeGreaterThan(0);
  });

  test("raised card uses material elevated shadow", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-raised").scrollIntoViewIfNeeded();
    const defaultRoot = page.getByTestId("card-preview").locator(".orbital-material").first();
    const raisedRoot = page.getByTestId("card-raised").locator(".orbital-material").first();
    const defaultShadow = await defaultRoot.evaluate((el) => getComputedStyle(el).boxShadow);
    const raisedShadow = await raisedRoot.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(raisedShadow).not.toEqual(defaultShadow);
    await expect(raisedRoot).toHaveAttribute("data-material-elevation", "raised");
  });

  test("compound card contains all slot regions", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-compound-preview").scrollIntoViewIfNeeded();
    const compound = page.getByTestId("card-compound-preview");
    await expect(compound.locator(".orbital-card-header")).toBeVisible();
    await expect(compound.locator("img.orbital-card-media")).toBeVisible();
    await expect(compound.locator(".orbital-card-content")).toBeVisible();
    await expect(compound.locator(".orbital-card-footer")).toBeVisible();
  });

  test("media card shows image at expected height", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-media-preview").scrollIntoViewIfNeeded();
    const img = page.getByTestId("card-media-preview").locator(".orbital-card-media");
    await expect(img).toBeVisible();
    const height = await img.evaluate((el) => getComputedStyle(el).height);
    expect(parseFloat(height)).toBeCloseTo(140, 0);
    await expect(page.getByTestId("card-media-preview").locator(".orbital-card-content")).toBeVisible();
    await expect(page.getByTestId("card-media-preview").locator(".orbital-card-footer")).toBeVisible();
  });

  test("solid card opts out of the outline and keeps a resting shadow", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-solid").scrollIntoViewIfNeeded();
    const root = page.getByTestId("card-solid").locator(".orbital-material").first();
    await expect(root).toHaveAttribute("data-material-variant", "solid");
    await expect(root).toHaveAttribute("data-material-elevation", "resting");
    const shadow = await root.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(shadow).not.toBe("none");
  });
});
