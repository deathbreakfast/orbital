import { test, expect, type Locator, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  openComponentPreview,
  expectPreviewVariants,
  scrollIntoPreviewView,
} from "../lib/preview/navigation";

const SLUG = "scheduler-timeline-markers";
const EXAMPLES = [
  "scheduler-timeline-markers-today",
  "scheduler-timeline-markers-tone",
  "scheduler-timeline-markers-collapse",
  "scheduler-timeline-markers-zoom",
];

async function openMarkers(page: Page, example: string): Promise<Locator> {
  await openComponentPreview(page, SLUG, "scheduler-timeline-markers-today");
  const wrapper = page.getByTestId(example).last();
  await scrollIntoPreviewView(wrapper);
  await expect(wrapper).toBeVisible({ timeout: 10_000 });
  return wrapper;
}

/** Resolve a CSS colour expression to its computed value inside `scope`. */
async function resolvedColor(scope: Locator, expression: string): Promise<string> {
  return scope.evaluate((root, expr) => {
    const probe = document.createElement("span");
    probe.style.background = expr;
    root.appendChild(probe);
    const value = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return value;
  }, expression);
}

test.describe("scheduler-timeline-markers preview", () => {
  test("shows every documented example", async ({ page }) => {
    await openComponentPreview(page, SLUG, "scheduler-timeline-markers-today");
    await expectPreviewVariants(page, EXAMPLES);
  });

  test("TM-01: today line sits inside the visible range", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-today");
    const line = wrapper.getByTestId("scheduler-timeline-today");
    await expect(line).toHaveCount(1);
    await expect(line).toHaveAttribute("aria-hidden", "true");
    await expect(line).toContainText("Today");

    const left = await line.evaluate((el) => parseFloat((el as HTMLElement).style.left));
    expect(left).toBeGreaterThanOrEqual(0);
    expect(left).toBeLessThanOrEqual(100);

    const lineBox = await line.boundingBox();
    const lanes = await wrapper.locator(".orb-scheduler-timeline__time-scroll").boundingBox();
    expect(lineBox).not.toBeNull();
    expect(lanes).not.toBeNull();
    expect(lineBox!.x).toBeGreaterThanOrEqual(lanes!.x - 1);
    expect(lineBox!.x).toBeLessThanOrEqual(lanes!.x + lanes!.width);
  });

  test("TM-02: a point event draws a diamond with its title as its name", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-today");
    const diamond = wrapper.getByRole("img", { name: "Contract due" });
    await expect(diamond).toBeVisible();
    await expect(diamond).toHaveAttribute("data-testid", "scheduler-event-diamond-pt-contract");

    const transform = await diamond.evaluate((el) => getComputedStyle(el).transform);
    expect(transform).not.toBe("none");
    const box = await diamond.boundingBox();
    expect(box!.width).toBeLessThan(24);

    const chip = wrapper.getByTestId("scheduler-event-pt-contract");
    await expect(chip).toHaveClass(/orb-scheduler-event--point/);
    await expect(chip).not.toHaveClass(/orb-scheduler-event--draggable/);
    await expect(chip).toContainText("Contract due");
  });

  test("TM-03: a danger-toned event uses the danger background token", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-tone");
    const chip = wrapper.getByTestId("scheduler-event-tone-late");
    await expect(chip).toHaveAttribute("data-tone", "danger");
    const expected = await resolvedColor(wrapper, "var(--orb-color-status-danger-bg)");
    const actual = await chip.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(actual).toBe(expected);

    // An explicit colour wins over the tone.
    const custom = wrapper.getByTestId("scheduler-event-tone-legacy");
    const customBg = await custom.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(customBg).not.toBe(expected);
  });

  test("TM-04: progress fills part of the bar", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-tone");
    const chip = wrapper.getByTestId("scheduler-event-tone-late");
    const fill = wrapper.getByTestId("scheduler-event-progress-tone-late");
    await expect(fill).toHaveAttribute("data-progress", "30");
    await expect(chip).toContainText("30% done");

    const chipBox = await chip.boundingBox();
    const fillBox = await fill.boundingBox();
    const ratio = fillBox!.width / chipBox!.width;
    expect(ratio).toBeGreaterThan(0.2);
    expect(ratio).toBeLessThan(0.4);
    const opacity = await fill.evaluate((el) => getComputedStyle(el).opacity);
    expect(Number(opacity)).toBeCloseTo(0.55, 2);
  });

  test("TM-05: collapsing a parent row hides its children and their events", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-collapse");
    await expect(wrapper.getByTestId("scheduler-resource-task-wireframes")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-event-task-wireframes-bar")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-event-ms-design-bar")).toBeVisible();

    await wrapper.getByTestId("scheduler-timeline-toggle-ms-design").click();

    await expect(wrapper.getByTestId("scheduler-resource-task-wireframes")).toHaveCount(0);
    await expect(wrapper.getByTestId("scheduler-resource-task-signoff")).toHaveCount(0);
    await expect(wrapper.getByTestId("scheduler-event-task-wireframes-bar")).toHaveCount(0);
    await expect(wrapper.getByTestId("scheduler-timeline-lane-task-wireframes")).toHaveCount(0);
    await expect(wrapper.getByTestId("scheduler-event-ms-design-bar")).toBeVisible();
    // The other milestone is untouched.
    await expect(wrapper.getByTestId("scheduler-resource-task-api")).toBeVisible();

    await wrapper.getByTestId("scheduler-timeline-toggle-ms-design").click();
    await expect(wrapper.getByTestId("scheduler-resource-task-wireframes")).toBeVisible();
  });

  test("TM-06: the collapse toggle is a button with aria-expanded", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-collapse");
    const toggle = wrapper.getByRole("button", { name: "Collapse Design" });
    await expect(toggle).toHaveAttribute("data-testid", "scheduler-timeline-toggle-ms-design");
    await expect(toggle).toHaveAttribute("aria-expanded", "true");
    const controls = await toggle.getAttribute("aria-controls");
    expect(controls).toBeTruthy();
    for (const id of controls!.split(" ")) {
      await expect(page.locator(`[id="${id}"]`)).toHaveCount(1);
    }

    await toggle.focus();
    await page.keyboard.press("Enter");
    const expand = wrapper.getByRole("button", { name: "Expand Design" });
    await expect(expand).toHaveAttribute("aria-expanded", "false");
    // Leaf rows have no toggle.
    await expect(wrapper.getByTestId("scheduler-timeline-toggle-task-api")).toHaveCount(0);
  });

  test("TM-07: dark theme changes the tone colours", async ({ page }) => {
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-tone");
    const chip = wrapper.getByTestId("scheduler-event-tone-late");
    const before = await chip.evaluate((el) => getComputedStyle(el).backgroundColor);

    const toggle = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    await toggle.click({ force: true });

    await expect
      .poll(() => chip.evaluate((el) => getComputedStyle(el).backgroundColor))
      .not.toBe(before);
    const expected = await resolvedColor(wrapper, "var(--orb-color-status-danger-bg)");
    expect(await chip.evaluate((el) => getComputedStyle(el).backgroundColor)).toBe(expected);
  });

  test("TM-08: phone width keeps the label column and scrolls the lanes", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const wrapper = await openMarkers(page, "scheduler-timeline-markers-zoom");
    const rail = wrapper.locator(".orb-scheduler-timeline__resource-rail");
    const lanes = wrapper.locator(".orb-scheduler-timeline__time-scroll");
    await expect(wrapper.getByTestId("scheduler-resource-ms-design")).toBeVisible();

    const railBox = await rail.boundingBox();
    expect(railBox!.width).toBeGreaterThan(40);

    const overflow = await lanes.evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(overflow).toBeGreaterThan(0);
    await lanes.evaluate((el) => {
      el.scrollLeft = el.scrollWidth;
    });
    await expect.poll(() => lanes.evaluate((el) => el.scrollLeft)).toBeGreaterThan(0);
    await expect(wrapper.getByTestId("scheduler-resource-ms-design")).toBeVisible();
    const railAfter = await rail.boundingBox();
    expect(Math.abs(railAfter!.x - railBox!.x)).toBeLessThan(1);
  });

  test("TM-09: axe finds no violations on the markers preview", async ({ page }) => {
    await openComponentPreview(page, SLUG, "scheduler-timeline-markers-today");
    await expectPreviewVariants(page, EXAMPLES);
    const found: string[] = [];
    for (const testId of EXAMPLES) {
      const results = await new AxeBuilder({ page })
        .include(`[data-testid='${testId}']`)
        .analyze();
      for (const v of results.violations) {
        found.push(`${testId}: ${v.id} (${v.impact}) ${v.nodes.map((n) => n.html.slice(0, 120)).join(" | ")}`);
      }
    }
    expect(found).toEqual([]);
  });
});
