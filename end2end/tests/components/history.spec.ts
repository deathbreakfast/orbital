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

  test("filter advanced chips toggle kind", async ({ page }) => {
    await openComponentPreview(page, "history-filter", "history-filter-advanced-preview");
    const preview = page.getByTestId("history-filter-advanced-preview");
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible({ timeout: 30_000 });

    const kindChip = preview.getByRole("button", { name: "comment" });
    await kindChip.click();
    await expect(preview.locator("[data-history-entry-id='1']")).toBeVisible();
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

  test("live scroll policy scrolls to top on push", async ({ page }) => {
    await openComponentPreview(page, "history-live-update", "history-live-scroll-preview");
    const preview = page.getByTestId("history-live-scroll-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });

    const scroll = preview.locator(".orbital-history__scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = 200;
    });
    await preview.getByRole("button", { name: "Push live entry (auto scroll)" }).click();
    await expect
      .poll(async () => scroll.evaluate((el) => el.scrollTop))
      .toBeLessThan(10);
  });

  test("unread watermark highlights newer entries", async ({ page }) => {
    await openComponentPreview(page, "history-handle", "history-unread-preview");
    const preview = page.getByTestId("history-unread-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orbital-history__entry--unread").first()).toBeVisible();
  });

  test("mark all read clears unread divider", async ({ page }) => {
    await openComponentPreview(page, "history-handle", "history-unread-preview");
    const preview = page.getByTestId("history-unread-preview");
    await expect(preview.getByTestId("history-unread-divider")).toBeVisible({ timeout: 30_000 });

    await preview.getByRole("button", { name: "Mark all read" }).click();
    await expect(preview.getByTestId("history-unread-divider")).toHaveCount(0);
    await expect(preview.locator(".orbital-history__entry--unread")).toHaveCount(0);
  });

  test("diff highlight styles new values", async ({ page }) => {
    await openComponentPreview(page, "history-multi-diff", "history-diff-highlight-preview");
    const preview = page.getByTestId("history-diff-highlight-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orbital-history__diff-new").first()).toBeVisible();
  });

  test("state export button does not panic", async ({ page }) => {
    await openComponentPreview(page, "history-handle", "history-state-preview");
    const preview = page.getByTestId("history-state-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await preview.getByRole("button", { name: "Export" }).click();
    await expect(preview.getByTestId("history-entry-list")).toBeVisible();
  });

  test("markdown citation refs render history anchors", async ({ page }) => {
    await openComponentPreview(page, "history-markdown", "history-markdown-citations-preview");
    const preview = page.getByTestId("history-markdown-citations-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orbital-history__citation-ref").first()).toBeVisible();
  });

  test("markdown mention refs render and show persona on hover", async ({ page }) => {
    await openComponentPreview(page, "history-markdown", "history-markdown-mentions-preview");
    const preview = page.getByTestId("history-markdown-mentions-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });

    const mention = preview.locator(".orbital-history__mention-ref").first();
    await expect(mention).toBeVisible();
    await mention.hover();
    await expect(
      page.locator(".orbital-history__mention-popover-anchor .orbital-popover-body"),
    ).toBeVisible();
  });

  test("markdown image attachments render inline images", async ({ page }) => {
    await openComponentPreview(page, "history-markdown", "history-markdown-images-preview");
    const preview = page.getByTestId("history-markdown-images-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("img.orbital-markdown__image").first()).toBeVisible();
  });

  test("group collapse toggles consecutive entries", async ({ page }) => {
    await openComponentPreview(page, "history-grouping", "history-grouping-preview");
    const preview = page.getByTestId("history-grouping-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });

    const jordanHeader = preview.getByRole("button", { name: /Jordan Lee.*3 actor/i });
    await expect(jordanHeader).toBeVisible();
    await expect(jordanHeader).toHaveAttribute("aria-expanded", "false");

    const groupedEntries = preview.locator("[data-history-entry-id^='group-a-']");
    await expect(groupedEntries).toHaveCount(0);

    await jordanHeader.click();
    await expect(jordanHeader).toHaveAttribute("aria-expanded", "true");
    await expect(preview.locator("[data-history-entry-id='group-a-0']")).toBeVisible();
  });
});
