import { expect, test } from "@playwright/test";
import * as fs from "node:fs";
import * as path from "node:path";
import {
  collectPreviewSlugs,
  collectReactiveWarningsForRoute,
  isOrbitalOwnedLocation,
  type ReactiveWarningReport,
} from "../_reactive_warnings";
import { previewUrl } from "../helpers";

// Dev builds use stock reactive_graph, so Leptos logs every read outside a tracking context.
// Any orbital-owned location in those client warnings fails the suite.
test.describe("reactive tracking warnings", () => {
  test("bare debug routes have no orbital-owned client warnings", async ({ page }) => {
    test.setTimeout(900_000);
    const slugs = await collectPreviewSlugs(page);
    expect(slugs.length).toBeGreaterThan(0);

    const reports: ReactiveWarningReport[] = [];
    const orbitalViolations: ReactiveWarningReport[] = [];

    for (const slug of slugs) {
      try {
        const bareWarnings = await collectReactiveWarningsForRoute(
          page,
          previewUrl(`/debug/${slug}`),
          "debug-bare-root",
        );
        const bareReport: ReactiveWarningReport = { slug, route: "bare", warnings: bareWarnings };
        reports.push(bareReport);

        const owned = bareWarnings.filter((w) => isOrbitalOwnedLocation(w.location));
        if (owned.length > 0) {
          orbitalViolations.push({ slug, route: "bare", warnings: owned });
        }
      } catch (error) {
        reports.push({
          slug,
          route: "bare",
          warnings: [
            {
              location: "navigation-error",
              raw: error instanceof Error ? error.message : String(error),
            },
          ],
        });
      }
    }

    const outDir = path.join(process.cwd(), "test-results");
    fs.mkdirSync(outDir, { recursive: true });
    fs.writeFileSync(
      path.join(outDir, "reactive-warnings-client-report.json"),
      JSON.stringify({ generatedAt: new Date().toISOString(), reports, orbitalViolations }, null, 2),
    );

    if (orbitalViolations.length > 0) {
      const summary = orbitalViolations
        .map(
          (entry) =>
            `${entry.slug}: ${entry.warnings.map((w) => w.location).join(", ")}`,
        )
        .join("\n");
      expect(orbitalViolations, `Orbital-owned client warnings:\n${summary}`).toHaveLength(0);
    }
  });

  test("attribute warnings: full catalog vs bare debug", async ({ page }) => {
    const slugs = await collectPreviewSlugs(page);
    const sample = slugs.slice(0, 12);

    const attribution: Array<{
      slug: string;
      bareOnly: string[];
      fullOnly: string[];
      both: string[];
    }> = [];

    for (const slug of sample) {
      const bare = await collectReactiveWarningsForRoute(
        page,
        previewUrl(`/debug/${slug}`),
        "debug-bare-root",
      );
      const full = await collectReactiveWarningsForRoute(
        page,
        previewUrl(`/${slug}`),
        "preview-catalog-shell",
      );

      const bareLocs = new Set(bare.map((w) => w.location));
      const fullLocs = new Set(full.map((w) => w.location));

      attribution.push({
        slug,
        bareOnly: [...bareLocs].filter((loc) => !fullLocs.has(loc)),
        fullOnly: [...fullLocs].filter((loc) => !bareLocs.has(loc)),
        both: [...bareLocs].filter((loc) => fullLocs.has(loc)),
      });
    }

    const outDir = path.join(process.cwd(), "test-results");
    fs.mkdirSync(outDir, { recursive: true });
    fs.writeFileSync(
      path.join(outDir, "reactive-warnings-attribution-sample.json"),
      JSON.stringify({ generatedAt: new Date().toISOString(), attribution }, null, 2),
    );
  });
});
