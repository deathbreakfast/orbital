import type { Locator, Page } from "@playwright/test";
import { expect } from "@playwright/test";
import { previewUrl, waitForPreviewShell } from "../../helpers";

/** Catalog scrollport selector (page or inset main ScrollArea). */
export function previewScrollport(page: Page) {
  return page.locator(".orbital-layout__page-scroll, .orbital-layout__main-scroll").first();
}

/** Scroll a target into view within the catalog scrollport (document scroll is disabled). */
export async function scrollIntoPreviewView(locator: Locator) {
  await locator.evaluate((el) => {
    const scrollport =
      el.closest(".orbital-layout__page-scroll, .orbital-layout__main-scroll") ??
      document.querySelector(".orbital-layout__page-scroll, .orbital-layout__main-scroll");
    if (!scrollport) {
      el.scrollIntoView({ block: "center" });
      return;
    }
    const elRect = el.getBoundingClientRect();
    const portRect = scrollport.getBoundingClientRect();
    const offset = elRect.top - portRect.top + scrollport.scrollTop;
    scrollport.scrollTop = offset - scrollport.clientHeight / 2 + elRect.height / 2;
  });
}

/** Scroll the preview catalog scrollport (page or inset main ScrollArea). */
export async function scrollPreviewMain(page: Page, scrollTop: number) {
  const scrollport = previewScrollport(page);
  await scrollport.evaluate((el, top) => {
    el.scrollTop = top;
    el.dispatchEvent(new Event("scroll", { bubbles: true }));
  }, scrollTop);
}

/** Read scrollTop from the preview catalog scrollport. */
export async function previewMainScrollTop(page: Page): Promise<number> {
  return page
    .locator(".orbital-layout__page-scroll, .orbital-layout__main-scroll")
    .first()
    .evaluate((el) => el.scrollTop);
}

/** Open a registry preview page and wait for the default preview wrapper. */
export async function openComponentPreview(
  page: Page,
  slug: string,
  defaultTestId: string = `${slug}-preview`,
) {
  await page.goto(previewUrl(`/${slug}`));
  await waitForPreviewShell(page);
  const wrapper = page.getByTestId(defaultTestId).first();
  await scrollIntoPreviewView(wrapper);
  await expect(wrapper).toBeVisible({ timeout: 10_000 });
}

/** Assert additional preview variant testids are visible on the page. */
export async function expectPreviewVariants(page: Page, testIds: string[]) {
  for (const id of testIds) {
    const el = page.getByTestId(id).first();
    await scrollIntoPreviewView(el);
    await expect(el).toBeVisible({ timeout: 10_000 });
  }
}

/** Assert the preview page title matches the catalog label. */
export async function expectPreviewPageTitle(page: Page, label: string) {
  const title = page.getByTestId("preview-page-title");
  await expect(title).toBeVisible();
  await expect(title).toHaveText(label);
}

/** Expand catalog nav folders and activate a link (keyboard avoids sidebar overlay hits). */
export async function clickCatalogNavLink(
  page: Page,
  folderPath: string[],
  linkLabel: string,
) {
  const nav = page.getByTestId("preview-catalog-nav");
  for (const folder of folderPath) {
    const folderButton = nav.getByRole("button", { name: folder, exact: true });
    if ((await folderButton.count()) > 0) {
      if ((await folderButton.getAttribute("aria-expanded")) !== "true") {
        await folderButton.click();
      }
    }
  }

  const link = nav.getByRole("link", { name: linkLabel, exact: true });
  await expect(link).toBeVisible();
  await link.evaluate((el) => (el as HTMLAnchorElement).click());
}

/** Expand Scheduling nav folders and click a catalog link (client-side router nav). */
export async function navigateSchedulingPreview(
  page: Page,
  fromSlug: string,
  toLinkLabel: string,
  destinationTestId: string,
) {
  await page.goto(previewUrl(`/${fromSlug}`));
  await waitForPreviewShell(page);
  const fromWrapper = page.getByTestId(`${fromSlug}-preview`).first();
  await scrollIntoPreviewView(fromWrapper);
  await expect(fromWrapper).toBeVisible({ timeout: 10_000 });
  await clickSchedulingCatalogLink(page, toLinkLabel, destinationTestId);
}

/** Sidebar-navigate to another scheduling preview from the current page. */
export async function clickSchedulingCatalogLink(
  page: Page,
  toLinkLabel: string,
  destinationTestId: string,
) {
  const nav = page.getByTestId("preview-catalog-nav");
  const destinationLink = nav.getByRole("link", { name: toLinkLabel, exact: true });

  const schedulingSection = nav.getByRole("button", { name: "Scheduling", exact: true });
  if ((await schedulingSection.count()) > 0) {
    if ((await schedulingSection.getAttribute("aria-expanded")) !== "true") {
      await schedulingSection.click();
    }
  }

  for (let attempt = 0; attempt < 6; attempt += 1) {
    if (await destinationLink.isVisible()) {
      break;
    }
    for (const folder of ["Event Calendar", "Event Timeline", "Shared"]) {
      const folderButton = nav.getByRole("button", { name: folder, exact: true });
      if ((await folderButton.count()) > 0) {
        if ((await folderButton.getAttribute("aria-expanded")) !== "true") {
          await folderButton.click();
        }
      }
    }
  }

  await destinationLink.click();

  const toSlug = destinationTestId.replace(/-preview$/, "");
  await expect(page).toHaveURL(new RegExp(`/${toSlug.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`));

  const destination = page.getByTestId(destinationTestId).first();
  await expect(destination).toBeVisible({ timeout: 30_000 });
  await scrollIntoPreviewView(destination);
}

/** Capture document scroll height for scrollbar regression checks. */
export async function documentScrollHeight(page: Page): Promise<number> {
  return page.evaluate(() => document.documentElement.scrollHeight);
}
