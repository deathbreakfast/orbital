import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("select primitive preview", () => {

  test("S-01: basic select changes value on option pick", async ({ page }) => {
    await openComponentPreview(page, "select");
    const select = page.getByTestId("select-preview").locator("select");
    await select.selectOption("b");
    await expect(select).toHaveValue("b");
  });

  test("S-02: default_value pre-selects option when signal is empty", async ({ page }) => {
    await openComponentPreview(page, "select");
    await page.getByTestId("select-default").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("select-default").locator("select")).toHaveValue("green");
  });

  test("S-03: disabled select cannot change", async ({ page }) => {
    await openComponentPreview(page, "select");
    await page.getByTestId("select-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("select-disabled").locator("select")).toBeDisabled();
  });

  test("S-04: small size class applies in size matrix", async ({ page }) => {
    await openComponentPreview(page, "select");
    await page.getByTestId("select-small").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("select-small").locator(".orbital-select--small")).toBeVisible();
  });

  test("S-05: select in field associates label", async ({ page }) => {
    await openComponentPreview(page, "select");
    await page.getByTestId("select-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("select-field").locator("label").getAttribute("for");
    const selectId = await page.getByTestId("select-field").locator("select").getAttribute("id");
    expect(labelFor).toEqual(selectId);
  });

  test("S-06: size matrix large select is taller than small", async ({ page }) => {
    await openComponentPreview(page, "select");
    await page.getByTestId("select-size-matrix").scrollIntoViewIfNeeded();
    const small = page.getByTestId("select-size-matrix").locator(".orbital-select--small select").first();
    const large = page.getByTestId("select-size-matrix").locator(".orbital-select--large select").first();
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const largeH = await large.evaluate((el) => el.getBoundingClientRect().height);
    expect(largeH).toBeGreaterThan(smallH);
  });

  test("S-SEC-01: selected option's secondary text shows as a caption and describes the select", async ({ page }) => {
    await openComponentPreview(page, "select");
    const root = page.getByTestId("select-secondary");
    await root.scrollIntoViewIfNeeded();
    const select = root.locator("select");
    const caption = root.locator(".orbital-select__secondary");
    await expect(caption).toHaveText("2 vCPU, 4 GB, 3 workers");
    expect(await select.getAttribute("aria-describedby")).toEqual(await caption.getAttribute("id"));
    await select.selectOption("large");
    await expect(caption).toHaveText("16 vCPU, 64 GB, 1 worker");
    await select.selectOption("plain");
    await expect(caption).toHaveCount(0);
    await expect(select).not.toHaveAttribute("aria-describedby", /.+/);
  });
});
