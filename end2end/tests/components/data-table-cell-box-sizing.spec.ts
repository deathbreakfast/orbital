import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("data-table cell sizing", () => {
  test("cells are border-box and columns do not overflow the measured width", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-sizing-preview");
    const preview = page.getByTestId("data-table-column-sizing-preview");
    await preview.scrollIntoViewIfNeeded();

    const header = preview.getByTestId("data-table-header-name");
    await expect(header).toBeVisible();
    await expect(header).toHaveCSS("box-sizing", "border-box");
    await expect(preview.locator("tbody td").first()).toHaveCSS("box-sizing", "border-box");

    const styleWidth = await header.evaluate((el) => parseFloat((el as HTMLElement).style.width));
    const renderedWidth = await header.evaluate((el) => el.getBoundingClientRect().width);
    expect(Math.abs(renderedWidth - styleWidth)).toBeLessThan(1);

    const { table, scroll } = await preview.evaluate((root) => {
      const scrollEl = root.querySelector('[data-testid="data-table-scroll"]') as HTMLElement;
      const tableEl = scrollEl.querySelector("table") as HTMLElement;
      return { table: tableEl.getBoundingClientRect().width, scroll: scrollEl.clientWidth };
    });
    expect(table).toBeLessThanOrEqual(scroll + 1);
  });
});
