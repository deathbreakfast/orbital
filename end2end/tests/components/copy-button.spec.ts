import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

test.describe("copy-button primitive preview", () => {
  test("copies text and confirms", async ({ page, context, browserName }) => {
    test.skip(browserName !== "chromium", "clipboard permissions are chromium-only");
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await openComponentPreview(page, "copy-button");

    const preview = page.getByTestId("copy-button-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    const button = preview.getByRole("button", { name: "Copy" });
    await button.click();

    await expect(preview.getByRole("button", { name: "Copied" })).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(
      "https://example.com/share/abc123",
    );
    await expect(preview.getByRole("button", { name: "Copy" })).toBeVisible({ timeout: 5_000 });
  });

  test("custom label preview renders", async ({ page }) => {
    await openComponentPreview(page, "copy-button");
    await expect(
      page.getByTestId("copy-button-label").getByRole("button", { name: "Copy command" }),
    ).toBeVisible({ timeout: 30_000 });
  });
});
