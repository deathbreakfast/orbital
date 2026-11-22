import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("list preview", () => {
  test("L-01 default list", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expect(page.getByTestId("list-preview").locator(".orbital-list__item")).toHaveCount(3);
  });

  test("L-02 nav mode", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expectPreviewVariants(page, ["list-nav"]);
    await expect(page.getByTestId("list-nav").locator(".orbital-list")).toHaveClass(/orbital-list--nav-nav/);
  });

  test("L-03 single select", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expectPreviewVariants(page, ["list-single-select"]);
    await expect(
      page.getByTestId("list-single-select").locator(".orbital-list__item--selected"),
    ).toHaveCount(1);
  });

  test("trailing action sits at the row end and does not click the row", async ({ page }) => {
    await openComponentPreview(page, "list");
    const wrapper = page.getByTestId("list-trailing");
    await wrapper.scrollIntoViewIfNeeded();
    const item = wrapper.locator(".orbital-list__item");
    const trailing = item.locator(".orbital-list__item-trailing");
    const [i, t] = await Promise.all([item.boundingBox(), trailing.boundingBox()]);
    // Pinned to the end of the row (within the row's end padding).
    expect(i!.x + i!.width - (t!.x + t!.width)).toBeLessThan(24);
    await trailing.getByRole("button", { name: "Remove" }).click();
    await expect(wrapper.getByTestId("list-trailing-counts")).toHaveText("opened=0 removed=1");
    await item.locator(".orbital-list__item-content").click();
    await expect(wrapper.getByTestId("list-trailing-counts")).toHaveText("opened=1 removed=1");
  });
});
