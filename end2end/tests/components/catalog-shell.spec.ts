import { test, expect } from "@playwright/test";
import {
  clickCatalogNavLink,
  openComponentPreview,
  previewMainScrollTop,
  scrollPreviewMain,
} from "../lib/preview/navigation";
import { getCssVariable } from "../lib/assertions/style";
import { previewUrl } from "../helpers";

test.describe("preview catalog shell", () => {
  test("renders shell with sidebar nav", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
    await expect(page.getByTestId("preview-catalog-nav")).toBeVisible();
    await expect(page.getByTestId("preview-index")).toBeVisible();
  });

  test("navigates to component preview from sidebar", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    await clickCatalogNavLink(page, ["Core Components", "Card"], "Card");

    await expect(page).toHaveURL(/\/card$/);
    await openComponentPreview(page, "card");
  });

  test("nested catalog links indent deeper than group folder headers", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const nav = page.getByTestId("preview-catalog-nav");
    await nav.getByRole("button", { name: "Core Components", exact: true }).click();
    await nav.getByRole("button", { name: "Card", exact: true }).click();

    const groupHeader = nav.getByRole("button", { name: "Card", exact: true });
    const cardLink = nav.getByRole("link", { name: "Card", exact: true });

    const headerPad = await groupHeader.evaluate(
      (el) => parseFloat(getComputedStyle(el).paddingInlineStart) || 0,
    );
    const linkPad = await cardLink.evaluate(
      (el) => parseFloat(getComputedStyle(el).paddingInlineStart) || 0,
    );
    expect(linkPad).toBeGreaterThan(headerPad);
  });

  test("sidebar nav starts below app bar", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const appBar = page.getByTestId("app-bar");
    const firstNav = page.getByTestId("preview-catalog-nav").getByRole("link", { name: "Introduction" });

    await expect(appBar).toBeVisible();
    await expect(firstNav).toBeVisible();

    const barBox = await appBar.boundingBox();
    const navBox = await firstNav.boundingBox();
    expect(barBox).toBeTruthy();
    expect(navBox).toBeTruthy();
    expect(navBox!.y).toBeGreaterThanOrEqual(barBox!.y + barBox!.height - 1);
  });

  test("page title is not obscured by app bar at scroll 0", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const appBar = page.getByTestId("app-bar");
    const title = page.getByTestId("preview-page-title");

    await expect(appBar).toBeVisible();
    await expect(title).toBeVisible();
    await expect(title).toHaveText("Introduction");

    const barBox = await appBar.boundingBox();
    const titleBox = await title.boundingBox();
    expect(barBox).toBeTruthy();
    expect(titleBox).toBeTruthy();
    expect(titleBox!.y).toBeGreaterThanOrEqual(barBox!.y + barBox!.height - 1);
  });

  test("theme lives under getting started section", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
    const nav = page.getByTestId("preview-catalog-nav");
    await expect(nav).toContainText("Getting Started");
    const gettingStarted = nav.getByRole("button", { name: "Getting Started", exact: true });
    if ((await gettingStarted.getAttribute("aria-expanded")) !== "true") {
      await gettingStarted.click();
    }
    await expect(nav.getByRole("link", { name: "Theme" })).toBeVisible();
    await expect(nav.getByRole("button", { name: "Theme" })).toHaveCount(0);
  });

  test("boot loader lives under getting started section", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
    const nav = page.getByTestId("preview-catalog-nav");
    await expect(nav).toContainText("Getting Started");
    const gettingStarted = nav.getByRole("button", { name: "Getting Started", exact: true });
    if ((await gettingStarted.getAttribute("aria-expanded")) !== "true") {
      await gettingStarted.click();
    }
    await expect(nav.getByRole("link", { name: "Boot loader" })).toBeVisible();
    await expect(nav.getByRole("button", { name: "Boot loader" })).toHaveCount(0);
  });

  test("toolbar social icon buttons render in app bar", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    for (const testId of [
      "preview-toolbar-link-sponsor",
      "preview-toolbar-link-github",
      "preview-toolbar-link-patreon",
    ]) {
      const wrapper = page.getByTestId(testId);
      await expect(wrapper).toBeVisible();
      const button = wrapper.getByRole("button");
      await expect(button).toBeEnabled();
      await expect(button).not.toHaveAttribute("aria-disabled", "true");
    }
  });

  test("theme toggle flips background tokens", async ({ page }) => {
    await page.goto(previewUrl("/card"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const appBar = page.getByTestId("app-bar");
    await expect(appBar).toBeVisible();
    const material = appBar.locator(".orbital-app-bar__material");
    await expect(material).toHaveAttribute("data-material-variant", "frost");
    await expect(material).toHaveAttribute("data-material-elevation", "flat");

    const before = await getCssVariable(page, ".orbital-theme-provider", "--orb-color-surface-canvas");
    await page.getByTestId("preview-theme-toggle").locator("input").click();
    const after = await getCssVariable(page, ".orbital-theme-provider", "--orb-color-surface-canvas");

    expect(before).toBeTruthy();
    expect(after).toBeTruthy();
    expect(after).not.toEqual(before);
  });

  test("introduction: font families shown side by side", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const section = page.getByTestId("intro-font-families");
    await section.scrollIntoViewIfNeeded();
    await expect(section).toBeVisible();

    for (const testId of [
      "intro-font-base",
      "intro-font-numeric",
      "intro-font-monospace",
      "intro-font-display",
    ]) {
      const column = page.getByTestId(testId);
      await expect(column).toBeVisible();
      await expect(column).toContainText("Orbital 0123");
    }
  });

  test("self-hosted fonts are served as binary, not HTML", async ({ request }) => {
    const response = await request.get(
      previewUrl("/fonts/league-spartan/LeagueSpartan-VF.woff2"),
    );
    expect(response.ok()).toBeTruthy();
    expect(response.headers()["content-type"]).not.toMatch(/text\/html/);

    const body = await response.body();
    expect(body.subarray(0, 4).toString("utf8")).toBe("wOF2");
  });

  test("hydration wasm asset is served for preview pages", async ({ request }) => {
    const page = await request.get(previewUrl("/text"));
    expect(page.ok()).toBeTruthy();
    const html = await page.text();
    expect(html).toContain("/pkg/orbital-preview.wasm");
    expect(html).not.toContain("/pkg/orbital-preview_bg.wasm");

    const wasm = await request.get(previewUrl("/pkg/orbital-preview.wasm"));
    expect(wasm.ok()).toBeTruthy();
    expect(wasm.headers()["content-type"]).toMatch(/wasm/);
  });

  test("introduction: catalog back to top returns to page top", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();

    const button = page.getByTestId("back-to-top-button");
    await expect(button).toBeHidden();
    await scrollPreviewMain(page, 800);
    await expect(button).toBeVisible({ timeout: 15_000 });
    await button.click();
    await page.waitForTimeout(500);
    expect(await previewMainScrollTop(page)).toBeLessThan(50);
  });

  test("stub page: catalog back to top returns to page top", async ({ page }) => {
    await openComponentPreview(page, "toolbar");
    const button = page.getByTestId("back-to-top-button");
    await expect(button).toBeHidden();
    await scrollPreviewMain(page, 800);
    await expect(button).toBeVisible({ timeout: 15_000 });
    await button.click();
    await page.waitForTimeout(500);
    expect(await previewMainScrollTop(page)).toBeLessThan(50);
  });

  test("app bar search navigates to component preview", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await expect(page.getByTestId("preview-catalog-search")).toBeVisible();

    const input = page.getByTestId("preview-catalog-search").locator("input");
    await input.click();
    await input.fill("card");
    await page
      .locator(".orbital-auto-complete-option", { hasText: "Card" })
      .first()
      .click();

    await expect(page).toHaveURL(/\/card$/);
    await openComponentPreview(page, "card");
  });
});
