import { test, expect } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table server source", () => {
  test("page 2 shows different rows than page 1", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-footer")).toContainText("1–5 of 30");

    const page1First = await preview.locator("tbody tr").first().getAttribute("data-testid");
    await preview.getByTestId("data-table-pagination").getByRole("button", { name: "2" }).click();
    await expect(preview.locator("tbody tr").first()).not.toHaveAttribute("data-testid", page1First!);
    await expect(preview.getByTestId("data-table-row-user-6")).toBeVisible();
  });

  test("server sort descending returns highest user first", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByText("Name", { exact: true }).click();
    await expect(preview.getByTestId("data-table-sort-asc")).toBeVisible();
    await preview.getByTestId("data-table-header-name").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();

    await expect(preview.locator("tbody tr").first()).toContainText("User 9");
  });

  test("server role filter reduces total count", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
    await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "Admin");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(preview.getByTestId("data-table-footer")).toContainText("1–5 of 10");
    await expect(preview.locator("tbody tr").first()).toContainText("Admin");
  });

  test("filter from page 2 clears stale total and resets to page 1", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-pagination").getByRole("button", { name: "2" }).click();
    await expect(preview.getByTestId("data-table-footer")).toContainText("6–10 of 30");

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "role");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "equals");
    await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "Admin");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(preview.getByTestId("data-table-footer")).toContainText("1–5 of 10");
    await expect(preview.getByTestId("data-table-footer")).not.toContainText("of 30");
    await expect(preview.getByTestId("data-table-row-user-3")).toBeVisible();
  });

  test("rapid pagination resolves to the last selected page", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();

    const pagination = preview.getByTestId("data-table-pagination");
    await pagination.getByRole("button", { name: "3" }).click();
    await pagination.getByRole("button", { name: "1" }).click();

    await expect(preview.getByTestId("data-table-row-user-1")).toBeVisible();
    await expect(preview.getByTestId("data-table-footer")).toContainText("1–5 of 30");
    await expect(preview.locator("tbody tr").first()).toContainText("User 1");
  });

  test("quick search invalidates server total", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-footer")).toContainText("of 30");

    const search = preview.getByTestId("data-table-quick-search").locator("input");
    await search.fill("User 30");
    await expect(preview.getByTestId("data-table-footer")).toContainText("of 1");
    await expect(preview.locator("tbody tr")).toHaveCount(1);
  });

  test("quick search with no server matches shows no-results, not empty", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-server-preview");
    const preview = page.getByTestId("data-table-server-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-footer")).toContainText("of 30");

    const search = preview.getByTestId("data-table-quick-search").locator("input");
    await search.fill("nobody-matches-this");
    await expect(preview.getByTestId("data-table-no-results")).toBeVisible({ timeout: 10_000 });
    await expect(preview.getByTestId("data-table-empty")).toHaveCount(0);

    await search.fill("");
    await expect(preview.getByTestId("data-table-footer")).toContainText("of 30");
    await expect(preview.getByTestId("data-table-no-results")).toHaveCount(0);
  });
});
