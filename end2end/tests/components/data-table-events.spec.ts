import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table event callbacks", () => {
  test("row, cell, sort, and selection events append to log", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-events-preview");
    const preview = page.getByTestId("data-table-events-preview");
    await preview.scrollIntoViewIfNeeded();
    const log = preview.getByTestId("event-log");

    await preview.getByText("Name").click();
    await expect(log).toContainText("sort:");

    await preview.getByTestId("data-table-row-1").click();
    await expect(log).toContainText("row_click: 1");

    await preview.getByTestId("data-table-cell-1-name").click();
    await expect(log).toContainText("cell_click: 1-name");

    const checkbox = preview.locator("input[type=checkbox]").nth(1);
    await checkbox.click({ force: true });
    await expect(log).toContainText("selection:");
  });

  test("a first click on a cell's text fires row and cell click", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-events-preview");
    const preview = page.getByTestId("data-table-events-preview");
    await preview.scrollIntoViewIfNeeded();
    const log = preview.getByTestId("event-log");

    // The press focuses the cell; the click must still reach the row and the cell.
    const cell = preview.getByTestId("data-table-cell-2-name");
    await cell.locator(".orbital-data-table__cell-focus").click();
    await expect(log).toContainText("row_click: 2");
    await expect(log).toContainText("cell_click: 2-name");
    await expect(cell.locator(".orbital-data-table__cell-focus")).toHaveAttribute("tabindex", "0");
  });
});
