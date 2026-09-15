import type { Locator } from "@playwright/test";
import { expect } from "@playwright/test";

/** Assert grid-template-columns resolves to the expected track count. */
export async function expectGridColumnCount(
  grid: Locator,
  count: number,
): Promise<void> {
  const tracks = await grid.evaluate((el) => {
    const value = getComputedStyle(el).gridTemplateColumns;
    return value.split(/\s+/).filter(Boolean);
  });
  expect(tracks.length).toBe(count);
}

/** Assert center element is horizontally between left and right elements. */
export async function expectHorizontallyBetween(
  left: Locator,
  center: Locator,
  right: Locator,
): Promise<void> {
  const [leftBox, centerBox, rightBox] = await Promise.all([
    left.first().boundingBox(),
    center.first().boundingBox(),
    right.first().boundingBox(),
  ]);
  expect(leftBox).not.toBeNull();
  expect(centerBox).not.toBeNull();
  expect(rightBox).not.toBeNull();

  const leftMid = leftBox!.x + leftBox!.width / 2;
  const centerMid = centerBox!.x + centerBox!.width / 2;
  const rightMid = rightBox!.x + rightBox!.width / 2;

  expect(leftMid).toBeLessThan(centerMid);
  expect(centerMid).toBeLessThan(rightMid);
}

/**
 * Assert no two elements matched by `selector` inside `container` have overlapping bounding
 * boxes. Layout-level (position/size), not a screenshot diff, so it stays stable across
 * environments/fonts. Catches overlapping axis tick labels (dense band axes) regardless of
 * whether the fix path taken is rotation or thinning — it asserts the rendered outcome, not
 * which mechanism produced it.
 */
export async function expectNoOverlappingLabels(
  container: Locator,
  selector = ".orb-axis-tick-label",
): Promise<void> {
  const labels = container.locator(selector);
  const count = await labels.count();
  const boxes: { x: number; y: number; width: number; height: number }[] = [];
  for (let i = 0; i < count; i++) {
    const box = await labels.nth(i).boundingBox();
    if (box) boxes.push(box);
  }

  const overlapping: [number, number][] = [];
  for (let i = 0; i < boxes.length; i++) {
    for (let j = i + 1; j < boxes.length; j++) {
      const a = boxes[i];
      const b = boxes[j];
      const intersects =
        a.x < b.x + b.width &&
        a.x + a.width > b.x &&
        a.y < b.y + b.height &&
        a.y + a.height > b.y;
      if (intersects) overlapping.push([i, j]);
    }
  }

  expect(
    overlapping,
    `overlapping label pairs (indices into rendered order): ${JSON.stringify(overlapping)}`,
  ).toEqual([]);
}
