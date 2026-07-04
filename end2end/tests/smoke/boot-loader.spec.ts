import { test, expect, type Page } from "@playwright/test";
import { previewUrl } from "../helpers";
import {
  readBootProgress,
  waitForBootProgress,
  type BootProgressSnapshot,
} from "../lib/preview/boot-progress";
import { blockWasm, delayWasm } from "../lib/preview/wasm-delay";

function extractHead(html: string): string {
  const match = html.match(/<head[^>]*>([\s\S]*?)<\/head>/i);
  return match?.[1] ?? "";
}

const STEP_IDS = ["theme", "scripts", "styles", "wasm", "hydrate"] as const;

async function sampleBootProgress(page: Page): Promise<BootProgressSnapshot> {
  const progress = await readBootProgress(page);
  expect(progress).not.toBeNull();
  return progress as BootProgressSnapshot;
}

test.describe("Boot loader", () => {
  test("B-01: SSR head and body include boot overlay contract", async ({ request }) => {
    const response = await request.get(previewUrl("/"));
    expect(response.ok()).toBeTruthy();

    const html = await response.text();
    const head = extractHead(html);

    expect(html).toContain('id="orbital-boot-overlay"');
    expect(html).toContain('data-testid="orbital-boot-overlay"');
    expect(html).toContain('data-testid="orbital-boot-loading"');
    expect(html).toContain('data-testid="orbital-boot-progress-bar"');
    expect(html).toContain('data-testid="orbital-boot-progress-label"');
    expect(html).toContain('data-testid="orbital-boot-progress-elapsed"');
    expect(html).toContain('class="orbital-boot-step__duration"');
    expect(html).toContain('data-testid="orbital-boot-steps"');
    for (const stepId of STEP_IDS) {
      expect(html).toContain(`data-testid="orbital-boot-step-${stepId}"`);
    }
    expect(html).toContain('role="status"');
    expect(html).toContain('aria-live="polite"');
    expect(html).toContain('role="dialog"');
    expect(html).not.toContain('data-testid="orbital-boot-spinner"');
    expect(head).toContain("#orbital-boot-overlay");
    expect(head).toContain("data-orbital-hydrated");
    expect(head).toContain("data-orbital-boot-loader");
    expect(head).toContain("data-orbital-boot-motion");
    expect(head).toContain("orbital-motion-fade-leave-active");
    expect(head).toContain("__orbitalBootProgress");
    expect(head).toContain("__orbitalBootDismissOverlay");
  });

  test("B-02: boot loading modal visible during slow WASM", async ({ page }) => {
    const WASM_DELAY_MS = 8_000;
    await delayWasm(page, WASM_DELAY_MS);

    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    const overlay = page.getByTestId("orbital-boot-overlay");
    await expect(overlay).toBeVisible({ timeout: 2_000 });
    await expect(page.getByTestId("orbital-boot-loading")).toBeVisible();
    await expect(page.getByTestId("orbital-boot-error")).toBeHidden();
    await expect(page.locator("html")).not.toHaveAttribute("data-orbital-hydrated", "true");
    await expect(overlay).toHaveCount(1);

    await page.waitForTimeout(2_000);
    await expect(overlay).toBeVisible();
  });

  test("B-03: boot progress advances during slow WASM and overlay clears after hydrate", async ({
    page,
  }) => {
    const WASM_DELAY_MS = 8_000;
    await delayWasm(page, WASM_DELAY_MS);

    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    const overlay = page.getByTestId("orbital-boot-overlay");
    await expect(overlay).toBeVisible({ timeout: 2_000 });

    await waitForBootProgress(
      page,
      (progress) =>
        progress.steps.theme === "complete" &&
        progress.steps.scripts === "complete" &&
        progress.steps.styles === "complete" &&
        progress.steps.wasm === "active" &&
        progress.percent > 0 &&
        progress.percent < 100,
      WASM_DELAY_MS,
    );

    const progressBar = page.getByTestId("orbital-boot-progress-bar");
    await expect(progressBar).toHaveAttribute("role", "progressbar");
    await expect(progressBar).toHaveAttribute("aria-valuemin", "0");
    await expect(progressBar).toHaveAttribute("aria-valuemax", "100");

    const progress = await sampleBootProgress(page);
    await expect(progressBar).toHaveAttribute("aria-valuenow", String(progress.percent));
    await expect(page.getByTestId("orbital-boot-progress-label")).toHaveText(
      `${progress.percent}%`,
    );

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
    await expect(page.getByTestId("orbital-boot-loading")).toBeHidden();
    await expect(page.getByTestId("orbital-boot-overlay")).toHaveAttribute("aria-busy", "false");
    await expect(page.getByTestId("orbital-boot-overlay")).toBeVisible();
  });

  test("B-06: boot steps complete in order before wasm activates", async ({ page }) => {
    await delayWasm(page, 8_000);
    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    await waitForBootProgress(page, (progress) => progress.currentStep === "wasm", 10_000);

    const progress = await sampleBootProgress(page);
    expect(progress.steps.theme).toBe("complete");
    expect(progress.steps.scripts).toBe("complete");
    expect(progress.steps.styles).toBe("complete");
    expect(progress.steps.wasm).toBe("active");
  });

  test("B-07: boot progress percent increases while wasm is active", async ({ page }) => {
    await delayWasm(page, 8_000);
    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    await waitForBootProgress(page, (progress) => progress.steps.wasm === "active", 10_000);

    const first = await sampleBootProgress(page);
    await page.waitForTimeout(1_000);
    const second = await sampleBootProgress(page);

    expect(second.percent).toBeGreaterThanOrEqual(first.percent);
  });

  test("B-08: loading modal exposes accessible progress semantics", async ({ page }) => {
    await delayWasm(page, 8_000);
    await page.goto(previewUrl("/"), { waitUntil: "domcontentloaded" });

    const overlay = page.getByTestId("orbital-boot-overlay");
    await expect(overlay).toHaveAttribute("aria-busy", "true");

    const progressBar = page.getByTestId("orbital-boot-progress-bar");
    await expect(progressBar).toHaveAttribute("role", "progressbar");
    await expect(progressBar).toHaveAttribute("aria-valuemin", "0");
    await expect(progressBar).toHaveAttribute("aria-valuemax", "100");

    const loadingDialog = page.getByTestId("orbital-boot-loading").getByRole("dialog");
    await expect(loadingDialog).toBeVisible();
    await expect(loadingDialog).toHaveAttribute("aria-modal", "true");
  });

  test("B-09: successful hydrate plays exit transition before overlay removal", async ({
    page,
  }) => {
    const reducedMotion = await page.evaluate(() =>
      window.matchMedia("(prefers-reduced-motion: reduce)").matches,
    );
    test.skip(reducedMotion, "Exit animation skipped when prefers-reduced-motion");

    await page.goto(previewUrl("/"), { waitUntil: "commit" });

    await page.waitForFunction(
      () => {
        const overlay = document.querySelector('[data-testid="orbital-boot-overlay"]');
        if (!overlay) return false;
        return (
          overlay.hasAttribute("data-orbital-boot-exiting") ||
          overlay.classList.contains("orbital-motion-fade-leave-active")
        );
      },
      { timeout: 15_000 },
    );

    await expect(page.getByTestId("orbital-boot-overlay")).toBeHidden({ timeout: 5_000 });
    await expect(page.locator("html")).toHaveAttribute("data-orbital-hydrated", "true");
    await expect(page.getByTestId("preview-catalog-shell")).toBeVisible();
  });
});
