import { test, expect } from "@playwright/test";
import { previewUrl } from "../helpers";
import { blockWasm, delayWasm } from "../lib/preview/wasm-delay";

function extractHead(html: string): string {
  const match = html.match(/<head[^>]*>([\s\S]*?)<\/head>/i);
  return match?.[1] ?? "";
}

test.describe("Boot loader", () => {
  test("B-01: SSR head and body include boot overlay contract", async ({ request }) => {
    const response = await request.get(previewUrl("/"));
    expect(response.ok()).toBeTruthy();

    const html = await response.text();
    const head = extractHead(html);

    expect(html).toContain('id="orbital-boot-overlay"');
    expect(html).toContain('data-testid="orbital-boot-overlay"');
    expect(html).toContain('role="status"');
    expect(html).toContain('aria-live="polite"');
    expect(head).toContain("#orbital-boot-overlay");
    expect(head).toContain("data-orbital-hydrated");
    expect(head).toContain("data-orbital-boot-loader");
  });

  test("B-02/B-03: boot overlay visible during slow WASM, gone after hydrate", async ({
    page,
  }) => {
    const WASM_DELAY_MS = 8_000;
    await delayWasm(page, WASM_DELAY_MS);

    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    const overlay = page.getByTestId("orbital-boot-overlay");
    await expect(overlay).toBeVisible({ timeout: 2_000 });
    await expect(page.getByTestId("orbital-boot-message")).toContainText(/loading/i);
    await expect(page.getByTestId("orbital-boot-spinner")).toBeVisible();
    await expect(page.locator("html")).not.toHaveAttribute("data-orbital-hydrated", "true");
    await expect(overlay).toHaveCount(1);

    await page.waitForTimeout(2_000);
    await expect(overlay).toBeVisible();

    await expect(overlay).toBeHidden({ timeout: WASM_DELAY_MS + 15_000 });
    await expect(page.locator("html")).toHaveAttribute("data-orbital-hydrated", "true");
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
  });

  test("B-04: fast path hides overlay after hydration with no duplicates", async ({ page }) => {
    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    await expect(page.getByTestId("orbital-boot-overlay")).toBeHidden({
      timeout: 15_000,
    });
    await expect(page.locator("html")).toHaveAttribute("data-orbital-hydrated", "true");
    await expect(page.locator("#orbital-boot-overlay")).toHaveCount(0);
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
  });

  test("B-05: WASM load failure shows static error on overlay", async ({ page }) => {
    await blockWasm(page);
    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    await expect(page.locator("html")).toHaveAttribute("data-orbital-boot-state", "error", {
      timeout: 15_000,
    });
    await expect(page.getByTestId("orbital-boot-error")).toBeVisible();
    await expect(page.getByRole("alertdialog")).toBeVisible();
    await expect(page.getByText("Unable to load application")).toBeVisible();
    await expect(page.getByText("Startup failed")).toBeVisible();
    await expect(page.getByTestId("orbital-boot-spinner")).toBeHidden();
    await expect(page.getByTestId("orbital-boot-overlay")).toBeVisible();
  });
});
