import { test, expect } from "@playwright/test";
import { navigateSchedulingPreview, clickSchedulingCatalogLink } from "../lib/preview/navigation";
import { WasmPanicGuard } from "../lib/preview/wasm-panics";

test.describe("scheduler catalog navigation regressions", () => {
  test("timeline to timeline-navigation", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-timeline",
      "Scheduler Event Timeline Navigation",
      "scheduler-timeline-navigation-preview",
    );

    const wrapper = page.getByTestId("scheduler-timeline-navigation-preview");
    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    const initial = await title.innerText();
    await wrapper.getByTestId("scheduler-timeline-nav-next").click();
    await expect(title).not.toHaveText(initial);
    guard.assertClean();
  });

  test("timeline to calendar", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-timeline",
      "Scheduler Calendar",
      "scheduler-calendar-preview",
    );

    const wrapper = page.getByTestId("scheduler-calendar-preview");
    await expect(wrapper.getByTestId("scheduler-calendar-header-title")).toBeVisible();
    const title = await wrapper.getByTestId("scheduler-calendar-header-title").innerText();
    await wrapper.getByTestId("scheduler-calendar-nav-next").click();
    await expect(wrapper.getByTestId("scheduler-calendar-header-title")).not.toHaveText(title);
    guard.assertClean();
  });

  test("calendar to calendar-navigation", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-calendar",
      "Scheduler Calendar Navigation",
      "scheduler-calendar-navigation-preview",
    );

    const wrapper = page.getByTestId("scheduler-calendar-navigation-preview");
    const title = wrapper.getByTestId("scheduler-calendar-header-title");
    const initial = await title.innerText();
    await wrapper.getByTestId("scheduler-calendar-nav-next").click();
    await expect(title).not.toHaveText(initial);
    guard.assertClean();
  });

  test("timeline to drag then drag", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-timeline",
      "Scheduler Calendar Drag Interactions",
      "scheduler-calendar-drag-interactions-preview",
    );

    const week = page.getByTestId("scheduler-calendar-week-preview").first();
    await expect(week).toBeVisible();
    const event = week.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const startBefore = await event.getAttribute("data-start-unix");
    expect(startBefore).not.toBeNull();

    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    const x = box!.x + box!.width / 2;
    const y = box!.y + box!.height / 2;

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x, y + 60, { steps: 8 });
    await page.mouse.up();

    await expect
      .poll(async () => event.getAttribute("data-start-unix"))
      .not.toBe(startBefore);
    await expect(page.getByTestId("scheduler-event-drag-ghost")).toHaveCount(0);
    guard.assertClean();
  });

  test("navigate away mid-drag via scheduler hop", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-timeline",
      "Scheduler Calendar Drag Interactions",
      "scheduler-calendar-drag-interactions-preview",
    );

    const week = page.getByTestId("scheduler-calendar-week-preview").first();
    await expect(week).toBeVisible();
    const event = week.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    const x = box!.x + box!.width / 2;
    const y = box!.y + box!.height / 2;

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x, y + 60, { steps: 8 });
    // DOM click avoids Playwright actionability checks while the drag gesture holds the pointer.
    const nav = page.getByTestId("preview-catalog-nav");
    const timeline = nav.getByRole("link", {
      name: "Scheduler Event Timeline",
      exact: true,
    });
    if (!(await timeline.isVisible())) {
      for (const folder of ["Scheduling", "Event Timeline"]) {
        const btn = nav.getByRole("button", { name: folder, exact: true });
        if ((await btn.count()) > 0 && (await btn.getAttribute("aria-expanded")) !== "true") {
          await btn.click();
        }
      }
    }
    await timeline.evaluate((el) => (el as HTMLAnchorElement).click());
    await page.mouse.up();

    await expect(page.getByTestId("scheduler-timeline-preview")).toBeVisible();
    await expect(page.getByTestId("scheduler-event-drag-ghost")).toHaveCount(0);
    guard.assertClean();
  });

  test("scheduling round-trip", async ({ page }) => {
    const guard = new WasmPanicGuard(page);
    await navigateSchedulingPreview(
      page,
      "scheduler-timeline",
      "Scheduler Event Timeline Navigation",
      "scheduler-timeline-navigation-preview",
    );

    let wrapper = page.getByTestId("scheduler-timeline-navigation-preview");
    await wrapper.getByTestId("scheduler-timeline-nav-next").click();

    await clickSchedulingCatalogLink(
      page,
      "Scheduler Calendar",
      "scheduler-calendar-preview",
    );
    wrapper = page.getByTestId("scheduler-calendar-preview");
    await wrapper.getByTestId("scheduler-calendar-nav-next").click();

    await clickSchedulingCatalogLink(
      page,
      "Scheduler Event Timeline",
      "scheduler-timeline-preview",
    );
    wrapper = page.getByTestId("scheduler-timeline-preview");
    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    const initial = await title.innerText();
    await wrapper.getByTestId("scheduler-timeline-nav-next").click();
    await expect(title).not.toHaveText(initial);
    guard.assertClean();
  });
});
