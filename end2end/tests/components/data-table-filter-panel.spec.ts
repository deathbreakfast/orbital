import { test, expect } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table filter panel", () => {
  test("apply role filter keeps matching rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filter-panel-preview");
    const preview = page.getByTestId("data-table-filter-panel-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
    await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "Admin");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr").first()).toContainText("Ada");
  });

  test("single-select column offers its choices and filters by one", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filter-panel-preview");
    const preview = page.getByTestId("data-table-filter-panel-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "status");
    const value = panel.getByTestId("data-table-filter-rule-0-value");
    await expect(value.getByRole("combobox")).toBeVisible();
    await expect(value.getByRole("option")).toHaveText(["Any value", "Active", "Archived"]);
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
    await selectPreviewOption(value, "Archived");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr")).toContainText("Grace");
  });

  test("column without choices keeps a text box", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filter-panel-preview");
    const preview = page.getByTestId("data-table-filter-panel-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
    const value = panel.getByTestId("data-table-filter-rule-0-value");
    await expect(value.getByRole("textbox")).toBeVisible();
    await expect(value.getByRole("combobox")).toHaveCount(0);
  });
});
