import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("data-table list view", () => {
  test("renders card layout instead of table rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-list-view")).toBeVisible();
    await expect(preview.getByTestId("data-table-list-card-1")).toBeVisible();
    await expect(preview.getByTestId("data-table-list-card-2")).toBeVisible();
    await expect(preview.locator("tbody tr[data-testid^='data-table-row-']")).toHaveCount(0);
  });

  test("fires on_row_click when a list card is clicked", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-list-card-1").click();
    await expect(preview.getByTestId("data-table-list-view-log")).toHaveText("row_click:1");
  });

  test("exposes button semantics when on_row_click is set", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    const card = preview.getByTestId("data-table-list-card-1");
    await expect(card).toHaveAttribute("role", "button");
    await expect(card).toHaveAttribute("tabindex", "0");
  });

  test("fires on_row_click on Enter and Space", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    const card = preview.getByTestId("data-table-list-card-1");
    const log = preview.getByTestId("data-table-list-view-log");

    await card.focus();
    await page.keyboard.press("Enter");
    await expect(log).toHaveText("row_click:1");

    await card.focus();
    await page.keyboard.press("Space");
    await expect(log).toHaveText("row_click:1");
  });

  test("does not fire on_row_click when the selection checkbox is clicked", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-list-select-1").click({ force: true });
    await expect(preview.getByTestId("data-table-list-view-log")).toHaveText("");
  });

  test("switches to cards below the breakpoint without horizontal scroll", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await openComponentPreview(page, "data-table-advanced", "data-table-list-breakpoint-preview");
    const preview = page.getByTestId("data-table-list-breakpoint-preview");
    await preview.scrollIntoViewIfNeeded();
    const scroll = preview.getByTestId("data-table-scroll");

    await expect(preview.getByTestId("data-table-list-card-1")).toBeVisible();
    await expect(scroll).not.toHaveClass(/orbital-scroll-area--horizontal/);
    expect(await scroll.evaluate((el) => el.scrollWidth - el.clientWidth)).toBeLessThanOrEqual(1);

    await page.setViewportSize({ width: 1440, height: 900 });
    await expect(preview.locator("tbody tr[data-testid^='data-table-row-']")).toHaveCount(2);
    await expect(scroll).toHaveClass(/orbital-scroll-area--horizontal/);
  });
});
