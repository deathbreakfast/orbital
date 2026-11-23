import { test, expect } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table filter logic", () => {
  test("OR matches more rows than AND", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filter-logic-preview");
    const preview = page.getByTestId("data-table-filter-logic-preview");
    await preview.scrollIntoViewIfNeeded();

    const applyTwoRules = async (logic: "and" | "or") => {
      await preview.getByTestId("data-table-filter-panel-trigger").click();
      const panel = page.getByTestId("data-table-filter-panel");
      await selectPreviewOption(panel.getByTestId("data-table-filter-logic"), logic);
      await panel.getByTestId("data-table-filter-add-rule").click();

      await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
      await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
      await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "Admin");

      await selectPreviewOption(panel.getByTestId("data-table-filter-rule-1-field"), "name");
      await selectPreviewOption(panel.getByTestId("data-table-filter-rule-1-operator"), "contains");
      await fillPreviewInput(panel.getByTestId("data-table-filter-rule-1-value"), "User 1");

      await panel.getByRole("button", { name: "Apply" }).click();
    };

    await applyTwoRules("and");
    await expect(preview.locator("tbody tr")).toHaveCount(0);

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    await page.getByTestId("data-table-filter-panel").getByRole("button", { name: "Clear" }).click();

    await applyTwoRules("or");
    await expect(preview.locator("tbody tr").count()).resolves.toBeGreaterThan(0);
  });

  test("filter panel selects have accessible names", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filter-logic-preview");
    const preview = page.getByTestId("data-table-filter-logic-preview");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await panel.getByTestId("data-table-filter-add-rule").click();

    await expect(panel.getByTestId("data-table-filter-logic").locator("select")).toHaveAccessibleName("Match");
    await expect(panel.getByTestId("data-table-filter-rule-0-field").locator("select")).toHaveAccessibleName("Field");
    await expect(panel.getByTestId("data-table-filter-rule-0-operator").locator("select")).toHaveAccessibleName("Operator");
  });
});
