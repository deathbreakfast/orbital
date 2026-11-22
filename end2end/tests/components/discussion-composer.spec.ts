import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion composer preview", () => {
  test("renders composer preview", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    await expect(page.getByTestId("discussion-composer-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("discussion-composer-input")).toBeVisible();
    await expect(page.getByTestId("discussion-composer-send")).toBeVisible();
    await expect(page.getByTestId("discussion-composer-format-toolbar")).toBeVisible();
  });

  test("top-level submit fires callback with no parent_id", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await input.fill("Hello from composer E2E");
    await preview.locator('[data-testid="discussion-composer-send"] button').click();

    await expect(page.getByTestId("discussion-composer-submit-log")).toHaveText(
      "none:Hello from composer E2E:attachments=0:citations=0",
    );
    await expect(preview.locator(".orbital-discussion__reply-body").getByText("Hello from composer E2E")).toBeVisible();
  });

  test("a posted reply draws Markdown tables, task lists and alerts as Orbital components", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await input.fill(
      "## Plan\n\n| Step | Owner |\n|---|---|\n| Ship | Sam |\n\n- [x] Draft\n- [ ] Review\n\n> [!WARNING]\n> Freeze on Friday.",
    );
    await preview.locator('[data-testid="discussion-composer-send"] button').click();

    const body = preview.locator(".orbital-discussion__markdown").filter({ hasText: "Freeze on Friday." });
    await expect(body).toBeVisible();
    await expect(body.locator("tr.orbital-table-row").first()).toHaveCSS("border-bottom-style", "solid");
    await expect(body.locator(".orbital-checkbox__indicator").first()).toHaveCSS("width", "16px");
    await expect(body.getByRole("checkbox", { name: "Task" }).first()).toBeChecked();
    const alert = body.locator(".orbital-message-bar--warning");
    await expect(alert).toHaveCSS("display", "grid");
    await expect(alert.locator(".orbital-message-bar-title")).toHaveText("Warning");
    await expect(body.getByRole("heading", { name: "Plan", level: 2 })).toBeVisible();
  });

  test("reply submit fires callback with expected parent_id", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const samNode = preview.locator("[data-reply-id='r-sam']").first();
    await samNode
      .locator(".orbital-discussion__reply-card")
      .getByRole("button", { name: "Reply" })
      .click();

    await expect(preview.getByTestId("discussion-composer-reply-banner")).toBeVisible();
    await expect(preview.getByTestId("discussion-composer-reply-banner")).toContainText(
      "Replying to Sam Rivera",
    );

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await input.fill("Replying in thread");
    await preview.locator('[data-testid="discussion-composer-send"] button').click();

    await expect(page.getByTestId("discussion-composer-submit-log")).toHaveText(
      "r-sam:Replying in thread:attachments=0:citations=0",
    );
  });

  test("format toolbar bold wraps draft text", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await input.click();
    await page.keyboard.type("word");
    await input.selectText();
    await preview.getByRole("button", { name: "Bold" }).click();
    await expect(input).toHaveValue("**word**");
  });

  test("citation insert adds chip and ref token", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await preview.getByRole("button", { name: "Insert citation" }).click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await dialog.locator('[data-testid="discussion-composer-citation-dialog-title"] input').fill(
      "Design spec",
    );
    await dialog.locator('[data-testid="discussion-composer-citation-dialog-url"] input').fill(
      "https://orbital.dev/spec",
    );
    await dialog.getByTestId("discussion-composer-citation-dialog-add").click();

    await expect(preview.getByTestId("discussion-composer-citations")).toBeVisible();
    await expect(preview.locator('[data-testid^="discussion-composer-citation-"]')).toContainText(
      "Design spec",
    );

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await expect(input).toHaveValue(/\[\^cit-\d+\]/);

    await preview.getByRole("button", { name: "Send" }).click();
    await expect(page.getByTestId("discussion-composer-submit-log")).toHaveText(/:citations=1$/);
  });

  test("attachment chip visible after file pick", async ({ page }) => {
    await openComponentPreview(page, "discussion-composer");
    const preview = page.getByTestId("discussion-composer-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("discussion-composer-attach")).toBeVisible();

    const fileInput = preview.locator('[data-testid="discussion-composer-file-input"]');
    await fileInput.setInputFiles({
      name: "notes.pdf",
      mimeType: "application/pdf",
      buffer: Buffer.from("sample attachment"),
    });

    await expect(preview.getByTestId("discussion-composer-attachments")).toBeVisible();
    await expect(preview.locator('[data-testid^="discussion-composer-attachment-"]')).toContainText(
      "notes.pdf",
    );
  });
});
