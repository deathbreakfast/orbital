import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

const FIELDS = ["name", "email", "role"];
const VIEWPORTS = [
  { name: "phone", width: 390, height: 844 },
  { name: "tablet", width: 1024, height: 768 },
  { name: "desktop", width: 1440, height: 900 },
];
const TOLERANCE_PX = 1.5;

async function openFill(page: Page) {
  await openComponentPreview(page, "data-table-columns", "data-table-fill-preview");
  const preview = page.getByTestId("data-table-fill-preview");
  await expect(preview.getByTestId("data-table-header-role")).toBeVisible();
  return preview;
}

async function widths(preview: Locator): Promise<number[]> {
  return Promise.all(
    FIELDS.map(async (f) => {
      // A header is briefly absent while the layout remounts across the sidebar breakpoint; NaN keeps the caller polling.
      const box = await preview.getByTestId(`data-table-header-${f}`).boundingBox();
      return box ? box.width : Number.NaN;
    }),
  );
}

async function containerWidth(preview: Locator): Promise<number> {
  return preview.getByTestId("data-table-scroll").evaluate((el) => el.clientWidth);
}

async function expectFilled(preview: Locator) {
  await expect
    .poll(async () => {
      const sum = (await widths(preview)).reduce((a, b) => a + b, 0);
      return Math.abs(sum - (await containerWidth(preview)));
    })
    .toBeLessThanOrEqual(TOLERANCE_PX);
}

async function dragFirstHandle(page: Page, preview: Locator, dx: number) {
  const handle = preview.getByTestId("data-table-header-name").locator(".orbital-table-resize-handle");
  const box = await handle.boundingBox();
  expect(box).not.toBeNull();
  const x = box!.x + box!.width / 2;
  const y = box!.y + box!.height / 2;
  const init = { button: 0, pointerId: 1, pointerType: "mouse" };
  await handle.dispatchEvent("pointerdown", { clientX: x, clientY: y, ...init });
  await page.mouse.move(x + dx, y, { steps: 10 });
  await page.dispatchEvent("body", "pointerup", { clientX: x + dx, clientY: y, ...init });
}

test.describe("data-table column fill", () => {
  for (const vp of VIEWPORTS) {
    test(`widths sum to the container at ${vp.width}px`, async ({ page }) => {
      await page.setViewportSize({ width: vp.width, height: vp.height });
      const preview = await openFill(page);
      await expectFilled(preview);
    });
  }

  test("columns refill after a window resize", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    const preview = await openFill(page);
    await expectFilled(preview);
    for (const width of [1024, 390, 1440]) {
      await page.setViewportSize({ width, height: 900 });
      await expectFilled(preview);
    }
  });

  test("a dragged column keeps its width while the others refill", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    const preview = await openFill(page);
    const [before] = await widths(preview);
    await dragFirstHandle(page, preview, 80);
    await expect.poll(async () => (await widths(preview))[0]).toBeGreaterThan(before + 40);
    await expectFilled(preview);
    const [dragged] = await widths(preview);

    await page.setViewportSize({ width: 1100, height: 900 });
    await expectFilled(preview);
    expect(Math.abs((await widths(preview))[0] - dragged)).toBeLessThanOrEqual(1);
  });

  test("fills the container in dark mode with readable headers", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    const preview = await openFill(page);
    const header = preview.getByTestId("data-table-header-name");
    const paint = () =>
      header.evaluate((el) => {
        const parse = (c: string) => c.match(/[\d.]+/g)!.map(Number);
        let bg = [0, 0, 0, 0];
        for (let n: Element | null = el; n; n = n.parentElement) {
          const c = parse(getComputedStyle(n).backgroundColor);
          if ((c[3] ?? 1) > 0.99) {
            bg = c;
            break;
          }
        }
        const fg = parse(getComputedStyle(el).color);
        const lum = (c: number[]) => {
          const [r, g, b] = c.slice(0, 3).map((v) => {
            const s = v / 255;
            return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
          });
          return 0.2126 * r + 0.7152 * g + 0.0722 * b;
        };
        const [hi, lo] = [lum(fg), lum(bg)].sort((a, b) => b - a);
        return { fg: fg.join(","), bg: bg.join(","), contrast: (hi + 0.05) / (lo + 0.05) };
      });
    const light = await paint();
    await page.getByTestId("preview-theme-toggle").getByRole("switch").click();
    await expect.poll(async () => (await paint()).bg).not.toBe(light.bg);
    const dark = await paint();
    expect(dark.fg).not.toBe(light.fg);
    expect(dark.contrast).toBeGreaterThanOrEqual(4.5);
    await expectFilled(preview);
    await preview.screenshot({ path: test.info().outputPath("data-table-fill-dark.png") });
  });
});
