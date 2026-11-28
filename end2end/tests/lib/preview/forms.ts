import type { Locator } from "@playwright/test";
import { expect } from "@playwright/test";

/** Select an option on an Orbital Select wrapper (`span.orbital-select` + native `<select>`). */
export async function selectPreviewOption(wrapper: Locator, value: string) {
  const select = wrapper.locator("select");
  await expect(async () => {
    await select.selectOption(value);
    await expect(select).toHaveValue(value);
  }).toPass({ timeout: 5_000 });
}

/** Fill an Orbital Input wrapper (`span.orbital-input` + native `<input>`). */
export async function fillPreviewInput(wrapper: Locator, value: string) {
  await wrapper.locator("input").fill(value);
}

/** Blur an Orbital Input wrapper. */
export async function blurPreviewInput(wrapper: Locator) {
  await wrapper.locator("input").blur();
}

/**
 * Fill one segment of a segmented date/time field and wait out its auto-advance.
 *
 * A complete segment focuses the next one immediately and again from a 16ms timer.
 * Under load that timer can fire after the next fill has started and pull focus back,
 * so the next segment's text lands here. A later timer with a longer delay only runs
 * after it, so awaiting one means the refocus has already happened.
 */
export async function fillSegment(segment: Locator, text: string) {
  await segment.fill(text);
  await expect(segment).toHaveValue(text);
  await segment.evaluate(() => new Promise((resolve) => setTimeout(resolve, 32)));
}
