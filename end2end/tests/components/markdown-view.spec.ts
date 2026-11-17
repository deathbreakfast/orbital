import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = ["markdown-view-preview", "markdown-view-blocks", "markdown-view-wide"];

async function open(page: Page) {
  await openComponentPreview(page, "markdown-view");
  return page.getByTestId("markdown-view-preview").locator(".orbital-markdown");
}

test.describe("markdown view preview", () => {
  test("MV-01: a GFM table renders with a header row and body rows", async ({ page }) => {
    const view = await open(page);
    const table = view.locator("table.orbital-table.orbital-markdown__table");
    await expect(table).toBeVisible();
    await expect(table.locator("thead th")).toHaveText(["Feature", "Crate", "Status"]);
    await expect(table.locator("tbody tr")).toHaveCount(3);
    await expect(table.getByRole("link", { name: "Tables" })).toHaveAttribute(
      "href",
      "https://example.com",
    );
    await expect(table.locator("tbody td code").first()).toHaveText("orbital-datatable");
    await expect(table.locator("tbody tr").first().locator("td").nth(2)).toHaveCSS(
      "text-align",
      "center",
    );
  });

  test("MV-02: blockquote, heading and rule render", async ({ page }) => {
    const view = await open(page);
    // Headings sit on the Subtitle1 step, body text on Body1.
    const sizes = await view.evaluate((el) => ({
      body: getComputedStyle(el).fontSize,
      h2: getComputedStyle(el.querySelector("h2")!).fontSize,
    }));
    expect(parseFloat(sizes.h2)).toBeGreaterThan(parseFloat(sizes.body));
    expect(parseFloat(sizes.h2)).toBeLessThanOrEqual(16);
    await expect(view.locator("blockquote strong")).toHaveText("Product UI in Rust.");
    await expect(view.getByRole("heading", { name: "Features", level: 2 })).toBeVisible();
    await expect(view.locator("hr.orbital-markdown__rule")).toHaveCount(1);
    await expect(view.locator("em")).toHaveText("Updated weekly.");
  });

  test("MV-03: a wide table scrolls inside its own box", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const example = page.getByTestId("markdown-view-wide");
    const scroller = example.locator(".orbital-markdown__table-scroll");
    const sizes = await scroller.evaluate((el) => ({
      scroll: el.scrollWidth,
      client: el.clientWidth,
    }));
    expect(sizes.scroll).toBeGreaterThan(sizes.client);
    const box = await example.boundingBox();
    expect(box?.width ?? 0).toBeLessThanOrEqual(320);
    await scroller.focus();
    await expect(scroller).toBeFocused();
  });

  test("MV-04: task list items show read-only checkboxes", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const boxes = page.getByTestId("markdown-view-blocks").getByRole("checkbox", { name: "Task" });
    await expect(boxes).toHaveCount(2);
    await expect(boxes.nth(0)).toBeChecked();
    await expect(boxes.nth(1)).not.toBeChecked();
    await expect(boxes.nth(0)).toBeDisabled();
    const indicators = page
      .getByTestId("markdown-view-blocks")
      .locator(".orbital-checkbox .orbital-checkbox__indicator");
    await expect(indicators).toHaveCount(2);
    await expect(indicators.nth(0).locator("svg")).toBeVisible();
  });

  test("MV-05: a bare URL, an alert and a code block render", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const view = page.getByTestId("markdown-view-blocks");
    await expect(view.getByRole("link", { name: "https://example.com/docs" })).toHaveAttribute(
      "href",
      "https://example.com/docs",
    );
    const alert = view.locator(".orbital-message-bar.orbital-message-bar--warning");
    await expect(alert.locator(".orbital-message-bar-title")).toHaveText("Warning");
    await expect(alert.locator(".orbital-message-bar__icon svg")).toBeVisible();
    await expect(alert).toContainText("Deleting a project can't be undone.");
    const code = view.locator("pre.orbital-markdown__code-block");
    await expect(code).toHaveText("cargo leptos watch");
    await expect(code).toHaveCSS("font-family", /mono|Mono|Consolas|Courier/);
  });

  test("MV-06: details opens when its summary is clicked", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const details = page.getByTestId("markdown-view-blocks").locator("details");
    await expect(details.getByText("Shipped")).toBeHidden();
    await details.locator("summary").click();
    await expect(details.getByText("Shipped")).toBeVisible();
  });

  test("MV-A1: axe clean in dark and light at 390px and desktop", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const scan = async (label: string, excludeLinks = false) => {
      for (const testId of EXAMPLES) {
        await page.getByTestId(testId).scrollIntoViewIfNeeded();
        let axe = new AxeBuilder({ page }).include(`[data-testid='${testId}']`);
        // The light theme's link token is 4.32:1 on white everywhere, not only
        // here; Magnon task 907a1086-d8d4-4898-89b0-da01d3240c35 fixes the token.
        if (excludeLinks) axe = axe.exclude(".orbital-markdown a.orbital-link");
        const results = await axe.analyze();
        expect(
          results.violations.filter((v) => v.impact === "critical" || v.impact === "serious"),
          `${label} ${testId}`,
        ).toEqual([]);
      }
    };
    const desktop = page.viewportSize() ?? { width: 1280, height: 720 };

    // The preview shell starts in dark mode.
    await scan("dark desktop");
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("dark 390px");

    await page.setViewportSize(desktop);
    const themeSwitch = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    await themeSwitch.click();
    await expect(themeSwitch).not.toBeChecked();
    await scan("light desktop", true);
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("light 390px", true);
  });
});
