import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("currency-input preview", () => {
  test("CI-01: symbol sits before the amount for en-US and after it for de-DE", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const basic = page.getByTestId("currency-input-preview");
    await expect(basic.locator(".orbital-input__prefix")).toHaveText("$");
    await expect(basic.locator(".orbital-input__suffix")).toHaveCount(0);
    await expect(basic.getByLabel("Price")).toHaveValue("1,234.56");
    const de = page.getByTestId("currency-input-locale");
    await expect(de.locator(".orbital-input__suffix")).toHaveText("€");
    await expect(de.locator(".orbital-input__prefix")).toHaveCount(0);
    await expect(de.getByLabel("Betrag")).toHaveValue("1.234,56");
  });

  test("CI-02: focus shows the plain number and blur formats it", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const input = page.getByTestId("currency-input-preview").getByLabel("Price");
    await input.focus();
    await expect(input).toHaveValue("1234.56");
    await input.fill("1234.5");
    await expect(input).toHaveValue("1234.5");
    await input.blur();
    await expect(input).toHaveValue("1,234.50");
  });

  test("CI-03: de-DE types with a decimal comma and formats with dot groups", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const input = page.getByTestId("currency-input-locale").getByLabel("Betrag");
    await input.focus();
    await input.fill("98765,4");
    await expect(input).toHaveValue("98765,4");
    await input.blur();
    await expect(input).toHaveValue("98.765,40");
  });

  test("CI-04: pasted text is normalised", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const input = page.getByTestId("currency-input-preview").getByLabel("Price");
    await input.focus();
    await input.fill("$7,654.32");
    await expect(input).toHaveValue("7654.32");
    await input.blur();
    await expect(input).toHaveValue("7,654.32");
  });

  test("CI-05: an unreadable edit is rejected and the last good text stays", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const input = page.getByTestId("currency-input-preview").getByLabel("Price");
    await input.focus();
    await input.fill("12.34");
    await expect(input).toHaveValue("12.34");
    await input.pressSequentially("5");
    await expect(input).toHaveValue("12.34");
    await input.pressSequentially("x");
    await expect(input).toHaveValue("12.34");
    await input.blur();
    await expect(input).toHaveValue("12.34");
  });

  test("CI-06: the currency select changes the symbol and decimals", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const multi = page.getByTestId("currency-input-multi");
    const input = multi.getByLabel("Amount");
    await expect(multi.locator(".orbital-input__prefix")).toHaveText("$");
    await multi.getByLabel("Currency").selectOption("JPY");
    await expect(multi.locator(".orbital-input__prefix")).toHaveText("¥");
    await expect(input).toHaveValue("1,235");
    await input.focus();
    await input.pressSequentially(".5");
    await expect(input).toHaveValue("1235");
  });

  test("CI-07: minus is accepted only when allowed", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const negative = page.getByTestId("currency-input-negative").getByLabel("Adjustment");
    await expect(negative).toHaveValue("-25.00");
    await negative.focus();
    await negative.fill("-3.5");
    await expect(negative).toHaveValue("-3.5");
    const basic = page.getByTestId("currency-input-preview").getByLabel("Price");
    await basic.focus();
    await basic.fill("1");
    await basic.pressSequentially("-");
    await expect(basic).toHaveValue("1");
  });

  test("CI-08: disabled currency input blocks editing", async ({ page }) => {
    await openComponentPreview(page, "currency-input");
    const input = page.getByTestId("currency-input-disabled").getByLabel("Total");
    await expect(input).toBeDisabled();
    await expect(input).toHaveValue("99.00");
  });
});
