const previewBasePath =
  process.env.ORBITAL_PREVIEW_BASE_PATH ??
  process.env.LEPTOS_BASE_PATH ??
  "";

export const PREVIEW_BASE_URL =
  process.env.COMPONENT_PREVIEW_BASE_URL ??
  `http://localhost:3010${previewBasePath}`;

export function previewUrl(path: string): string {
  const base = PREVIEW_BASE_URL.replace(/\/$/, "");
  const normalized = path.startsWith("/") ? path : `/${path}`;
  if (normalized === "/") {
    return previewBasePath ? base : `${base}/`;
  }
  return `${base}${normalized}`;
}

export async function waitForPreviewShell(page: import("@playwright/test").Page) {
  const { expect } = await import("@playwright/test");
  await page.waitForLoadState("domcontentloaded");
  await expect(page.getByTestId("preview-catalog-shell")).toBeVisible({ timeout: 10_000 });
  await page.waitForLoadState("networkidle");
  // networkidle fires once the wasm is downloaded, but compiling and hydrating it can
  // finish later on a loaded box; input typed before then is reset by hydration.
  await expect(page.locator("html")).toHaveAttribute("data-orbital-hydrated", "true", {
    timeout: 30_000,
  });
}
