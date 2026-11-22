import { test, expect, type Locator, type Page } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";

// The cursor preview's fetcher never reports a total and records whether each
// request continued from a cursor ("cursor <offset>") or paged by offset.

async function openCursorPreview(page: Page): Promise<Locator> {
  await openComponentPreview(page, "data-table-data-source", "data-table-cursor-preview");
  const preview = page.getByTestId("data-table-cursor-preview");
  await preview.scrollIntoViewIfNeeded();
  await expect(preview.getByTestId("data-table-footer")).toContainText("1–5 of more than 5");
  return preview;
}

function pager(preview: Locator) {
  const bar = preview.getByTestId("data-table-pagination");
  return {
    next: bar.getByRole("button", { name: "Next page" }),
    previous: bar.getByRole("button", { name: "Previous page" }),
  };
}

async function expectPage(preview: Locator, firstRow: string, request: string, range: string) {
  await expect(preview.locator("tbody tr").first()).toHaveAttribute(
    "data-testid",
    `data-table-row-${firstRow}`,
  );
  await expect(preview.getByTestId("data-table-cursor-last-request")).toHaveText(request);
  await expect(preview.getByTestId("data-table-footer")).toContainText(range);
}

// Phones narrow the rows with quick search: the filter panel's popover opens
// partly off-screen at phone width. Both are query changes to the table.
async function filterAdmins(page: Page, preview: Locator) {
  if (test.info().project.name.includes("mobile")) {
    await preview.getByTestId("data-table-quick-search").locator("input").fill("Admin");
    return;
  }
  await preview.getByTestId("data-table-filter-panel-trigger").click();
  const panel = page.getByTestId("data-table-filter-panel");
  await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
  await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
  await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "Admin");
  await panel.getByRole("button", { name: "Apply" }).click();
}

for (const scheme of ["light", "dark"] as const) {
  test.describe(`data-table cursor paging (${scheme})`, () => {
    test.beforeEach(async ({ page }) => {
      await page.emulateMedia({ colorScheme: scheme });
    });

    test("next and previous continue from recorded cursors", async ({ page }) => {
      const preview = await openCursorPreview(page);
      const { next, previous } = pager(preview);
      await expectPage(preview, "user-1", "offset 0", "1–5 of more than 5");

      await next.click();
      await expectPage(preview, "user-6", "cursor 5", "6–10 of more than 10");
      await next.click();
      await expectPage(preview, "user-11", "cursor 10", "11–15 of more than 15");

      await previous.click();
      await expectPage(preview, "user-6", "cursor 5", "6–10 of more than 10");
      await previous.click();
      await expectPage(preview, "user-1", "offset 0", "1–5 of more than 5");
    });

    test("a filter change restarts at page 1 by offset", async ({ page }) => {
      const preview = await openCursorPreview(page);
      const { next } = pager(preview);
      await next.click();
      await expectPage(preview, "user-6", "cursor 5", "6–10 of more than 10");

      await filterAdmins(page, preview);
      // Admins are every third user: 3, 6, 9, … 30.
      await expectPage(preview, "user-3", "offset 0", "1–5 of more than 5");

      await next.click();
      await expectPage(preview, "user-18", "cursor 5", "6–10 of 10");
      await expect(next).toBeDisabled();
    });
  });
}
