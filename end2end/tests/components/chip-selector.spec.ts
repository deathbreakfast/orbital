import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = [
  "chip-selector-default",
  "chip-selector-linked",
  "chip-selector-badges",
  "chip-selector-wrap",
  "chip-selector-remove-last",
  "chip-selector-single",
  "chip-selector-persona",
  "chip-selector-persona-multi",
];

// The default example's wrapper is chip-selector-default, not the helper's
// `${slug}-preview` fallback.
const openChipSelector = (page: Page) =>
  openComponentPreview(page, "chip-selector", "chip-selector-default");

const control = (wrapper: Locator) => wrapper.locator(".orbital-tag-picker-control");
const chips = (wrapper: Locator) => wrapper.locator(".orbital-tag");
const input = (wrapper: Locator) => wrapper.locator("input[role='combobox']");
const option = (wrapper: Locator, name: string) =>
  wrapper.locator(".orbital-tag-picker-option", { hasText: name });

test.describe("chip-selector preview", () => {
  test("CS-01: picking an option adds an outlined chip inside the input", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-default");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(0);
    await expect(input(wrapper)).toHaveAttribute("placeholder", "No tags. Search or create one");
    await input(wrapper).click();
    await option(wrapper, "security").click();
    await expect(chips(wrapper)).toHaveCount(1);
    await expect(chips(wrapper)).toHaveText("security");
    await expect(chips(wrapper)).toHaveClass(/orbital-tag--outline/);
    await expect(control(wrapper).locator(".orbital-tag")).toHaveCount(1);
    await expect(input(wrapper)).toHaveAttribute("placeholder", "Add a tag");
  });

  test("CS-02: typing filters the option list", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-default");
    await wrapper.scrollIntoViewIfNeeded();
    await input(wrapper).click();
    await input(wrapper).pressSequentially("sec");
    await expect(option(wrapper, "security")).toBeVisible();
    await expect(option(wrapper, "billing")).toHaveCount(0);
  });

  test("CS-03: chips link to their record and each has a remove button", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-linked");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(2);
    await expect(wrapper.locator("a[href='/tag/tg-7c1e']")).toHaveText("billing");
    await expect(wrapper.getByRole("button", { name: "Remove billing" })).toBeVisible();
    await wrapper.getByRole("button", { name: "Remove billing" }).click();
    await expect(chips(wrapper)).toHaveCount(1);
    await expect(chips(wrapper)).toHaveText("security");
  });

  test("CS-04: Backspace in the empty input removes the last chip", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-linked");
    await wrapper.scrollIntoViewIfNeeded();
    await input(wrapper).focus();
    await input(wrapper).press("Backspace");
    await expect(chips(wrapper)).toHaveCount(1);
    await expect(chips(wrapper)).toHaveText("billing");
    // Text in the input is edited first; chips stay.
    await input(wrapper).pressSequentially("x");
    await input(wrapper).press("Backspace");
    await expect(chips(wrapper)).toHaveCount(1);
  });

  test("CS-05: blocked-by chips carry a status badge", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-badges");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(2);
    await expect(chips(wrapper).first().locator(".orbital-badge")).toHaveText("Done");
    await expect(wrapper.getByRole("button", { name: "Remove Draft invoice schema" })).toBeVisible();
  });

  test("CS-06: many chips wrap and the field grows without sideways overflow", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-wrap");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(9);
    const box = await control(wrapper).boundingBox();
    expect(box).not.toBeNull();
    expect(box!.height).toBeGreaterThan(90);
    const overflow = await control(wrapper).evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
    for (const chip of await chips(wrapper).all()) {
      const c = await chip.boundingBox();
      expect(c).not.toBeNull();
      expect(c!.x).toBeGreaterThanOrEqual(box!.x - 1);
      expect(c!.x + c!.width).toBeLessThanOrEqual(box!.x + box!.width + 1);
    }
  });

  test("CS-07: removing the last chip shows the empty hint and keeps the height", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-remove-last");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(1);
    const before = await control(wrapper).boundingBox();
    await wrapper.getByRole("button", { name: "Remove Billing revamp" }).click();
    await expect(chips(wrapper)).toHaveCount(0);
    await expect(input(wrapper)).toHaveAttribute("placeholder", "Not in a project. Search projects");
    await expect(input(wrapper)).toBeFocused();
    const after = await control(wrapper).boundingBox();
    expect(Math.abs(after!.height - before!.height)).toBeLessThanOrEqual(1);
  });

  test("CS-08: single mode replaces the chip on the next pick", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-single");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveText("Payments roadmap");
    await input(wrapper).click();
    await option(wrapper, "Security review Q4").click();
    await expect(chips(wrapper)).toHaveCount(1);
    await expect(chips(wrapper)).toHaveText("Security review Q4");
  });

  test("CS-09: Persona owner shows a Persona, hides it while typing, and shows the pick", async ({
    page,
  }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-persona");
    await wrapper.scrollIntoViewIfNeeded();
    const selected = control(wrapper).locator(".orbital-persona");
    await expect(selected).toHaveCount(1);
    await expect(selected).toContainText("Theo Brandt");
    await expect(selected).toContainText("Person");
    await expect(chips(wrapper)).toHaveCount(0);

    await input(wrapper).click();
    await input(wrapper).pressSequentially("Pa");
    await expect(selected).toHaveCount(0);
    await expect(option(wrapper, "Payments").locator(".orbital-persona")).toBeVisible();
    await expect(option(wrapper, "Paula Mendes")).toBeVisible();
    await expect(option(wrapper, "Theo Brandt")).toHaveCount(0);

    await option(wrapper, "Payments").click();
    await expect(selected).toHaveCount(1);
    await expect(selected).toContainText("Payments");
    await expect(selected).toContainText("Team · Acme Robotics");
  });

  test("CS-10: Persona chips carry an avatar", async ({ page }) => {
    await openChipSelector(page);
    const wrapper = page.getByTestId("chip-selector-persona-multi");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(chips(wrapper)).toHaveCount(1);
    await expect(chips(wrapper).locator(".orbital-chip-selector__chip-avatar")).toHaveCount(1);
    await input(wrapper).click();
    await option(wrapper, "Theo Brandt").click();
    await expect(chips(wrapper)).toHaveCount(2);
  });

  test("CS-A1: axe clean in dark and light at 390px and desktop", async ({ page }) => {
    await openChipSelector(page);
    const scan = async (label: string, excludeLinks = false) => {
      for (const testId of EXAMPLES) {
        await page.getByTestId(testId).scrollIntoViewIfNeeded();
        let axe = new AxeBuilder({ page }).include(`[data-testid='${testId}']`);
        // The light theme's link token is 4.32:1 on white everywhere, not only
        // here; Magnon task 907a1086-d8d4-4898-89b0-da01d3240c35 fixes the token.
        if (excludeLinks) axe = axe.exclude(".orbital-chip-selector a.orbital-link");
        const results = await axe.analyze();
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
    await scan("light desktop", true);
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("light 390px", true);
  });
});
