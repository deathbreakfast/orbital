import { test, expect, type Locator, type Page } from "@playwright/test";
import {
  expectPreviewVariants,
  openComponentPreview,
  scrollIntoPreviewView,
} from "../lib/preview/navigation";

type Corner = "top-start" | "top-end" | "bottom-start" | "bottom-end";

async function cornerDistance(badge: Locator, target: Locator, corner: Corner) {
  const b = await badge.boundingBox();
  const t = await target.boundingBox();
  if (!b || !t) return Number.POSITIVE_INFINITY;
  const x = corner.endsWith("end") ? t.x + t.width : t.x;
  const y = corner.startsWith("top") ? t.y : t.y + t.height;
  return Math.hypot(b.x + b.width / 2 - x, b.y + b.height / 2 - y);
}

/** The badge's centre sits on the target's corner (within 2px). */
async function expectOnCorner(badge: Locator, target: Locator, corner: Corner = "top-end") {
  await expect(badge).toBeVisible();
  await expect.poll(() => cornerDistance(badge, target, corner)).toBeLessThan(2);
}

async function scrollScroller(page: Page, top: number) {
  await page.getByTestId("anchored-counter-badge-scroller").evaluate((el, value) => {
    el.scrollTop = value;
  }, top);
}

function chartBadge(page: Page) {
  return page.getByRole("button", { name: "3 comments on Results chart" });
}

test.describe("anchored-counter-badge preview", () => {
  test("ACB-01 badges sit on element corners with a 24px tap target", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const cases = [
      { name: "3 comments on Results chart", target: "acb-results-chart", text: "3" },
      { name: "42 comments on Ballot", target: "acb-ballot", text: "42" },
      { name: "1200 comments on Comments hub", target: "acb-comments-hub", text: "999+" },
    ];
    for (const c of cases) {
      const badge = page.getByRole("button", { name: c.name });
      await expectOnCorner(badge, page.getByTestId(c.target));
      await expect(badge.locator(".orbital-badge")).toHaveText(c.text);
      const box = await badge.boundingBox();
      expect(box!.width).toBeGreaterThanOrEqual(24);
      expect(box!.height).toBeGreaterThanOrEqual(24);
    }
  });

  test("ACB-02 badge follows its element when the container scrolls", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const badge = chartBadge(page);
    const chart = page.getByTestId("acb-results-chart");
    await expectOnCorner(badge, chart);
    const before = (await badge.boundingBox())!.y;

    await scrollScroller(page, 8);
    await expectOnCorner(badge, chart);
    await expect.poll(async () => before - (await badge.boundingBox())!.y).toBeCloseTo(8, 0);
  });

  test("ACB-03 badge hides while its corner is scrolled out of the container", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const badge = chartBadge(page);
    await expect(badge).toBeVisible();

    await scrollScroller(page, 200);
    await expect(badge).toBeHidden();

    await scrollScroller(page, 0);
    await expectOnCorner(badge, page.getByTestId("acb-results-chart"));
  });

  test("ACB-04 badge follows its element when the element resizes", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const badge = chartBadge(page);
    const chart = page.getByTestId("acb-results-chart");
    await expectOnCorner(badge, chart);
    const before = (await badge.boundingBox())!.x;

    await page
      .getByTestId("anchored-counter-badge-preview")
      .getByRole("button", { name: "Resize chart" })
      .click();
    await expect.poll(async () => (await badge.boundingBox())!.x).toBeGreaterThan(before + 20);
    await expectOnCorner(badge, chart);
  });

  test("ACB-05 badge stays anchored when the window resizes", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const badge = chartBadge(page);
    const chart = page.getByTestId("acb-results-chart");
    await expectOnCorner(badge, chart);

    await page.setViewportSize({ width: 900, height: 700 });
    await scrollIntoPreviewView(page.getByTestId("anchored-counter-badge-preview"));
    await expectOnCorner(badge, chart);
  });

  test("ACB-06 clicking a badge runs on_click", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    const clicked = page.getByTestId("anchored-counter-badge-clicked");
    await expect(clicked).toHaveText("Opened: none");

    await page.getByRole("button", { name: "42 comments on Ballot" }).click();
    await expect(clicked).toHaveText("Opened: Ballot");

    await chartBadge(page).focus();
    await page.keyboard.press("Enter");
    await expect(clicked).toHaveText("Opened: Results chart");
  });

  test("ACB-07 corner prop places the badge on each corner", async ({ page }) => {
    await openComponentPreview(page, "anchored-counter-badge");
    await expectPreviewVariants(page, ["anchored-counter-badge-corners"]);
    const target = page.getByTestId("acb-corners-target");
    const corners: Array<[string, Corner]> = [
      ["Top start", "top-start"],
      ["Top end", "top-end"],
      ["Bottom start", "bottom-start"],
      ["Bottom end", "bottom-end"],
    ];
    for (const [name, corner] of corners) {
      await expectOnCorner(page.getByRole("button", { name, exact: true }), target, corner);
    }
  });
});
