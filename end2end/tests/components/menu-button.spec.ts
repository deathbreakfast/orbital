import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { clickMenuTriggerInPreview, expectOverlayAnchoredNearTrigger } from "../lib/preview/overlays";
test.describe("menu-button primitive preview", () => {

  test("MB-01: click opens default menu button menu", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    await clickMenuTriggerInPreview(page, "menu-button-preview", "Actions");
    await expect(page.getByRole("menuitem", { name: "Edit" })).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Share" })).toBeVisible();
    await expectOverlayAnchoredNearTrigger(
      page,
      "menu-button-preview",
      ".orbital-menu",
      "Actions",
    );
  });

  test("MB-02: primary appearance differs from default", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    const defaultBtn = page.getByTestId("menu-button-preview").getByRole("button");
    await page.getByTestId("menu-button-primary").scrollIntoViewIfNeeded();
    const primaryBtn = page.getByTestId("menu-button-primary").getByRole("button");
    const defaultBg = await defaultBtn.evaluate((el) => getComputedStyle(el).backgroundColor);
    const primaryBg = await primaryBtn.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(primaryBg).not.toEqual(defaultBg);
  });

  test("MB-03: custom label text renders on trigger", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    await page.getByTestId("menu-button-label").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("menu-button-label").getByRole("button", { name: "Options" })).toBeVisible();
  });

  test("MB-04: selecting item closes menu", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    await clickMenuTriggerInPreview(page, "menu-button-preview", "Actions");
    await page.getByRole("menuitem", { name: "Share" }).click();
    await expect(page.getByRole("menuitem", { name: "Share" })).toBeHidden();
  });

  test("MB-05: trigger includes trailing chevron", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    await expect(
      page.getByTestId("menu-button-preview").locator(".orbital-menu-button__chevron"),
    ).toBeVisible();
  });

  test("MB-06: icon-only trigger is named and opens its menu", async ({ page }) => {
    await openComponentPreview(page, "menu-button");
    const root = page.getByTestId("menu-button-icon");
    await root.scrollIntoViewIfNeeded();
    const trigger = root.getByRole("button", { name: "More actions" });
    await expect(trigger).toBeVisible();
    await expect(root.locator(".orbital-menu-button__chevron")).toHaveCount(0);
    await trigger.click();
    await expect(page.getByRole("menuitem", { name: "Rename" })).toBeVisible();
  });
});
