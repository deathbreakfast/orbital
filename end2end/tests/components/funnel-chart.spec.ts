import AxeBuilder from "@axe-core/playwright";
import { test, expect } from "@playwright/test";
import { expectPreviewVariants, openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = ["funnel-chart-preview", "funnel-chart-many-steps", "funnel-chart-empty"];

test.describe("funnel chart preview", () => {
  test("FC-01: each step shows its count and share on a shrinking bar", async ({ page }) => {
    await openComponentPreview(page, "funnel-chart");
    const chart = page.getByTestId("funnel-chart-preview");
    const list = chart.getByRole("list", { name: "Checkout funnel" });
    await expect(list.getByRole("listitem")).toHaveCount(3);
    const bars = chart.locator(".orbital-funnel-chart__bar");
    await expect(bars).toHaveText(["4,120 · 100%", "1,566 · 38%", "402 · 10%"]);
    const widths = await bars.evaluateAll((els) => els.map((el) => el.getBoundingClientRect().width));
    expect(widths[0]).toBeGreaterThan(widths[1]);
    expect(widths[1]).toBeGreaterThan(widths[2]);
  });

  test("FC-02: the drop-off from the step before sits under each later bar", async ({ page }) => {
    await openComponentPreview(page, "funnel-chart");
    const chart = page.getByTestId("funnel-chart-preview");
    await expect(chart.locator(".orbital-funnel-chart__drop-off")).toHaveText([
      "62% stopped here",
      "74% of the step before stopped here",
    ]);
    await expect(
      chart.locator(".orbital-funnel-chart__step[data-step='0'] .orbital-funnel-chart__drop-off"),
    ).toHaveCount(0);
  });

  test("FC-03: a first step of zero shows 0% and no drop-off", async ({ page }) => {
    await openComponentPreview(page, "funnel-chart");
    const chart = page.getByTestId("funnel-chart-empty");
    await chart.scrollIntoViewIfNeeded();
    await expect(chart.locator(".orbital-funnel-chart__bar")).toHaveText(["0 · 0%", "0 · 0%"]);
    await expect(chart.locator(".orbital-funnel-chart__drop-off")).toHaveCount(0);
  });

  test("FC-04: shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "funnel-chart");
    await expectPreviewVariants(page, EXAMPLES);
    await expect(
      page.getByTestId("funnel-chart-many-steps").locator(".orbital-funnel-chart__bar").last(),
    ).toHaveText("40 · <1%");
  });

  test("FC-A1: axe clean in dark and light", async ({ page }) => {
    await openComponentPreview(page, "funnel-chart");
    const scan = async (label: string) => {
      for (const testId of EXAMPLES) {
        await page.getByTestId(testId).scrollIntoViewIfNeeded();
        const results = await new AxeBuilder({ page })
          .include(`[data-testid='${testId}']`)
          .analyze();
        expect(
          results.violations.filter((v) => v.impact === "critical" || v.impact === "serious"),
          `${label} ${testId}`,
        ).toEqual([]);
      }
    };
    // The preview shell starts in dark mode.
    await scan("dark");
    const themeSwitch = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    await themeSwitch.click();
    await expect(themeSwitch).not.toBeChecked();
    await scan("light");
  });
});
