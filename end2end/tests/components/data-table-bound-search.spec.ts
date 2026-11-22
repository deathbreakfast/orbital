import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("data-table bound quick search", () => {
  test("BQ-01 typing in the table writes the bound signal", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-bound-search-preview");
    const preview = page.getByTestId("data-table-bound-search-preview");
    await preview.scrollIntoViewIfNeeded();
    const search = preview.getByTestId("data-table-quick-search").locator("input");
    await search.fill("Row 1");
    await expect(preview.getByTestId("bound-search-value")).toHaveText("q=Row 1");
    // Row 1, Row 10 and Row 11 match.
    await expect(preview.locator("tbody tr")).toHaveCount(3);
  });

  test("BQ-02 setting the bound signal fills the search box and filters", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-bound-search-preview");
    const preview = page.getByTestId("data-table-bound-search-preview");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByTestId("bound-search-set").click();
    await expect(preview.getByTestId("data-table-quick-search").locator("input")).toHaveValue("Row 7");
    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody")).toContainText("Row 7");
  });
});
