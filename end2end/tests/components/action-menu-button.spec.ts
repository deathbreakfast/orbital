import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("action-menu-button preview", () => {
  test("AMB-01 default split control", async ({ page }) => {
    await openComponentPreview(page, "action-menu-button");
    await expect(page.getByTestId("action-menu-button-preview").locator("button")).toHaveCount(2);
    await expect(
      page.getByTestId("action-menu-button-preview").getByRole("button", { name: "Save" }),
    ).toBeVisible();
  });

  test("AMB-02 menu trigger keeps rounded right edge", async ({ page }) => {
    await openComponentPreview(page, "action-menu-button");
    const menuButton = page
      .getByTestId("action-menu-button-preview")
      .locator(".orbital-action-menu-button__menu");
    await expect(menuButton).toBeVisible();

    const radii = await menuButton.evaluate((el) => {
      const style = getComputedStyle(el);
      return {
        topRight: style.borderTopRightRadius,
        bottomRight: style.borderBottomRightRadius,
        topLeft: style.borderTopLeftRadius,
      };
    });

    expect(radii.topRight).not.toBe("0px");
    expect(radii.bottomRight).not.toBe("0px");
    expect(radii.topRight).toBe(radii.bottomRight);
    expect(radii.topLeft).toBe("0px");
  });

  test("AMB-03 with icon", async ({ page }) => {
    await openComponentPreview(page, "action-menu-button");
    await expectPreviewVariants(page, ["action-menu-button-icon"]);
    await expect(
      page.getByTestId("action-menu-button-icon").locator(".orbital-button__icon").first(),
    ).toBeVisible();
  });

  test("AMB-04 chevron menu button has an accessible name", async ({ page }) => {
    await openComponentPreview(page, "action-menu-button");
    const menu = page
      .getByTestId("action-menu-button-preview")
      .getByRole("button", { name: "More options" });
    await expect(menu).toBeVisible();
    await expect(menu).toHaveClass(/orbital-action-menu-button__menu/);
    await expect(menu).toHaveAttribute("aria-haspopup", "menu");
    await expect(
      page.getByTestId("action-menu-button-icon").getByRole("button", { name: "More save options" }),
    ).toBeVisible();
  });
});
