import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
const VARIANTS = [
  "layout-with-sidebar",
  "layout-overlay-header",
  "layout-inset-header",
  "layout-app-shell",
  "layout-sidebar-toggle",
  "layout-sidebar-closed",
  "layout-responsive-auto",
];

test.describe("layout primitive preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "layout");
    await expect(page.getByTestId("layout-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "layout");
    await expectPreviewVariants(page, VARIANTS);
  });

  test("overlay header bar matches standard height", async ({ page }) => {
    await openComponentPreview(page, "layout");
    await page.getByTestId("layout-overlay-header").scrollIntoViewIfNeeded();

    const host = page.getByTestId("layout-overlay-header");
    const barRow = host.locator(".orbital-app-bar__row");

    await expect(barRow).toHaveCSS("height", "56px");
  });

  test("overlay main content starts below app bar at scroll 0", async ({ page }) => {
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-overlay-header");
    await host.scrollIntoViewIfNeeded();

    const appBar = host.locator("[data-testid='app-bar']");
    const main = host.locator(".orbital-layout__main");
    const firstLine = main.getByText("First line starts below the bar.");

    await expect(appBar).toBeVisible();
    await expect(firstLine).toBeAttached();

    const barBox = await appBar.boundingBox();
    const lineBox = await firstLine.boundingBox();
    expect(barBox).toBeTruthy();
    expect(lineBox).toBeTruthy();
    expect(lineBox!.y).toBeGreaterThanOrEqual(barBox!.y + barBox!.height - 1);
  });

  test("overlay main scrolls content behind header region", async ({ page }) => {
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-overlay-header");
    await host.scrollIntoViewIfNeeded();

    const scrollText = host.getByText("Scroll to see content pass under the frosted header.");
    const before = await scrollText.boundingBox();

    const scrollTop = await host.evaluate((el) => {
      el.scrollTop = 200;
      return el.scrollTop;
    });

    expect(scrollTop).toBeGreaterThanOrEqual(200);

    const after = await scrollText.boundingBox();
    expect(before).toBeTruthy();
    expect(after).toBeTruthy();
    expect(after!.y).toBeLessThan(before!.y);
  });

  test("inset mode main scroll starts below app bar", async ({ page }) => {
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-inset-header");
    await host.scrollIntoViewIfNeeded();

    const appBar = host.locator("[data-testid='app-bar']");
    const mainScroll = host.locator(".orbital-layout__main-scroll");
    const firstLine = mainScroll.getByText("First line starts below the bar.");

    await expect(appBar).toBeVisible();
    await expect(mainScroll).toBeVisible();
    await expect(firstLine).toBeAttached();

    const barBox = await appBar.boundingBox();
    const scrollBox = await mainScroll.boundingBox();
    const lineBox = await firstLine.boundingBox();
    expect(barBox).toBeTruthy();
    expect(scrollBox).toBeTruthy();
    expect(lineBox).toBeTruthy();
    expect(scrollBox!.y).toBeGreaterThanOrEqual(barBox!.y + barBox!.height - 1);
    expect(lineBox!.y).toBeGreaterThanOrEqual(barBox!.y + barBox!.height - 1);
  });
});
