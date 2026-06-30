import { test, expect } from "@playwright/test";
import { previewUrl } from "../helpers";
import { getCssVariable } from "../lib/assertions/style";
import { blockWasm, delayWasm } from "../lib/preview/wasm-delay";

const FOUC_SMOKE_PATH = "/debug/fouc-smoke";
const ROOT_SCOPE_SELECTOR = '.orbital-theme-provider[data-orbital-theme-id="0"]';

function extractHead(html: string): string {
  const match = html.match(/<head[^>]*>([\s\S]*?)<\/head>/i);
  return match?.[1] ?? "";
}

test.describe("FOUC baseline smoke", () => {
  test("F-01: SSR head includes baseline contract", async ({ request }) => {
    const response = await request.get(previewUrl(FOUC_SMOKE_PATH));
    expect(response.ok()).toBeTruthy();

    const html = await response.text();
    const head = extractHead(html);

    expect(head).toContain('data-orbital-theme-baseline="0"');
    expect(head).toContain("@font-face");
    expect(head).toContain("--orb-color-brand-bg");
    expect(head).toContain('data-orbital-theme-id="0"');
  });

  test("F-02: theme tokens resolve with WASM blocked", async ({ page }) => {
    await blockWasm(page);
    await page.goto(previewUrl(FOUC_SMOKE_PATH), { waitUntil: "commit" });

    const brandBg = await getCssVariable(page, ROOT_SCOPE_SELECTOR, "--orb-color-brand-bg");
    expect(brandBg.length).toBeGreaterThan(0);
  });

  test("F-03: primary button is styled before WASM hydrates", async ({ page }) => {
    await delayWasm(page, 15_000);
    await page.goto(previewUrl(FOUC_SMOKE_PATH), { waitUntil: "domcontentloaded" });

    const brandBg = await getCssVariable(page, ROOT_SCOPE_SELECTOR, "--orb-color-brand-bg");
    expect(brandBg.length).toBeGreaterThan(0);

    const button = page.getByTestId("fouc-smoke-button").getByRole("button");
    await expect(button).toBeVisible();

    const bg = await button.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bg).not.toMatch(/^(rgba?\(0,\s*0,\s*0,\s*0\)|transparent)$/i);
    await expect(button).toHaveCSS("background-color", /rgb/);
  });
});
