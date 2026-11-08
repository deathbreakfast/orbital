import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("dropdown primitive preview", () => {

  test("DD-01: basic dropdown changes selection", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    const select = page.getByTestId("dropdown-preview").locator("select");
    await select.selectOption("b");
    await expect(select).toHaveValue("b");
  });

  test("DD-02: disabled dropdown", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    await page.getByTestId("dropdown-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("dropdown-disabled").locator("select")).toBeDisabled();
  });

  test("DD-03: dropdown in field associates label", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    await page.getByTestId("dropdown-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("dropdown-field").locator("label").getAttribute("for");
    const selectId = await page.getByTestId("dropdown-field").locator("select").getAttribute("id");
    expect(labelFor).toEqual(selectId);
    await expect(page.getByTestId("dropdown-field").locator("label")).toContainText("Status");
  });

  test("DD-SEC-01: dropdown shows the selected option's secondary text", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    const root = page.getByTestId("dropdown-secondary");
    await root.scrollIntoViewIfNeeded();
    await expect(root.locator(".orbital-select__secondary")).toHaveText("Shared hardware, 2 workers");
    await root.locator("select").selectOption("b");
    await expect(root.locator(".orbital-select__secondary")).toHaveText("Reserved 8 vCPU, 5 workers");
  });
});
