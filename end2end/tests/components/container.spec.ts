import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("container preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "container");
  });

  test("CT-01 renders default centered preview", async ({ page }) => {
    await expect(page.getByTestId("container-preview")).toBeVisible();
    await expect(page.getByText("Centered content at 1200px max-width")).toBeVisible();
  });

  test("CT-02 shows width variants", async ({ page }) => {
    await expectPreviewVariants(page, ["container-narrow", "container-wide"]);
  });

  test("CT-03 narrow variant constrains width", async ({ page }) => {
    await page.getByTestId("container-narrow").scrollIntoViewIfNeeded();
    const container = page.getByTestId("container-narrow").locator("div").first();
    const width = await container.evaluate((el) => getComputedStyle(el).width);
    expect(width).toBe("720px");
  });

  test("CT-04 max_width follows a signal", async ({ page }) => {
    const wrapper = page.getByTestId("container-reactive");
    await wrapper.scrollIntoViewIfNeeded();
    const inner = page.getByTestId("container-reactive-inner");
    await expect(inner).toHaveCSS("width", "480px");
    await page.getByTestId("container-reactive-toggle").click();
    await expect(inner).toHaveCSS("width", "880px");
  });
});
