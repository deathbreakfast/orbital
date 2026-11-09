import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("spotlight-popover preview", () => {
  test("SP-01 default spotlight popover opens on click", async ({ page }) => {
    await openComponentPreview(page, "spotlight-popover");
    await expect(page.getByTestId("spotlight-popover-preview")).toBeVisible({ timeout: 30_000 });
    await page.getByTestId("spotlight-popover-preview").getByRole("button", { name: "Show tip" }).click();
    await expect(page.getByTestId("spotlight-header")).toContainText("Welcome");
    await expect(page.getByTestId("spotlight-body")).toBeVisible();
    await expect(page.getByTestId("spotlight-actions")).toBeVisible();
  });

  test("SP-02 media and footer variant", async ({ page }) => {
    await openComponentPreview(page, "spotlight-popover");
    await expectPreviewVariants(page, ["spotlight-popover-media"]);
    await page.getByTestId("spotlight-popover-media").getByRole("button", { name: "Tour" }).click();
    await expect(page.getByTestId("spotlight-media")).toBeVisible();
    await expect(page.getByTestId("spotlight-footer")).toContainText("1 of 3");
  });
});

test.describe("spotlight-tip preview", () => {
  test("SP-03 controlled anchor opens on callback", async ({ page }) => {
    await openComponentPreview(page, "spotlight-tip", "spotlight-tip-controlled");
    await expectPreviewVariants(page, ["spotlight-tip-controlled"]);
    const section = page.getByTestId("spotlight-tip-controlled");
    await section.getByRole("button", { name: "Start" }).click();
    await expect(page.getByTestId("spotlight-header")).toContainText("Filters");
    await expect(page.getByTestId("spotlight-target-1")).toBeVisible();
  });

  test("SP-05 arrow and spotlight backdrop visible", async ({ page }) => {
    await openComponentPreview(page, "spotlight-tip", "spotlight-tip-controlled");
    await expectPreviewVariants(page, ["spotlight-tip-spotlight"]);
    const section = page.getByTestId("spotlight-tip-spotlight");
    await section.getByRole("button", { name: "Highlight feature" }).click();
    await expect(page.locator(".orbital-popover-surface__angle")).toBeVisible();
    await expect(page.locator(".orbital-backdrop-spotlight, .orbital-backdrop--spotlight-panel").first()).toBeVisible();
  });
});

test.describe("spotlight-tour preview", () => {
  test("SP-04 multi-step tour advances", async ({ page }) => {
    await openComponentPreview(page, "spotlight-tour", "spotlight-tour");
    await expectPreviewVariants(page, ["spotlight-tour"]);
    const section = page.getByTestId("spotlight-tour");
    await section.getByRole("button", { name: "Start tour" }).click();
    const visibleHeader = page.locator('[data-testid="spotlight-header"]:visible');
    const visibleFooter = page.locator('[data-testid="spotlight-footer"]:visible');
    await expect(visibleHeader).toContainText("Step 1");
    await expect(visibleFooter.getByTestId("spotlight-footer-count")).toContainText("1 of 2");
    // Per-step footers stay mounted; inactive steps are display:none — scope to :visible.
    await visibleFooter.getByTestId("spotlight-tour-next").click();
    await expect(visibleHeader).toContainText("Step 2");
    await expect(visibleFooter.getByTestId("spotlight-footer-count")).toContainText("2 of 2");
  });

  test("SP-06 spotlight cutout moves on next step", async ({ page }) => {
    await openComponentPreview(page, "spotlight-tour", "spotlight-tour");
    const section = page.getByTestId("spotlight-tour");
    await section.getByRole("button", { name: "Start tour" }).click();
    await expect(page.getByTestId("spotlight-tour-target-1")).toBeVisible();
    await page.locator('[data-testid="spotlight-tour-next"]:visible').click();
    await expect(page.locator('[data-testid="spotlight-header"]:visible')).toContainText("Step 2");
    await expect(page.getByTestId("spotlight-tour-target-2")).toBeVisible();
  });

  test("SP-07 tour back navigates to previous step", async ({ page }) => {
    await openComponentPreview(page, "spotlight-tour", "spotlight-tour");
    const section = page.getByTestId("spotlight-tour");
    await section.getByRole("button", { name: "Start tour" }).click();
    await page.locator('[data-testid="spotlight-tour-next"]:visible').click();
    await expect(page.locator('[data-testid="spotlight-header"]:visible')).toContainText("Step 2");
    await page.locator('[data-testid="spotlight-tour-back"]:visible').click();
    await expect(page.locator('[data-testid="spotlight-header"]:visible')).toContainText("Step 1");
  });
});

test.describe("element-picker preview", () => {
  test("EP-01 hover outlines, click picks, page handler does not fire", async ({ page }) => {
    await openComponentPreview(page, "element-picker", "element-picker-default");
    const section = page.getByTestId("element-picker-default");
    await section.getByRole("button", { name: "Pick element" }).click();
    await page.getByTestId("picker-card-a").hover();
    await expect(page.getByTestId("element-picker-chip")).toHaveText("picker-card-a");
    await expect(page.getByTestId("element-picker-outline")).toBeVisible();
    await page.getByTestId("picker-card-b").getByRole("button", { name: "Vote" }).hover();
    await expect(page.getByTestId("element-picker-chip")).toHaveText("picker-card-b");
    await page.getByTestId("picker-card-b").getByRole("button", { name: "Vote" }).click();
    await expect(page.getByTestId("element-picker-picked")).toHaveText("picked picker-card-b");
    await expect(page.getByTestId("element-picker")).toHaveCount(0);
  });

  test("EP-02 element without an anchor is dimmed and not picked", async ({ page }) => {
    await openComponentPreview(page, "element-picker", "element-picker-default");
    const section = page.getByTestId("element-picker-default");
    await section.getByRole("button", { name: "Pick element" }).click();
    await page.getByTestId("picker-no-anchor").hover();
    await expect(page.getByTestId("element-picker-chip")).toHaveText("Can't pick this element");
    await expect(page.getByTestId("element-picker-outline")).toHaveClass(/--unavailable/);
    await page.getByTestId("picker-no-anchor").click();
    // The root has no box of its own (its children are fixed), so check the Cancel button.
    await expect(page.getByTestId("element-picker").getByRole("button", { name: "Cancel" })).toBeVisible();
    await expect(page.getByTestId("element-picker-picked")).toHaveText("");
  });

  test("EP-03 Esc and Cancel leave picker mode", async ({ page }) => {
    await openComponentPreview(page, "element-picker", "element-picker-default");
    const section = page.getByTestId("element-picker-default");
    await section.getByRole("button", { name: "Pick element" }).click();
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("element-picker")).toHaveCount(0);
    await expect(page.getByTestId("element-picker-picked")).toHaveText("cancelled");
    await section.getByRole("button", { name: "Pick element" }).click();
    await page.getByTestId("element-picker").getByRole("button", { name: "Cancel" }).click();
    await expect(page.getByTestId("element-picker")).toHaveCount(0);
  });
});
