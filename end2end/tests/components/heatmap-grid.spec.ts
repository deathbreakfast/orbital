import AxeBuilder from "@axe-core/playwright";
import { test, expect } from "@playwright/test";
import { expectPreviewVariants, openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = ["heatmap-grid-preview", "heatmap-grid-domain"];

test.describe("heatmap grid preview", () => {
  test("HG-01: the header row lists the row header, leading and value columns", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
    const grid = page.getByTestId("heatmap-grid-preview");
    await expect(grid.getByRole("table", { name: "Weekly retention" })).toBeVisible();
    await expect(grid.getByRole("columnheader")).toHaveText([
      "Cohort week",
      "Accounts",
      "Week 1",
      "Week 2",
      "Week 3",
    ]);
    await expect(grid.getByRole("rowheader")).toHaveText(["Oct 27", "Nov 3", "Nov 10"]);
  });

  test("HG-02: valued cells carry a shade level and print their label", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
    const row = page.getByTestId("heatmap-grid-preview").getByRole("row", { name: /Oct 27/ });
    await expect(row.locator(".orbital-heatmap-grid__leading")).toHaveText("214");
    const cells = row.locator(".orbital-heatmap-grid__cell");
    await expect(cells).toHaveText(["41%", "33%", "29%"]);
    for (const cell of await cells.all()) {
      await expect(cell).toHaveAttribute("data-level", /^[1-5]$/);
    }
  });

  test("HG-03: empty cells read 'not yet' and stay unshaded", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
    const empty = page.getByTestId("heatmap-grid-preview").locator(".orbital-heatmap-grid__cell--empty");
    await expect(empty).toHaveCount(3);
    await expect(empty).toHaveText(["not yet", "not yet", "not yet"]);
    for (const cell of await empty.all()) {
      await expect(cell).not.toHaveAttribute("data-level");
    }
  });

  test("HG-04: a fixed scale shades by the given range", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
    const grid = page.getByTestId("heatmap-grid-domain");
    await grid.scrollIntoViewIfNeeded();
    const high = grid.locator(".orbital-heatmap-grid__cell", { hasText: "95%" });
    const low = grid.locator(".orbital-heatmap-grid__cell", { hasText: /^5%$/ });
    await expect(high).toHaveAttribute("data-level", "5");
    await expect(low).toHaveAttribute("data-level", "1");
    const [highBg, lowBg] = await Promise.all(
      [high, low].map((cell) => cell.evaluate((el) => getComputedStyle(el).backgroundColor)),
    );
    expect(highBg).not.toEqual(lowBg);
    await expect(grid.locator(".orbital-heatmap-grid__cell--empty")).toHaveText(["no data", "no data"]);
  });

  test("HG-05: shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
    await expectPreviewVariants(page, EXAMPLES);
  });

  test("HG-A1: axe clean in dark and light", async ({ page }) => {
    await openComponentPreview(page, "heatmap-grid");
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
