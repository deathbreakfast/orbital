import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table overlays", () => {
  test("empty dataset shows empty overlay", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-overlays-preview");
    const preview = page.getByTestId("data-table-overlays-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-empty")).toBeVisible({ timeout: 10_000 });
  });

  test("skeleton loading rows keep headers and search usable", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-skeleton-preview");
    const preview = page.getByTestId("data-table-skeleton-preview");
    await preview.scrollIntoViewIfNeeded();
    const rows = preview.getByTestId("data-table-skeleton-row");
    await expect(rows).toHaveCount(4);
    // One placeholder cell per column, under the real headers.
    await expect(rows.first().locator("td")).toHaveCount(2);
    await expect(preview.locator("thead").getByText("Name")).toBeVisible();
    await expect(preview.locator("thead").getByText("Email")).toBeVisible();
    // No spinner overlay in skeleton mode.
    await expect(preview.getByTestId("data-table-loading")).toHaveCount(0);
    const search = preview.getByTestId("data-table-quick-search").locator("input");
    await search.fill("Ada");
    await expect(search).toHaveValue("Ada");
    await preview.getByTestId("skeleton-toggle-loading").click();
    await expect(rows).toHaveCount(0);
    await expect(preview.locator("tbody").getByText("Ada", { exact: true })).toBeVisible();
  });
});
