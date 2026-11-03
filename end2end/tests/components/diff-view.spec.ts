import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";

test.describe("diff-view primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "diff-view");
  });

  test("DV-01 renders default preview", async ({ page }) => {
    await expect(page.getByTestId("diff-view-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("DV-02 shows all variants", async ({ page }) => {
    await expectPreviewVariants(page, ["diff-view-preview", "diff-view-identical", "diff-view-added"]);
  });

  test("DV-03 marks removed and added lines", async ({ page }) => {
    const diff = page.getByTestId("diff-view-preview");
    const removed = diff.locator('[data-diff-kind="removed"]');
    const added = diff.locator('[data-diff-kind="added"]');
    await expect(removed).toHaveCount(1);
    await expect(added).toHaveCount(2);
    await expect(removed).toContainText("allow write");
    await expect(added.nth(0)).toContainText("deny write");
    await expect(removed.locator("[aria-hidden]")).toHaveText("-");
    await expect(added.nth(0).locator("[aria-hidden]")).toHaveText("+");
  });

  test("DV-04 identical inputs show no changes", async ({ page }) => {
    const diff = page.getByTestId("diff-view-identical");
    await expect(diff).toContainText("No changes");
    await expect(diff.locator("[data-diff-kind]")).toHaveCount(0);
  });

  test("DV-05 renders in monospace", async ({ page }) => {
    const root = page.getByTestId("diff-view-preview").getByRole("figure");
    const family = await root.evaluate((el) => getComputedStyle(el).fontFamily);
    expect(family.toLowerCase()).toMatch(/mono|courier|consolas/);
  });
});
