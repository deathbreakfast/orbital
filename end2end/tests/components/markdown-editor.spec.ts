import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

const TOOLBAR_NAMES = ["Bold", "Italic", "Insert link", "Bulleted list", "Numbered list"];

function editor(page: Page, testId = "markdown-editor-preview"): Locator {
  return page.getByTestId(testId);
}

async function select(textarea: Locator, start: number, end: number) {
  await textarea.focus();
  await textarea.evaluate(
    (el, [s, e]) => (el as HTMLTextAreaElement).setSelectionRange(s, e),
    [start, end],
  );
}

test.describe("markdown editor preview", () => {
  test("ME-01: Bold wraps the selected word", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const textarea = editor(page).locator("textarea");
    await textarea.fill("make this bold");
    await select(textarea, 5, 9);
    await editor(page).getByRole("button", { name: "Bold" }).click();
    await expect(textarea).toHaveValue("make **this** bold");
    await expect(textarea).toBeFocused();
  });

  test("ME-02: Ctrl+I wraps the selection in italics", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const textarea = editor(page).locator("textarea");
    await textarea.fill("tilt");
    await select(textarea, 0, 4);
    await textarea.press("Control+i");
    await expect(textarea).toHaveValue("_tilt_");
  });

  test("ME-03: Preview renders strong text", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    await editor(page).locator("textarea").fill("Some **strong** words");
    await editor(page).getByRole("tab", { name: "Preview" }).click();
    await expect(
      editor(page).locator(".orbital-markdown-editor__preview strong"),
    ).toHaveText("strong");
  });

  test("ME-04: a script tag in the source does not render", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    await editor(page)
      .locator("textarea")
      .fill("<script>window.__mdEditorXss = 1</script>\n\nplain text");
    await editor(page).getByRole("tab", { name: "Preview" }).click();
    const preview = editor(page).locator(".orbital-markdown-editor__preview");
    await expect(preview).toContainText("plain text");
    await expect(preview.locator("script")).toHaveCount(0);
    expect(await page.evaluate(() => (window as { __mdEditorXss?: number }).__mdEditorXss))
      .toBeUndefined();
  });

  test("ME-05: empty source shows the empty preview message", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    await editor(page).locator("textarea").fill("");
    await editor(page).getByRole("tab", { name: "Preview" }).click();
    await expect(editor(page).getByText("Nothing to preview")).toBeVisible();
  });

  test("ME-06: going past max_chars sets aria-invalid", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const limit = editor(page, "markdown-editor-limit");
    await limit.scrollIntoViewIfNeeded();
    const textarea = limit.locator("textarea");
    await expect(textarea).not.toHaveAttribute("aria-invalid", "true");
    await textarea.fill("x".repeat(41));
    await expect(textarea).toHaveAttribute("aria-invalid", "true");
    await expect(limit.getByText("41 / 40")).toBeVisible();
    await textarea.fill("short");
    await expect(textarea).not.toHaveAttribute("aria-invalid", "true");
  });

  test("ME-07: every toolbar button has an accessible name", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const buttons = editor(page).locator(".orbital-markdown-editor__toolbar button");
    await expect(buttons).toHaveCount(TOOLBAR_NAMES.length);
    for (const name of TOOLBAR_NAMES) {
      await expect(editor(page).getByRole("button", { name, exact: true })).toBeVisible();
    }
  });

  test("ME-08: Field label names the textarea", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const field = editor(page, "markdown-editor-field");
    await field.scrollIntoViewIfNeeded();
    await expect(field.getByRole("textbox", { name: "Description" })).toBeVisible();
  });

  test("ME-A1: axe clean in dark and light at 390px and desktop", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const scan = async (label: string) => {
      for (const testId of [
        "markdown-editor-preview",
        "markdown-editor-limit",
        "markdown-editor-field",
        "markdown-editor-disabled",
      ]) {
        await editor(page, testId).scrollIntoViewIfNeeded();
        const results = await new AxeBuilder({ page })
          .include(`[data-testid='${testId}']`)
          .analyze();
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
    await scan("light desktop");
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("light 390px");
  });
});
