import type { Page } from "@playwright/test";

export type BootProgressSnapshot = {
  percent: number;
  currentStep: string;
  steps: Record<string, string>;
};

/** Read the boot loader progress hook exposed by OrbitalBootLoaderHeadAssets. */
export async function readBootProgress(page: Page): Promise<BootProgressSnapshot | null> {
  return page.evaluate(() => {
    const progress = (
      window as unknown as {
        __orbitalBootProgress?: BootProgressSnapshot;
      }
    ).__orbitalBootProgress;
    if (!progress) return null;
    return {
      percent: progress.percent,
      currentStep: progress.currentStep,
      steps: { ...progress.steps },
    };
  });
}

/** Poll boot progress until predicate passes or timeout. */
export async function waitForBootProgress(
  page: Page,
  predicate: (progress: BootProgressSnapshot) => boolean,
  timeoutMs = 10_000,
): Promise<BootProgressSnapshot> {
  await page.waitForFunction(
    (fnSource) => {
      const predicateFn = new Function("progress", `return (${fnSource})(progress);`) as (
        progress: BootProgressSnapshot,
      ) => boolean;
      const progress = (
        window as unknown as {
          __orbitalBootProgress?: BootProgressSnapshot;
        }
      ).__orbitalBootProgress;
      if (!progress) return false;
      return predicateFn({
        percent: progress.percent,
        currentStep: progress.currentStep,
        steps: { ...progress.steps },
      });
    },
    predicate.toString(),
    { timeout: timeoutMs },
  );

  const progress = await readBootProgress(page);
  if (!progress) {
    throw new Error("Boot progress hook missing after waitForBootProgress");
  }
  return progress;
}
