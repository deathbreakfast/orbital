// Opens the preview once so a lane's server and browser are warm before its
// first spec runs. Best effort: run-lanes ignores a failure here.
import { chromium } from "@playwright/test";

const url = process.argv[2];
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.goto(url, { timeout: 90_000 });
  await page.waitForLoadState("networkidle", { timeout: 60_000 });
  await page.locator("#orbital-boot-overlay").waitFor({ state: "detached", timeout: 60_000 });
} finally {
  await browser.close();
}
