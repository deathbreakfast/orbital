import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("toggle-button primitive preview", () => {
  test("TB-01: renders default preview", async ({ page }) => {
    await openComponentPreview(page, "toggle-button");
    await expect(page.getByTestId("toggle-button-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("TB-02/TB-04: variant examples are visible", async ({ page }) => {
    await openComponentPreview(page, "toggle-button");
    await expectPreviewVariants(page, ["toggle-button-on", "toggle-button-click", "toggle-button-handler"]);
  });

  test("TB-03: clicking toggles aria-pressed", async ({ page }) => {
    await openComponentPreview(page, "toggle-button");
    const button = page.getByTestId("toggle-button-click").getByRole("button");
    await expect(button).toHaveAttribute("aria-pressed", "false");
    await button.click();
    await expect(button).toHaveAttribute("aria-pressed", "true");
  });

  test("TB-04: disabled toggle keeps its pressed state", async ({ page }) => {
    await openComponentPreview(page, "toggle-button");
    const button = page.getByTestId("toggle-button-disabled").getByRole("button");
    await button.scrollIntoViewIfNeeded();
    await expect(button).toBeDisabled();
    await expect(button).toHaveAttribute("aria-pressed", "true");
    await button.click({ force: true });
    await expect(button).toHaveAttribute("aria-pressed", "true");
  });
});
