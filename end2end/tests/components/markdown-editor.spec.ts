import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

// The default example renders images, so Insert image shows; it has no people provider.
const TOOLBAR_NAMES = [
  "Bold",
  "Italic",
  "Strikethrough",
  "Insert link",
  "Insert image",
  "Insert table",
  "Bulleted list",
  "Numbered list",
];

function editor(page: Page, testId = "markdown-editor-preview"): Locator {
  return page.getByTestId(testId);
}

async function height(locator: Locator): Promise<number> {
  return locator.evaluate((el) => el.getBoundingClientRect().height);
}

function lines(count: number): string {
  return Array.from({ length: count }, (_, i) => `Line ${i + 1}`).join("\n");
}

function grip(root: Locator): Locator {
  return root.getByRole("separator", { name: "Resize editor height" });
}

async function selectedText(textarea: Locator): Promise<string> {
  return textarea.evaluate((el) => {
    const area = el as HTMLTextAreaElement;
    return area.value.slice(area.selectionStart, area.selectionEnd);
  });
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
    const buttons = editor(page).locator(".orbital-markdown-editor__toolbar button[data-action]");
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

  test("ME-09: Preview renders a heading and a GFM table", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    await editor(page)
      .locator("textarea")
      .fill("## Plan\n\n| Step | Owner |\n|---|---|\n| Ship | Sam |\n");
    await editor(page).getByRole("tab", { name: "Preview" }).click();
    const preview = editor(page).locator(".orbital-markdown-editor__preview");
    await expect(preview.getByRole("heading", { name: "Plan", level: 2 })).toBeVisible();
    await expect(preview.locator("table thead th")).toHaveText(["Step", "Owner"]);
    await expect(preview.locator("table tbody td")).toHaveText(["Ship", "Sam"]);
  });

  test("ME-10: the writing area grows with the text up to its max, then scrolls", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await openComponentPreview(page, "markdown-editor");
    const textarea = editor(page).locator("textarea");
    await textarea.fill("");
    const empty = await height(textarea);
    await textarea.fill(lines(12));
    await expect.poll(() => height(textarea)).toBeGreaterThan(empty);

    await textarea.fill(lines(40));
    const max = await textarea.evaluate((el) => parseFloat(getComputedStyle(el).maxHeight));
    expect(Math.abs(max - 560)).toBeLessThanOrEqual(1);
    await expect.poll(async () => Math.abs((await height(textarea)) - max)).toBeLessThanOrEqual(1);
    expect(await textarea.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);

    // Typing at the end keeps the caret in view.
    await textarea.focus();
    await textarea.press("Control+End");
    await textarea.pressSequentially("\nmore");
    await expect
      .poll(() =>
        textarea.evaluate((el) => el.scrollHeight - el.scrollTop - el.clientHeight),
      )
      .toBeLessThanOrEqual(2);
  });

  test("ME-11: dragging the grip changes only the height", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    const textarea = root.locator("textarea");
    await textarea.fill("short");
    const before = await textarea.boundingBox();
    const handle = await grip(root).boundingBox();
    if (!before || !handle) throw new Error("editor not laid out");
    const x = handle.x + handle.width / 2;
    const y = handle.y + handle.height / 2;
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x - 80, y + 120, { steps: 6 });
    await page.mouse.up();

    const after = await textarea.boundingBox();
    if (!after) throw new Error("editor not laid out");
    expect(after.height).toBeGreaterThan(before.height + 100);
    expect(Math.abs(after.width - before.width)).toBeLessThanOrEqual(1);

    // The dragged height holds while the text still fits...
    await textarea.press("End");
    await textarea.pressSequentially(" text");
    expect(Math.abs((await height(textarea)) - after.height)).toBeLessThanOrEqual(1);
    // ...and gives way to auto-grow once the text outgrows it.
    await textarea.fill(lines(30));
    await expect.poll(() => height(textarea)).toBeGreaterThan(after.height + 50);
  });

  test("ME-12: the grip resizes from the keyboard", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    const textarea = root.locator("textarea");
    await textarea.fill("short");
    const start = await height(textarea);
    const line = await textarea.evaluate((el) => parseFloat(getComputedStyle(el).lineHeight));
    const handle = grip(root);
    await expect(handle).toHaveAttribute("aria-valuenow", "6");
    await expect(handle).toHaveAttribute("aria-valuemin", "6");

    await handle.focus();
    await handle.press("ArrowDown");
    await expect.poll(() => height(textarea)).toBeCloseTo(start + line, 0);
    await expect(handle).toHaveAttribute("aria-valuenow", "7");
    await handle.press("Shift+ArrowDown");
    await expect.poll(() => height(textarea)).toBeCloseTo(start + 6 * line, 0);
    await handle.press("ArrowUp");
    await expect.poll(() => height(textarea)).toBeCloseTo(start + 5 * line, 0);

    await handle.press("End");
    const max = await textarea.evaluate((el) => parseFloat(getComputedStyle(el).maxHeight));
    await expect.poll(() => height(textarea)).toBeCloseTo(max, 0);
    await handle.press("Home");
    await expect.poll(() => height(textarea)).toBeCloseTo(start, 0);

    await handle.press("Shift+ArrowDown");
    await handle.press("Enter");
    await expect.poll(() => height(textarea)).toBeCloseTo(start, 0);
  });

  test("ME-13: Preview keeps the Write height", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    await root.locator("textarea").fill(lines(15));
    const write = root.locator(".orbital-markdown-editor__write");
    await expect.poll(() => height(write)).toBeGreaterThan(200);
    const writeHeight = await height(write);
    await root.getByRole("tab", { name: "Preview" }).click();
    const preview = root.locator(".orbital-markdown-editor__preview");
    await expect(preview).toBeVisible();
    expect(Math.abs((await height(preview)) - writeHeight)).toBeLessThanOrEqual(1);
  });

  test("ME-14: Strikethrough wraps the selection from the button and Ctrl+Shift+X", async ({
    page,
  }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    const textarea = root.locator("textarea");
    await textarea.fill("old plan");
    await select(textarea, 0, 3);
    await root.getByRole("button", { name: "Strikethrough", exact: true }).click();
    await expect(textarea).toHaveValue("~~old~~ plan");
    expect(await selectedText(textarea)).toBe("old");

    await textarea.fill("drop");
    await select(textarea, 0, 4);
    await textarea.press("Control+Shift+KeyX");
    await expect(textarea).toHaveValue("~~drop~~");
  });

  test("ME-15: Ctrl+K inserts a link and Ctrl+Shift+8 / 7 make lists", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const textarea = editor(page).locator("textarea");
    await textarea.fill("docs");
    await select(textarea, 0, 4);
    await textarea.press("Control+k");
    await expect(textarea).toHaveValue("[docs](https://)");
    expect(await selectedText(textarea)).toBe("https://");

    await textarea.fill("a\nb");
    await select(textarea, 0, 3);
    await textarea.press("Control+Shift+Digit8");
    await expect(textarea).toHaveValue("- a\n- b");

    await textarea.fill("a\nb");
    await select(textarea, 0, 3);
    await textarea.press("Control+Shift+Digit7");
    await expect(textarea).toHaveValue("1. a\n2. b");
  });

  test("ME-16: Insert image writes the URL and alt text from the dialog", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    const textarea = root.locator("textarea");
    await textarea.fill("see ");
    await select(textarea, 4, 4);
    await root.getByRole("button", { name: "Insert image", exact: true }).click();

    const dialog = page.getByRole("dialog", { name: "Insert image" });
    await expect(dialog).toBeVisible();
    const insert = dialog.getByRole("button", { name: "Insert", exact: true });
    await expect(insert).toBeDisabled();
    // No upload handler in this example, so the dialog takes a URL only.
    await expect(dialog.getByRole("button", { name: "Upload image" })).toHaveCount(0);

    await dialog.getByRole("textbox", { name: "Image URL" }).fill("https://files.example.test/chart.png");
    await dialog.getByRole("textbox", { name: "Alt text" }).fill("Write rate");
    await expect(insert).toBeEnabled();
    await insert.click();
    await expect(dialog).toBeHidden();
    await expect(textarea).toHaveValue("see ![Write rate](https://files.example.test/chart.png)");
  });

  test("ME-17: Insert table writes the size picked and selects the first header", async ({
    page,
  }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    const textarea = root.locator("textarea");
    await textarea.fill("Owners:");
    await select(textarea, 7, 7);
    await root.getByRole("button", { name: "Insert table", exact: true }).click();
    const picker = root.getByRole("dialog", { name: "Insert table" });
    await expect(picker).toBeVisible();
    const cell = picker.getByRole("button", { name: "3 by 4 table" });
    await cell.hover();
    await expect(picker.getByText("3 × 4")).toBeVisible();
    await cell.click();
    await expect(picker).toBeHidden();
    await expect(textarea).toHaveValue(
      "Owners:\n\n| Column 1 | Column 2 | Column 3 | Column 4 |\n| --- | --- | --- | --- |\n| | | | |\n| | | | |",
    );
    await expect.poll(() => selectedText(textarea)).toBe("Column 1");

    // From the keyboard: the 1 by 1 cell takes focus, arrows grow the size, Enter inserts.
    await textarea.fill("");
    await root.getByRole("button", { name: "Insert table", exact: true }).click();
    await expect(picker.getByRole("button", { name: "1 by 1 table" })).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("ArrowDown");
    await expect(picker.getByText("2 × 2")).toBeVisible();
    await page.keyboard.press("Enter");
    await expect(textarea).toHaveValue("| Column 1 | Column 2 |\n| --- | --- |\n| | |");

    // Escape closes the picker and hands focus back to the textarea.
    await root.getByRole("button", { name: "Insert table", exact: true }).click();
    await expect(picker).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(picker).toBeHidden();
    await expect(textarea).toBeFocused();
  });

  test("ME-18: a mention picked from the @ list renders in Preview", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page, "markdown-editor-mentions");
    await root.scrollIntoViewIfNeeded();
    const textarea = root.locator("textarea");
    await textarea.click();
    await textarea.press("Control+End");

    await textarea.pressSequentially("@");
    await expect(root.getByText("Type two or more letters to find people and teams")).toBeVisible();
    await textarea.pressSequentially("pr");
    const list = root.getByRole("listbox", { name: "People and teams" });
    await expect(list.getByRole("option")).toHaveCount(1);
    await expect(list.getByRole("option")).toContainText("Priya Nair");
    await textarea.press("Enter");
    await expect(list).toBeHidden();
    await expect(textarea).toHaveValue("Can you check step 2? cc @[Priya Nair](u_priya) ");

    await root.getByRole("tab", { name: "Preview" }).click();
    await expect(
      root.locator('.orbital-markdown-editor__preview a[data-mention-id="u_priya"]'),
    ).toHaveText("@Priya Nair");
  });

  test("ME-19: the @ list moves with arrows, says when nothing matches, and closes on Escape", async ({
    page,
  }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page, "markdown-editor-mentions");
    await root.scrollIntoViewIfNeeded();
    const textarea = root.locator("textarea");
    await textarea.fill("");

    // The @ button starts a mention at the caret.
    await root.getByRole("button", { name: "Mention someone", exact: true }).click();
    await expect(textarea).toHaveValue("@");
    await expect(textarea).toBeFocused();
    await textarea.pressSequentially("na");
    const options = root.getByRole("listbox", { name: "People and teams" }).getByRole("option");
    await expect(options).toHaveCount(2);
    await expect(options.nth(0)).toHaveAttribute("aria-selected", "true");
    await textarea.press("ArrowDown");
    await expect(options.nth(1)).toHaveAttribute("aria-selected", "true");
    const active = await options.nth(1).getAttribute("id");
    await expect(textarea).toHaveAttribute("aria-activedescendant", active ?? "");
    await textarea.press("Tab");
    await expect(textarea).toHaveValue("@[Lena Kovac](u_lena) ");

    await textarea.pressSequentially("@zz");
    await expect(root.getByText("No one matches “zz”")).toBeVisible();
    await textarea.press("Escape");
    await expect(root.locator(".orbital-markdown-editor__mentions")).toHaveCount(0);
    await textarea.pressSequentially("z");
    await expect(root.locator(".orbital-markdown-editor__mentions")).toHaveCount(0);
  });

  test("ME-20: with no people provider there's no @ button and @ opens nothing", async ({
    page,
  }) => {
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    await expect(root.getByRole("button", { name: "Mention someone" })).toHaveCount(0);
    const textarea = root.locator("textarea");
    await textarea.fill("");
    await textarea.pressSequentially("Ask @pr");
    await expect(root.locator(".orbital-markdown-editor__mentions")).toHaveCount(0);
    await expect(textarea).not.toHaveAttribute("aria-autocomplete", "list");
  });

  test("ME-21: at phone width the extra actions move into More", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await openComponentPreview(page, "markdown-editor");
    const root = editor(page);
    await expect(root.getByRole("button", { name: "Bold", exact: true })).toBeVisible();
    await expect(root.getByRole("button", { name: "Insert link", exact: true })).toBeVisible();
    await expect(root.getByRole("button", { name: "Strikethrough", exact: true })).toBeHidden();
    await expect(root.getByRole("button", { name: "Insert table", exact: true })).toBeHidden();

    const textarea = root.locator("textarea");
    await textarea.fill("old");
    await select(textarea, 0, 3);
    await root.getByRole("button", { name: "More" }).click();
    await page.getByRole("menuitem", { name: "Strikethrough" }).click();
    await expect(textarea).toHaveValue("~~old~~");
  });

  test("ME-A1: axe clean in dark and light at 390px and desktop", async ({ page }) => {
    await openComponentPreview(page, "markdown-editor");
    const scan = async (label: string) => {
      for (const testId of [
        "markdown-editor-preview",
        "markdown-editor-limit",
        "markdown-editor-field",
        "markdown-editor-disabled",
        "markdown-editor-mentions",
        "markdown-editor-compact",
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
