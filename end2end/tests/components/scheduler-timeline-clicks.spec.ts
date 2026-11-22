import { test, expect, type Page } from "@playwright/test";
import { openComponentPreview, scrollIntoPreviewView } from "../lib/preview/navigation";

async function openClicks(page: Page) {
  await openComponentPreview(page, "scheduler-timeline", "scheduler-timeline-clicks");
  const wrapper = page.getByTestId("scheduler-timeline-clicks").last();
  await scrollIntoPreviewView(wrapper);
  return wrapper;
}

test.describe("scheduler timeline row and event clicks", () => {
  test("STC-01 clicking a resource label fires on_row_click with its id", async ({ page }) => {
    const wrapper = await openClicks(page);
    await wrapper.getByTestId("scheduler-resource-room-b").click();
    await expect(wrapper.getByTestId("scheduler-timeline-clicked")).toHaveText("Clicked: row room-b");
  });

  test("STC-02 the collapse toggle does not count as a row click", async ({ page }) => {
    const wrapper = await openClicks(page);
    await wrapper.getByTestId("scheduler-timeline-toggle-building").click();
    await expect(wrapper.getByTestId("scheduler-resource-room-a")).toHaveCount(0);
    await expect(wrapper.getByTestId("scheduler-timeline-clicked")).toHaveText("Clicked: nothing");
  });

  test("STC-03 clicking a bar or a point fires on_event_click", async ({ page }) => {
    const wrapper = await openClicks(page);
    await wrapper.getByTestId("scheduler-event-pt-review").click();
    await expect(wrapper.getByTestId("scheduler-timeline-clicked")).toHaveText("Clicked: event pt-review");
    await wrapper.getByTestId("scheduler-event-pt-gate").click();
    await expect(wrapper.getByTestId("scheduler-timeline-clicked")).toHaveText("Clicked: event pt-gate");
  });

  test("STC-04 the toolbar slot gets the padded toolbar chrome", async ({ page }) => {
    const wrapper = await openClicks(page);
    const slot = wrapper.locator(".orb-scheduler-toolbar--slot");
    await expect(slot.getByTestId("scheduler-timeline-custom-toolbar")).toBeVisible();
    const padding = await slot.evaluate((el) => parseFloat(getComputedStyle(el).paddingLeft));
    expect(padding).toBeGreaterThan(0);
  });
});
