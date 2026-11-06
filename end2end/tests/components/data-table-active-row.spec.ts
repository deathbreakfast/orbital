import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("data-table active row", () => {
  test("j and k move the active row without a selection column", async ({ page }) => {
    await openComponentPreview(page, "data-table-selection", "data-table-active-row-preview");
    const preview = page.getByTestId("data-table-active-row-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.locator("input[type=checkbox]")).toHaveCount(0);
    await expect(preview.getByTestId("data-table-row-1")).toHaveAttribute("aria-current", "true");

    await page.keyboard.press("j");
    await expect(preview.getByTestId("data-table-row-2")).toHaveClass(/orbital-data-table__row--active/);
    await expect(preview.getByTestId("data-table-row-1")).not.toHaveAttribute("aria-current", "true");

    await page.keyboard.press("k");
    await expect(preview.getByTestId("data-table-row-1")).toHaveClass(/orbital-data-table__row--active/);
  });
});
