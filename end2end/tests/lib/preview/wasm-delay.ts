import type { Page } from "@playwright/test";

/** Delay all preview WASM responses (simulates slow bundle download). */
export async function delayWasm(page: Page, delayMs: number): Promise<void> {
  await page.route("**/*orbital-preview*.wasm", async (route) => {
    await new Promise((resolve) => setTimeout(resolve, delayMs));
    await route.continue();
  });
}

/** Abort WASM so hydration never runs (no-JS / blocked-bundle scenario). */
export async function blockWasm(page: Page): Promise<void> {
  await page.route("**/*orbital-preview*.wasm", (route) => route.abort());
}
