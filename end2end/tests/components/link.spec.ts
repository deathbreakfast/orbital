import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle, expectNonEmptyResolvedStyle } from "../lib/assertions/style";
import { seriousAxeViolations, setPreviewTheme } from "../lib/preview/axe";
test.describe("link primitive preview", () => {
  test("LK-01 default anchor link", async ({ page }) => {
    await openComponentPreview(page, "link");
    const link = page.getByTestId("link-preview").locator("a[role=link]");
    await expect(link).toHaveAttribute("href", "#docs");
    await expect(link).toHaveText("View documentation");
    await expect(link).toHaveClass(/orbital-link/);
  });

  test("LK-02 inline link underline", async ({ page }) => {
    await openComponentPreview(page, "link");
    await expectPreviewVariants(page, ["link-inline"]);
    await expectComputedStyle(page, "link-inline", {
      "text-decoration-line": "underline",
    }, { childSelector: "a.orbital-link--inline" });
  });

  test("LK-03 disabled link state", async ({ page }) => {
    await openComponentPreview(page, "link");
    await expectPreviewVariants(page, ["link-disabled"]);
    const link = page.getByTestId("link-disabled").locator("a[role=link]");
    await expect(link).toHaveAttribute("aria-disabled", "true");
    await expect(link).toHaveClass(/orbital-link--disabled/);
  });

  test("LK-04 external href", async ({ page }) => {
    await openComponentPreview(page, "link");
    await expectPreviewVariants(page, ["link-external"]);
    const link = page.getByTestId("link-external").locator("a[role=link]");
    await expect(link).toHaveAttribute("href", "https://example.com/docs");
  });

  test("LK-05 theme link color token", async ({ page }) => {
    await openComponentPreview(page, "link");
    await expectPreviewVariants(page, ["link-theme"]);
    await expectNonEmptyResolvedStyle(page, "link-theme", "color", {
      childSelector: "a.orbital-link",
    });
  });

  test("LK-06 subtle link uses the primary text colour and passes axe contrast", async ({ page }) => {
    await openComponentPreview(page, "link");
    await expectPreviewVariants(page, ["link-subtle"]);
    const link = page.getByTestId("link-subtle").locator("a.orbital-link--subtle");
    await expect(link).toHaveText("Ada Lovelace");
    for (const mode of ["dark", "light"] as const) {
      await setPreviewTheme(page, mode);
      const [color, textPrimary] = await link.evaluate((el) => {
        const probe = document.createElement("span");
        probe.style.color = "var(--orb-color-text-primary)";
        el.parentElement!.appendChild(probe);
        const expected = getComputedStyle(probe).color;
        probe.remove();
        return [getComputedStyle(el).color, expected];
      });
      expect(color, mode).toBe(textPrimary);
      expect(await seriousAxeViolations(page, "[data-testid='link-subtle']"), mode).toEqual([]);
    }
  });
});
