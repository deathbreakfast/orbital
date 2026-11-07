import { test, expect, type Locator, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { openComponentPreview, scrollIntoPreviewView } from "../lib/preview/navigation";

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
});
