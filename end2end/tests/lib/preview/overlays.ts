import type { Locator, Page } from "@playwright/test";
import { expect } from "@playwright/test";
import { documentScrollHeight } from "./navigation";

/** Set scrollTop on a scrollport and dispatch a scroll event (for inner ScrollArea repros). */
export async function scrollScrollport(scrollport: Locator, scrollTop: number) {
  await scrollport.evaluate((el, top) => {
    el.scrollTop = top;
    el.dispatchEvent(new Event("scroll", { bubbles: true }));
  }, scrollTop);
}

/** Assert a popover panel was dismissed (hidden or removed from DOM). */
export async function expectPopoverDismissed(content: Locator) {
  await expect(content).toBeHidden({ timeout: 10_000 });
}

/**
 * Assert a top-placed popover panel stays horizontally centered and vertically
 * adjacent to its trigger. Tight tolerance catches anchor drift after scroll.
 */
export async function expectPopoverTopAlignedWithTrigger(
  trigger: Locator,
  overlay: Locator,
  tolerancePx = 16,
) {
  await expect(trigger).toBeVisible({ timeout: 10_000 });
  await expect(overlay).toBeVisible({ timeout: 10_000 });

  const triggerBox = await trigger.boundingBox();
  const overlayBox = await overlay.boundingBox();
  expect(triggerBox).not.toBeNull();
  expect(overlayBox).not.toBeNull();

  const triggerCenterX = triggerBox!.x + triggerBox!.width / 2;
  const overlayCenterX = overlayBox!.x + overlayBox!.width / 2;
  expect(Math.abs(triggerCenterX - overlayCenterX)).toBeLessThanOrEqual(tolerancePx);

  const verticalGap = triggerBox!.y - (overlayBox!.y + overlayBox!.height);
  expect(verticalGap).toBeGreaterThanOrEqual(-4);
  expect(verticalGap).toBeLessThanOrEqual(tolerancePx);
}

/** Click a menu trigger inside a preview wrapper (menu surface is teleported). */
export async function clickMenuTriggerInPreview(
  page: Page,
  wrapperTestId: string,
  triggerName?: string,
) {
  const wrapper = page.getByTestId(wrapperTestId);
  const trigger = triggerName
    ? wrapper.getByRole("button", { name: triggerName })
    : wrapper.getByRole("button").first();
  await trigger.click();
}

/** Hover a menu trigger inside a preview wrapper (for hover-trigger menus). */
export async function hoverMenuTriggerInPreview(
  page: Page,
  wrapperTestId: string,
  triggerName: string,
) {
  const wrapper = page.getByTestId(wrapperTestId);
  await wrapper.getByRole("button", { name: triggerName }).hover();
}

/** Hover a tooltip trigger inside a preview wrapper (tooltip body is teleported). */
export async function hoverTooltipInPreview(
  page: Page,
  wrapperTestId: string,
  triggerName?: string,
) {
  const wrapper = page.getByTestId(wrapperTestId);
  const trigger = triggerName
    ? wrapper.getByRole("button", { name: triggerName })
    : wrapper.getByRole("button").first();
  await trigger.hover();
}

/** Assert a teleported element by ARIA role (and optional accessible name). */
export async function expectTeleportedRole(
  page: Page,
  role: Parameters<Page["getByRole"]>[0],
  name?: string | RegExp,
) {
  const locator = name !== undefined
    ? page.getByRole(role, { name })
    : page.getByRole(role);
  await expect(locator.first()).toBeVisible({ timeout: 10_000 });
}

/** Assert follower placement on a teleported panel (checks data-orbital-placement). */
export async function expectFollowerPlacement(
  page: Page,
  testId: string,
  placement: string,
) {
  const el = page.getByTestId(testId).first();
  await expect(el).toBeVisible({ timeout: 10_000 });

  const teleportedFollower = page
    .locator(`[data-orbital-placement="${placement}"]`)
    .filter({ has: el });
  if ((await teleportedFollower.count()) > 0) {
    await expect(teleportedFollower.first()).toHaveAttribute("data-orbital-placement", placement);
    return;
  }

  const ancestor = el.locator("xpath=ancestor::*[@data-orbital-placement][1]");
  await expect(ancestor).toHaveAttribute("data-orbital-placement", placement);
}

/** Assert an overlay element has non-zero dimensions (catches zero-size binder targets). */
export async function expectOverlayNonZeroSize(page: Page, selector: string) {
  const el = page.locator(selector).first();
  await expect(el).toBeVisible({ timeout: 10_000 });
  const box = await el.boundingBox();
  expect(box).not.toBeNull();
  expect(box!.width).toBeGreaterThan(0);
  expect(box!.height).toBeGreaterThan(0);
}

/** Assert a teleported overlay is anchored near its trigger (not stuck at viewport origin). */
export async function expectOverlayAnchoredNearTrigger(
  page: Page,
  wrapperTestId: string,
  overlaySelector: string,
  triggerName?: string,
) {
  const wrapper = page.getByTestId(wrapperTestId);
  const trigger = triggerName
    ? wrapper.getByRole("button", { name: triggerName })
    : wrapper.getByRole("button").first();
  const overlay = page.locator(overlaySelector).first();
  await expect(trigger).toBeVisible({ timeout: 10_000 });
  await expect(overlay).toBeVisible({ timeout: 10_000 });

  const triggerBox = await trigger.boundingBox();
  const overlayBox = await overlay.boundingBox();
  expect(triggerBox).not.toBeNull();
  expect(overlayBox).not.toBeNull();

  expect(overlayBox!.x).toBeGreaterThan(50);
  expect(overlayBox!.y).toBeGreaterThan(50);

  const horizontalDistance = Math.abs(
    overlayBox!.x + overlayBox!.width / 2 - (triggerBox!.x + triggerBox!.width / 2),
  );
  const verticalDistance = Math.abs(overlayBox!.y - triggerBox!.y);
  expect(horizontalDistance).toBeLessThan(250);
  expect(verticalDistance).toBeLessThan(500);
}

/** Assert an element's bounding box lies within the viewport. */
export async function expectOverlayInViewport(page: Page, selector: string) {
  const el = page.locator(selector).first();
  await expect(el).toBeVisible({ timeout: 10_000 });
  const box = await el.boundingBox();
  expect(box).not.toBeNull();
  const viewport = page.viewportSize();
  expect(viewport).not.toBeNull();
  expect(box!.x).toBeGreaterThanOrEqual(0);
  expect(box!.y).toBeGreaterThanOrEqual(0);
  expect(box!.x + box!.width).toBeLessThanOrEqual(viewport!.width + 1);
  expect(box!.y + box!.height).toBeLessThanOrEqual(viewport!.height + 1);
}

/** Assert opening an overlay does not grow document scroll height (extra scrollbar). */
export async function expectNoBodyScrollbarGrowth(
  page: Page,
  action: () => Promise<void>,
) {
  const before = await documentScrollHeight(page);
  await action();
  await page.waitForTimeout(100);
  const after = await documentScrollHeight(page);
  expect(after).toBeLessThanOrEqual(before + 1);
}
