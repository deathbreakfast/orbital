import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("spinner primitive preview", () => {

  test("SP-01: default spinner exposes progressbar role", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    const spinner = page.getByTestId("spinner-preview").getByRole("progressbar");
    await expect(spinner).toBeVisible();
    const ring = page.getByTestId("spinner-preview").locator(".orbital-spinner__spinner");
    await expect(ring).toBeVisible();
    const box = await ring.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
  });

  test("SP-02: labeled spinner links aria-labelledby to label text", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    await page.getByTestId("spinner-labeled").scrollIntoViewIfNeeded();
    const spinner = page.getByTestId("spinner-labeled").getByRole("progressbar");
    const labelledby = await spinner.getAttribute("aria-labelledby");
    expect(labelledby).toBeTruthy();
    await expect(page.getByTestId("spinner-labeled").getByText("Loading data...")).toBeVisible();
  });

  test("SP-03: large spinner indicator is taller than default", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    await page.getByTestId("spinner-large").scrollIntoViewIfNeeded();
    // offsetHeight ignores the rotate animation; the bounding box of a spinning ring
    // grows by up to sqrt(2) depending on the frame it is sampled at.
    const defaultH = await page.getByTestId("spinner-preview").locator(".orbital-spinner__spinner")
      .evaluate((el) => (el as HTMLElement).offsetHeight);
    const largeH = await page.getByTestId("spinner-large").locator(".orbital-spinner__spinner")
      .evaluate((el) => (el as HTMLElement).offsetHeight);
    expect(largeH).toBeGreaterThan(defaultH);
  });

  test("SP-04: size matrix heights increase across presets", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    await page.getByTestId("spinner-sizes").scrollIntoViewIfNeeded();
    const tiny = page.getByTestId("spinner-size-tiny").locator(".orbital-spinner__spinner");
    const huge = page.getByTestId("spinner-size-huge").locator(".orbital-spinner__spinner");
    const tinyH = await tiny.evaluate((el) => (el as HTMLElement).offsetHeight);
    const hugeH = await huge.evaluate((el) => (el as HTMLElement).offsetHeight);
    expect(hugeH).toBeGreaterThan(tinyH);
  });

  test("SP-05: children label provides accessible name", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    await page.getByTestId("spinner-children").scrollIntoViewIfNeeded();
    const spinner = page.getByTestId("spinner-children").getByRole("progressbar");
    const labelledby = await spinner.getAttribute("aria-labelledby");
    expect(labelledby).toBeTruthy();
    await expect(page.getByTestId("spinner-children").getByText("Loading")).toBeVisible();
  });

  test("SP-06: theme preview spinner stroke uses computed color", async ({ page }) => {
    await openComponentPreview(page, "spinner");
    await page.getByTestId("spinner-theme").scrollIntoViewIfNeeded();
    const stroke = await page.getByTestId("spinner-theme").locator(".orbital-spinner__spinner").evaluate(
      (el) => getComputedStyle(el).borderTopColor,
    );
    expect(stroke).toMatch(/rgb\(\d+,\s*\d+,\s*\d+\)/);
    const animation = await page.getByTestId("spinner-theme").locator(".orbital-spinner__spinner").evaluate(
      (el) => getComputedStyle(el).animationName,
    );
    expect(animation).not.toBe("none");
  });
});
