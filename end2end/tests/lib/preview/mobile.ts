import type { Locator, Page } from "@playwright/test";
import { expect } from "@playwright/test";

/** Assert an element's box is mostly inside the viewport (rejects off-screen Top). */
export async function expectOverlayMostlyInViewport(
  page: Page,
  locator: Locator,
  tolerancePx = 4,
) {
  await expect(locator).toBeVisible({ timeout: 10_000 });
  const box = await locator.boundingBox();
  expect(box).not.toBeNull();
  const viewport = page.viewportSize();
  expect(viewport).not.toBeNull();
  expect(box!.y).toBeGreaterThanOrEqual(-tolerancePx);
  expect(box!.x).toBeGreaterThanOrEqual(-tolerancePx);
  expect(box!.y + box!.height).toBeLessThanOrEqual(viewport!.height + tolerancePx);
  expect(box!.x + box!.width).toBeLessThanOrEqual(viewport!.width + tolerancePx);
}

/** Assert layout sidebar drawer is open and in viewport. */
export async function expectDrawerOpen(page: Page, testId = "layout-sidebar-drawer") {
  const drawer = page.getByTestId(testId);
  await expect(drawer).toBeVisible({ timeout: 10_000 });
  await expectOverlayMostlyInViewport(page, drawer);
}

/** Assert no anchored popover panel is visible. */
export async function expectNoAnchoredPopover(page: Page) {
  await expect(page.locator(".orbital-popover-shell").first()).toBeHidden({
    timeout: 5_000,
  });
}

export type ViewportPreset = "phone" | "tablet" | "desktop";

const PRESETS: Record<ViewportPreset, { width: number; height: number }> = {
  phone: { width: 390, height: 844 },
  tablet: { width: 768, height: 1024 },
  desktop: { width: 1280, height: 720 },
};

export async function setViewportPreset(page: Page, preset: ViewportPreset) {
  await page.setViewportSize(PRESETS[preset]);
}
