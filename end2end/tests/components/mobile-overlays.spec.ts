import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import {
  expectOverlayMostlyInViewport,
  expectNoAnchoredPopover,
  setViewportPreset,
} from "../lib/preview/mobile";

test.describe("MOB / ADM overlays", () => {
  test("MOB-01: app-bar popover stays in viewport on phone", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "popover");
    const host = page.getByTestId("popover-appbar-stress");
    await host.scrollIntoViewIfNeeded();
    await host.getByRole("button", { name: "Platform" }).click();
    const panel = page.getByTestId("popover-appbar-stress-body");
    await expect(panel).toBeVisible({ timeout: 10_000 });
    const shell = page.locator(".orbital-positioning-content").filter({ has: panel });
    await expect(shell).toBeVisible({ timeout: 10_000 });
    await expect
      .poll(async () => {
        return shell.evaluate((el) => getComputedStyle(el).maxHeight);
      })
      .not.toBe("none");
    await expectOverlayMostlyInViewport(page, shell);
    expect((await shell.boundingBox())!.y).toBeGreaterThanOrEqual(0);
  });

  test("MOB-04: desktop popover opens under AppBar", async ({ page }) => {
    await setViewportPreset(page, "desktop");
    await openComponentPreview(page, "popover");
    const host = page.getByTestId("popover-appbar-stress");
    await host.scrollIntoViewIfNeeded();
    await host.getByRole("button", { name: "Platform" }).click();
    const panel = page.getByTestId("popover-appbar-stress-body");
    await expect(panel).toBeVisible({ timeout: 10_000 });
    const shell = page.locator(".orbital-positioning-content").filter({ has: panel });
    await expect(shell).toBeVisible({ timeout: 10_000 });
    await expectOverlayMostlyInViewport(page, shell);
  });

  test("ADM-02: phone opens drawer not clipped popover", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "adaptive-menu");
    const host = page.getByTestId("adaptive-menu-preview");
    await host.scrollIntoViewIfNeeded();
    await host.getByTestId("adaptive-menu-trigger").click();
    const drawer = page.getByTestId("adaptive-menu-drawer");
    await expect(drawer).toBeVisible({ timeout: 10_000 });
    await expectOverlayMostlyInViewport(page, drawer);
    await expect(
      page.getByTestId("adaptive-menu-drawer").getByTestId("adaptive-menu-body").first(),
    ).toBeVisible();
    await expectNoAnchoredPopover(page);
  });

  test("ADM-01: desktop opens popover", async ({ page }) => {
    await setViewportPreset(page, "desktop");
    await openComponentPreview(page, "adaptive-menu");
    const host = page.getByTestId("adaptive-menu-preview");
    await host.scrollIntoViewIfNeeded();
    await host.getByTestId("adaptive-menu-trigger").click();
    await expect(page.getByTestId("adaptive-menu-popover")).toBeVisible({
      timeout: 10_000,
    });
    await expectOverlayMostlyInViewport(
      page,
      page.getByTestId("adaptive-menu-popover"),
    );
  });

  test("ADM-02-sad: escape dismisses drawer", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "adaptive-menu");
    const host = page.getByTestId("adaptive-menu-preview");
    await host.getByTestId("adaptive-menu-trigger").click();
    await expect(page.getByTestId("adaptive-menu-drawer")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("adaptive-menu-drawer")).toBeHidden({
      timeout: 10_000,
    });
  });

  test("MOB-02: wide AppBar panel stays in viewport", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "popover");
    const host = page.getByTestId("popover-appbar-stress");
    await host.scrollIntoViewIfNeeded();
    await host.getByRole("button", { name: "Platform" }).click();
    const panel = page.getByTestId("popover-appbar-stress-body");
    await expect(panel).toBeVisible({ timeout: 10_000 });
    const shell = page.locator(".orbital-positioning-content").filter({ has: panel });
    await expect(shell).toBeVisible({ timeout: 10_000 });
    await expect
      .poll(async () => {
        return shell.evaluate((el) => getComputedStyle(el).maxHeight);
      })
      .not.toBe("none");
    await expectOverlayMostlyInViewport(page, shell);
    expect((await shell.boundingBox())!.x).toBeGreaterThanOrEqual(-2);
  });

  test("MOB-03: short viewport clamps tall click popover", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 600 });
    await openComponentPreview(page, "popover");
    const host = page.getByTestId("popover-appbar-stress");
    await host.scrollIntoViewIfNeeded();
    await host.getByRole("button", { name: "Platform" }).click();
    const panel = page.getByTestId("popover-appbar-stress-body");
    await expect(panel).toBeVisible({ timeout: 10_000 });
    const shell = page.locator(".orbital-positioning-content").filter({ has: panel });
    await expect(shell).toBeVisible({ timeout: 10_000 });
    await expect
      .poll(async () => {
        return shell.evaluate((el) => getComputedStyle(el).maxHeight);
      })
      .not.toBe("none");
    await expectOverlayMostlyInViewport(page, shell);
  });

  test("ADM-04: drawer body visible on phone", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "adaptive-menu");
    const host = page.getByTestId("adaptive-menu-preview");
    await host.getByTestId("adaptive-menu-trigger").click();
    const body = page
      .getByTestId("adaptive-menu-drawer")
      .getByTestId("adaptive-menu-body")
      .first();
    await expect(body).toBeVisible({ timeout: 10_000 });
    await expectOverlayMostlyInViewport(page, page.getByTestId("adaptive-menu-drawer"));
  });

  test("ADM-05: open drawer closes when resizing to desktop", async ({
    page,
  }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, "adaptive-menu");
    const host = page.getByTestId("adaptive-menu-preview");
    await host.getByTestId("adaptive-menu-trigger").click();
    await expect(page.getByTestId("adaptive-menu-drawer")).toBeVisible();
    await setViewportPreset(page, "desktop");
    await expect(page.getByTestId("adaptive-menu-drawer")).toHaveCount(0, {
      timeout: 10_000,
    });
    await expect(page.getByTestId("adaptive-menu-popover")).toHaveCount(0);
  });
});
