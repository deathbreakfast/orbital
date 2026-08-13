import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("overflow preview", () => {
  test("O-01 horizontal overflow", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    await expect(page.getByTestId("overflow-preview").locator(".orbital-overflow")).toBeVisible();
    await expect(page.getByTestId("overflow-preview").locator("button")).toHaveCount(5);
  });

  test("O-02 overflow menu affordance", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    await expectPreviewVariants(page, ["overflow-menu"]);
    const menu = page.getByTestId("overflow-menu");
    await expect(menu.getByTestId("overflow-menu-trigger")).toBeVisible();
    await menu.getByTestId("overflow-menu-trigger").click();
    await expect(page.getByRole("menuitem", { name: "Export" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Share" }).first()).toBeVisible();
  });

  test("O-03 horizontal overflow menu items", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    const preview = page.getByTestId("overflow-preview");
    await expect(preview.locator(".orbital-overflow")).toHaveAttribute("data-overflow", "true", {
      timeout: 10_000,
    });
    await expect(preview.getByTestId("overflow-menu-trigger")).toBeVisible();
    await preview.getByTestId("overflow-menu-trigger").click();
    await expect(page.getByRole("menuitem", { name: "Copy" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Paste" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Delete" }).first()).toBeVisible();
  });
});
