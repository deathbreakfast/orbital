import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview, scrollIntoPreviewView } from "../lib/preview/navigation";
import { setViewportPreset } from "../lib/preview/mobile";
import { WasmPanicGuard } from "../lib/preview/wasm-panics";

const browserErrors = new WeakMap<Page, string[]>();

async function openVariant(page: Page, testId: string): Promise<Locator> {
  await openComponentPreview(page, "diff-view");
  const variant = page.getByTestId(testId);
  await scrollIntoPreviewView(variant);
  await expect(variant).toBeVisible({ timeout: 10_000 });
  return variant;
}

async function box(locator: Locator) {
  const rect = await locator.boundingBox();
  expect(rect).not.toBeNull();
  return rect!;
}

test.describe("diff-view preview", () => {
  test.beforeEach(async ({ page }) => {
    const errors: string[] = [];
    browserErrors.set(page, errors);
    page.on("console", (msg) => {
      if (msg.type() === "error") errors.push(`console: ${msg.text()}`);
    });
    page.on("pageerror", (err) => errors.push(`pageerror: ${err.message}`));
    page.on("requestfailed", (req) =>
      errors.push(`requestfailed: ${req.url()} ${req.failure()?.errorText ?? ""}`),
    );
  });

  test.afterEach(async ({ page }, testInfo) => {
    if (testInfo.status !== testInfo.expectedStatus) {
      console.log(`[${testInfo.title}] browser errors:\n${(browserErrors.get(page) ?? []).join("\n")}`);
    }
  });

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "diff-view");
  });

  test("side by side names each side and aligns paired rows", async ({ page }) => {
    const variant = await openVariant(page, "diff-view-side-by-side");
    const from = variant.getByRole("group", { name: "From: v2" });
    const to = variant.getByRole("group", { name: "To: v3" });
    await expect(from).toHaveCount(1);
    await expect(to).toHaveCount(1);

    const removed = from.locator('[data-diff-kind="delete"]', { hasText: "90 days" });
    const added = to.locator('[data-diff-kind="insert"]', { hasText: "30 days" });
    await expect(removed).toBeVisible();
    await expect(added).toBeVisible();
    await expect(removed).toContainText("Removed: ");
    await expect(added).toContainText("Added: ");
    await expect(removed.locator(".orbital-diff-view__marker")).toHaveText("-");
    await expect(added.locator(".orbital-diff-view__marker")).toHaveText("+");

    const oldRect = await box(removed);
    const newRect = await box(added);
    expect(Math.abs(oldRect.y - newRect.y)).toBeLessThan(1);
    expect(newRect.x).toBeGreaterThan(oldRect.x);

    // The long added line has no partner; its filler keeps the next row aligned.
    const nextOld = from.locator('[data-diff-kind="equal"]', { hasText: "## Contacts" });
    const nextNew = to.locator('[data-diff-kind="equal"]', { hasText: "## Contacts" });
    const nextOldRect = await box(nextOld);
    const nextNewRect = await box(nextNew);
    expect(Math.abs(nextOldRect.y - nextNewRect.y)).toBeLessThan(1);
  });

  test("layout switch shows the unified view", async ({ page }) => {
    const panics = new WasmPanicGuard(page);
    const variant = await openVariant(page, "diff-view-side-by-side");
    await variant.getByRole("tab", { name: "Unified" }).click();
    const body = variant.getByRole("group", { name: "Changes from v2 to v3" });
    await expect(body).toBeVisible();
    await expect(variant.getByRole("group", { name: "From: v2" })).toHaveCount(0);

    const removed = body.locator('[data-diff-kind="delete"]', { hasText: "90 days" });
    const added = body.locator('[data-diff-kind="insert"]', { hasText: "30 days" });
    expect((await box(removed)).y).toBeLessThan((await box(added)).y);
    await expect(body.getByText("Unchanged lines skipped")).toHaveCount(1);
    await expect(variant.getByText("From: v2")).toBeVisible();
    await expect(variant.getByText("To: v3")).toBeVisible();
    panics.assertClean();
  });

  test("phone width stacks From above To", async ({ page }) => {
    await setViewportPreset(page, "phone");
    const variant = await openVariant(page, "diff-view-side-by-side");
    const from = variant.getByRole("group", { name: "From: v2" });
    const to = variant.getByRole("group", { name: "To: v3" });
    const fromRect = await box(from);
    const toRect = await box(to);
    expect(toRect.y).toBeGreaterThanOrEqual(fromRect.y + fromRect.height);
    expect(Math.abs(toRect.x - fromRect.x)).toBeLessThan(1);

    const fillersHidden = await variant
      .locator(".orbital-diff-view__cell--empty")
      .evaluateAll((els) => els.length > 0 && els.every((el) => getComputedStyle(el).display === "none"));
    expect(fillersHidden).toBe(true);
  });

  test("word spans mark the changed words", async ({ page }) => {
    const variant = await openVariant(page, "diff-view-words");
    await expect(variant.locator("del.orbital-diff-view__span")).toHaveText("30");
    await expect(variant.locator("ins.orbital-diff-view__span")).toHaveText("45");
  });

  test("identical and empty states", async ({ page }) => {
    const identical = await openVariant(page, "diff-view-identical");
    await expect(identical.getByText("No differences.")).toBeVisible();
    await expect(identical.getByText("From: v3")).toBeVisible();
    const empty = page.getByTestId("diff-view-empty");
    await scrollIntoPreviewView(empty);
    await expect(empty.getByText("Pick a From and a To version.")).toBeVisible();
    // MessageBar is itself a group, so check for the diff containers instead.
    await expect(empty.locator(".orbital-diff-view__body, .orbital-diff-view__sides")).toHaveCount(0);
  });

  test("dark mode swaps the status backgrounds", async ({ page }) => {
    const panics = new WasmPanicGuard(page);
    const variant = await openVariant(page, "diff-view-words");
    const added = variant.locator('[data-diff-kind="insert"]');
    const background = () => added.evaluate((el) => getComputedStyle(el).backgroundColor);
    const light = await background();
    expect(light).not.toBe("rgba(0, 0, 0, 0)");
    await page.getByTestId("preview-theme-toggle").locator("input").click();
    await expect.poll(background).not.toBe(light);
    expect(await background()).not.toBe("rgba(0, 0, 0, 0)");
    panics.assertClean();
  });
});
