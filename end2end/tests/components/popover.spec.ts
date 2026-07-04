import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import {
  expectFollowerPlacement,
  expectOverlayInViewport,
  expectOverlayNonZeroSize,
  expectNoBodyScrollbarGrowth,
  expectPopoverTopAlignedWithTrigger,
  expectPopoverDismissed,
  scrollScrollport,
} from "../lib/preview/overlays";
test.describe("popover primitive preview", () => {

  test("PO-01: click opens popover panel content", async ({ page }) => {
    await openComponentPreview(page, "popover");
    const trigger = page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" });
    await expectNoBodyScrollbarGrowth(page, async () => {
      await trigger.click();
    });
    await expect(page.getByTestId("popover-content")).toBeVisible();
    await expect(page.getByTestId("popover-content")).toContainText("Popover body content");
    await expectOverlayNonZeroSize(page, ".orbital-popover-surface");
    await expectOverlayInViewport(page, ".orbital-popover-surface");
  });

  test("PO-02: click outside closes click-triggered popover", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" }).click();
    await expect(page.getByTestId("popover-content")).toBeVisible();
    await page.getByTestId("preview-doc-panel").click({ position: { x: 8, y: 8 } });
    await expect(page.getByTestId("popover-content")).toBeHidden();
  });

  test("PO-03: hover reveals hover popover body", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-hover").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-hover").getByRole("button", { name: "Hover" }).hover();
    await expect(page.getByTestId("popover-hover-body")).toBeVisible();
    await expect(page.getByTestId("popover-hover-body")).toContainText("Details on hover");
  });

  test("PO-04: mouse leave hides hover popover body", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-hover").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-hover").getByRole("button", { name: "Hover" }).hover();
    await expect(page.getByTestId("popover-hover-body")).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(page.getByTestId("popover-hover-body")).toBeHidden();
  });

  test("PO-05: second click on trigger closes popover", async ({ page }) => {
    await openComponentPreview(page, "popover");
    const trigger = page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" });
    await trigger.click();
    await expect(page.getByTestId("popover-content")).toBeVisible();
    await trigger.click();
    await expect(page.getByTestId("popover-content")).toBeHidden();
  });

  test("PO-06: popover content is teleported outside preview wrapper", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" }).click();
    const content = page.getByTestId("popover-content");
    await expect(content).toBeVisible();
    const insideWrapper = page.getByTestId("popover-preview").locator('[data-testid="popover-content"]');
    await expect(insideWrapper).toHaveCount(0);
  });

  test("PO-07: open popover exposes teleported panel with body testid", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" }).click();
    await expectFollowerPlacement(page, "popover-content", "top");
  });

  test("PO-08: open popover renders popover surface element", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-preview").getByRole("button", { name: "Open popover" }).click();
    await expect(
      page.locator(".orbital-popover-surface").filter({ has: page.getByTestId("popover-content") }),
    ).toBeVisible();
  });

  test("PO-09: brand popover uses brand modifier class", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-brand").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-brand").getByRole("button", { name: "Brand popover" }).click();
    await expect(page.getByTestId("popover-brand-body")).toBeVisible();
    await expect(page.locator(".orbital-popover-surface--brand").first()).toBeVisible();
  });

  test("PO-10: inverted popover uses inverted modifier class", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-inverted").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-inverted").getByRole("button", { name: "Inverted" }).click();
    await expect(page.getByTestId("popover-inverted-body")).toBeVisible();
    await expect(page.locator(".orbital-popover-surface--inverted").first()).toBeVisible();
  });

  test("PO-11: small popover uses small size modifier class", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-small").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-small").getByRole("button", { name: "Small" }).click();
    await expect(page.locator(".orbital-popover-surface--small").first()).toBeVisible();
    await expect(page.getByText("Compact panel")).toBeVisible();
  });

  test("PO-12: large popover uses large size modifier class", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-large").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-large").getByRole("button", { name: "Large" }).click();
    await expect(page.locator(".orbital-popover-surface--large").first()).toBeVisible();
    await expect(page.getByText("Roomier panel").first()).toBeVisible();
  });

  test("PO-13: left position sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-left").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-left").getByRole("button", { name: "Left" }).click();
    await expect(page.getByText("Opens to the left").first()).toBeVisible();
    await expect(page.locator('[data-orbital-placement="left"]').first()).toHaveAttribute("data-orbital-placement", "left");
  });

  test("PO-14: lifecycle hook sets opened signal when popover opens", async ({ page }) => {
    await openComponentPreview(page, "popover");
    await page.getByTestId("popover-lifecycle").scrollIntoViewIfNeeded();
    await page.getByTestId("popover-lifecycle").getByRole("button", { name: "Lifecycle" }).click();
    await expect(page.locator(".orbital-popover-surface").filter({ hasText: "Panel body" }).first()).toBeVisible();
  });

  test("PO-15: popover stays aligned when inner scroll area scrolls", async ({ page }) => {
    await openComponentPreview(page, "popover");
    const wrapper = page.getByTestId("popover-scroll-area");
    await wrapper.scrollIntoViewIfNeeded();
    await expect(wrapper).toBeVisible();
    const scrollport = page.getByTestId("popover-scroll-area-scrollport");
    const trigger = wrapper.getByRole("button", { name: "Open in scroll area" });
    const content = page.getByTestId("popover-scroll-area-content");
    const overlay = page
      .locator(".orbital-popover-surface")
      .filter({ has: content });

    await scrollScrollport(scrollport, 140);
    await trigger.click();
    await expect(content).toBeVisible();
    await expectPopoverTopAlignedWithTrigger(trigger, overlay);

    await scrollScrollport(scrollport, 260);
    await expectPopoverTopAlignedWithTrigger(trigger, overlay);
  });

  test("PO-16: popover dismisses when trigger scrolls out of scrollport", async ({ page }) => {
    await openComponentPreview(page, "popover");
    const wrapper = page.getByTestId("popover-scroll-area");
    await wrapper.scrollIntoViewIfNeeded();
    const scrollport = page.getByTestId("popover-scroll-area-scrollport");
    const trigger = wrapper.getByRole("button", { name: "Open in scroll area" });
    const content = page.getByTestId("popover-scroll-area-content");

    await trigger.click();
    await expect(content).toBeVisible();

    await scrollScrollport(scrollport, 700);
    await expectPopoverDismissed(content);
  });
});
