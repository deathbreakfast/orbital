import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("breadcrumb primitive preview", () => {
  test("BC-01 default trail item count", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    const nav = page.getByTestId("breadcrumb-preview").locator("nav.orbital-breadcrumb");
    await expect(nav).toBeVisible();
    const items = nav.locator(".orbital-breadcrumb-item");
    await expect(items).toHaveCount(2);
    await expect(nav.locator(".orbital-breadcrumb-divider")).toHaveCount(1);
  });

  test("BC-01b trail is a labelled navigation landmark", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    const nav = page.getByTestId("breadcrumb-preview").getByRole("navigation", { name: "Breadcrumb" });
    await expect(nav).toBeVisible();
    await expect(nav.locator(".orbital-breadcrumb-item")).toHaveCount(2);
  });

  test("BC-02 current page aria", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    await expectPreviewVariants(page, ["breadcrumb-current"]);
    const current = page
      .getByTestId("breadcrumb-current")
      .locator(".orbital-breadcrumb-button--current");
    await expect(current).toHaveAttribute("aria-current", "page");
    await expect(current).toHaveAttribute("aria-disabled", "true");
    await expect(current).toHaveText("Counter");
  });

  test("BC-03 long multi-level trail", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    await expectPreviewVariants(page, ["breadcrumb-long"]);
    const nav = page.getByTestId("breadcrumb-long").locator("nav.orbital-breadcrumb");
    await expect(nav.locator(".orbital-breadcrumb-item")).toHaveCount(4);
    await expect(nav.locator(".orbital-breadcrumb-divider")).toHaveCount(3);
    await expect(nav.getByRole("button", { name: "Keyboards" })).toHaveClass(
      /orbital-breadcrumb-button--current/,
    );
  });

  test("BC-04 link inside breadcrumb item", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    await expectPreviewVariants(page, ["breadcrumb-link"]);
    const link = page.getByTestId("breadcrumb-link").locator("a.orbital-link");
    await expect(link).toHaveAttribute("href", "#home");
    await expect(link).toHaveText("Home");
  });

  test("BC-05 theme button background token", async ({ page }) => {
    await openComponentPreview(page, "breadcrumb");
    await expectPreviewVariants(page, ["breadcrumb-theme"]);
    await expectNonEmptyResolvedStyle(page, "breadcrumb-theme", "color", {
      childSelector: ".orbital-breadcrumb-button",
    });
  });
});
