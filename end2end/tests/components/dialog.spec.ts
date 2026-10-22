import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { expectTeleportedRole } from "../lib/preview/overlays";
test.describe("dialog primitive preview", () => {

  test("DL-01: trigger opens dialog with preview content", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-basic").getByRole("button", { name: "Open dialog" }).click();
    await expectTeleportedRole(page, "dialog");
    await expect(page.getByTestId("dialog-preview")).toContainText("Save your changes?");
  });

  test("DL-02: trigger opens closed dialog", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    const trigger = page.getByTestId("dialog-trigger");
    await trigger.scrollIntoViewIfNeeded();
    await expect(page.getByTestId("dialog-trigger-content")).toBeHidden();
    await trigger.getByRole("button", { name: "Open dialog" }).click();
    await expect(page.getByTestId("dialog-trigger-content")).toBeVisible();
  });

  test("DL-03: non-dismissible mask ignores backdrop click", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-modal").scrollIntoViewIfNeeded();
    await page.getByTestId("dialog-modal").getByRole("button", { name: "Open modal" }).click();
    await expect(page.getByTestId("dialog-modal-content")).toBeVisible();
    const backdrop = page
      .locator(".orbital-dialog")
      .filter({ has: page.getByTestId("dialog-modal-content") })
      .locator(".orbital-backdrop");
    await backdrop.click({ force: true });
    await expect(page.getByTestId("dialog-modal-content")).toBeVisible();
  });

  test("DL-04: dialog actions footer renders cancel button", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-actions").scrollIntoViewIfNeeded();
    await page.getByTestId("dialog-actions").getByRole("button", { name: "Open dialog" }).click();
    await expect(page.getByRole("dialog").getByRole("button", { name: "Cancel" })).toBeVisible();
  });

  test("DL-05: escape closes esc-enabled dialog", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-esc").scrollIntoViewIfNeeded();
    await page.getByTestId("dialog-esc").getByRole("button", { name: "Open dialog" }).click();
    const dialog = page.getByRole("dialog").filter({ hasText: "Press Escape to close" });
    await expect(dialog).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
  });

  test("DL-06: opening moves focus in and Tab wraps inside the dialog", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    const section = page.getByTestId("dialog-focus");
    await section.scrollIntoViewIfNeeded();
    await section.getByRole("button", { name: "Open dialog" }).focus();
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog").filter({ hasText: "First" });
    const first = dialog.getByRole("button", { name: "First" });
    const last = dialog.getByRole("button", { name: "Last" });
    await expect(first).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(last).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(first).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expect(last).toBeFocused();
  });

  test("DL-09: closing returns focus to the trigger", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    const section = page.getByTestId("dialog-focus");
    await section.scrollIntoViewIfNeeded();
    const trigger = section.getByRole("button", { name: "Open dialog" });
    await trigger.focus();
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog").filter({ hasText: "First" });
    await expect(dialog.getByRole("button", { name: "First" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("DL-10: a dialog with no controls focuses its surface and keeps Tab there", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    const section = page.getByTestId("dialog-theme");
    await section.scrollIntoViewIfNeeded();
    await section.getByRole("button", { name: "Open dialog" }).focus();
    await page.keyboard.press("Enter");
    const surface = page.getByRole("dialog").filter({ hasText: "Themed dialog surface" });
    await expect(surface).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(surface).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expect(surface).toBeFocused();
  });

  test("DL-07: dialog surface uses theme background token", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-theme").scrollIntoViewIfNeeded();
    await page.getByTestId("dialog-theme").getByRole("button", { name: "Open dialog" }).click();
    const surface = page.getByRole("dialog").filter({ hasText: "Themed dialog surface" });
    await expect(surface).toBeVisible();
    const opacity = await surface.evaluate((el) => getComputedStyle(el).opacity);
    expect(Number(opacity)).toBeGreaterThan(0);
    const box = await surface.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
  });

  test("DL-08: dialog scrim uses shared orbital-backdrop", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-basic").getByRole("button", { name: "Open dialog" }).click();
    const scrim = page.locator(".orbital-dialog .orbital-backdrop").first();
    await expect(scrim).toBeVisible();
    const bg = await scrim.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bg).not.toBe("rgba(0, 0, 0, 0)");
    expect(bg).not.toBe("transparent");
  });

  async function openLate(page: import("@playwright/test").Page, testId: string) {
    await openComponentPreview(page, "dialog", "dialog-basic");
    const section = page.getByTestId(testId);
    await section.scrollIntoViewIfNeeded();
    const trigger = section.getByRole("button", { name: "Open dialog" });
    await trigger.focus();
    await page.keyboard.press("Enter");
    return { trigger, dialog: page.getByRole("dialog") };
  }

  test("DL-11: content that mounts after the dialog opens receives focus", async ({ page }) => {
    const { trigger, dialog } = await openLate(page, "dialog-late-focus");
    await expect(dialog).toBeFocused();
    const late = dialog.getByRole("button", { name: "Late control" });
    await expect(late).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(late).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("DL-12: closing before late content mounts leaves focus on the trigger", async ({ page }) => {
    const { trigger, dialog } = await openLate(page, "dialog-late-focus");
    await expect(dialog).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
    await page.waitForTimeout(700);
    await expect(trigger).toBeFocused();
  });

  test("DL-13: reopening focuses the late content again", async ({ page }) => {
    const { trigger, dialog } = await openLate(page, "dialog-late-focus");
    const late = dialog.getByRole("button", { name: "Late control" });
    await expect(late).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(dialog).toBeFocused();
    await expect(late).toBeFocused();
  });

  test("DL-14: a control enabled after the dialog opens receives focus", async ({ page }) => {
    const { dialog } = await openLate(page, "dialog-late-enabled");
    await expect(dialog.getByRole("button", { name: "Enabled later" })).toBeFocused();
  });

  test("DL-14b: a control revealed after the dialog opens receives focus", async ({ page }) => {
    const { dialog } = await openLate(page, "dialog-late-revealed");
    await expect(dialog.getByRole("button", { name: "Revealed later" })).toBeFocused();
  });

  test("DL-15: late content does not take focus the user already placed in the dialog", async ({ page }) => {
    const { dialog } = await openLate(page, "dialog-late-focus");
    const block = dialog.getByTestId("dialog-late-block");
    await block.click();
    await expect(block).toBeFocused();
    const late = dialog.getByRole("button", { name: "Late control" });
    await expect(late).toBeVisible();
    await page.waitForTimeout(300);
    await expect(block).toBeFocused();
    await expect(late).not.toBeFocused();
  });
});
