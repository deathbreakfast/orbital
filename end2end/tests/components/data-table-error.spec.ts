import { test, expect, type Locator, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

// The load-error previews use a fetcher that always fails and counts its calls
// ("data-table-error-calls"), so Try again shows up as one more call.

async function openPreview(page: Page, testId: string): Promise<Locator> {
  await openComponentPreview(page, "data-table-data-source", testId);
  const preview = page.getByTestId(testId);
  await preview.scrollIntoViewIfNeeded();
  return preview;
}

test.describe("data-table load error", () => {
  test("a failed fetch shows the error and Try again, not the empty state", async ({ page }) => {
    const preview = await openPreview(page, "data-table-error-preview");

    const error = preview.getByTestId("data-table-error");
    await expect(error).toBeVisible();
    await expect(error.getByTestId("data-table-error-message")).toHaveText(
      /The run store didn't answer \(503\)/,
    );
    await expect(error.getByTestId("data-table-error-retry")).toHaveText("Try again");
    await expect(preview.getByTestId("data-table-empty")).toHaveCount(0);
    await expect(preview.getByTestId("data-table-no-results")).toHaveCount(0);
    // The toolbar and its search stay usable.
    await expect(preview.getByTestId("data-table-quick-search")).toBeVisible();
  });

  test("Try again calls the fetcher again", async ({ page }) => {
    const preview = await openPreview(page, "data-table-error-preview");
    const calls = preview.getByTestId("data-table-error-calls");
    await expect(preview.getByTestId("data-table-error")).toBeVisible();
    const before = Number(await calls.textContent());
    expect(before).toBeGreaterThan(0);

    await preview.getByTestId("data-table-error-retry").click();

    await expect(calls).toHaveText(String(before + 1));
    await expect(preview.getByTestId("data-table-error")).toBeVisible();
  });

  test("a custom error slot replaces the default view", async ({ page }) => {
    const preview = await openPreview(page, "data-table-error-custom-preview");
    const custom = preview.getByTestId("custom-error");
    await expect(custom).toContainText("Couldn't load runs: The run store didn't answer (503)");
    await expect(preview.getByTestId("data-table-error-retry")).toHaveCount(0);
    await expect(custom.getByRole("button", { name: "Reload runs" })).toBeVisible();
  });
});
