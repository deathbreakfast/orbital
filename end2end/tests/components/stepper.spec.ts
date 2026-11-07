import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("stepper preview", () => {
  test("ST-01: default vertical stepper shows pipeline steps", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    const preview = page.getByTestId("stepper-preview");
    await expect(preview.getByText("Build Docker image")).toBeVisible();
    await expect(preview.getByText("Compressing layers...")).toBeVisible();
  });

  test("ST-02: failure variant shows error message", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    await expect(page.getByText("Connection refused")).toBeVisible();
  });

  test("ST-03: steps form a list with one current step", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    const preview = page.getByTestId("stepper-preview");
    await expect(preview.getByRole("list")).toBeVisible();
    await expect(preview.getByRole("listitem")).toHaveCount(4);
    await expect(preview.locator('[aria-current="step"]')).toHaveCount(1);
  });

  test("ST-04: reactive stepper moves the current step", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    const preview = page.getByTestId("stepper-reactive-preview");
    const current = preview.locator('[aria-current="step"]');
    await expect(current).toContainText("Account");
    await preview.getByRole("button", { name: "Next step" }).click();
    await expect(current).toContainText("Profile");
  });
});
