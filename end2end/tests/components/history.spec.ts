import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("history preview", () => {
  test("renders default timeline preview", async ({ page }) => {
    await openComponentPreview(page, "history-timeline");
    await expect(page.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("history-entry-list")).toBeVisible();
    await expect(page.locator("[data-history-entry-id='1']")).toBeVisible();
  });

  test("filter chrome narrows visible entries", async ({ page }) => {
    await openComponentPreview(page, "history-filter", "history-filter-chrome-preview");
    const preview = page.getByTestId("history-filter-chrome-preview");
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible({ timeout: 30_000 });

    const input = preview.locator(".orbital-history__filter-chrome input");
    await input.fill("jordan");
    await expect(preview.locator("[data-history-entry-id='1']")).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='2']")).toHaveCount(0);
  });

  test("sort chrome toggles entry order", async ({ page }) => {
    await openComponentPreview(page, "history-sort", "history-sort-chrome-preview");
    const preview = page.getByTestId("history-sort-chrome-preview");
    await expect(preview.getByTestId("history-sort-chrome")).toBeVisible({ timeout: 30_000 });

    const firstEntry = preview.locator("[data-history-entry-id]").first();
    await expect(firstEntry).toHaveAttribute("data-history-entry-id", "b1");

    await preview.getByRole("button", { name: "Oldest first" }).click();
    await expect(preview.locator("[data-history-entry-id]").first()).toHaveAttribute(
      "data-history-entry-id",
      "b5",
    );
  });

  test("paged server list shows pagination footer", async ({ page }) => {
    await openComponentPreview(page, "history-paged");
    const preview = page.getByTestId("history-paged-preview");
    await expect(preview.getByTestId("history-pagination")).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("history-entry-list")).toBeVisible();
  });

  test("live head preview prepends entries", async ({ page }) => {
    await openComponentPreview(page, "history-live-update", "history-live-head-preview");
    const preview = page.getByTestId("history-live-head-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });

    await preview.getByRole("button", { name: "Push live entry" }).click();
    await expect(preview.locator("[data-history-entry-id^='live-']")).toBeVisible();
  });
});
