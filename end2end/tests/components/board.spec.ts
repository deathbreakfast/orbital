import { test, expect } from "@playwright/test";
import type { Locator, Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { openComponentPreview, expectPreviewVariants, previewScrollport } from "../lib/preview/navigation";
import { setViewportPreset } from "../lib/preview/mobile";

const SLUG = "components/patterns/board";

async function dragCard(source: Locator, target: Locator) {
  await source.dispatchEvent("dragstart", { dataTransfer: await source.page().evaluateHandle(() => new DataTransfer()) });
  await target.dispatchEvent("dragover", { dataTransfer: await target.page().evaluateHandle(() => new DataTransfer()) });
  await target.dispatchEvent("drop", { dataTransfer: await target.page().evaluateHandle(() => new DataTransfer()) });
  await source.dispatchEvent("dragend");
}

async function expectNoAxeViolations(page: Page, testId: string) {
  const results = await new AxeBuilder({ page }).include(`[data-testid='${testId}']`).analyze();
  expect(results.violations).toEqual([]);
}

test.describe("board pattern preview", () => {
  test("BD-01 dragging a card from To do to Doing fires one move", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    const board = page.getByTestId("board-default");
    const card = board.getByTestId("board-default-card-tokens");
    await expect(board.getByTestId("board-default-lane-todo").getByTestId("board-default-card-tokens")).toBeVisible();

    await dragCard(card, board.getByTestId("board-default-lane-doing"));

    await expect(board.getByTestId("board-default-lane-doing").getByTestId("board-default-card-tokens")).toBeVisible();
    await expect(board.getByTestId("board-default-lane-todo").getByTestId("board-default-card-tokens")).toHaveCount(0);
    const log = page.getByTestId("board-move-log");
    await expect(log).toHaveText("Moved tokens to doing");
    await expect(board.getByTestId("board-default-lane-doing")).toHaveAttribute("aria-label", "Doing (2)");
  });

  test("BD-02 dropping a card on its own lane is a no-op", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    const board = page.getByTestId("board-default");
    await dragCard(board.getByTestId("board-default-card-spec"), board.getByTestId("board-default-lane-todo"));
    await expect(board.getByTestId("board-default-lane-todo").getByTestId("board-default-card-spec")).toBeVisible();
    await expect(page.getByTestId("board-move-log")).toHaveText("");
  });

  test("BD-03 keyboard move: Space, ArrowRight, Space", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    const board = page.getByTestId("board-default");
    const live = board.getByTestId("board-default-live");

    // Start focus at the To do heading so Tab lands on the lane's first card.
    await board.getByTestId("board-default-lane-todo").getByText("To do (2)").click();
    await page.keyboard.press("Tab");
    const card = board.getByTestId("board-default-card-spec");
    await expect(card).toBeFocused();

    await page.keyboard.press("Space");
    await expect(live).toHaveText("Picked up Write the board spec");
    await page.keyboard.press("ArrowRight");
    await expect(live).toHaveText("Over Doing");
    await page.keyboard.press("Space");
    await expect(live).toHaveText("Moved Write the board spec to Doing");

    await expect(board.getByTestId("board-default-lane-doing").getByTestId("board-default-card-spec")).toBeVisible();
    await expect(page.getByTestId("board-move-log")).toHaveText("Moved spec to doing");
    await expect(board.getByTestId("board-default-card-spec")).toBeFocused();
  });

  test("BD-04 Escape cancels a keyboard move", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    const board = page.getByTestId("board-default");
    await board.getByTestId("board-default-lane-todo").getByText("To do (2)").click();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Space");
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("Escape");
    await expect(board.getByTestId("board-default-live")).toHaveText("Move cancelled");
    await expect(board.getByTestId("board-default-lane-todo").getByTestId("board-default-card-spec")).toBeVisible();
    await expect(page.getByTestId("board-move-log")).toHaveText("");
  });

  test("BD-05 read-only board has no draggable cards", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    await expectPreviewVariants(page, ["board-readonly"]);
    const board = page.getByTestId("board-readonly");
    await expect(board.getByTestId("board-readonly-card-notes")).toBeVisible();
    await expect(board.locator("[draggable='true']")).toHaveCount(0);
    await expect(board.locator("[tabindex='0']")).toHaveCount(0);
  });

  test("BD-06 empty lane and custom count", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    await expectPreviewVariants(page, ["board-empty"]);
    const board = page.getByTestId("board-empty");
    await expect(board.getByTestId("board-empty-lane-new")).toHaveAttribute("aria-label", "New (1 of 40)");
    await expect(board.getByTestId("board-empty-lane-triaged")).toContainText("Nothing here yet");
  });

  test("BD-07 lanes scroll sideways at 390px without page overflow", async ({ page }) => {
    await setViewportPreset(page, "phone");
    await openComponentPreview(page, SLUG, "board-preview");
    const lanes = page.getByTestId("board-default-lanes");
    const overflow = await lanes.evaluate((el) => ({
      scrollWidth: el.scrollWidth,
      clientWidth: el.clientWidth,
      overflowX: getComputedStyle(el).overflowX,
      snap: getComputedStyle(el).scrollSnapType,
    }));
    expect(overflow.scrollWidth).toBeGreaterThan(overflow.clientWidth);
    expect(overflow.overflowX).toBe("auto");
    expect(overflow.snap).toContain("x");

    const pageOverflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
    expect(pageOverflow).toBeLessThanOrEqual(0);
    const port = await previewScrollport(page).evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(port).toBeLessThanOrEqual(0);
  });

  test("BD-08 axe clean in dark and light", async ({ page }) => {
    await openComponentPreview(page, SLUG, "board-preview");
    // The catalog starts in dark mode.
    await expectNoAxeViolations(page, "board-preview");

    const toggle = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    const wasDark = await toggle.isChecked();
    await toggle.click({ force: true });
    await expect(toggle).toBeChecked({ checked: !wasDark });
    await expectNoAxeViolations(page, "board-preview");
  });
});
