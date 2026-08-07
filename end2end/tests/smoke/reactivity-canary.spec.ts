import { expect, test } from "@playwright/test";
import {
  collectReactiveWarningsForRoute,
  parseReactiveWarning,
} from "../_reactive_warnings";
import { openComponentPreview } from "../lib/preview/navigation";
import { previewUrl, waitForPreviewShell } from "../helpers";

test.describe("reactivity canary", () => {
  test("catalog shell loads without reactive tracking console warnings", async ({ page }) => {
    const warnings: string[] = [];
    page.on("console", (msg) => {
      const parsed = parseReactiveWarning(msg.text());
      if (parsed) {
        warnings.push(parsed.location);
      }
    });

    await page.goto(previewUrl("/card"));
    await waitForPreviewShell(page);
    await page.waitForTimeout(200);

    expect(
      warnings,
      `Reactive tracking warnings should be silenced in dev:\n${warnings.join("\n")}`,
    ).toHaveLength(0);
  });

  test("catalog nav updates page title when a sidebar link is clicked", async ({ page }) => {
    await page.goto(previewUrl("/"));
    await waitForPreviewShell(page);

    const nav = page.getByTestId("preview-catalog-nav");
    const gettingStarted = nav.getByRole("button", { name: "Getting Started", exact: true });
    if ((await gettingStarted.getAttribute("aria-expanded")) !== "true") {
      await gettingStarted.click();
    }
    await nav.getByRole("link", { name: "Theme" }).click();
    await expect(page).toHaveURL(/\/theme$/);
    await expect(page.getByTestId("preview-page-title")).toHaveText("Theme");

    await nav.getByRole("link", { name: "Introduction" }).click();
    await expect(page).toHaveURL(previewUrl("/"));
    await expect(page.getByTestId("preview-page-title")).toHaveText("Introduction");
  });

  test("switch toggles checked state on pointer click", async ({ page }) => {
    await openComponentPreview(page, "switch");
    const input = page.getByTestId("switch-off").locator("input[role=switch]");
    await expect(input).not.toBeChecked();
    await input.click({ force: true });
    await expect(input).toBeChecked();
    await input.click({ force: true });
    await expect(input).not.toBeChecked();
  });

  test("preview theme toggle switch reacts to click", async ({ page }) => {
    await page.goto(previewUrl("/card"));
    await waitForPreviewShell(page);
    const input = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    const before = await input.isChecked();
    await input.click({ force: true });
    await expect(input).toBeChecked({ checked: !before });
  });

  test("bare debug route stays free of reactive tracking warnings", async ({ page }) => {
    const warnings = await collectReactiveWarningsForRoute(
      page,
      previewUrl("/debug/switch"),
      "debug-bare-root",
    );
    expect(warnings).toHaveLength(0);
  });
});
