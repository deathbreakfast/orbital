import { test, expect, type Locator, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { openComponentPreview, scrollIntoPreviewView } from "../lib/preview/navigation";
import { WasmPanicGuard } from "../lib/preview/wasm-panics";

let panicGuard: WasmPanicGuard;

test.beforeEach(({ page }) => {
  panicGuard = new WasmPanicGuard(page);
});

test.afterEach(() => {
  panicGuard.assertClean();
});

const PROJECT_NODES = 11;

/** Scroll a graph example into view and wait until it has hydrated (markers exist only after mount). */
async function graphIn(page: Page, testId: string): Promise<Locator> {
  const scope = page.getByTestId(testId);
  await scrollIntoPreviewView(scope);
  await expect(scope.locator("svg.orbital-graph__svg")).toBeVisible();
  await expect(scope.locator("marker")).toHaveCount(4);
  return scope;
}

function node(scope: Locator, id: string): Locator {
  return scope.locator(`[data-node-id="${id}"]`);
}

async function viewportScale(scope: Locator): Promise<number> {
  return Number(await scope.locator(".orbital-graph__viewport").getAttribute("data-scale"));
}

async function viewportPan(scope: Locator): Promise<{ x: number; y: number }> {
  const style = (await scope.locator(".orbital-graph__viewport").getAttribute("style")) ?? "";
  const match = /translate\((-?[\d.]+)px, (-?[\d.]+)px\)/.exec(style);
  expect(match, style).not.toBeNull();
  return { x: Number(match![1]), y: Number(match![2]) };
}

/** Container point near the svg's top-left corner, inside the fit padding, so it hits no node. */
async function emptyPoint(scope: Locator): Promise<{ x: number; y: number }> {
  const box = await scope.locator("svg.orbital-graph__svg").boundingBox();
  expect(box).not.toBeNull();
  return { x: box!.x + 6, y: box!.y + 6 };
}

test.describe("graph", () => {
  test("GR-01: project graph renders nodes, group box, stubs and lines", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-project");

    const svg = scope.locator("svg.orbital-graph__svg");
    await expect(svg).toHaveAttribute("role", "group");
    await expect(svg).toHaveAttribute("aria-label", "Billing revamp task graph");
    await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES);
    await expect(scope.locator('[data-group-id="billing"]')).toContainText("Billing revamp");
    await expect(scope.locator(".orbital-graph__stub")).toHaveCount(2);
    await expect(scope.locator(".orbital-graph__edge--stub")).toHaveCount(2);
    await expect(scope.locator('[data-broken-cycle="true"]')).toHaveCount(1);
    expect(await scope.locator("path[data-edge-from]").count()).toBeGreaterThanOrEqual(13);
    // Arrow markers attach after hydration with a per-instance id.
    await expect(scope.locator('path[data-edge-from="schema"][data-edge-to="vault"]')).toHaveAttribute(
      "marker-end",
      /^url\(#orbital-graph-\d+-arrow\)$/,
    );
    await expect(node(scope, "vault")).toHaveAttribute(
      "aria-label",
      "Vault cutover, Vault cutover · Lena · Blocked",
    );
  });

  test("GR-02: clicking a node selects it and dims the rest", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-selection");

    await node(scope, "invoice-api").click();
    await expect(node(scope, "invoice-api")).toHaveAttribute("aria-pressed", "true");
    await expect(node(scope, "invoice-api")).toHaveClass(/orbital-graph__node--selected/);
    await expect(scope.getByTestId("graph-selection-selected")).toHaveText("invoice-api");

    // Direct blocker and dependants stay bright; everything else dims.
    for (const id of ["invoice-api", "schema", "retry", "billing-ui"]) {
      await expect(node(scope, id)).not.toHaveClass(/orbital-graph--dimmed/);
    }
    for (const id of ["launch", "vault", "auth-tokens"]) {
      await expect(node(scope, id)).toHaveClass(/orbital-graph--dimmed/);
    }
    await expect(node(scope, "launch")).toHaveCSS("opacity", "0.35");
    await expect(
      scope.locator('path[data-edge-from="load-test"][data-edge-to="launch"]'),
    ).toHaveClass(/orbital-graph--dimmed/);
    await expect(scope.locator('[data-group-id="billing"]')).not.toHaveClass(/dimmed/);
  });

  test("GR-03: chain mode keeps every predecessor and successor", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-selection");

    await scope.getByTestId("graph-selection-chain").locator("input").click();
    await node(scope, "invoice-api").click();
    for (const id of ["schema", "retry", "receipts", "billing-ui", "load-test", "docs", "launch"]) {
      await expect(node(scope, id)).not.toHaveClass(/orbital-graph--dimmed/);
    }
    // `vault` blocks `billing-ui` but is not upstream of `invoice-api`.
    await expect(node(scope, "vault")).toHaveClass(/orbital-graph--dimmed/);
    await expect(node(scope, "auth-tokens")).toHaveClass(/orbital-graph--dimmed/);
  });

  test("GR-04: background click clears the selection", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-selection");

    await node(scope, "retry").click();
    await expect(scope.getByTestId("graph-selection-selected")).toHaveText("retry");
    const point = await emptyPoint(scope);
    await page.mouse.click(point.x, point.y);
    await expect(scope.getByTestId("graph-selection-selected")).toHaveText("none");
    await expect(scope.locator(".orbital-graph--dimmed")).toHaveCount(0);
    await expect(node(scope, "retry")).toHaveAttribute("aria-pressed", "false");
  });

  test("GR-05: arrow keys move focus between columns and Enter selects", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-selection");

    // One tab stop for the whole graph.
    await expect(scope.locator('[data-node-id][tabindex="0"]')).toHaveCount(1);

    await node(scope, "schema").focus();
    await page.keyboard.press("ArrowRight");
    const focusedId = () => page.evaluate(() => document.activeElement?.getAttribute("data-node-id"));
    await expect.poll(focusedId).toMatch(/^(vault|invoice-api)$/);
    const next = (await focusedId())!;
    await expect(node(scope, next)).toHaveAttribute("tabindex", "0");
    await expect(node(scope, "schema")).toHaveAttribute("tabindex", "-1");

    await page.keyboard.press("ArrowLeft");
    await expect.poll(focusedId).toBe("schema");

    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("Enter");
    await expect(scope.getByTestId("graph-selection-selected")).toHaveText(next);
    await expect(node(scope, next)).toHaveAttribute("aria-pressed", "true");

    await page.keyboard.press("Escape");
    await expect(scope.getByTestId("graph-selection-selected")).toHaveText("none");
  });

  test("GR-06: wheel zoom stays within 0.2 to 2.2 and Fit resets", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-fit");
    await expect.poll(() => viewportScale(scope)).toBeLessThan(1);
    const fitted = await viewportScale(scope);

    const box = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    for (let i = 0; i < 20; i++) {
      await page.mouse.wheel(0, -120);
    }
    await expect.poll(() => viewportScale(scope)).toBe(2.2);
    for (let i = 0; i < 30; i++) {
      await page.mouse.wheel(0, 120);
    }
    await expect.poll(() => viewportScale(scope)).toBe(0.2);

    await scope.getByTestId("graph-fit-button").locator("button").click();
    await expect.poll(() => viewportScale(scope)).toBe(fitted);

    await scope.getByTestId("graph-fit-zoom-in").locator("button").click();
    await expect.poll(() => viewportScale(scope)).toBe(2);
  });

  test("GR-07: drag on the background pans", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-fit");
    const before = await viewportPan(scope);
    const scaleBefore = await viewportScale(scope);

    const start = await emptyPoint(scope);
    await page.mouse.move(start.x, start.y);
    await page.mouse.down();
    await page.mouse.move(start.x + 60, start.y + 20, { steps: 4 });
    await page.mouse.move(start.x + 120, start.y + 60, { steps: 4 });
    await page.mouse.up();

    const after = await viewportPan(scope);
    expect(after.x - before.x).toBeCloseTo(120, 0);
    expect(after.y - before.y).toBeCloseTo(60, 0);
    expect(await viewportScale(scope)).toBe(scaleBefore);
  });

  test("GR-13: middle-drag over a node pans and doesn't select", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-fit");
    const before = await viewportPan(scope);

    const box = await scope.locator("[data-node-id]").first().boundingBox();
    if (!box) throw new Error("node has no box");
    const x = box.x + box.width / 2;
    const y = box.y + box.height / 2;
    await page.mouse.move(x, y);
    await page.mouse.down({ button: "middle" });
    await page.mouse.move(x + 60, y, { steps: 4 });
    await page.mouse.move(x + 120, y, { steps: 4 });
    await page.mouse.up({ button: "middle" });

    const after = await viewportPan(scope);
    expect(after.x - before.x).toBeCloseTo(120, 0);
    await expect(scope.locator('[data-node-id][aria-pressed="true"]')).toHaveCount(0);
  });

  test("GR-14: middle-drag on the background pans", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-fit");
    const before = await viewportPan(scope);

    const start = await emptyPoint(scope);
    await page.mouse.move(start.x, start.y);
    await page.mouse.down({ button: "middle" });
    await page.mouse.move(start.x + 60, start.y + 20, { steps: 4 });
    await page.mouse.move(start.x + 120, start.y + 60, { steps: 4 });
    await page.mouse.up({ button: "middle" });

    const after = await viewportPan(scope);
    expect(after.x - before.x).toBeCloseTo(120, 0);
    expect(after.y - before.y).toBeCloseTo(60, 0);
  });

  test("GR-08: legend chip hides its set and shows the hidden count", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-legend-toggle");
    const chip = scope.getByTestId("graph-legend-toggle-legend-chip-todo");
    const button = chip.locator("button");

    await expect(button).toHaveAttribute("aria-pressed", "true");
    await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES);

    await button.click();
    await expect(button).toHaveAttribute("aria-pressed", "false");
    await expect(chip).toContainText("· 5 hidden");
    await expect(chip.locator(".orbital-graph-legend__label")).toHaveCSS(
      "text-decoration-line",
      "line-through",
    );
    await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES - 5);
    await expect(node(scope, "launch")).toHaveCount(0);
    await expect(scope.locator('path[data-edge-to="launch"]')).toHaveCount(0);

    await button.click();
    await expect(button).toHaveAttribute("aria-pressed", "true");
    await expect(chip).not.toContainText("hidden");
    await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES);
  });

  test("GR-09: dark theme changes node and edge colours", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-theme");
    const rect = node(scope, "neutral").locator("rect:not(.orbital-graph__focus)");
    const edge = scope.locator("path[data-edge-from]").first();
    const colours = async () => ({
      fill: await rect.evaluate((el) => getComputedStyle(el).fill),
      stroke: await edge.evaluate((el) => getComputedStyle(el).stroke),
    });

    const before = await colours();
    await page.getByTestId("preview-theme-toggle").locator("input").click();
    await expect.poll(async () => (await colours()).fill).not.toBe(before.fill);
    const after = await colours();
    expect(after.stroke).not.toBe(before.stroke);
  });

  test("GR-10: phone width fits the graph inside the preview", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-project");
    await expect.poll(() => viewportScale(scope)).toBeLessThan(0.5);

    const svgBox = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
    const scopeBox = (await scope.boundingBox())!;
    expect(svgBox.width).toBeLessThanOrEqual(scopeBox.width + 1);
    expect(svgBox.width).toBeLessThanOrEqual(390);

    // Wait out the 150ms fit transition, then every node sits inside the svg.
    await page.waitForTimeout(300);
    for (const box of await scope.locator("[data-node-id]").evaluateAll((els) =>
      els.map((el) => el.getBoundingClientRect().toJSON()),
    )) {
      expect(box.left).toBeGreaterThanOrEqual(svgBox.x - 1);
      expect(box.right).toBeLessThanOrEqual(svgBox.x + svgBox.width + 1);
      expect(box.top).toBeGreaterThanOrEqual(svgBox.y - 1);
      expect(box.bottom).toBeLessThanOrEqual(svgBox.y + svgBox.height + 1);
    }
  });

  test("GR-11: reduced motion removes the viewport transition", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    let scope = await graphIn(page, "graph-project");
    const viewport = () => scope.locator(".orbital-graph__viewport");
    await expect(viewport()).toHaveCSS("transition-duration", "0.15s");

    await page.emulateMedia({ reducedMotion: "reduce" });
    scope = await graphIn(page, "graph-project");
    await expect(viewport()).toHaveCSS("transition-duration", "0s");
    await expect(node(scope, "schema")).toHaveCSS("transition-duration", "0s");
  });

  test("GR-12: axe finds no violations on the graph preview", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    await graphIn(page, "graph-project");

    const results = await new AxeBuilder({ page })
      .include("[data-testid='graph-project']")
      .analyze();
    expect(results.violations).toEqual([]);
  });
});

type Mode = "light" | "dark" | "phone";
const MODES: Mode[] = ["light", "dark", "phone"];

/** Open a graph preview page in light, dark or at phone width. */
async function openIn(page: Page, mode: Mode, slug: string, testId: string) {
  if (mode === "phone") {
    await page.setViewportSize({ width: 390, height: 844 });
  }
  await openComponentPreview(page, slug, testId);
  if (mode === "dark") {
    await page.getByTestId("preview-theme-toggle").locator("input").click();
  }
}

/**
 * Point near the canvas's bottom-left corner, inside the fit padding and clear of the toolbar.
 * Whole pixels: Chrome reports `MouseEvent.clientX` as an integer, so a fractional point would
 * zoom around a spot up to 1px away from where the test thinks it clicked.
 */
async function cornerPoint(scope: Locator): Promise<{ x: number; y: number }> {
  const box = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
  return { x: Math.ceil(box.x) + 6, y: Math.floor(box.y + box.height) - 6 };
}

/** Screen distance between two layout points at `scale`. */
function screenGap(a: { x: number; y: number }, b: { x: number; y: number }, scale: number) {
  return Math.hypot(a.x - b.x, a.y - b.y) * scale;
}

/** Layout point under a page point, from the root's box and the current transform. */
async function layoutAt(scope: Locator, x: number, y: number) {
  const box = (await scope.locator(".orbital-graph").first().boundingBox())!;
  const pan = await viewportPan(scope);
  const scale = await viewportScale(scope);
  return { x: (x - box.x - pan.x) / scale, y: (y - box.y - pan.y) / scale };
}

/** Let the 150ms zoom transition and any auto-fit settle. */
async function settle(page: Page) {
  await page.waitForTimeout(350);
}

async function dragBy(page: Page, from: { x: number; y: number }, dx: number, dy: number) {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move(from.x + dx / 2, from.y + dy / 2, { steps: 4 });
  await page.mouse.move(from.x + dx, from.y + dy, { steps: 4 });
  await page.mouse.up();
}

/** The element's box lies inside the graph frame (and so inside the canvas's clip). */
async function expectInsideGraph(scope: Locator, el: Locator) {
  const frame = (await scope.locator(".orbital-graph").first().boundingBox())!;
  const box = (await el.boundingBox())!;
  expect(box.x).toBeGreaterThanOrEqual(frame.x - 1);
  expect(box.y).toBeGreaterThanOrEqual(frame.y - 1);
  expect(box.x + box.width).toBeLessThanOrEqual(frame.x + frame.width + 1);
  expect(box.y + box.height).toBeLessThanOrEqual(frame.y + frame.height + 1);
}

async function clickUntilDisabled(button: Locator, max = 30) {
  for (let i = 0; i < max && (await button.isEnabled()); i++) {
    await button.click();
  }
}

for (const mode of MODES) {
  test.describe(`graph canvas (${mode})`, () => {
    test(`GR-16 (${mode}): double-click zooms in at the pointer and Ctrl+double-click zooms out`, async ({ page }) => {
      await openIn(page, mode, "graph", "graph-project");
      const scope = await graphIn(page, "graph-canvas");
      await settle(page);
      const s0 = await viewportScale(scope);
      const p = await cornerPoint(scope);
      const before = await layoutAt(scope, p.x, p.y);

      await page.mouse.dblclick(p.x, p.y);
      const zoomed = Math.min(s0 * 2, 2.2);
      await expect.poll(() => viewportScale(scope)).toBeCloseTo(zoomed, 6);
      // The layout point under the pointer stays under it (within a pixel on screen).
      const after = await layoutAt(scope, p.x, p.y);
      expect(screenGap(after, before, zoomed)).toBeLessThan(1.5);

      await page.keyboard.down("Control");
      await page.mouse.dblclick(p.x, p.y);
      await page.keyboard.up("Control");
      const back = Math.max(s0, 0.2);
      await expect.poll(() => viewportScale(scope)).toBeCloseTo(back, 6);
      expect(screenGap(await layoutAt(scope, p.x, p.y), before, back)).toBeLessThan(1.5);
    });

    test(`GR-17 (${mode}): Zoom in disables at the maximum and Zoom out at the minimum`, async ({ page }) => {
      await openIn(page, mode, "graph", "graph-project");
      const scope = await graphIn(page, "graph-canvas");
      const zoomIn = scope.getByTestId("graph-canvas-toolbar-zoom-in");
      const zoomOut = scope.getByTestId("graph-canvas-toolbar-zoom-out");
      await expect(zoomIn).toHaveAttribute("aria-label", "Zoom in");
      await expect(zoomOut).toHaveAttribute("aria-label", "Zoom out");

      await clickUntilDisabled(zoomIn);
      await expect(zoomIn).toBeDisabled();
      await expect(zoomOut).toBeEnabled();
      expect(await viewportScale(scope)).toBeCloseTo(2.2, 6);

      await clickUntilDisabled(zoomOut);
      await expect(zoomOut).toBeDisabled();
      await expect(zoomIn).toBeEnabled();
      expect(await viewportScale(scope)).toBeCloseTo(0.2, 6);
    });

    test(`GR-18 (${mode}): Fit after a pan restores the fitted view`, async ({ page }) => {
      await openIn(page, mode, "graph", "graph-project");
      const scope = await graphIn(page, "graph-canvas");
      await settle(page);
      const fitted = await viewportPan(scope);
      const fittedScale = await viewportScale(scope);

      await dragBy(page, await cornerPoint(scope), 80, -40);
      const panned = await viewportPan(scope);
      expect(panned.x - fitted.x).toBeCloseTo(80, 0);

      await scope.getByTestId("graph-canvas-toolbar-fit").click();
      await expect.poll(async () => (await viewportPan(scope)).x).toBeCloseTo(fitted.x, 1);
      expect((await viewportPan(scope)).y).toBeCloseTo(fitted.y, 1);
      expect(await viewportScale(scope)).toBeCloseTo(fittedScale, 6);
    });

    test(`GR-19 (${mode}): hiding a legend set refits until the user pans`, async ({ page }) => {
      await openIn(page, mode, "graph-toolbar", "graph-toolbar-preview");
      const scope = await graphIn(page, "graph-toolbar-legend");
      await settle(page);
      const transform = () => scope.locator(".orbital-graph__viewport").getAttribute("style");
      const before = await transform();

      const toggle = scope.getByTestId("graph-toolbar-legend-legend-toggle");
      await expect(toggle).toHaveAttribute("aria-expanded", "false");
      await toggle.click();
      await expect(toggle).toHaveAttribute("aria-expanded", "true");
      const chip = scope.getByTestId("graph-toolbar-legend-legend-chip-todo").locator("button");
      await expectInsideGraph(scope, chip);
      await chip.click();
      await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES - 5);
      await expect.poll(transform).not.toBe(before);

      // Refitted: every node sits inside the canvas.
      await settle(page);
      const svg = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
      for (const box of await scope.locator("[data-node-id]").evaluateAll((els) =>
        els.map((el) => el.getBoundingClientRect().toJSON()),
      )) {
        expect(box.left).toBeGreaterThanOrEqual(svg.x - 1);
        expect(box.right).toBeLessThanOrEqual(svg.x + svg.width + 1);
      }

      // After a pan the graph stays put when the set comes back.
      await dragBy(page, await cornerPoint(scope), 60, -30);
      const panned = await transform();
      // The wider "· 5 hidden" chip still sits inside the graph frame.
      await expectInsideGraph(scope, chip);
      await chip.click();
      await expect(scope.locator("[data-node-id]")).toHaveCount(PROJECT_NODES);
      await settle(page);
      expect(await transform()).toBe(panned);
    });

    test(`GR-20 (${mode}): a minimap click moves the view`, async ({ page }) => {
      await openIn(page, mode, "graph-minimap", "graph-minimap-preview");
      const scope = await graphIn(page, "graph-minimap-preview");
      const minimap = scope.getByTestId("graph-minimap-preview-graph-minimap");
      // The whole graph is in view, so the minimap waits.
      await settle(page);
      await expect(minimap).toHaveCount(0);

      const zoomIn = scope.getByTestId("graph-minimap-preview-toolbar-zoom-in");
      for (let i = 0; i < 3; i++) await zoomIn.click();
      await expect(minimap).toBeVisible();
      await expect(minimap).toHaveAttribute("aria-hidden", "true");
      const view = scope.getByTestId("graph-minimap-preview-graph-minimap-view");
      const viewX = Number(await view.getAttribute("x"));
      const before = await viewportPan(scope);

      const box = (await minimap.boundingBox())!;
      await page.mouse.click(box.x + box.width * 0.9, box.y + box.height / 2);
      await expect.poll(async () => (await viewportPan(scope)).x).toBeLessThan(before.x - 10);
      expect(Number(await view.getAttribute("x"))).toBeGreaterThan(viewX);
    });

    test(`GR-21 (${mode}): the dot grid draws and follows the zoom`, async ({ page }) => {
      await openIn(page, mode, "graph", "graph-project");
      const scope = await graphIn(page, "graph-project");
      await expect(scope.locator(".orbital-graph")).toHaveAttribute("data-background", "dots");
      const grid = scope.locator(".orbital-graph__grid");
      await expect(grid).toHaveCount(1);
      expect(await grid.evaluate((el) => getComputedStyle(el).backgroundImage)).toContain(
        "radial-gradient",
      );
      const token = await grid.evaluate((el) =>
        getComputedStyle(el).getPropertyValue("--orb-graph-grid").trim(),
      );
      expect(token).not.toBe("");

      const s0 = await viewportScale(scope);
      const box = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      for (let i = 0; i < 4; i++) await page.mouse.wheel(0, -120);
      // Four notches of x1.2 each, clamped to the 2.2 maximum.
      const target = Math.min(s0 * 1.2 ** 4, 2.2);
      await expect.poll(() => viewportScale(scope)).toBeCloseTo(target, 6);

      // Read scale, pan and grid style in one go so nothing moves between them.
      const read = await scope.locator(".orbital-graph").evaluate((root) => ({
        scale: Number(root.querySelector(".orbital-graph__viewport")!.getAttribute("data-scale")),
        transform: root.querySelector(".orbital-graph__viewport")!.getAttribute("style") ?? "",
        grid: root.querySelector(".orbital-graph__grid")!.getAttribute("style") ?? "",
      }));
      // The grid pitch is 18px scaled, doubled while it is under 6px.
      let pitch = 18 * read.scale;
      while (pitch < 6) pitch *= 2;
      const size = Number(/background-size: ([\d.]+)px/.exec(read.grid)![1]);
      expect(size).toBeCloseTo(pitch, 2);
      const pan = /translate\((-?[\d.]+)px, (-?[\d.]+)px\)/.exec(read.transform)!;
      const pos = /background-position: (-?[\d.]+)px (-?[\d.]+)px/.exec(read.grid)!;
      expect(Number(pos[1])).toBeCloseTo(Number(pan[1]), 2);
      expect(Number(pos[2])).toBeCloseTo(Number(pan[2]), 2);
    });

    test(`GR-22 (${mode}): a host action in the toolbar slot fires`, async ({ page }) => {
      await openIn(page, mode, "graph-toolbar", "graph-toolbar-preview");
      const scope = await graphIn(page, "graph-toolbar-preview");
      await settle(page);
      const scale = await viewportScale(scope);
      await expect(scope.getByTestId("graph-toolbar-preview-clicks")).toHaveText("0");
      await scope.getByTestId("graph-toolbar-preview-action").locator("button").click();
      await expect(scope.getByTestId("graph-toolbar-preview-clicks")).toHaveText("1");
      expect(await viewportScale(scope)).toBe(scale);
      await expect(scope.locator('[data-node-id][aria-pressed="true"]')).toHaveCount(0);
    });

    test(`GR-23 (${mode}): axe finds no violations on the toolbar, minimap and legend`, async ({ page }) => {
      await openIn(page, mode, "graph-minimap", "graph-minimap-preview");
      const scope = await graphIn(page, "graph-minimap-preview");
      const zoomIn = scope.getByTestId("graph-minimap-preview-toolbar-zoom-in");
      for (let i = 0; i < 3; i++) await zoomIn.click();
      await expect(scope.getByTestId("graph-minimap-preview-graph-minimap")).toBeVisible();
      await settle(page);
      let results = await new AxeBuilder({ page })
        .include("[data-testid='graph-minimap-preview']")
        .analyze();
      expect(results.violations).toEqual([]);

      await openComponentPreview(page, "graph-toolbar", "graph-toolbar-preview");
      const legend = await graphIn(page, "graph-toolbar-legend");
      await legend.getByTestId("graph-toolbar-legend-legend-toggle").click();
      await settle(page);
      results = await new AxeBuilder({ page })
        .include("[data-testid='graph-toolbar-preview']")
        .include("[data-testid='graph-toolbar-legend']")
        .include("[data-testid='graph-toolbar-outside']")
        .analyze();
      expect(results.violations).toEqual([]);
    });
  });
}

test.describe("graph canvas", () => {
  test("GR-24: the toolbar's Minimap button shows and hides the minimap", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-canvas");
    const toggle = scope.getByTestId("graph-canvas-toolbar-minimap").locator("button");
    const minimap = scope.getByTestId("graph-canvas-graph-minimap");
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
    await expect(minimap).toHaveCount(0);

    const zoomIn = scope.getByTestId("graph-canvas-toolbar-zoom-in");
    for (let i = 0; i < 3; i++) await zoomIn.click();
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
    await expect(minimap).toBeVisible();
    await toggle.click();
    await expect(minimap).toHaveCount(0);
  });

  test("GR-25: double-click on a node opens it when on_node_open is set", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-open");
    await settle(page);
    const scale = await viewportScale(scope);

    await node(scope, "vault").dblclick();
    await expect(scope.getByTestId("graph-open-opened")).toHaveText("vault");
    expect(await viewportScale(scope)).toBe(scale);

    await node(scope, "schema").focus();
    await page.keyboard.press("Shift+Enter");
    await expect(scope.getByTestId("graph-open-opened")).toHaveText("schema");
  });

  test("GR-26: on_viewport reports the view after it settles", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-open");
    await settle(page);
    const box = (await scope.locator("svg.orbital-graph__svg").boundingBox())!;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, -120);
    await page.mouse.wheel(0, -120);
    const scale = await viewportScale(scope);
    await expect(scope.getByTestId("graph-open-scale")).toHaveText(scale.toFixed(2));
  });

  test("GR-27: the background prop switches to a line grid, and the grid token follows the theme", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-background");
    await expect(scope.locator(".orbital-graph")).toHaveAttribute("data-background", "lines");
    const grid = scope.locator(".orbital-graph__grid");
    expect(await grid.evaluate((el) => getComputedStyle(el).backgroundImage)).toContain(
      "linear-gradient",
    );
    const token = () =>
      grid.evaluate((el) => getComputedStyle(el).getPropertyValue("--orb-graph-grid").trim());
    const light = await token();
    await page.getByTestId("preview-theme-toggle").locator("input").click();
    await expect.poll(token).not.toBe(light);
  });
});

test.describe("graph-legend", () => {
  test("GL-01: legend page shows chips, key lines and the actions slot", async ({ page }) => {
    await openComponentPreview(page, "graph-legend", "graph-legend-preview");
    const scope = page.getByTestId("graph-legend-preview");

    await expect(scope.locator(".orbital-graph-legend__item")).toHaveCount(5);
    await expect(scope.locator('[role="group"][aria-label="Graph legend"]')).toBeVisible();
    await expect(scope).toContainText("blocks");
    await expect(scope).toContainText("dashed box: a task in another project");
    await expect(scope.getByRole("button", { name: "Customise colours" })).toBeVisible();
  });

  test("GL-02: chips toggle their set in hidden_sets", async ({ page }) => {
    await openComponentPreview(page, "graph-legend", "graph-legend-preview");
    const scope = page.getByTestId("graph-legend-preview");
    const hidden = scope.getByTestId("graph-legend-preview-hidden");

    await scope.getByTestId("graph-legend-preview-legend-chip-blocked").locator("button").click();
    await expect(hidden).toHaveText("blocked");
    await scope.getByTestId("graph-legend-preview-legend-chip-done").locator("button").click();
    await expect(hidden).toHaveText("blocked, done");
    await expect(scope.getByTestId("graph-legend-preview-legend-chip-done")).toContainText(
      "· 1 hidden",
    );
    await scope.getByTestId("graph-legend-preview-legend-chip-blocked").locator("button").click();
    await expect(hidden).toHaveText("done");
  });

  test("GL-03: axe finds no violations on the legend preview", async ({ page }) => {
    await openComponentPreview(page, "graph-legend", "graph-legend-preview");
    const results = await new AxeBuilder({ page })
      .include("[data-testid='graph-legend-preview']")
      .analyze();
    expect(results.violations).toEqual([]);
  });

  test("GR-15: height prop sizes the graph without a sized container", async ({ page }) => {
    await openComponentPreview(page, "graph", "graph-project");
    const scope = await graphIn(page, "graph-project");
    const root = scope.locator(".orbital-graph");
    const box = await root.boundingBox();
    expect(Math.abs(box!.height - 460)).toBeLessThanOrEqual(1);
  });
});
