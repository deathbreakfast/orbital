import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import {
  expectDrawerOpen,
  setViewportPreset,
} from "../lib/preview/mobile";

test.describe("layout responsive sidebar", () => {
  test("LAY-MOB-01: Auto @ phone uses overlay drawer", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-responsive-auto");
    await host.scrollIntoViewIfNeeded();
    await expect(host.getByTestId("layout-sidebar-drawer")).toHaveCount(0);
    await host.getByTestId("layout-sidebar-toggle").getByRole("button").click();
    await expectDrawerOpen(page);
    await expect(page.getByTestId("layout-responsive-nav")).toBeVisible();
  });

  test("LAY-MOB-02: Auto @ desktop keeps inline sidebar when open", async ({
    page,
  }) => {
    await setViewportPreset(page, "desktop");
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-responsive-auto");
    await host.scrollIntoViewIfNeeded();
    await host.getByTestId("layout-sidebar-toggle").getByRole("button").click();
    await expect(host.getByTestId("layout-responsive-nav")).toBeVisible();
    await expect(page.getByTestId("layout-sidebar-drawer")).toHaveCount(0);
  });

  test("LAY-MOB-01-sad: drawer dismiss closes", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-responsive-auto");
    await host.getByTestId("layout-sidebar-toggle").getByRole("button").click();
    await expectDrawerOpen(page);
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("layout-sidebar-drawer")).toBeHidden({
      timeout: 10_000,
    });
  });

  test("LAY-MOB-03: sidebar closed by default on phone", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-responsive-auto");
    await host.scrollIntoViewIfNeeded();
    await expect(page.getByTestId("layout-sidebar-drawer")).toHaveCount(0);
  });

  test("LAY-MOB-04: open drawer closes across breakpoint", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "layout");
    const host = page.getByTestId("layout-responsive-auto");
    await host.getByTestId("layout-sidebar-toggle").getByRole("button").click();
    await expectDrawerOpen(page);
    await setViewportPreset(page, "desktop");
    await expect(page.getByTestId("layout-sidebar-drawer")).toHaveCount(0, {
      timeout: 10_000,
    });
  });
});
