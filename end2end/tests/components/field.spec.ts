import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("field primitive preview", () => {

  test("F-01: labeled input associates label with control", async ({ page }) => {
    await openComponentPreview(page, "field");
    const field = page.getByTestId("field-preview");
    const labelFor = await field.locator("label").getAttribute("for");
    const inputId = await field.locator("input").getAttribute("id");
    expect(labelFor).toBeTruthy();
    expect(labelFor).toEqual(inputId);
  });

  test("F-02: required field shows asterisk", async ({ page }) => {
    await openComponentPreview(page, "field");
    await page.getByTestId("field-required").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("field-required").locator(".orbital-label__required")).toBeVisible();
  });

  test("F-03: required field validates on blur", async ({ page }) => {
    await openComponentPreview(page, "field");
    await page.getByTestId("field-required").scrollIntoViewIfNeeded();
    const input = page.getByTestId("field-required").locator("input");
    await input.focus();
    await input.blur();
    await expect(page.getByTestId("field-required").locator(".orbital-field__validation-message"))
      .toBeVisible();
  });

  test("F-04: horizontal layout uses horizontal modifier class", async ({ page }) => {
    await openComponentPreview(page, "field");
    await page.getByTestId("field-horizontal").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("field-horizontal").locator(".orbital-field--horizontal")).toBeVisible();
  });

  test("F-05: select in field associates label and validates on empty choice", async ({ page }) => {
    await openComponentPreview(page, "field");
    await page.getByTestId("field-select").scrollIntoViewIfNeeded();
    const field = page.getByTestId("field-select");
    const labelFor = await field.locator("label").getAttribute("for");
    const selectId = await field.locator("select").getAttribute("id");
    expect(labelFor).toEqual(selectId);
    const select = field.locator("select");
    await select.selectOption("");
    await expect(field.locator(".orbital-field__validation-message")).toBeVisible();
  });

  test("F-06: label_hidden hides the label but keeps the accessible name", async ({ page }) => {
    await openComponentPreview(page, "field");
    const field = page.getByTestId("field-label-hidden");
    await field.scrollIntoViewIfNeeded();
    const label = field.locator("label");
    await expect(label).toHaveClass(/sr-only/);
    const box = await label.boundingBox();
    expect(box!.width).toBeLessThanOrEqual(1);
    expect(box!.height).toBeLessThanOrEqual(1);
    await expect(field.getByRole("textbox", { name: "Quick add task" })).toBeVisible();
  });

  test("F-07: date pickers fill their fields and fit a narrow two-column card", async ({ page }) => {
    await openComponentPreview(page, "field");
    const card = page.getByTestId("field-date-grid");
    await card.scrollIntoViewIfNeeded();
    const cardBox = (await card.boundingBox())!;
    const fields = card.locator(".orbital-field");
    await expect(fields).toHaveCount(2);
    for (let i = 0; i < 2; i++) {
      const fieldBox = (await fields.nth(i).boundingBox())!;
      const pickerBox = (await fields.nth(i).locator(".orbital-input").boundingBox())!;
      expect(pickerBox.x + pickerBox.width).toBeLessThanOrEqual(cardBox.x + cardBox.width);
      expect(Math.abs(pickerBox.width - fieldBox.width)).toBeLessThanOrEqual(1);
    }
    const overflow = await card.evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(overflow).toBeLessThanOrEqual(0);
  });

  test("F-08: a labeled input fills its field", async ({ page }) => {
    await openComponentPreview(page, "field");
    const field = page.getByTestId("field-preview").locator(".orbital-field");
    const [f, i] = await Promise.all([
      field.boundingBox(),
      field.locator(".orbital-input").boundingBox(),
    ]);
    expect(Math.abs(f!.width - i!.width)).toBeLessThanOrEqual(1);
  });
});
