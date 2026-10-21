import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("combobox primitive preview", () => {

  test("CB-01: basic combobox preview renders", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await expect(page.getByTestId("combobox-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("CB-02: typing opens listbox options", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.fill("ap");
    await expect(page.locator(".orbital-listbox .orbital-combobox-option").first()).toBeVisible();
  });

  test("CB-03: arrow down and enter select active option", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await expect(page.locator(".orbital-combobox-option[data-activedescendant-focusvisible]").first())
      .toBeVisible();
    await input.press("Enter");
    await expect(input).toHaveValue("Apple");
  });

  test("CB-04: clearable combobox clears current selection", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-clearable").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("combobox-clearable");
    await expect(wrapper.locator(".orbital-combobox__clear-icon")).toBeVisible();
    await wrapper.locator(".orbital-combobox__clear-icon").click();
    await expect(wrapper.locator("input[role='combobox']")).toHaveValue("");
  });

  test("CB-05: disabled combobox cannot be edited", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("combobox-disabled").locator("input[role='combobox']")).toBeDisabled();
  });

  test("CB-06: multiselect toggles selected options", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-multiselect").scrollIntoViewIfNeeded();
    const input = page.getByTestId("combobox-multiselect").locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await input.press(" ");
    await expect(page.locator(".orbital-combobox-option--selected").first()).toBeVisible();
  });

  test("CB-07: field label is associated with combobox input", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("combobox-field").locator("label").getAttribute("for");
    const inputId = await page.getByTestId("combobox-field")
      .locator("input[role='combobox']")
      .getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });

  test("CB-08: typing filters options visually", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.fill("ban");
    await expect(
      page.locator(".orbital-combobox-option:not(.orbital-combobox-option--hidden)", { hasText: "Apple" }),
    ).toHaveCount(0);
    await expect(
      page.locator(".orbital-combobox-option:not(.orbital-combobox-option--hidden)", { hasText: "Banana" }),
    ).toBeVisible();
  });

  test("CB-A1: no aria-prohibited-attr in empty and selected states", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const scan = async (testId: string) => {
      const region = page.getByTestId(testId);
      await region.scrollIntoViewIfNeeded();
      // The bare demo inputs have no Field label (CB-07 covers labelling), so skip that one rule.
      const results = await new AxeBuilder({ page })
        .include(`[data-testid='${testId}']`)
        .disableRules(["label"])
        .analyze();
      expect(results.violations.filter((v) => v.id === "aria-prohibited-attr")).toEqual([]);
      expect(
        results.violations.filter((v) => v.impact === "critical" || v.impact === "serious"),
      ).toEqual([]);
    };

    // Empty state renders the chevron.
    await scan("combobox-preview");
    await expect(
      page.getByTestId("combobox-preview").locator(".orbital-combobox__expand-icon"),
    ).toHaveCount(1);

    // Selected state renders the Clear button.
    const preview = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await preview.click();
    await preview.press("ArrowDown");
    await preview.press("Enter");
    await expect(preview).toHaveAttribute("aria-expanded", "false");
    await scan("combobox-preview");

    await scan("combobox-clearable");
    await scan("combobox-disabled");
    await scan("combobox-disabled-selected");

    const multi = page.getByTestId("combobox-clearable-multiselect");
    await multi.scrollIntoViewIfNeeded();
    const multiInput = multi.locator("input[role='combobox']");
    await multiInput.click();
    await multiInput.press("ArrowDown");
    await multiInput.press(" ");
    await multiInput.press("Tab");
    await expect(multiInput).toHaveAttribute("aria-expanded", "false");
    await scan("combobox-clearable-multiselect");
  });

  test("CB-A2: expand icon is decorative and still toggles the listbox", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const wrapper = page.getByTestId("combobox-preview");
    const icon = wrapper.locator(".orbital-combobox__expand-icon");
    const input = wrapper.locator("input[role='combobox']");
    await expect(icon).toHaveAttribute("aria-hidden", "true");
    await expect(icon).not.toHaveAttribute("aria-label", /.*/);
    await expect(wrapper.getByRole("button", { name: "Open" })).toHaveCount(0);

    await icon.click();
    await expect(input).toHaveAttribute("aria-expanded", "true");
    await expect(input).toBeFocused();
    await icon.click();
    await expect(input).toHaveAttribute("aria-expanded", "false");
    await expect(input).toBeFocused();
  });

  for (const key of ["Enter", "Space"]) {
    test(`CB-A3: Clear button is a Tab stop and ${key} clears`, async ({ page }) => {
      await openComponentPreview(page, "combobox");
      const wrapper = page.getByTestId("combobox-clearable");
      await wrapper.scrollIntoViewIfNeeded();
      const input = wrapper.locator("input[role='combobox']");
      const clear = wrapper.getByRole("button", { name: "Clear" });
      await expect(clear).toBeVisible();

      await input.focus();
      await page.keyboard.press("Tab");
      await expect(clear).toBeFocused();
      const outline = await clear.evaluate((el) => getComputedStyle(el).outlineStyle);
      expect(outline).not.toBe("none");

      await page.keyboard.press(key);
      await expect(input).toHaveValue("");
      await expect(clear).toHaveCount(0);
      await expect(input).toBeFocused();
    });
  }

  test("CB-A4: mouse click on Clear resets and leaves the input focused", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const wrapper = page.getByTestId("combobox-clearable");
    await wrapper.scrollIntoViewIfNeeded();
    await wrapper.getByRole("button", { name: "Clear" }).click();
    const input = wrapper.locator("input[role='combobox']");
    await expect(input).toHaveValue("");
    await expect(input).toBeFocused();
  });

  test("CB-A5: keyboard Clear empties a multiselect selection", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const wrapper = page.getByTestId("combobox-clearable-multiselect");
    await wrapper.scrollIntoViewIfNeeded();
    const input = wrapper.locator("input[role='combobox']");
    const selected = page.locator(".orbital-combobox-option--selected");
    await input.click();
    await input.press("ArrowDown");
    await input.press(" ");
    await input.press("ArrowDown");
    await input.press(" ");
    await expect(selected).toHaveCount(2);

    await input.press("Escape");
    await input.press("Tab");
    const clear = wrapper.getByRole("button", { name: "Clear" });
    await expect(clear).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(clear).toHaveCount(0);
    await input.click();
    await expect(selected).toHaveCount(0);
  });

  test("CB-A6: Clear is disabled with a disabled combobox", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const wrapper = page.getByTestId("combobox-disabled-selected");
    await wrapper.scrollIntoViewIfNeeded();
    const input = wrapper.locator("input[role='combobox']");
    const clear = wrapper.getByRole("button", { name: "Clear" });
    await expect(input).toBeDisabled();
    await expect(clear).toBeDisabled();
    await clear.click({ force: true });
    await expect(input).toHaveValue("Apple");
    await expect(clear).toBeVisible();
  });
});
