import { test, expect } from "@playwright/test";
import { selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column menu", () => {
  test("menu sort descending reorders rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Column menu" }).first().click();
    await page.getByRole("menuitem", { name: "Sort descending" }).click();

    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("Grace");
  });

  test("filter apply closes popover", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Filter column" }).first().click();
    await expect(page.getByRole("button", { name: "Apply" })).toBeVisible();

    await page.getByRole("button", { name: "Apply" }).click();
    await expect(page.getByRole("button", { name: "Apply" })).toHaveCount(0);
  });

  test("menu hide column removes header", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Column menu" }).nth(1).click();
    await page.getByRole("menuitem", { name: "Hide column" }).click();

    await expect(preview.getByTestId("data-table-header-role")).toHaveCount(0);
  });

  test("single-select filter offers its choices and filters by one", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-column-filter-status").click();
    const value = page.getByTestId("data-table-column-filter-value-status");
    await expect(value.getByRole("combobox")).toBeVisible();
    await expect(value.getByRole("option")).toHaveText(["Any value", "Active", "Archived"]);
    await selectPreviewOption(value, "Archived");
    await page.getByRole("button", { name: "Apply" }).click();

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr")).toContainText("Ada");
  });

  test("filter on a column without choices keeps a text box", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-column-filter-role").click();
    const panel = page.getByTestId("data-table-column-filter-panel-role");
    await expect(panel.getByRole("textbox")).toBeVisible();
    await expect(panel.getByTestId("data-table-column-filter-value-role")).toHaveCount(0);
  });
});
