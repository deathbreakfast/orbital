import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("table primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "table");
  });

  test("TB-01 simple semantic table", async ({ page }) => {
    const table = page.getByTestId("table-preview").locator("table.orbital-table");
    await expect(table).toBeVisible();
    await expect(table.locator("thead")).toBeVisible();
    await expect(table.locator("tbody")).toBeVisible();
    await expect(page.getByTestId("table-cell-name")).toHaveText("Ada");
    await expect(page.getByTestId("table-cell-role")).toHaveText("Admin");
  });

  test("TB-02 full compound table", async ({ page }) => {
    await expectPreviewVariants(page, ["table-compound"]);
    const table = page.getByTestId("table-compound").locator("table");
    await expect(table.locator("th")).toHaveCount(3);
    await expect(table.locator("tbody tr")).toHaveCount(2);
  });

  test("TB-03 multiple body rows", async ({ page }) => {
    await expectPreviewVariants(page, ["table-rows"]);
    await expect(page.getByTestId("table-row-id-1")).toHaveText("1");
    await expect(page.getByTestId("table-row-id-2")).toHaveText("2");
    await expect(page.getByTestId("table-row-id-3")).toHaveText("3");
  });

  test("TB-04 truncated cell", async ({ page }) => {
    await expectPreviewVariants(page, ["table-truncate"]);
    const main = page.getByTestId("table-truncate").locator(".orbital-table-cell-layout__main");
    await expect(main).toHaveCSS("text-overflow", "ellipsis");
    await expect(main).toHaveCSS("overflow-x", "hidden");
  });

  test("TB-05 resizable columns", async ({ page }) => {
    await expectPreviewVariants(page, ["table-resizable"]);
    const wrapper = page.getByTestId("table-resizable");
    const handle = wrapper.locator(".orbital-table-resize-handle").first();
    await expect(handle).toBeAttached();
    const th = wrapper.locator("th").first();
    const before = await th.evaluate((el) => el.getBoundingClientRect().width);
    const box = await handle.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      const x = box.x + box.width / 2;
      const y = box.y + box.height / 2;
      await page.mouse.move(x, y);
      await page.mouse.down();
      await page.mouse.move(x - 50, y, { steps: 8 });
      await page.mouse.up();
    }
    await expect
      .poll(async () => th.evaluate((el) => el.getBoundingClientRect().width), {
        timeout: 5_000,
      })
      .toBeLessThan(before - 5);
  });

  test("TB-06 theme tokens on table", async ({ page }) => {
    await expectPreviewVariants(page, ["table-theme"]);
    const row = page.getByTestId("table-theme").locator(".orbital-table-row").first();
    await expectNonEmptyResolvedStyle(page, "table-theme", "border-bottom-color", {
      childSelector: ".orbital-table-row",
    });
    await expect(row).toBeVisible();
  });

  test("TB-07 status badges in cells", async ({ page }) => {
    await expectPreviewVariants(page, ["table-badges"]);
    const label = page.getByTestId("table-badge-active");
    await expect(label).toHaveText("Active");
    const badge = label.locator("xpath=ancestor::*[contains(@class,'orbital-badge')][1]");
    await expect(badge).toBeVisible();
    const box = await badge.boundingBox();
    expect(box?.width ?? 0).toBeGreaterThan(0);
  });

  test("TB-08 wide resizable admin grid", async ({ page }) => {
    await expectPreviewVariants(page, ["table-resizable-wide"]);
    await expect(page.getByTestId("table-resizable-wide").locator(".orbital-table-resize-handle")).toHaveCount(3);
  });

  test("TB-09 row actions with Button", async ({ page }) => {
    await expectPreviewVariants(page, ["table-actions"]);
    await expect(page.getByTestId("table-action-edit")).toHaveText("Edit");
  });
});
