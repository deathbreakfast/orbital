import { test, expect } from "@playwright/test";
import { fillPreviewInput, blurPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table header filters", () => {
  test("inline role filter keeps matching row", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-header-filters-preview");
    const preview = page.getByTestId("data-table-header-filters-preview");
    await preview.scrollIntoViewIfNeeded();

    const roleFilter = preview.getByTestId("data-table-header-filter-role");
    await fillPreviewInput(roleFilter, "Admin");
    await blurPreviewInput(roleFilter);

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr").first()).toContainText("Ada");
  });

  test("single-select column offers its choices and filters by one", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-header-filters-preview");
    const preview = page.getByTestId("data-table-header-filters-preview");
    await preview.scrollIntoViewIfNeeded();

    const status = preview.getByTestId("data-table-header-filter-status");
    await expect(status.getByRole("combobox")).toBeVisible();
    await expect(status.getByRole("option")).toHaveText(["All", "Active", "Archived"]);
    await selectPreviewOption(status, "Archived");

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr")).toContainText("Grace");
  });

  test("column without choices keeps a text box", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-header-filters-preview");
    const preview = page.getByTestId("data-table-header-filters-preview");
    await preview.scrollIntoViewIfNeeded();

    const role = preview.getByTestId("data-table-header-filter-role");
    await expect(role.getByRole("textbox")).toBeVisible();
    await expect(role.getByRole("combobox")).toHaveCount(0);
  });
});
