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

    // PreviewCatalogNav resets its open folders to the current route's ancestors in an effect
    // that can land after the destination page is already visible. Wait for that reset (the
    // timeline folder, open on the previous page, closes) so it can't undo the expansion below.
    const nav = page.getByTestId("preview-catalog-nav");
    await expect(nav.getByRole("button", { name: "Event Timeline", exact: true })).toHaveAttribute(
      "aria-expanded",
      "false",
    );

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x, y + 60, { steps: 8 });
    // DOM clicks avoid Playwright actionability checks while the drag gesture holds the pointer.
    const timeline = nav.getByRole("link", {
      name: "Scheduler Event Timeline",
      exact: true,
    });
    await expect(async () => {
      for (const folder of ["Scheduling", "Event Timeline"]) {
        const btn = nav.getByRole("button", { name: folder, exact: true });
        if ((await btn.getAttribute("aria-expanded", { timeout: 1_000 })) !== "true") {
          await btn.evaluate((el) => (el as HTMLButtonElement).click());
        }
      }
      await expect(timeline).toBeVisible({ timeout: 1_000 });
    }).toPass({ timeout: 15_000 });
    await timeline.evaluate((el) => (el as HTMLAnchorElement).click());
    // Release the pointer only after the hop has happened, so the drag ends on the new page.
    await expect(page).toHaveURL(/\/scheduler-timeline$/);
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
