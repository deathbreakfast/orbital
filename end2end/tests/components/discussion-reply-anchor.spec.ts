import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion reply anchor", () => {
  test("navigate_to_reply reveals nested reply without manual drill-in", async ({ page }) => {
    await openComponentPreview(page, "discussion-reply-anchor");
    const preview = page.getByTestId("discussion-reply-anchor-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await expect(preview.locator("[data-reply-id='d-l4']")).not.toBeVisible();
    await preview.getByTestId("discussion-reply-anchor-d-l4").click();

    await expect(preview.locator("[data-reply-id='d-l4']")).toBeVisible({ timeout: 10_000 });
    await expect(preview.locator("[data-reply-id='d-root']")).not.toBeVisible();
  });

  test("shallow anchor stays at root focus", async ({ page }) => {
    await openComponentPreview(page, "discussion-reply-anchor");
    const preview = page.getByTestId("discussion-reply-anchor-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await preview.getByTestId("discussion-reply-anchor-d-l2").click();
    await expect(preview.locator("[data-reply-id='d-l2']")).toBeVisible({ timeout: 10_000 });
    await expect(preview.locator("[data-reply-id='d-root']")).toBeVisible();
  });

  test("highlighted marks only the linked reply", async ({ page }) => {
    await openComponentPreview(page, "discussion-reply-anchor");
    const preview = page.getByTestId("discussion-reply-anchor-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await expect(preview.locator("[data-highlighted='true']")).toHaveCount(0);
    await preview.getByTestId("discussion-reply-anchor-d-l2").click();

    const target = preview.locator("[data-reply-id='d-l2']");
    await expect(target).toHaveAttribute("data-highlighted", "true", { timeout: 10_000 });
    await expect(preview.locator("[data-highlighted='true']")).toHaveCount(1);
    await expect(preview.locator("[data-reply-id='d-root']")).toHaveAttribute(
      "data-highlighted",
      "false",
    );
  });
});
