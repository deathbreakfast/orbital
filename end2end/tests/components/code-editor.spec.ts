import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Locator, type Page } from "@playwright/test";
import { expectPreviewVariants, openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = [
  "code-editor-preview",
  "code-editor-error",
  "code-editor-readonly",
  "code-editor-field",
];

function editor(page: Page, testId = "code-editor-preview"): Locator {
  return page.getByTestId(testId);
}

async function typeAtEnd(textarea: Locator, text: string) {
  await textarea.focus();
  await textarea.press("Control+End");
  await textarea.pressSequentially(text);
}

test.describe("code editor preview", () => {
  test("CE-01: keywords from the grammar are highlighted", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const keywords = editor(page).locator(".orbital-code-editor__tok--keyword");
    await expect(keywords.first()).toHaveText("SELECT");
    await expect(keywords).toHaveCount(5);
    await expect(editor(page).locator(".orbital-code-editor__tok--string")).toHaveText("'paid'");
    await expect(editor(page).locator(".orbital-code-editor__tok--number")).toHaveText("1000");
  });

  test("CE-02: line numbers follow the text and the editor grows", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const lineNumbers = editor(page).locator(".orbital-code-editor__line-number");
    await expect(lineNumbers).toHaveCount(4);
    const frame = editor(page).locator(".orbital-code-editor__frame");
    const before = (await frame.boundingBox())!.height;

    await typeAtEnd(editor(page).locator("textarea"), "\nGROUP BY region\n");
    await expect(lineNumbers).toHaveCount(6);
    await expect(lineNumbers.last()).toHaveText("6");
    const after = (await frame.boundingBox())!.height;
    expect(after).toBeGreaterThan(before + 20);
    await expect(editor(page).locator(".orbital-code-editor__tok--keyword", { hasText: "GROUP" }))
      .toBeVisible();
  });

  test("CE-03: typing a prefix offers completions and Enter inserts one", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const textarea = editor(page).locator("textarea");
    await typeAtEnd(textarea, "\nJOIN ref");
    const listbox = editor(page).getByRole("listbox", { name: "Suggestions" });
    await expect(listbox).toBeVisible();
    await expect(listbox.getByRole("option")).toHaveText(["refund_event", "refund_reason"]);
    await expect(listbox.getByRole("option").first()).toHaveAttribute("aria-selected", "true");

    await textarea.press("ArrowDown");
    const second = listbox.getByRole("option", { name: "refund_reason" });
    await expect(second).toHaveAttribute("aria-selected", "true");
    await expect(textarea).toHaveAttribute("aria-activedescendant", (await second.getAttribute("id"))!);

    await textarea.press("Enter");
    await expect(listbox).toBeHidden();
    await expect(textarea).toHaveValue(/\nJOIN refund_reason$/);
    await expect(textarea).toBeFocused();
  });

  test("CE-04: Escape closes the list and Tab accepts the highlighted name", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const textarea = editor(page).locator("textarea");
    const listbox = editor(page).getByRole("listbox", { name: "Suggestions" });

    await typeAtEnd(textarea, "\nJOIN che");
    await expect(listbox).toBeVisible();
    await textarea.press("Escape");
    await expect(listbox).toBeHidden();
    await expect(textarea).toHaveValue(/\nJOIN che$/);

    await textarea.pressSequentially("c");
    await expect(listbox.getByRole("option")).toHaveText(["checkout_event"]);
    await textarea.press("Tab");
    await expect(textarea).toHaveValue(/\nJOIN checkout_event$/);
    await expect(textarea).toBeFocused();
  });

  test("CE-05: an error underlines the word at its line and column", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const errorEditor = editor(page, "code-editor-error");
    await errorEditor.scrollIntoViewIfNeeded();
    await expect(errorEditor.locator(".orbital-code-editor__error")).toHaveText("refnd_event");
    await expect(errorEditor.locator(".orbital-code-editor__line-number--error")).toHaveText("2");

    const message = errorEditor.locator(".orbital-code-editor__message");
    await expect(message).toHaveText(
      "Line 2, column 6: refnd_event isn't a table. Did you mean refund_event?",
    );
    const textarea = errorEditor.locator("textarea");
    await expect(textarea).toHaveAttribute("aria-invalid", "true");
    await expect(textarea).toHaveAttribute("aria-describedby", (await message.getAttribute("id"))!);
  });

  test("CE-06: the read-only editor blocks typing and completion", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const readonly = editor(page, "code-editor-readonly");
    await readonly.scrollIntoViewIfNeeded();
    const textarea = readonly.locator("textarea");
    await expect(textarea).toHaveAttribute("readonly", "");
    const before = await textarea.inputValue();
    await textarea.focus();
    await textarea.press("Control+End");
    await page.keyboard.type(" extra");
    await expect(textarea).toHaveValue(before);
    await expect(readonly.locator(".orbital-code-editor__tok--keyword").first()).toHaveText("SELECT");
  });

  test("CE-07: Field labels the textarea and min_rows sets the empty height", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    const field = editor(page, "code-editor-field");
    await field.scrollIntoViewIfNeeded();
    const textarea = field.getByLabel("Query");
    await expect(textarea).toHaveJSProperty("tagName", "TEXTAREA");
    await textarea.fill("SELECT 1");
    await expect(field.locator(".orbital-code-editor__tok--keyword")).toHaveText("SELECT");
    const height = (await field.locator(".orbital-code-editor__frame").boundingBox())!.height;
    const preview = (await editor(page).locator(".orbital-code-editor__frame").boundingBox())!.height;
    // Five starting rows against the four-line default example.
    expect(height).toBeGreaterThan(preview);
  });

  test("CE-08: shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
    await expectPreviewVariants(page, EXAMPLES);
  });

  test("CE-A1: axe clean in dark and light", async ({ page }) => {
    await openComponentPreview(page, "code-editor");
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
