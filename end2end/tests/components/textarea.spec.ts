import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("textarea primitive preview", () => {

  test("TA-01: empty textarea accepts typing", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    const textarea = page.getByTestId("textarea-preview").locator("textarea");
    await textarea.fill("New note");
    await expect(textarea).toHaveValue("New note");
  });

  test("TA-02: filled textarea shows initial value", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    await page.getByTestId("textarea-filled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("textarea-filled").locator("textarea")).toHaveValue("Hello");
  });

  test("TA-03: disabled textarea cannot be edited", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    await page.getByTestId("textarea-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("textarea-disabled").locator("textarea")).toBeDisabled();
  });

  test("TA-04: fixed size textarea disables resize", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    await page.getByTestId("textarea-fixed").scrollIntoViewIfNeeded();
    const resize = await page.getByTestId("textarea-fixed").locator("textarea")
      .evaluate((el) => getComputedStyle(el).resize);
    expect(resize).toBe("none");
  });

  test("TA-05: required field validates on blur", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    await page.getByTestId("textarea-field").scrollIntoViewIfNeeded();
    const textarea = page.getByTestId("textarea-field").locator("textarea");
    await textarea.focus();
    await textarea.blur();
    await expect(page.getByTestId("textarea-field").locator(".orbital-field__validation-message"))
      .toBeVisible();
  });

  test("TA-06: rows sets the attribute and the height follows it", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    const rowsArea = page.getByTestId("textarea-rows").locator("textarea");
    await rowsArea.scrollIntoViewIfNeeded();
    await expect(rowsArea).toHaveAttribute("rows", "6");
    const defaultHeight = await page.getByTestId("textarea-preview").locator("textarea")
      .evaluate((el) => el.getBoundingClientRect().height);
    const rowsHeight = await rowsArea.evaluate((el) => el.getBoundingClientRect().height);
    const lineHeight = await rowsArea.evaluate((el) => parseFloat(getComputedStyle(el).lineHeight));
    expect(rowsHeight).toBeGreaterThan(defaultHeight);
    expect(rowsHeight).toBeGreaterThanOrEqual(lineHeight * 6);
  });

  test("TA-07: readonly textarea keeps its value and blocks typing", async ({ page }) => {
    await openComponentPreview(page, "textarea");
    const textarea = page.getByTestId("textarea-readonly").locator("textarea");
    await textarea.scrollIntoViewIfNeeded();
    await expect(textarea).toHaveAttribute("readonly", "");
    await expect(textarea).toBeEnabled();
    await textarea.focus();
    await page.keyboard.type("x");
    await expect(textarea).toHaveValue("Signed off on 12 March");
  });
});
