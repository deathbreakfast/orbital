import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("tag primitive preview", () => {
  test("TG-01: default tag renders label text", async ({ page }) => {
    await openComponentPreview(page, "tag");
    const tag = page.getByTestId("tag-preview").locator(".orbital-tag");
    await expect(tag).toHaveText("Design");
    await expect(tag).not.toHaveClass(/orbital-tag--dismissible/);
  });

  test("TG-02: dismissible tag shows dismiss button and removes on click", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-dismissible"]);
    const tag = page.getByTestId("tag-dismissible").locator(".orbital-tag");
    await expect(tag).toHaveClass(/orbital-tag--dismissible/);
    await expect(tag.locator(".orbital-tag__dismiss")).toBeVisible();
    await tag.locator(".orbital-tag__dismiss").click();
    await expect(page.getByTestId("tag-dismissed")).toHaveText("Dismissed");
  });

  test("TG-03: size matrix tags differ in height", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-sizes"]);
    const medium = page.getByTestId("tag-size-medium").locator(".orbital-tag");
    const small = page.getByTestId("tag-size-small").locator(".orbital-tag");
    const extraSmall = page.getByTestId("tag-size-extra-small").locator(".orbital-tag");
    await expect(medium).toHaveClass(/orbital-tag--medium/);
    await expect(small).toHaveClass(/orbital-tag--small/);
    await expect(extraSmall).toHaveClass(/orbital-tag--extra-small/);
    const mediumH = await medium.evaluate((el) => el.getBoundingClientRect().height);
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const xsH = await extraSmall.evaluate((el) => el.getBoundingClientRect().height);
    expect(mediumH).toBeGreaterThan(smallH);
    expect(smallH).toBeGreaterThan(xsH);
  });

  test("TG-04: appearance matrix applies appearance modifiers", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-appearances"]);
    const root = page.getByTestId("tag-appearances");
    await expect(root.locator(".orbital-tag--filled")).toHaveCount(1);
    await expect(root.locator(".orbital-tag--outline")).toHaveCount(1);
    await expect(root.locator(".orbital-tag--brand")).toHaveCount(1);
  });

  test("TG-05: tag with icon renders media slot", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-with-icon"]);
    const tag = page.getByTestId("tag-with-icon").locator(".orbital-tag");
    await expect(tag).toHaveClass(/orbital-tag--with-media/);
    await expect(tag.locator(".orbital-tag__media")).toBeVisible();
  });

  test("TG-06: icon dismissible tag shows icon and dismiss", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-icon-dismissible"]);
    const tag = page.getByTestId("tag-icon-dismissible").locator(".orbital-tag");
    await expect(tag).toHaveClass(/orbital-tag--with-media/);
    await expect(tag).toHaveClass(/orbital-tag--dismissible/);
    await expect(tag.locator(".orbital-tag__media")).toBeVisible();
    await expect(tag.locator(".orbital-tag__dismiss")).toBeVisible();
  });

  test("TG-07: custom class merges onto tag root", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-custom"]);
    await expect(page.getByTestId("tag-custom").locator(".orbital-tag")).toHaveClass(/tag-custom-class/);
  });

  test("TG-08: dismiss button is named after the tag", async ({ page }) => {
    await openComponentPreview(page, "tag");
    await expectPreviewVariants(page, ["tag-dismissible", "tag-dismiss-label"]);
    const remove = page.getByTestId("tag-dismissible").getByRole("button", { name: "Remove Removable" });
    await expect(remove).toBeVisible();
    await expect(remove).toHaveAttribute("type", "button");
    await expect(
      page.getByTestId("tag-dismiss-label").getByRole("button", { name: "Unlink Design", exact: true }),
    ).toBeVisible();
  });
});
