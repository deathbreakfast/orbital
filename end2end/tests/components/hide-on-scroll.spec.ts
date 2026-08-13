import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { scrollScrollport } from "../lib/preview/overlays";

function pageScrollInHost(host: Locator) {
  return host.locator(".orbital-layout__page-scroll").first();
}

function enabledHost(page: Page) {
  return page.getByTestId("hide-on-scroll-preview");
}

/** True when the bar's bottom edge is at or above the scrollport's top (tucked). */
async function barTuckedAboveScrollport(appBar: Locator, scrollport: Locator) {
  const barBox = await appBar.boundingBox();
  const portBox = await scrollport.boundingBox();
  if (!barBox || !portBox) return false;
  return barBox.y + barBox.height <= portBox.y + 1;
}

/** True when the bar overlaps the top of the scrollport (visible sticky chrome). */
async function barPinnedAtScrollportTop(appBar: Locator, scrollport: Locator) {
  const barBox = await appBar.boundingBox();
  const portBox = await scrollport.boundingBox();
  if (!barBox || !portBox) return false;
  return Math.abs(barBox.y - portBox.y) <= 2;
}

test.describe("hide-on-scroll preview", () => {
  test("HOS-05: visible at scroll 0", async ({ page }) => {
    await openComponentPreview(page, "hide-on-scroll");
    const host = enabledHost(page);

    const scrollport = pageScrollInHost(host);
    const wrapper = host.getByTestId("hide-on-scroll");
    const appBar = host.locator("[data-testid='app-bar']");

    await expect(appBar).toBeVisible({ timeout: 30_000 });
    await expect(wrapper).not.toHaveAttribute("data-app-bar-scroll-hidden", "true");
    await expect.poll(async () => barPinnedAtScrollportTop(appBar, scrollport)).toBe(true);
  });

  test("HOS-01: scroll down hides sticky bar on page ScrollArea", async ({ page }) => {
    await openComponentPreview(page, "hide-on-scroll");
    const host = enabledHost(page);

    const scrollport = pageScrollInHost(host);
    await expect(scrollport).toBeVisible({ timeout: 30_000 });

    const wrapper = host.getByTestId("hide-on-scroll");
    const appBar = host.locator("[data-testid='app-bar']");

    await expect(wrapper).not.toHaveAttribute("data-app-bar-scroll-hidden", "true");

    await scrollScrollport(scrollport, 400);

    await expect
      .poll(async () => scrollport.evaluate((el) => el.scrollTop))
      .toBeGreaterThanOrEqual(400);

    await expect(wrapper).toHaveAttribute("data-app-bar-scroll-hidden", "true", {
      timeout: 10_000,
    });

    await expect.poll(async () => barTuckedAboveScrollport(appBar, scrollport)).toBe(true);
  });

  test("HOS-02: scroll up shows bar again", async ({ page }) => {
    await openComponentPreview(page, "hide-on-scroll");
    const host = enabledHost(page);

    const scrollport = pageScrollInHost(host);
    const wrapper = host.getByTestId("hide-on-scroll");
    const appBar = host.locator("[data-testid='app-bar']");

    await scrollScrollport(scrollport, 400);
    await expect(wrapper).toHaveAttribute("data-app-bar-scroll-hidden", "true", {
      timeout: 10_000,
    });

    await scrollScrollport(scrollport, 200);
    await expect(wrapper).not.toHaveAttribute("data-app-bar-scroll-hidden", "true", {
      timeout: 10_000,
    });

    await expect.poll(async () => barPinnedAtScrollportTop(appBar, scrollport)).toBe(true);
  });

  test("HOS-03: app bar stays mounted while hidden", async ({ page }) => {
    await openComponentPreview(page, "hide-on-scroll");
    const host = enabledHost(page);

    const scrollport = pageScrollInHost(host);
    await scrollScrollport(scrollport, 400);

    const wrapper = host.getByTestId("hide-on-scroll");
    await expect(wrapper).toHaveAttribute("data-app-bar-scroll-hidden", "true", {
      timeout: 10_000,
    });

    await expect(host.locator("[data-testid='app-bar']")).toHaveCount(1);
    await expect(host.getByTestId("hide-on-scroll-trailing-action")).toBeAttached();
  });

  test("HOS-04: enabled=false never tucks", async ({ page }) => {
    await openComponentPreview(page, "hide-on-scroll");
    const host = page.getByTestId("hide-on-scroll-disabled");

    const scrollport = pageScrollInHost(host);
    const wrapper = host.getByTestId("hide-on-scroll");
    const appBar = host.locator("[data-testid='app-bar']");

    await scrollScrollport(scrollport, 400);
    await expect
      .poll(async () => scrollport.evaluate((el) => el.scrollTop))
      .toBeGreaterThanOrEqual(400);

    await expect(wrapper).not.toHaveAttribute("data-app-bar-scroll-hidden", "true");
    await expect.poll(async () => barPinnedAtScrollportTop(appBar, scrollport)).toBe(true);
  });
});
