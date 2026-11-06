import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column sizing", () => {
  test("resize handle changes column width style", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-sizing-preview");
    const preview = page.getByTestId("data-table-column-sizing-preview");
    await preview.scrollIntoViewIfNeeded();

    const nameHeader = preview.getByTestId("data-table-header-name");
    const handle = nameHeader.locator(".orbital-table-resize-handle");
    await expect(handle).toBeVisible();

    const styleBefore = await nameHeader.getAttribute("style");
    const handleBox = await handle.boundingBox();
    expect(handleBox).not.toBeNull();

    const startX = handleBox!.x + handleBox!.width / 2;
    const startY = handleBox!.y + handleBox!.height / 2;
    await handle.dispatchEvent("pointerdown", { clientX: startX, clientY: startY, button: 0, pointerId: 1, pointerType: "mouse" });
    await page.mouse.move(startX + 80, startY, { steps: 10 });
    await page.dispatchEvent("body", "pointerup", { clientX: startX + 80, clientY: startY, button: 0, pointerId: 1, pointerType: "mouse" });

    await expect
      .poll(async () => nameHeader.getAttribute("style"))
      .not.toBe(styleBefore);
  });

  test("columns fit the table width without sideways scroll", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-sizing-preview");
    const preview = page.getByTestId("data-table-column-sizing-preview");
    await preview.scrollIntoViewIfNeeded();

    const scroll = preview.getByTestId("data-table-scroll");
    const { scrollWidth, clientWidth } = await scroll.evaluate((el) => ({
      scrollWidth: el.scrollWidth,
      clientWidth: el.clientWidth,
    }));
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth + 1);

    const headerWidths = await preview
      .locator("thead th")
      .evaluateAll((cells) => cells.map((c) => c.getBoundingClientRect().width));
    const tableWidth = await preview
      .locator("table")
      .first()
      .evaluate((t) => t.getBoundingClientRect().width);
    const sum = headerWidths.reduce((a, b) => a + b, 0);
    expect(Math.abs(sum - tableWidth)).toBeLessThanOrEqual(1);
  });
});
