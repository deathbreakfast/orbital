import { test, expect, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

// DataTableToolbarEnd: product controls on the default toolbar's search row.
test.describe("data-table toolbar end slot", () => {
  const open = async (page: Page) => {
    await openComponentPreview(page, "data-table-slots", "data-table-toolbar-end-preview");
    const preview = page.getByTestId("data-table-toolbar-end-preview");
    await preview.scrollIntoViewIfNeeded();
    return preview;
  };

  test("TE-01: slot content shares the search row, between search and Filters", async ({ page }) => {
    const preview = await open(page);
    const toolbar = preview.getByTestId("data-table-toolbar");
    const search = preview.getByTestId("data-table-quick-search");
    const end = preview.getByTestId("data-table-toolbar-end");
    const filters = preview.getByTestId("data-table-filter-panel-trigger");
    await expect(end).toBeVisible();
    await expect(end.getByRole("button", { name: "Advanced" })).toBeVisible();
    await expect(end.locator("select")).toBeVisible();

    const [s, e, f] = await Promise.all([search.boundingBox(), end.boundingBox(), filters.boundingBox()]);
    expect(s && e && f).toBeTruthy();
    const mid = (b: { y: number; height: number }) => b.y + b.height / 2;
    // One row: vertical centres within a few pixels of each other.
    expect(Math.abs(mid(s!) - mid(e!))).toBeLessThan(12);
    expect(Math.abs(mid(e!) - mid(f!))).toBeLessThan(12);
    // Search at the start, a gap, then the slot, then the built-in Filters.
    expect(e!.x).toBeGreaterThan(s!.x + s!.width + 16);
    expect(f!.x).toBeGreaterThanOrEqual(e!.x + e!.width);

    const padTop = await toolbar.evaluate((el) => parseFloat(getComputedStyle(el).paddingTop));
    expect(padTop).toBeGreaterThan(0);
  });

  test("TE-02: built-in buttons have no raised toolbar surface", async ({ page }) => {
    const preview = await open(page);
    const surface = preview.locator(".orbital-data-table__toolbar-actions .orbital-toolbar").first();
    await expect(surface).toBeVisible();
    const { bg, minHeight } = await surface.evaluate((el) => {
      const cs = getComputedStyle(el);
      return { bg: cs.backgroundColor, minHeight: cs.minHeight };
    });
    expect(bg).toBe("rgba(0, 0, 0, 0)");
    expect(minHeight).toBe("0px");
  });

  test("TE-03: the row wraps at phone width with search first", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const preview = await open(page);
    const search = preview.getByTestId("data-table-quick-search");
    const end = preview.getByTestId("data-table-toolbar-end");
    await expect(end).toBeVisible();
    const [s, e] = await Promise.all([search.boundingBox(), end.boundingBox()]);
    expect(e!.y).toBeGreaterThanOrEqual(s!.y + s!.height - 1);
    const overflow = await preview.evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
  });
});
