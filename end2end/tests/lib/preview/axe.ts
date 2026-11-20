import AxeBuilder from "@axe-core/playwright";
import { expect, type Page } from "@playwright/test";

// Critical and serious axe violations inside `selector`.
export async function seriousAxeViolations(page: Page, selector: string) {
  const results = await new AxeBuilder({ page }).include(selector).analyze();
  return results.violations.filter((v) => v.impact === "critical" || v.impact === "serious");
}

// Switch the preview shell to `mode`. The shell starts in dark mode; the toggle is checked
// while dark.
export async function setPreviewTheme(page: Page, mode: "light" | "dark") {
  const themeSwitch = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
  if ((await themeSwitch.isChecked()) !== (mode === "dark")) {
    await themeSwitch.click();
  }
  if (mode === "dark") {
    await expect(themeSwitch).toBeChecked();
  } else {
    await expect(themeSwitch).not.toBeChecked();
  }
}
