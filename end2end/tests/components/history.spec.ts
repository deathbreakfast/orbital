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
    await expect(preview.locator("[data-history-entry-id='comment-1']")).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='1']")).toHaveCount(0);
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

    await expect(preview.locator("[data-history-entry-id='group-a-0']")).toHaveCount(0);
    await expect(preview.locator("[data-history-entry-id='group-a-1']")).toHaveCount(0);
    await expect(preview.locator("[data-history-entry-id='group-a-2']")).toHaveCount(0);

    await jordanHeader.click();
    await expect(jordanHeader).toHaveAttribute("aria-expanded", "true");
    await expect(preview.locator("[data-history-entry-id='group-a-0']")).toBeVisible();
  });

  test("empty client list shows custom empty slot sad path", async ({ page }) => {
    await openComponentPreview(page, "history-slots", "history-slots-preview");
    const preview = page.getByTestId("history-slots-preview");
    await expect(preview.getByTestId("history-custom-empty")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-history-entry-id]")).toHaveCount(0);
  });

  test("filter chrome with no matches shows empty overlay sad path", async ({ page }) => {
    await openComponentPreview(page, "history-filter", "history-filter-chrome-preview");
    const preview = page.getByTestId("history-filter-chrome-preview");
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible({ timeout: 30_000 });

    const input = preview.locator(".orbital-history__filter-chrome input");
    await input.fill("zzz-no-match-orbital-history");
    await expect(preview.locator("[data-history-entry-id]")).toHaveCount(0);
    await expect(
      preview.getByTestId("history-no-matches-default").or(preview.getByTestId("history-empty-default")),
    ).toBeVisible();
  });

  test("server fetch failure shows error overlay sad path", async ({ page }) => {
    await openComponentPreview(page, "history-slots", "history-error-preview");
    const preview = page.getByTestId("history-error-preview");
    await expect(preview.getByTestId("history-error-default")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-history-entry-id]")).toHaveCount(0);
  });

  test("kind_views paints custom row; other kinds fall through happy path", async ({ page }) => {
    await openComponentPreview(page, "history-renderers");
    const preview = page.getByTestId("history-renderers-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("history-custom-comment")).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='comment-1']")).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='1']")).toBeVisible();
    await expect(
      preview.locator("[data-history-entry-id='1']").getByTestId("history-custom-comment"),
    ).toHaveCount(0);
    await expect(preview.getByTestId("history-custom-comment")).toHaveCount(1);
  });

  test("natural and compact layouts both render entries happy path", async ({ page }) => {
    await openComponentPreview(page, "history-layout");
    const preview = page.getByTestId("history-layout-preview");
    const natural = preview.getByTestId("history-layout-natural");
    const compact = preview.getByTestId("history-layout-compact");
    await expect(natural.locator(".orbital-history__entry--natural").first()).toBeVisible({
      timeout: 30_000,
    });
    await expect(compact.locator(".orbital-history__entry--compact").first()).toBeVisible();
    await expect(compact.locator(".orbital-history__compact-line").first()).toBeVisible();
    await expect(natural.locator("[data-history-entry-id='1']")).toBeVisible();
    await expect(compact.locator("[data-history-entry-id='1']")).toBeVisible();
  });

  test("date bucket dividers appear for fixture ages happy path", async ({ page }) => {
    await openComponentPreview(page, "history-date-dividers");
    const preview = page.getByTestId("history-date-dividers-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("history-date-divider")).toHaveCount(5);
    await expect(preview.getByRole("separator", { name: "Today" })).toBeVisible();
    await expect(preview.getByRole("separator", { name: "Older" })).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='b1']")).toBeVisible();
  });

  test("paged server next page changes visible entries happy path", async ({ page }) => {
    await openComponentPreview(page, "history-paged");
    const preview = page.getByTestId("history-paged-preview");
    await expect(preview.getByTestId("history-pagination")).toBeVisible({ timeout: 30_000 });
    const firstId = await preview.locator("[data-history-entry-id]").first().getAttribute(
      "data-history-entry-id",
    );
    await preview.getByRole("button", { name: "Next page" }).click();
    await expect
      .poll(async () =>
        preview.locator("[data-history-entry-id]").first().getAttribute("data-history-entry-id"),
      )
      .not.toBe(firstId);
    await expect(preview.getByRole("button", { name: "Previous page" })).toBeEnabled();
  });

  test("paged server previous disabled on first page sad path", async ({ page }) => {
    await openComponentPreview(page, "history-paged");
    const preview = page.getByTestId("history-paged-preview");
    await expect(preview.getByTestId("history-pagination")).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByRole("button", { name: "Previous page" })).toBeDisabled();
  });

  test("refresh reloads from first page happy path", async ({ page }) => {
    await openComponentPreview(page, "history-refresh");
    const preview = page.getByTestId("history-refresh-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    const scroll = preview.locator(".orbital-history__scroll");
    for (let i = 0; i < 8; i++) {
      await scroll.evaluate((el) => {
        el.scrollTop = el.scrollHeight;
      });
      await page.waitForTimeout(200);
    }
    const deepVisible = await preview.locator("[data-history-entry-id^='page-']").count();
    expect(deepVisible).toBeGreaterThan(0);
    // Capture a late page id if present; refresh should drop accumulated pages.
    const lateId = await preview
      .locator("[data-history-entry-id^='page-']")
      .last()
      .getAttribute("data-history-entry-id");
    await preview.getByRole("button", { name: "Refresh" }).click();
    // Refresh bumps refresh_trigger / reloads page 1 — does not reset scrollTop.
    await expect(preview.locator("[data-history-entry-id='page-0']")).toBeVisible({
      timeout: 30_000,
    });
    await expect(preview.locator("[data-history-entry-id='1']")).toBeVisible();
    if (lateId && lateId !== "page-0" && Number(lateId.replace("page-", "")) >= 8) {
      await expect(preview.locator(`[data-history-entry-id='${lateId}']`)).toHaveCount(0);
    }
  });

  test("scroll_to_entry_or_load finds distant page entry happy path", async ({ page }) => {
    await openComponentPreview(page, "history-scroll-load");
    const preview = page.getByTestId("history-scroll-load-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-history-entry-id='page-25']")).toHaveCount(0);
    await preview.getByRole("button", { name: "Find page-25" }).click();
    await expect(preview.locator("[data-history-entry-id='page-25']")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("handle scroll to entry and scroll to top happy path", async ({ page }) => {
    await openComponentPreview(page, "history-handle", "history-handle-preview");
    const preview = page.getByTestId("history-handle-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await preview.getByRole("button", { name: "Scroll to entry 3" }).click();
    await expect(preview.locator("[data-history-entry-id='3']")).toBeVisible();
    const scroll = preview.locator(".orbital-history__scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = 200;
    });
    await preview.getByRole("button", { name: "Scroll to top" }).click();
    await expect
      .poll(async () => scroll.evaluate((el) => el.scrollTop))
      .toBeLessThan(10);
  });

  test("virtualized list mounts distant rows after scroll happy path", async ({ page }) => {
    await openComponentPreview(page, "history-virtualized");
    const preview = page.getByTestId("history-virtualized-preview");
    await expect(preview.getByTestId("history-timeline")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-history-entry-id='large-0']")).toBeVisible();
    await expect(preview.locator("[data-history-entry-id='large-70']")).toHaveCount(0);
    const scroll = preview.locator(".orbital-history__scroll");
    // Spacers + scroll_top listener: step through the list and fire scroll so the
    // virtual window advances (a single scrollHeight jump lands past large-70).
    for (let i = 0; i < 40; i++) {
      await scroll.evaluate((el) => {
        el.scrollTop = Math.min(el.scrollTop + el.clientHeight, el.scrollHeight);
        el.dispatchEvent(new Event("scroll", { bubbles: true }));
      });
      if ((await preview.locator("[data-history-entry-id='large-70']").count()) > 0) {
        break;
      }
      await page.waitForTimeout(50);
    }
    await expect(preview.locator("[data-history-entry-id='large-70']")).toBeVisible({
      timeout: 30_000,
    });
  });

  test("server filter chrome narrows via fetch params happy path", async ({ page }) => {
    await openComponentPreview(page, "history-server-filter");
    const preview = page.getByTestId("history-server-filter-preview");
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible({ timeout: 30_000 });
    const before = await preview.locator("[data-history-entry-id]").count();
    expect(before).toBeGreaterThan(1);
    const input = preview.locator(".orbital-history__filter-chrome input");
    await input.fill("jordan");
    await expect
      .poll(async () => preview.locator("[data-history-entry-id]").count())
      .toBeLessThan(before);
    await expect(preview.getByText(/Jordan/i).first()).toBeVisible();
  });

  test("server filter with no matches shows overlay sad path", async ({ page }) => {
    await openComponentPreview(page, "history-server-filter");
    const preview = page.getByTestId("history-server-filter-preview");
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible({ timeout: 30_000 });
    const input = preview.locator(".orbital-history__filter-chrome input");
    await input.fill("zzz-no-match-server-filter");
    await expect
      .poll(async () => preview.locator("[data-history-entry-id]").count(), { timeout: 30_000 })
      .toBe(0);
    await expect(preview.getByTestId("history-filter-chrome")).toBeVisible();
    await expect(preview.getByTestId("history-no-matches-default")).toBeVisible({
      timeout: 30_000,
    });
  });

  test("loading skeleton shows while empty client list happy path", async ({ page }) => {
    await openComponentPreview(page, "history-loading");
    const preview = page.getByTestId("history-loading-preview");
    await expect(preview.getByTestId("history-timeline-skeleton")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-history-entry-id]")).toHaveCount(0);
  });
});
