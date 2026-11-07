import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-presets preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    await expect(page.getByTestId("scheduler-timeline-presets-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    await expectPreviewVariants(page, ["scheduler-timeline-presets-preview"]);
  });

  test("switching preset changes header title", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    const wrapper = page.getByTestId("scheduler-timeline-presets-preview");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    const initial = await title.innerText();

    const presetSelect = wrapper.getByTestId("scheduler-timeline-preset-select").locator("select");
    await presetSelect.selectOption("business_day");
    await expect(title).not.toHaveText(initial);

    const businessDayTitle = await title.innerText();
    await presetSelect.selectOption("week");
    await expect(title).not.toHaveText(businessDayTitle);
  });

  test("switching to Month shows one column per day", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    const wrapper = page.getByTestId("scheduler-timeline-presets-preview");
    const presetSelect = wrapper.getByTestId("scheduler-timeline-preset-select").locator("select");
    await presetSelect.selectOption("month");

    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    await expect(title).toHaveText(/^[A-Z][a-z]+ \d{4}$/);

    const content = wrapper.locator(".orb-scheduler-timeline__time-content");
    const start = Number(await content.getAttribute("data-range-start-unix"));
    const end = Number(await content.getAttribute("data-range-end-unix"));
    // Month length (28 to 31 days, give or take a DST hour) from the visible range.
    const days = Math.round((end + 1 - start) / 86_400);
    expect(days).toBeGreaterThanOrEqual(28);
    expect(days).toBeLessThanOrEqual(31);

    const columns = wrapper.locator(".orb-scheduler-timeline__time-column");
    await expect(columns).toHaveCount(days);
    await expect(columns.first()).toHaveText("1");
    await expect(columns.last()).toHaveText(String(days));
  });

  test("switching to Quarter shows week columns", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    const wrapper = page.getByTestId("scheduler-timeline-presets-preview");
    const presetSelect = wrapper.getByTestId("scheduler-timeline-preset-select").locator("select");
    await presetSelect.selectOption("quarter");

    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    await expect(title).toHaveText(/^[A-Z][a-z]{2} – [A-Z][a-z]{2} \d{4}$/);

    const columns = wrapper.locator(".orb-scheduler-timeline__time-column");
    await expect.poll(() => columns.count()).toBeGreaterThanOrEqual(13);
    const labels = await columns.allInnerTexts();
    expect(labels.length).toBeLessThanOrEqual(15);
    // The first column starts on the quarter's first day; every later one on a Monday.
    expect(labels[0]).toMatch(/^\d{4}-(01|04|07|10)-01$/);
    for (const label of labels.slice(1)) {
      expect(new Date(`${label}T00:00:00Z`).getUTCDay()).toBe(1);
    }
  });
});
