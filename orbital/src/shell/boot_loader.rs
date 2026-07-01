//! Bootstrap loading overlay shown before WASM hydration completes.
//!
//! All markup and styles are SSR-safe (no WASM or hydrated Leptos components).
//! Call [`hide_boot_loader`] from the app `hydrate()` entrypoint immediately after
//! `leptos::mount::hydrate_body(...)`.

use leptos::prelude::*;
use orbital_core_components::feedback::dialog::dialog_styles;
use orbital_core_components::feedback::progress_bar::progress_bar_styles;
use orbital_motion::presence_styles;
use orbital_style::inject_style;
use orbital_theme::{OrbitalThemeProvider, Theme, ThemeMode};

use crate::components::{DialogBody, DialogContent, DialogTitle, MessageBar, MessageBarBody, MessageBarIntent, MessageBarLayout, MessageBarTitle, Body1};

pub(crate) const BOOT_LOADER_CSS: &str = r#"
#orbital-boot-overlay {
  position: fixed;
  inset: 0;
  z-index: 2147483646;
  display: flex;
  align-items: center;
  justify-content: center;
  background: color-mix(
    in srgb,
    var(--orb-color-text-primary, #232425) 28%,
    var(--orb-color-surface-canvas, #ffffff)
  );
  color: var(--orb-color-text-primary, #232425);
  font-family: var(--orb-type-family-sans, ui-sans-serif, system-ui, -apple-system, sans-serif);
  font-size: var(--orb-type-size-md, 16px);
  line-height: var(--orb-type-line-md, 20px);
}

#orbital-boot-overlay[data-orbital-boot-theme="dark"] {
  background: color-mix(
    in srgb,
    var(--orb-color-text-primary, #ffffff) 28%,
    var(--orb-color-surface-canvas, #141414)
  );
  color: var(--orb-color-text-primary, #ffffff);
}

html[data-orbital-hydrated="true"] #orbital-boot-overlay {
  display: none !important;
}

#orbital-boot-overlay[data-orbital-boot-exiting] {
  pointer-events: none;
}

.orbital-boot-loading {
  width: min(90vw, 32rem);
  padding: 0 var(--orb-space-inline-md, 12px);
  box-sizing: border-box;
}

html[data-orbital-boot-state="error"] .orbital-boot-loading {
  display: none !important;
}

.orbital-boot-loading .orbital-dialog-surface {
  position: relative;
  inset: unset;
  margin: 0;
  width: 100%;
  max-width: 100%;
  box-shadow: var(
    --orb-elev-modal,
    0 0 8px rgba(0, 0, 0, 0.11),
    0 32px 68px rgba(0, 0, 0, 0.13)
  );
}

.orbital-boot-progress-section {
  display: flex;
  flex-direction: column;
  gap: var(--orb-space-block-sm, 8px);
  margin-block-end: var(--orb-space-block-md, 12px);
}

.orbital-boot-progress-meta {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--orb-space-inline-md, 12px);
  margin: 0;
  color: var(--orb-color-text-secondary, #3f4345);
  font-size: var(--orb-type-size-sm, 14px);
  line-height: var(--orb-type-line-sm, 18px);
}

.orbital-boot-progress-meta__percent {
  flex: 1 1 auto;
  min-width: 0;
}

.orbital-boot-progress-meta__elapsed {
  flex: 0 0 auto;
  min-width: 4.5rem;
  text-align: right;
  font-variant-numeric: tabular-nums;
  font-family: var(--orb-type-family-monospace, ui-monospace, monospace);
}

.orbital-boot-steps {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--orb-space-block-sm, 8px);
}

.orbital-boot-step {
  display: flex;
  align-items: center;
  gap: var(--orb-space-inline-sm, 8px);
  margin: 0;
  color: var(--orb-color-text-secondary, #3f4345);
  font-size: var(--orb-type-size-sm, 14px);
  line-height: var(--orb-type-line-sm, 18px);
}

.orbital-boot-step__label {
  flex: 1 1 auto;
  min-width: 0;
}

.orbital-boot-step__duration {
  flex: 0 0 auto;
  min-width: 4.5rem;
  text-align: right;
  font-variant-numeric: tabular-nums;
  font-family: var(--orb-type-family-monospace, ui-monospace, monospace);
  color: var(--orb-color-text-secondary, #3f4345);
}

.orbital-boot-step__icon {
  flex-shrink: 0;
  width: 1rem;
  height: 1rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.orbital-boot-step--pending .orbital-boot-step__icon::before {
  content: "";
  width: 0.5rem;
  height: 0.5rem;
  border-radius: var(--orb-radius-circular, 10000px);
  border: var(--orb-stroke-thin, 1px) solid var(--orb-color-border-subtle, #dfe0e1);
  box-sizing: border-box;
}

.orbital-boot-step--active {
  color: var(--orb-color-text-primary, #232425);
}

.orbital-boot-step--active .orbital-boot-step__icon::before {
  content: "";
  width: 0.75rem;
  height: 0.75rem;
  border-radius: var(--orb-radius-circular, 10000px);
  border: var(--orb-stroke-thick, 2px) solid var(--orb-color-border-subtle, #dfe0e1);
  border-top-color: var(--orb-color-brand-bg, #1a6f94);
  animation: orbital-boot-spin 0.9s linear infinite;
  box-sizing: border-box;
}

.orbital-boot-step--complete {
  color: var(--orb-color-text-primary, #232425);
}

.orbital-boot-step--complete .orbital-boot-step__icon::before {
  content: "✓";
  font-size: 0.75rem;
  line-height: 1;
  color: var(--orb-color-palette-green-bg, #107c10);
}

.orbital-boot-step--error {
  color: var(--orb-color-palette-red-bg, #c50f1f);
}

.orbital-boot-error {
  display: none;
  width: min(90vw, 32rem);
  padding: 0 var(--orb-space-inline-md, 12px);
  box-sizing: border-box;
}

html[data-orbital-boot-state="error"] .orbital-boot-error {
  display: block !important;
}

.orbital-boot-error .orbital-dialog-surface {
  position: relative;
  inset: unset;
  margin: 0;
  width: 100%;
  max-width: 100%;
  box-shadow: var(
    --orb-elev-modal,
    0 0 8px rgba(0, 0, 0, 0.11),
    0 32px 68px rgba(0, 0, 0, 0.13)
  );
}

.orbital-boot-error .orbital-message-bar {
  margin-block-end: var(--orb-space-block-md, 12px);
}

@keyframes orbital-boot-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .orbital-boot-step--active .orbital-boot-step__icon::before {
    animation: none;
    border-top-color: var(--orb-color-brand-bg, #1a6f94);
    opacity: 0.85;
  }
}
"#;

const BOOT_LOADER_SCRIPT: &str = r#"
(function () {
  var STEP_IDS = ["theme", "scripts", "styles", "wasm", "hydrate"];
  var stepStates = {};
  var bootFailed = false;
  var wasmAssets = {};
  var wasmCompleted = {};
  var headObserver = null;
  var bootExiting = false;
  var bootStartTime = performance.now();
  var stepStartedAt = {};
  var stepElapsedMs = {};
  var timingTimer = null;

  window.__orbitalBootExiting = false;

  STEP_IDS.forEach(function (id) {
    stepStates[id] = "pending";
  });

  function isHydrated() {
    return document.documentElement.getAttribute("data-orbital-hydrated") === "true";
  }

  function getCurrentStep() {
    for (var i = 0; i < STEP_IDS.length; i++) {
      var state = stepStates[STEP_IDS[i]];
      if (state === "active" || state === "pending") {
        return STEP_IDS[i];
      }
    }
    return STEP_IDS[STEP_IDS.length - 1];
  }

  function computePercent() {
    var total = STEP_IDS.length;
    var completed = 0;
    for (var i = 0; i < STEP_IDS.length; i++) {
      var id = STEP_IDS[i];
      if (id === "wasm") {
        if (stepStates.wasm === "complete") {
          completed += 1;
        } else {
          var loaded = 0;
          var totalBytes = 0;
          for (var url in wasmAssets) {
            if (!Object.prototype.hasOwnProperty.call(wasmAssets, url)) continue;
            loaded += wasmAssets[url].loaded || 0;
            totalBytes += wasmAssets[url].total || 0;
          }
          if (totalBytes > 0) {
            completed += Math.min(1, loaded / totalBytes);
          } else {
            var expected = 0;
            var done = 0;
            for (var wasmUrl in wasmCompleted) {
              if (!Object.prototype.hasOwnProperty.call(wasmCompleted, wasmUrl)) continue;
              expected += 1;
              if (wasmCompleted[wasmUrl]) done += 1;
            }
            if (expected > 0) {
              completed += done / expected;
            }
          }
        }
      } else if (stepStates[id] === "complete") {
        completed += 1;
      }
    }
    return Math.min(100, Math.max(0, Math.round((completed / total) * 100)));
  }

  function formatDuration(ms) {
    if (!isFinite(ms) || ms < 0) {
      return "—";
    }
    if (ms < 1000) {
      return Math.round(ms) + "ms";
    }
    var totalSeconds = Math.round(ms / 1000);
    var minutes = Math.floor(totalSeconds / 60);
    var seconds = totalSeconds % 60;
    if (minutes > 0) {
      return minutes + "m" + seconds + "s";
    }
    return totalSeconds + "s";
  }

  function noteStepActive(id) {
    if (!stepStartedAt[id]) {
      stepStartedAt[id] = performance.now();
    }
  }

  function noteStepComplete(id) {
    if (stepStartedAt[id] && stepElapsedMs[id] == null) {
      stepElapsedMs[id] = performance.now() - stepStartedAt[id];
    }
  }

  function stepDurationText(id) {
    var state = stepStates[id];
    if (state === "pending") {
      return "—";
    }
    if (state === "complete" && stepElapsedMs[id] != null) {
      return formatDuration(stepElapsedMs[id]);
    }
    if (state === "active" && stepStartedAt[id]) {
      return formatDuration(performance.now() - stepStartedAt[id]);
    }
    return "—";
  }

  function updateTimingDom() {
    var elapsedEl = document.querySelector(
      '[data-testid="orbital-boot-progress-elapsed"]'
    );
    if (elapsedEl) {
      elapsedEl.textContent = formatDuration(performance.now() - bootStartTime);
    }
    for (var i = 0; i < STEP_IDS.length; i++) {
      var id = STEP_IDS[i];
      var row = document.querySelector('[data-step-id="' + id + '"]');
      if (!row) continue;
      var durationEl = row.querySelector(".orbital-boot-step__duration");
      if (durationEl) {
        durationEl.textContent = stepDurationText(id);
      }
    }
  }

  function startTimingTicker() {
    if (timingTimer) return;
    timingTimer = setInterval(function () {
      if (isHydrated() || bootExiting || bootFailed) {
        clearInterval(timingTimer);
        timingTimer = null;
        return;
      }
      updateTimingDom();
    }, 100);
  }

  function updateStepDom(id, state) {
    var row = document.querySelector('[data-step-id="' + id + '"]');
    if (!row) return;
    row.classList.remove(
      "orbital-boot-step--pending",
      "orbital-boot-step--active",
      "orbital-boot-step--complete",
      "orbital-boot-step--error"
    );
    row.classList.add("orbital-boot-step--" + state);
  }

  function updateProgressDom(percent) {
    var bar = document.querySelector('[data-testid="orbital-boot-progress-bar"]');
    var fill = bar && bar.querySelector(".orbital-progress-bar__bar");
    var label = document.querySelector('[data-testid="orbital-boot-progress-label"]');
    if (fill) {
      fill.style.width = percent + "%";
    }
    if (bar) {
      bar.setAttribute("aria-valuenow", String(percent));
    }
    if (label) {
      label.textContent = percent + "%";
    }
    updateTimingDom();
  }

  function publishProgress() {
    var percent = computePercent();
    window.__orbitalBootProgress = {
      percent: percent,
      currentStep: getCurrentStep(),
      steps: Object.assign({}, stepStates),
    };
    updateProgressDom(percent);
  }

  function setStepState(id, state) {
    if (state === "active") {
      noteStepActive(id);
    } else if (state === "complete") {
      noteStepComplete(id);
    }
    stepStates[id] = state;
    updateStepDom(id, state);
    publishProgress();
  }

  function activateStep(id) {
    if (stepStates[id] === "complete" || stepStates[id] === "error") return;
    setStepState(id, "active");
  }

  function completeStep(id) {
    if (stepStates[id] === "complete") return;
    setStepState(id, "complete");
    for (var i = 0; i < STEP_IDS.length; i++) {
      if (stepStates[STEP_IDS[i]] === "pending") {
        var next = STEP_IDS[i];
        activateStep(next);
        if (next === "scripts") {
          trackScripts();
        } else if (next === "styles") {
          trackStyles();
        } else if (next === "wasm") {
          ensureWasmTracking();
        }
        break;
      }
    }
    publishProgress();
  }

  window.__orbitalBootCompleteStep = function (id) {
    completeStep(id || "hydrate");
  };

  function showOrbitalBootError() {
    if (isHydrated()) return;

    var overlay = document.getElementById("orbital-boot-overlay");
    if (overlay) {
      overlay.setAttribute("aria-busy", "false");
      var err = overlay.querySelector('[data-testid="orbital-boot-error"]');
      if (err) {
        err.removeAttribute("hidden");
      }
    }

    if (bootFailed) return;
    bootFailed = true;
    document.documentElement.setAttribute("data-orbital-boot-state", "error");
    for (var i = 0; i < STEP_IDS.length; i++) {
      if (stepStates[STEP_IDS[i]] !== "complete") {
        setStepState(STEP_IDS[i], "error");
      }
    }
  }

  function resourceAlreadyLoaded(node) {
    if (node.tagName === "LINK" && node.rel === "stylesheet") {
      return !!node.sheet;
    }
    if (node.tagName === "SCRIPT") {
      if (!node.src) {
        return true;
      }
      // Module scripts do not expose readyState reliably — always attach listeners.
      if (node.type === "module") {
        return false;
      }
      return node.readyState === "complete" || node.readyState === "loaded";
    }
    return false;
  }

  function waitForResources(selectors, onComplete, onError) {
    var nodes = Array.prototype.slice.call(document.querySelectorAll(selectors));
    if (nodes.length === 0) {
      onComplete();
      return;
    }
    var remaining = 0;
    nodes.forEach(function (node) {
      if (resourceAlreadyLoaded(node)) {
        return;
      }
      remaining += 1;
      var settled = false;
      function finish() {
        if (settled) return;
        settled = true;
        remaining -= 1;
        if (remaining <= 0) onComplete();
      }
      node.addEventListener("load", finish, { once: true });
      node.addEventListener(
        "error",
        function () {
          onError();
        },
        { once: true }
      );
      // Module scripts may finish before listeners attach.
      if (node.tagName === "SCRIPT" && node.type === "module" && node.src) {
        try {
          var entries = performance.getEntriesByName(node.src, "resource");
          if (entries.length > 0 && entries[0].responseEnd > 0) {
            finish();
          }
        } catch (e) {}
      }
    });
    if (remaining === 0) {
      onComplete();
    }
  }

  function trackTheme() {
    activateStep("theme");
    waitForResources(
      'link[data-orbital-theme-baseline][rel="stylesheet"]',
      function () {
        if (stepStates.theme === "active") {
          completeStep("theme");
        }
      },
      showOrbitalBootError
    );
  }

  function trackScripts() {
    waitForResources(
      'script[type="module"]:not([data-orbital-boot-loader])',
      function () {
        if (stepStates.scripts === "active") {
          completeStep("scripts");
        }
      },
      showOrbitalBootError
    );
  }

  function trackStyles() {
    waitForResources(
      'link[rel="stylesheet"]:not([data-orbital-theme-baseline])',
      function () {
        if (stepStates.styles === "active") {
          completeStep("styles");
        }
      },
      showOrbitalBootError
    );
  }

  function registerWasmUrl(url) {
    if (!url || wasmAssets[url]) return;
    wasmAssets[url] = { loaded: 0, total: 0 };
  }

  function updateWasmBytes(url, loaded, total) {
    if (!wasmAssets[url]) {
      registerWasmUrl(url);
    }
    wasmAssets[url].loaded = loaded;
    if (total > 0) {
      wasmAssets[url].total = total;
    }
    if (stepStates.wasm === "pending" || stepStates.wasm === "active") {
      publishProgress();
    }
  }

  function onWasmFetchComplete(url) {
    wasmCompleted[url] = true;
    if (wasmAssets[url]) {
      var total = wasmAssets[url].total || wasmAssets[url].loaded;
      wasmAssets[url].loaded = total;
      wasmAssets[url].total = total;
    }
    maybeCompleteWasm();
    publishProgress();
  }

  function maybeCompleteWasm() {
    if (stepStates.wasm !== "active") return;
    var urls = Object.keys(wasmAssets);
    if (urls.length === 0) return;
    for (var i = 0; i < urls.length; i++) {
      if (!wasmCompleted[urls[i]]) return;
    }
    completeStep("wasm");
  }

  function discoverWasmPreloads() {
    var links = document.querySelectorAll('link[rel="preload"][href*=".wasm"]');
    links.forEach(function (link) {
      registerWasmUrl(link.href);
    });
  }

  function discoverSplitManifest() {
    var manifestLinks = document.querySelectorAll(
      'link[href*="wasm_split_manifest"], link[href*="__wasm_split_manifest"]'
    );
    if (manifestLinks.length === 0) return;
    var manifestUrl = manifestLinks[0].href;
    if (!manifestUrl) return;
    window
      .fetch(manifestUrl)
      .then(function (response) {
        if (!response.ok) return null;
        return response.json();
      })
      .then(function (manifest) {
        if (!manifest || typeof manifest !== "object") return;
        var pkgBase = manifestUrl.replace(/[^/]+$/, "");
        Object.keys(manifest).forEach(function (key) {
          var value = manifest[key];
          if (typeof value === "string" && /\.wasm(\?|$)/i.test(value)) {
            registerWasmUrl(value.indexOf("/") === 0 ? value : pkgBase + value);
          }
        });
        maybeCompleteWasm();
      })
      .catch(function () {});
  }

  function ensureWasmTracking() {
    if (stepStates.wasm === "complete") return;
    activateStep("wasm");
    discoverWasmPreloads();
    discoverSplitManifest();
    maybeCompleteWasm();
  }

  var originalFetch = window.fetch;
  window.fetch = function (input, init) {
    var url =
      typeof input === "string"
        ? input
        : input && typeof input.url === "string"
          ? input.url
          : "";
    if (!isHydrated() && /\.wasm(\?|$)/i.test(url)) {
      registerWasmUrl(url);
      if (stepStates.styles === "complete" && stepStates.wasm !== "complete") {
        activateStep("wasm");
      }
      return originalFetch.apply(this, arguments).then(function (response) {
        if (!response.ok) {
          showOrbitalBootError();
          return response;
        }
        var contentLength = parseInt(response.headers.get("Content-Length") || "0", 10);
        if (contentLength > 0 && response.body && response.body.getReader) {
          var reader = response.body.getReader();
          var loaded = 0;
          wasmAssets[url] = wasmAssets[url] || { loaded: 0, total: contentLength };
          wasmAssets[url].total = contentLength;
          var stream = new ReadableStream({
            pull: function (controller) {
              return reader.read().then(function (result) {
                if (result.done) {
                  controller.close();
                  onWasmFetchComplete(url);
                  return;
                }
                loaded += result.value.byteLength;
                updateWasmBytes(url, loaded, contentLength);
                controller.enqueue(result.value);
              });
            },
          });
          return new Response(stream, {
            status: response.status,
            statusText: response.statusText,
            headers: response.headers,
          });
        }
        return response.arrayBuffer().then(function (buffer) {
          updateWasmBytes(url, buffer.byteLength, buffer.byteLength);
          onWasmFetchComplete(url);
          return new Response(buffer, {
            status: response.status,
            statusText: response.statusText,
            headers: response.headers,
          });
        });
      }).catch(function (err) {
        showOrbitalBootError();
        throw err;
      });
    }
    return originalFetch.apply(this, arguments);
  };

  function observeHead() {
    if (headObserver || !document.head) return;
    headObserver = new MutationObserver(function () {
      if (stepStates.theme === "active") {
        waitForResources(
          'link[data-orbital-theme-baseline][rel="stylesheet"]',
          function () {
            if (stepStates.theme === "active") completeStep("theme");
          },
          showOrbitalBootError
        );
      }
      if (stepStates.scripts === "active") {
        waitForResources(
          'script[type="module"]:not([data-orbital-boot-loader])',
          function () {
            if (stepStates.scripts === "active") completeStep("scripts");
          },
          showOrbitalBootError
        );
      }
      if (stepStates.styles === "active") {
        waitForResources(
          'link[rel="stylesheet"]:not([data-orbital-theme-baseline])',
          function () {
            if (stepStates.styles === "active") completeStep("styles");
          },
          showOrbitalBootError
        );
      }
      discoverWasmPreloads();
    });
    headObserver.observe(document.head, { childList: true, subtree: true });
  }

  function prefersReducedMotion() {
    return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  }

  function applyMotionExitVars(el) {
    if (!el) return;
    el.style.setProperty("--orbital-motion-exit-duration", "200ms");
    el.style.setProperty(
      "--orbital-motion-exit-curve",
      "cubic-bezier(0.33, 0, 0.67, 1)"
    );
  }

  function finalizeDismiss() {
    if (isHydrated()) return;
    if (timingTimer) {
      clearInterval(timingTimer);
      timingTimer = null;
    }
    document.documentElement.setAttribute("data-orbital-hydrated", "true");
    window.__orbitalBootExiting = false;
    bootExiting = false;
    var overlay = document.getElementById("orbital-boot-overlay");
    if (overlay) {
      overlay.remove();
    }
  }

  window.__orbitalBootDismissOverlay = function () {
    if (isHydrated() || bootExiting) return;
    if (document.documentElement.getAttribute("data-orbital-boot-state") === "error") {
      return;
    }

    var overlay = document.getElementById("orbital-boot-overlay");
    if (!overlay) {
      finalizeDismiss();
      return;
    }

    if (prefersReducedMotion()) {
      finalizeDismiss();
      return;
    }

    bootExiting = true;
    window.__orbitalBootExiting = true;
    overlay.setAttribute("data-orbital-boot-exiting", "");
    overlay.setAttribute("aria-busy", "false");

    var dialog = overlay.querySelector(".orbital-boot-loading .orbital-dialog-surface");
    applyMotionExitVars(overlay);
    applyMotionExitVars(dialog);

    overlay.classList.add("orbital-motion-respects-reduced");
    overlay.classList.add("orbital-motion-fade-leave-active");
    if (dialog) {
      dialog.classList.add("orbital-motion-fade-scale-leave-active");
    }

    void overlay.offsetWidth;

    overlay.classList.add("orbital-motion-fade-leave-to");
    if (dialog) {
      dialog.classList.add("orbital-motion-fade-scale-leave-to");
    }

    var finished = false;
    function finishExit() {
      if (finished) return;
      finished = true;
      overlay.removeEventListener("transitionend", onTransitionEnd);
      clearTimeout(fallbackTimer);
      finalizeDismiss();
    }

    function onTransitionEnd(event) {
      if (event.target !== overlay) return;
      if (event.propertyName !== "opacity") return;
      finishExit();
    }

    var fallbackTimer = setTimeout(finishExit, 350);
    overlay.addEventListener("transitionend", onTransitionEnd);
  };

  function bootInit() {
    bootStartTime = performance.now();
    startTimingTicker();
    window.__orbitalBootProgress = {
      percent: 0,
      currentStep: "theme",
      steps: Object.assign({}, stepStates),
    };
    observeHead();
    trackTheme();
    publishProgress();
    if (bootFailed) {
      showOrbitalBootError();
    }
  }

  window.addEventListener(
    "error",
    function (event) {
      var target = event.target;
      if (target && (target.tagName === "SCRIPT" || target.tagName === "LINK")) {
        showOrbitalBootError();
      }
    },
    true
  );

  window.addEventListener("unhandledrejection", function (event) {
    if (isHydrated()) return;
    var reason = event.reason;
    var message = (reason && (reason.message || String(reason))) || "";
    if (/wasm|import|orbital-preview|fetch|unreachable/i.test(message)) {
      showOrbitalBootError();
    }
  });

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", bootInit);
  } else {
    bootInit();
  }
})();
"#;

const DEFAULT_BOOT_TITLE: &str = "Loading application";

const BOOT_STEPS: &[(&str, &str)] = &[
    ("theme", "Theme and fonts"),
    ("scripts", "Application scripts"),
    ("styles", "Application styles"),
    ("wasm", "WebAssembly"),
    ("hydrate", "Starting application"),
];

/// Inline critical CSS and a WASM/script load error listener for the boot overlay.
///
/// Wire into the document `<head>` after [`super::OrbitalFirstPaintHeadAssets`] and
/// **before** `<HydrationScripts>`.
#[component]
pub fn OrbitalBootLoaderHeadAssets() -> impl IntoView {
    view! {
        <style data-orbital-boot-loader="">
            {BOOT_LOADER_CSS}
        </style>
        <style data-orbital-boot-motion="">
            {presence_styles()}
        </style>
        <script data-orbital-boot-loader="">
            {BOOT_LOADER_SCRIPT}
        </script>
    }
}

/// SSR-safe loading modal with progress bar and boot step checklist.
#[component]
pub(crate) fn OrbitalBootLoadingPanel(
    #[prop(into)] title: String,
    /// Light or dark token set for the static dialog surface (defaults to light).
    #[prop(default = ThemeMode::Light)]
    theme_mode: ThemeMode,
) -> impl IntoView {
    inject_style("orbital-dialog", dialog_styles());
    inject_style("orbital-progress-bar", progress_bar_styles());

    let theme = RwSignal::new(Theme::for_mode(theme_mode));
    let aria_label = title.clone();

    view! {
        <div class="orbital-boot-loading" data-testid="orbital-boot-loading">
            <OrbitalThemeProvider theme=theme>
                <div
                    class="orbital-dialog-surface"
                    role="dialog"
                    aria-modal="true"
                    aria-label=aria_label
                >
                    <DialogBody>
                        <DialogTitle>{title}</DialogTitle>
                        <DialogContent>
                            <div class="orbital-boot-progress-section">
                                <div
                                    class="orbital-progress-bar"
                                    data-testid="orbital-boot-progress-bar"
                                    role="progressbar"
                                    aria-valuemin="0"
                                    aria-valuemax="100"
                                    aria-valuenow="0"
                                >
                                    <div class="orbital-progress-bar__bar" style="width: 0%;"></div>
                                </div>
                                <div class="orbital-boot-progress-meta">
                                    <span
                                        class="orbital-boot-progress-meta__percent"
                                        data-testid="orbital-boot-progress-label"
                                    >
                                        "0%"
                                    </span>
                                    <span
                                        class="orbital-boot-progress-meta__elapsed"
                                        data-testid="orbital-boot-progress-elapsed"
                                    >
                                        "0ms"
                                    </span>
                                </div>
                            </div>
                            <ul class="orbital-boot-steps" data-testid="orbital-boot-steps">
                                {BOOT_STEPS
                                    .iter()
                                    .map(|&(id, label)| {
                                        let test_id = format!("orbital-boot-step-{id}");
                                        view! {
                                            <li
                                                class="orbital-boot-step orbital-boot-step--pending"
                                                data-step-id=id
                                                data-testid=test_id
                                            >
                                                <span
                                                    class="orbital-boot-step__icon"
                                                    aria-hidden="true"
                                                ></span>
                                                <span class="orbital-boot-step__label">{label}</span>
                                                <span class="orbital-boot-step__duration">"—"</span>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </DialogContent>
                    </DialogBody>
                </div>
            </OrbitalThemeProvider>
        </div>
    }
}

/// SSR-safe error dialog content for preview demos and the boot overlay error panel.
#[component]
pub(crate) fn OrbitalBootErrorContent(
    /// Light or dark token set for the static dialog surface (defaults to light).
    #[prop(default = ThemeMode::Light)]
    theme_mode: ThemeMode,
) -> impl IntoView {
    inject_style("orbital-dialog", dialog_styles());

    let theme = RwSignal::new(Theme::for_mode(theme_mode));

    view! {
        <OrbitalThemeProvider theme=theme>
            <div
                class="orbital-dialog-surface"
                role="alertdialog"
                aria-modal="true"
                aria-label="Unable to load application"
            >
                <DialogBody>
                    <DialogTitle>"Unable to load application"</DialogTitle>
                    <DialogContent>
                        <MessageBar
                            intent=MessageBarIntent::Error
                            layout=MessageBarLayout::Multiline
                        >
                            <MessageBarTitle>"Startup failed"</MessageBarTitle>
                            <MessageBarBody>
                                "The application bundle could not be downloaded or started."
                            </MessageBarBody>
                        </MessageBar>
                        <Body1 block=true>
                            "Refresh the page or try again later. If the problem persists, check your network connection."
                        </Body1>
                    </DialogContent>
                </DialogBody>
            </div>
        </OrbitalThemeProvider>
    }
}

/// SSR-safe modal-style error panel using dialog layout primitives and [`MessageBar`].
///
/// [`Dialog`] itself requires hydration (portal/focus trap), so this panel composes
/// [`DialogBody`], [`DialogTitle`], and [`DialogContent`] inside a static dialog surface.
#[component]
fn OrbitalBootErrorPanel(
    #[prop(default = ThemeMode::Light)] theme_mode: ThemeMode,
) -> impl IntoView {
    view! {
        <div
            class="orbital-boot-error"
            data-testid="orbital-boot-error"
            hidden
            role="alert"
            aria-live="assertive"
        >
            <OrbitalBootErrorContent theme_mode=theme_mode />
        </div>
    }
}

/// Full-viewport loading overlay rendered before the hydrated app tree.
///
/// Wire into the document `<body>` **after** the app root (`{app_fn()}`).
/// DOM order must keep the hydrated app as the first body child; fixed positioning
/// still covers the viewport until [`hide_boot_loader`] runs.
#[component]
pub fn OrbitalBootOverlay(
    /// Title shown in the loading modal dialog.
    #[prop(optional, into)]
    message: Option<String>,
    /// Light or dark appearance for the loading and error panels (defaults to light).
    #[prop(default = ThemeMode::Light)]
    theme_mode: ThemeMode,
) -> impl IntoView {
    let title = message.unwrap_or_else(|| DEFAULT_BOOT_TITLE.to_string());
    let boot_theme = theme_mode.as_name();

    view! {
        <div
            id="orbital-boot-overlay"
            data-testid="orbital-boot-overlay"
            data-orbital-boot-theme=boot_theme
            role="status"
            aria-live="polite"
            aria-busy="true"
        >
            <OrbitalBootLoadingPanel title=title theme_mode=theme_mode />
            <OrbitalBootErrorPanel theme_mode=theme_mode />
        </div>
    }
}

/// Hides the bootstrap loading overlay after hydration completes.
///
/// Completes the hydrate boot step, then runs the exit fade via
/// `__orbitalBootDismissOverlay` (sets `data-orbital-hydrated` after the animation).
/// Call immediately after `leptos::mount::hydrate_body(...)` in every app `hydrate()` entrypoint.
#[cfg(feature = "hydrate")]
pub fn hide_boot_loader() {
    use js_sys::{Function, Reflect};
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::window;

    let Some(window) = window() else {
        return;
    };

    if let Ok(value) = Reflect::get(&window, &JsValue::from_str("__orbitalBootCompleteStep")) {
        if let Ok(func) = value.dyn_into::<Function>() {
            let _ = func.call1(&window, &JsValue::from_str("hydrate"));
        }
    }

    if let Ok(value) = Reflect::get(&window, &JsValue::from_str("__orbitalBootDismissOverlay")) {
        if let Ok(func) = value.dyn_into::<Function>() {
            let _ = func.call0(&window);
            return;
        }
    }

    // Fallback when boot script did not load (e.g. tests without overlay).
    let Some(document) = window.document() else {
        return;
    };
    if let Some(html) = document.document_element() {
        let _ = html.set_attribute("data-orbital-hydrated", "true");
    }
    if let Some(overlay) = document.get_element_by_id("orbital-boot-overlay") {
        let _ = overlay.remove();
    }
}

/// No-op when the `hydrate` feature is disabled (SSR-only builds).
#[cfg(not(feature = "hydrate"))]
pub fn hide_boot_loader() {}

#[cfg(test)]
mod tests {
    use super::{BOOT_LOADER_CSS, BOOT_LOADER_SCRIPT, BOOT_STEPS};

    #[test]
    fn boot_loader_css_contains_required_selectors() {
        assert!(BOOT_LOADER_CSS.contains("#orbital-boot-overlay"));
        assert!(BOOT_LOADER_CSS.contains("data-orbital-hydrated"));
        assert!(BOOT_LOADER_CSS.contains(".orbital-boot-loading"));
        assert!(BOOT_LOADER_CSS.contains("data-orbital-boot-exiting"));
        assert!(BOOT_LOADER_CSS.contains(".orbital-boot-step--active"));
        assert!(BOOT_LOADER_CSS.contains("prefers-reduced-motion"));
        assert!(BOOT_LOADER_CSS.contains("data-orbital-boot-theme=\"dark\""));
        assert!(BOOT_LOADER_CSS.contains(".orbital-boot-progress-meta"));
        assert!(BOOT_LOADER_CSS.contains(".orbital-boot-step__duration"));
        assert!(BOOT_LOADER_SCRIPT.contains("formatDuration"));
        assert!(BOOT_LOADER_SCRIPT.contains("orbital-boot-progress-elapsed"));
    }

    #[test]
    fn boot_loader_script_exports_progress_hook() {
        assert!(BOOT_LOADER_SCRIPT.contains("__orbitalBootProgress"));
        assert!(BOOT_LOADER_SCRIPT.contains("__orbitalBootCompleteStep"));
        assert!(BOOT_LOADER_SCRIPT.contains("__orbitalBootDismissOverlay"));
        assert!(BOOT_LOADER_SCRIPT.contains("data-orbital-boot-exiting"));
        assert!(BOOT_LOADER_SCRIPT.contains("orbital-motion-fade-leave-active"));
        assert!(BOOT_LOADER_SCRIPT.contains("orbital-motion-fade-scale-leave-active"));
        assert!(BOOT_LOADER_SCRIPT.contains("wasm_split_manifest"));
    }

    #[test]
    fn boot_steps_cover_boot_phases() {
        assert_eq!(BOOT_STEPS.len(), 5);
        assert_eq!(BOOT_STEPS[0].0, "theme");
        assert_eq!(BOOT_STEPS[4].0, "hydrate");
    }
}
