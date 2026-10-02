import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("badge primitive preview", () => {

  test("BD-01: brand filled badge renders label text", async ({ page }) => {
    await openComponentPreview(page, "badge");
    const badge = page.getByTestId("badge-preview").locator(".orbital-badge");
    await expect(badge).toHaveText("New");
    await expect(badge).toHaveClass(/orbital-badge--filled/);
    await expect(badge).toHaveClass(/orbital-badge--brand/);
  });

  test("BD-02: appearance matrix badges differ by modifier class", async ({ page }) => {
    await openComponentPreview(page, "badge");
    await page.getByTestId("badge-appearance").scrollIntoViewIfNeeded();
    const badges = page.getByTestId("badge-appearance").locator(".orbital-badge");
    await expect(badges).toHaveCount(4);
    await expect(badges.nth(0)).toHaveClass(/orbital-badge--filled/);
    await expect(badges.nth(1)).toHaveClass(/orbital-badge--tint/);
    await expect(badges.nth(2)).toHaveClass(/orbital-badge--outline/);
    await expect(badges.nth(3)).toHaveClass(/orbital-badge--ghost/);
  });

  test("BD-03: size matrix badges differ in height", async ({ page }) => {
    await openComponentPreview(page, "badge");
    await page.getByTestId("badge-sizes").scrollIntoViewIfNeeded();
    const small = page.getByTestId("badge-size-small").locator(".orbital-badge");
    const large = page.getByTestId("badge-size-large").locator(".orbital-badge");
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const largeH = await large.evaluate((el) => el.getBoundingClientRect().height);
    expect(largeH).toBeGreaterThan(smallH);
  });

  test("BD-06: small label badge keeps inline space between text and edge", async ({ page }) => {
    await openComponentPreview(page, "badge");
    await page.getByTestId("badge-sizes").scrollIntoViewIfNeeded();
    const badge = page.getByTestId("badge-size-small-label").locator(".orbital-badge");
    await expect(badge).toHaveText("Metric");
    const gaps = await badge.evaluate((el) => {
      const range = document.createRange();
      range.selectNodeContents(el);
      const text = range.getBoundingClientRect();
      const box = el.getBoundingClientRect();
      return { left: text.left - box.left, right: box.right - text.right };
    });
    expect(gaps.left).toBeGreaterThanOrEqual(5.5);
    expect(gaps.right).toBeGreaterThanOrEqual(5.5);
  });

  test("BD-04: semantic color matrix uses distinct color classes", async ({ page }) => {
    await openComponentPreview(page, "badge");
    await page.getByTestId("badge-colors").scrollIntoViewIfNeeded();
    const badges = page.getByTestId("badge-colors").locator(".orbital-badge");
    await expect(badges.nth(0)).toHaveClass(/orbital-badge--success/);
    await expect(badges.nth(1)).toHaveClass(/orbital-badge--warning/);
    await expect(badges.nth(2)).toHaveClass(/orbital-badge--danger/);
    await expect(badges.nth(3)).toHaveClass(/orbital-badge--informative/);
  });

  test("BD-05: navigation item badge shows count label", async ({ page }) => {
    await openComponentPreview(page, "badge");
    await page.getByTestId("badge-nav").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("badge-nav").getByText("Inbox")).toBeVisible();
    await expect(page.getByTestId("badge-nav").locator(".orbital-badge")).toHaveText("3");
  });
});
