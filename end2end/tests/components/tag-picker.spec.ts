import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("tag-picker primitive preview", () => {

  test("TP-01: default tag picker renders large control", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await expect(page.getByTestId("TP-01")).toBeVisible({ timeout: 30_000 });
    await expect(
      page.getByTestId("TP-01").locator(".orbital-tag-picker-control--large"),
    ).toBeVisible();
    await expect(page.getByTestId("TP-01").locator("input[role='combobox']")).toBeVisible();
  });

  test("TP-02: preselected value renders dismissible tag", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-02").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-02");
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(1);
    await expect(wrapper.locator(".orbital-tag")).toHaveText("Apple");
    await expect(wrapper.locator(".orbital-tag--dismissible")).toBeVisible();
  });

  test("TP-03: keyboard arrow down and enter selects option", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-03").scrollIntoViewIfNeeded();
    const input = page.getByTestId("TP-03").locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await expect(
      page.locator(".orbital-tag-picker-option[data-activedescendant-focusvisible]").first(),
    ).toBeVisible();
    await input.press("Enter");
    await expect(page.getByTestId("TP-03").locator(".orbital-tag")).toHaveCount(1);
  });

  test("TP-04: dismiss button removes selected tag", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-04").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-04");
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(2);
    await wrapper.locator(".orbital-tag__dismiss").first().click();
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(1);
  });

  test("TP-05: disabled option cannot be selected", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-05").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-05");
    const input = wrapper.locator("input[role='combobox']");
    await input.click();
    await expect(wrapper.locator(".orbital-tag-picker-option--disabled")).toBeVisible();
    await wrapper.locator(".orbital-tag-picker-option--disabled").click({ force: true });
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(0);
  });

  test("TP-06: option groups render grouped labels", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-06").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-06");
    const input = wrapper.locator("input[role='combobox']");
    await input.click();
    await expect(wrapper.locator(".orbital-tag-picker-option-group__label", { hasText: "Warm" })).toBeVisible();
    await expect(wrapper.locator(".orbital-tag-picker-option-group__label", { hasText: "Cool" })).toBeVisible();
  });

  test("TP-07: extra-large size applies control class", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-07").scrollIntoViewIfNeeded();
    await expect(
      page.getByTestId("TP-07").locator(".orbital-tag-picker-control--extra-large"),
    ).toBeVisible();
  });

  test("TP-08: on_search reports typed text and resets after selection", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-08").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-08");
    const input = wrapper.locator("input[role='combobox']");
    const query = page.getByTestId("TP-08-query");
    await input.click();
    await input.pressSequentially("gr");
    await expect(query).toHaveText("Query: gr");
    await expect(wrapper.locator(".orbital-tag-picker-option")).toHaveText(["Groceries"]);
    await input.fill("");
    await expect(query).toHaveText("Query:");
    await input.pressSequentially("rent");
    await wrapper.locator(".orbital-tag-picker-option", { hasText: "Rent" }).click();
    await expect(wrapper.locator(".orbital-tag")).toHaveText(["Rent"]);
    await expect(query).toHaveText("Query:");
    await expect(input).toHaveValue("");
  });

  test("TP-09: clicking an action option runs its callback without selecting", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-09").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-09");
    await wrapper.locator("input[role='combobox']").click();
    await wrapper.locator(".orbital-tag-picker-option", { hasText: "Travel (new)" }).click();
    await expect(page.getByTestId("TP-09-created")).toHaveText("Created: 1");
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(0);
    await expect(
      wrapper.locator(".orbital-tag-picker-option", { hasText: "Travel (new)" }),
    ).toBeHidden();
  });

  test("TP-09: Enter on a focused action option runs its callback", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-09").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-09");
    const input = wrapper.locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await expect(
      wrapper.locator(".orbital-tag-picker-option[data-activedescendant-focusvisible]", {
        hasText: "Travel (new)",
      }),
    ).toBeVisible();
    await input.press("Enter");
    await expect(page.getByTestId("TP-09-created")).toHaveText("Created: 1");
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(0);
  });

  test("TP-09: disabled action option does nothing", async ({ page }) => {
    await openComponentPreview(page, "tag-picker");
    await page.getByTestId("TP-09").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("TP-09");
    await wrapper.locator("input[role='combobox']").click();
    await wrapper
      .locator(".orbital-tag-picker-option--disabled", { hasText: "Locked (new)" })
      .click({ force: true });
    await expect(page.getByTestId("TP-09-created")).toHaveText("Created: 0");
    await expect(wrapper.locator(".orbital-tag")).toHaveCount(0);
  });
});
