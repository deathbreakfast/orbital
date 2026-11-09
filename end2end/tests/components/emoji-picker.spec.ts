import { test, expect, type Page } from "@playwright/test";
import { expectPreviewVariants, openComponentPreview } from "../lib/preview/navigation";

function emojiGrid(page: Page) {
  return page.getByRole("grid", { name: "Emoji", exact: true });
}

function cell(page: Page, name: string) {
  return emojiGrid(page).getByRole("button", { name, exact: true });
}

async function openButtonPicker(page: Page) {
  await openComponentPreview(page, "emoji-picker-button");
  const trigger = page
    .getByTestId("emoji-picker-button-preview")
    .getByRole("button", { name: "Add reaction" });
  await trigger.click();
  await expect(emojiGrid(page)).toBeVisible();
  return trigger;
}

test.describe("emoji-picker", () => {
  test("EMJ-01: opening the popover focuses the search box", async ({ page }) => {
    await openButtonPicker(page);
    await expect(page.getByRole("searchbox", { name: "Search emoji" })).toBeFocused();
  });

  test("EMJ-02: search filters by name and keyword and hides Recent", async ({ page }) => {
    await openComponentPreview(page, "emoji-picker", "emoji-picker-inline");
    const inline = page.getByTestId("emoji-picker-inline");
    await expect(inline.getByRole("grid", { name: "Recent emoji" })).toBeVisible();
    await inline.getByRole("searchbox", { name: "Search emoji" }).fill("party");
    for (const name of ["tada", "partying face", "confetti ball", "pinata"]) {
      await expect(inline.getByRole("button", { name, exact: true })).toBeVisible();
    }
    await expect(inline.getByRole("button", { name: "grinning", exact: true })).toHaveCount(0);
    await expect(inline.getByRole("grid", { name: "Recent emoji" })).toHaveCount(0);
  });

  test("EMJ-03: a query with no matches says so", async ({ page }) => {
    await openComponentPreview(page, "emoji-picker", "emoji-picker-inline");
    const inline = page.getByTestId("emoji-picker-inline");
    await inline.getByRole("searchbox", { name: "Search emoji" }).fill("zzzqqq");
    await expect(inline.getByText("No emoji match")).toBeVisible();
    await expect(inline.getByRole("grid", { name: "Emoji", exact: true })).toHaveCount(0);
  });

  test("EMJ-04: arrow keys move through the grid and Enter picks", async ({ page }) => {
    const trigger = await openButtonPicker(page);
    const search = page.getByRole("searchbox", { name: "Search emoji" });
    await expect(search).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(cell(page, "grinning")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(cell(page, "smiley")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(cell(page, "upside down face")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(cell(page, "smiley")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(search).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("End");
    await expect(cell(page, "joy")).toBeFocused();
    await expect(cell(page, "joy")).toHaveAttribute("tabindex", "0");
    await expect(cell(page, "grinning")).toHaveAttribute("tabindex", "-1");

    await page.keyboard.press("Enter");
    await expect(page.getByTestId("emoji-picker-button-picked")).toContainText("Picked: 😂");
    await expect(emojiGrid(page)).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("EMJ-05: a pick moves to the front of Recent", async ({ page }) => {
    await openComponentPreview(page, "emoji-picker", "emoji-picker-inline");
    const inline = page.getByTestId("emoji-picker-inline");
    const recent = inline.getByRole("grid", { name: "Recent emoji" });
    await expect(recent.getByRole("button")).toHaveCount(3);
    await inline.getByRole("grid", { name: "Emoji", exact: true }).getByRole("button", { name: "tada", exact: true }).click();
    await expect(page.getByTestId("emoji-picker-inline-picked")).toContainText("Picked: 🎉");
    await expect(recent.getByRole("button")).toHaveCount(4);
    await expect(recent.getByRole("button").nth(0)).toHaveAccessibleName("tada");
  });

  test("EMJ-06: Escape closes the popover and returns focus to the button", async ({ page }) => {
    const trigger = await openButtonPicker(page);
    await page.getByRole("searchbox", { name: "Search emoji" }).fill("cat");
    await page.keyboard.press("Escape");
    await expect(emojiGrid(page)).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("EMJ-07: documented examples render", async ({ page }) => {
    await openComponentPreview(page, "emoji-picker", "emoji-picker-inline");
    await expectPreviewVariants(page, ["emoji-picker-inline", "emoji-picker-custom"]);
    const custom = page.getByTestId("emoji-picker-custom");
    await expect(custom.getByRole("row")).toHaveCount(1);
    await expect(custom.getByRole("button")).toHaveCount(4);
  });
});
