import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { clickMenuTriggerInPreview, hoverMenuTriggerInPreview, expectOverlayAnchoredNearTrigger, expectOverlayInViewport, expectOverlayNonZeroSize } from "../lib/preview/overlays";
test.describe("menu primitive preview", () => {

  test("MN-01: click opens default menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await clickMenuTriggerInPreview(page, "menu-preview", "Actions");
    await expect(page.getByRole("menuitem", { name: "Edit" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Delete" }).first()).toBeVisible();
    await expectOverlayNonZeroSize(page, ".orbital-menu");
    await expectOverlayInViewport(page, ".orbital-menu");
    await expectOverlayAnchoredNearTrigger(page, "menu-preview", ".orbital-menu", "Actions");
  });

  test("MN-02: icon trigger opens menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-icons").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-icons");
    await expect(page.getByRole("menuitem", { name: "Copy" }).first()).toBeVisible();
  });

  test("MN-03: hover opens menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-hover").scrollIntoViewIfNeeded();
    await hoverMenuTriggerInPreview(page, "menu-hover", "Hover me");
    await expect(page.getByRole("menuitem", { name: "Item A" }).first()).toBeVisible();
  });

  test("MN-04: brand menu surface uses brand modifier class", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-brand").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-brand", "Brand menu");
    await expect(page.locator(".orbital-menu--brand").first()).toBeVisible();
  });

  test("MN-05: inverted menu surface uses inverted modifier class", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-inverted").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-inverted", "Inverted");
    await expect(page.locator(".orbital-menu--inverted").first()).toBeVisible();
  });

  test("MN-06: top position opens menu above trigger", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-position-top").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-position-top", "Open above");
    await expect(page.locator('[data-orbital-placement="top"]').first()).toHaveAttribute("data-orbital-placement", "top");
    await expect(page.getByRole("menuitem", { name: "Above" }).first()).toBeVisible();
  });

  test("MN-07: click selects item and closes menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await clickMenuTriggerInPreview(page, "menu-preview", "Actions");
    await page.getByRole("menuitem", { name: "Edit" }).first().click();
    await expect(page.getByRole("menuitem", { name: "Edit" })).toHaveCount(0);
  });

  test("MN-08: disabled menu item cannot be selected", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-disabled").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-disabled", "Actions");
    const disabledItem = page.getByRole("menuitem", { name: "Blocked" });
    await expect(disabledItem).toHaveAttribute("aria-disabled", "true");
    await disabledItem.click({ force: true });
    await expect(disabledItem).toBeVisible();
  });

  test("MN-09: keyboard ArrowDown and Enter selects menu item", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-keyboard").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-keyboard", "Keyboard menu");
    await expect(page.getByRole("menuitem", { name: "Edit" })).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(page.getByRole("menuitem", { name: "Edit" })).toHaveCount(0);
  });

  test("MN-10: Enter on a focused item selects it and closes the menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-keyboard").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-keyboard", "Keyboard menu");
    const copy = page.getByRole("menuitem", { name: "Copy" });
    await expect(copy).toBeVisible();
    await copy.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByRole("menuitem", { name: "Copy" })).toHaveCount(0);
  });

  test("MN-11: Space on a focused item selects it and closes the menu", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-keyboard").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-keyboard", "Keyboard menu");
    const del = page.getByRole("menuitem", { name: "Delete" });
    await expect(del).toBeVisible();
    await del.focus();
    await page.keyboard.press(" ");
    await expect(page.getByRole("menuitem", { name: "Delete" })).toHaveCount(0);
  });

  test("MN-12: Enter on a focused disabled item keeps the menu open", async ({ page }) => {
    await openComponentPreview(page, "menu");
    await page.getByTestId("menu-disabled").scrollIntoViewIfNeeded();
    await clickMenuTriggerInPreview(page, "menu-disabled", "Actions");
    const blocked = page.getByRole("menuitem", { name: "Blocked" });
    await blocked.focus();
    await page.keyboard.press("Enter");
    await expect(blocked).toBeVisible();
  });
});
