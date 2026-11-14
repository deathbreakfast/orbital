import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-slots preview", () => {
  test("renders preview page with custom toolbar and event badges", async ({ page }) => {
    await openComponentPreview(page, "scheduler-slots");
    const wrapper = page.getByTestId("scheduler-slots-preview");
    await expect(wrapper).toBeVisible({ timeout: 30_000 });
    await expect(wrapper.getByTestId("scheduler-custom-toolbar")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-slot-event-badge").first()).toBeVisible();
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-slots");
    await expectPreviewVariants(page, ["scheduler-slots-preview"]);
  });

  test("editing tools slot opens dialog", async ({ page }) => {
    await openComponentPreview(page, "scheduler-slots");
    const wrapper = page.getByTestId("scheduler-slots-preview").first();
    await wrapper.getByTestId("scheduler-event-dialog-open").click();
    await expect(page.getByTestId("scheduler-event-dialog")).toBeVisible();
  });

  test("custom toolbar slot sits in the padded toolbar chrome", async ({ page }) => {
    await openComponentPreview(page, "scheduler-slots");
    const wrapper = page.getByTestId("scheduler-slots-preview").first();
    const slot = wrapper.locator(".orb-scheduler-toolbar--slot");
    await expect(slot.getByTestId("scheduler-custom-toolbar")).toBeVisible();
    const padding = await slot.evaluate((el) => parseFloat(getComputedStyle(el).paddingLeft));
    expect(padding).toBeGreaterThan(0);
  });
});
