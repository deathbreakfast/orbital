import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { getCssVariable } from "../lib/assertions/style";
test.describe("theme primitive preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "theme");
    await expect(page.getByTestId("theme-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows theme designer variant", async ({ page }) => {
    await openComponentPreview(page, "theme");
    await expectPreviewVariants(page, ["theme-designer"]);
  });

  test("T-01: default light sample surface is visible", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const designer = page.getByTestId("theme-designer");
    await expect(designer.getByTestId("theme-sample-surface")).toBeVisible();
    const bg = await designer.getByTestId("theme-sample-surface").evaluate(
      (el) => getComputedStyle(el).backgroundColor,
    );
    expect(bg).toBeTruthy();
  });

  test("T-02: dark mode toggle changes sample surface background", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const designer = page.getByTestId("theme-designer");
    await designer.scrollIntoViewIfNeeded();
    const surface = designer.getByTestId("theme-sample-surface");
    const lightBg = await surface.evaluate((el) => getComputedStyle(el).backgroundColor);
    await designer.getByTestId("theme-mode-switch").getByRole("switch").click();
    const darkBg = await surface.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(darkBg).not.toEqual(lightBg);
  });

  test("T-03: brand color input updates brand button background", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const designer = page.getByTestId("theme-designer");
    await designer.scrollIntoViewIfNeeded();
    const btn = designer.getByTestId("theme-sample-brand-button").getByRole("button");
    const before = await btn.evaluate((el) => getComputedStyle(el).backgroundColor);
    const input = designer.getByTestId("theme-brand-input").locator("input");
    await input.click();
    await input.fill("");
    await input.pressSequentially("#FF0000");
    await input.dispatchEvent("input");
    // The theme applies on the next reactive tick; poll instead of reading once.
    await expect
      .poll(() => btn.evaluate((el) => getComputedStyle(el).backgroundColor))
      .not.toEqual(before);
  });

  test("T-04/T-05: density changes spaced row gap", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const designer = page.getByTestId("theme-designer");
    await designer.scrollIntoViewIfNeeded();
    const row = designer.getByTestId("theme-sample-spaced-row");
    const defaultGap = await row.evaluate((el) => getComputedStyle(el).gap);
    await designer.getByTestId("theme-density-decrease").getByRole("button").click();
    const compactGap = await row.evaluate((el) => getComputedStyle(el).gap);
    expect(compactGap).not.toEqual(defaultGap);
    await designer.getByTestId("theme-density-increase").getByRole("button").click();
    await designer.getByTestId("theme-density-increase").getByRole("button").click();
    const spaciousGap = await row.evaluate((el) => getComputedStyle(el).gap);
    expect(spaciousGap).not.toEqual(compactGap);
  });

  test("T-07: elevation control changes elevated card shadow", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const designer = page.getByTestId("theme-designer");
    await designer.scrollIntoViewIfNeeded();
    const card = designer.getByTestId("theme-sample-elevated-card");
    const before = await card.evaluate((el) => getComputedStyle(el).boxShadow);
    await designer.getByTestId("theme-elevation-increase").getByRole("button").click();
    const after = await card.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(after).not.toEqual(before);
  });

  test("F-T1: theme provider exposes orb spacing variables on scope", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const spacing = await getCssVariable(page, ".orbital-theme-provider", "--orb-space-inline-md");
    expect(spacing).toBeTruthy();
  });

  test("F-T2: theme provider exposes orb brand color token", async ({ page }) => {
    await openComponentPreview(page, "theme");
    const orbBrand = await getCssVariable(page, ".orbital-theme-provider", "--orb-color-brand-bg");
    expect(orbBrand).toBeTruthy();
  });
});
