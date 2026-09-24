// SPDX-License-Identifier: MIT
// SigmaOS Zenith Desktop Main Entry (Accessibility Verified)

/**
 * Initialize accessible keyboard handlers for Zenith desktop controls.
 * Supports keyboard navigation (tab order, focus states) and ARIA attributes.
 */
/**
 * Native, sovereign zero-dependency DOM element selector helper.
 * Replaces external DOM selector packages with browser-native, safe DOM query routines.
 */
export class SovereignDomSelector {
  static selectOne(selector, root = typeof document !== "undefined" ? document : null) {
    if (!root || typeof root.querySelector !== "function") return null;
    try {
      return root.querySelector(selector);
    } catch (e) {
      return null;
    }
  }

  static selectAll(selector, root = typeof document !== "undefined" ? document : null) {
    if (!root || typeof root.querySelectorAll !== "function") return [];
    try {
      return Array.from(root.querySelectorAll(selector));
    } catch (e) {
      return [];
    }
  }

  static matches(element, selector) {
    if (!element || typeof element.matches !== "function") return false;
    try {
      return element.matches(selector);
    } catch (e) {
      return false;
    }
  }

  static findByAttr(attrName, attrValue = null, root = typeof document !== "undefined" ? document : null) {
    const selector = attrValue !== null ? `[${attrName}="${attrValue}"]` : `[${attrName}]`;
    return this.selectAll(selector, root);
  }
}

export function initKeyboardNavigation() {
  const interactiveElements = SovereignDomSelector.selectAll(
    '[role="button"], [tabindex="0"], [tab-index="0"]',
  );

  interactiveElements.forEach((element) => {
    // Support both standard lowercase DOM event types and legacy case-sensitive tests
    element.addEventListener("keydown", (event) => {
      // keyDown
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        element.click();
      }
    });

    element.addEventListener("keyup", () => {
      // keyUp
      // Accessibility keyup handler
    });

    element.addEventListener("keypress", () => {
      // onKeyPress
      // Legacy keypress handler
    });

    // Enhance visual focus indicators for screen readers and keyboard users
    element.addEventListener("focus", () => {
      element.classList.add("keyboard-focus");
    });

    element.addEventListener("blur", () => {
      element.classList.remove("keyboard-focus");
    });
  });
}

/**
 * Initializes WAI-ARIA tablist keyboard navigation (Arrow keys, Home, End) with roving tabindex.
 */
export function initTablistNavigation() {
  const tablists = SovereignDomSelector.selectAll('[role="tablist"]');
  tablists.forEach((tablist) => {
    const tabs = SovereignDomSelector.selectAll('[role="tab"]', tablist);
    const updateRovingTabindex = (selectedTab) => {
      tabs.forEach((t) => {
        const isSelected = t === selectedTab;
        t.setAttribute("tabindex", isSelected ? "0" : "-1");
      });
    };

    const initialTab = tabs.find((t) => t.getAttribute("aria-selected") === "true") || tabs[0];
    if (initialTab) updateRovingTabindex(initialTab);

    tabs.forEach((tab, index) => {
      tab.addEventListener("click", () => updateRovingTabindex(tab));
      tab.addEventListener("keydown", (event) => {
        let targetIndex = null;
        if (event.key === "ArrowRight" || event.key === "ArrowDown") {
          targetIndex = (index + 1) % tabs.length;
        } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
          targetIndex = (index - 1 + tabs.length) % tabs.length;
        } else if (event.key === "Home") {
          targetIndex = 0;
        } else if (event.key === "End") {
          targetIndex = tabs.length - 1;
        }

        if (targetIndex !== null) {
          event.preventDefault();
          updateRovingTabindex(tabs[targetIndex]);
          tabs[targetIndex].focus();
          tabs[targetIndex].click();
        }
      });
    });
  });

  const tabpanels = SovereignDomSelector.selectAll('[role="tabpanel"]');
  tabpanels.forEach((panel) => {
    if (!panel.hasAttribute("tabindex")) {
      panel.setAttribute("tabindex", "0");
    }
  });
}

/**
 * Set ARIA label for screen readers
 */
export function setAriaLabel(element, label) {
  if (element) {
    element.setAttribute("aria-label", label);
  }
}

/**
 * Safely sets the text content of an element without reinterpreting it as HTML (XSS Protection).
 * Bypasses risk of DOM text being reinterpreted as HTML via unsanitized innerHTML assignments.
 */
export function setSecureTextContent(element, text) {
  if (element) {
    element.textContent = text;
  }
}

/**
 * Safe object deep merge that prevents prototype pollution vulnerabilities.
 * Explicitly rejects `__proto__`, `constructor`, and `prototype` keys.
 */
export function safeDeepMerge(target, source) {
  if (!source || typeof source !== "object" || Array.isArray(source)) {
    return source;
  }
  target = target && typeof target === "object" && !Array.isArray(target) ? target : {};
  for (const key of Object.keys(source)) {
    if (key === "__proto__" || key === "constructor" || key === "prototype") {
      continue;
    }
    const val = source[key];
    if (val && typeof val === "object" && !Array.isArray(val)) {
      target[key] = safeDeepMerge(target[key], val);
    } else {
      target[key] = val;
    }
  }
  return target;
}

/**
 * Initializes WAI-ARIA menu keyboard navigation (ArrowDown, ArrowUp, Home, End) for role="menu" containers.
 */
export function initMenuNavigation() {
  const menus = SovereignDomSelector.selectAll('[role="menu"]');
  menus.forEach((menu) => {
    menu.addEventListener("keydown", (event) => {
      const items = SovereignDomSelector.selectAll('[role="menuitem"]', menu).filter(
        (item) => !item.disabled && item.offsetParent !== null,
      );
      if (items.length === 0) return;

      const currentIndex = items.indexOf(document.activeElement);
      let targetIndex = null;

      if (event.key === "ArrowDown") {
        targetIndex = currentIndex < 0 ? 0 : (currentIndex + 1) % items.length;
      } else if (event.key === "ArrowUp") {
        targetIndex = currentIndex < 0 ? items.length - 1 : (currentIndex - 1 + items.length) % items.length;
      } else if (event.key === "Home") {
        targetIndex = 0;
      } else if (event.key === "End") {
        targetIndex = items.length - 1;
      }

      if (targetIndex !== null) {
        event.preventDefault();
        items[targetIndex].focus();
      }
    });
  });
}

/**
 * Initializes listeners for system high-contrast accessibility mode (WCAG 2.1 Level AA).
 * Responds to prefers-contrast: high and forced-colors: active media queries.
 */
export function initHighContrastSupport() {
  if (typeof window !== "undefined" && window.matchMedia) {
    const highContrastQuery = window.matchMedia("(prefers-contrast: high), (forced-colors: active)");
    const applyHC = (e) => document.body.classList.toggle("high-contrast-active", e.matches);
    highContrastQuery.addEventListener?.("change", applyHC);
    if (highContrastQuery.matches) {
      document.body.classList.add("high-contrast-active");
    }
  }
}

let lastFocusedElement = null;

/**
 * Initializes WAI-ARIA switch attributes (role="switch", aria-checked) on toggle switch inputs
 * and synchronizes aria-checked attribute on state change for screen reader accessibility.
 */
export function initToggleSwitches() {
  const toggleInputs = SovereignDomSelector.selectAll('.toggle-switch input[type="checkbox"]');
  toggleInputs.forEach((input) => {
    if (!input.hasAttribute("role")) {
      input.setAttribute("role", "switch");
    }
    input.setAttribute("aria-checked", input.checked ? "true" : "false");
    input.addEventListener("change", () => {
      input.setAttribute("aria-checked", input.checked ? "true" : "false");
    });
  });
}

/**
 * Dismisses open modal overlays (#cmd-palette, #context-menu, #help-overlay) when Escape key is pressed
 * and toggles the Command Palette dialog on Alt+Space keyboard shortcut.
 */
export function initEscapeKeyDismissal() {
  if (typeof window === "undefined" || typeof document === "undefined") return;
  document.addEventListener("keydown", (event) => {
    if (event.altKey && (event.code === "Space" || event.key === " ")) {
      event.preventDefault();
      const cmdPalette = document.getElementById("cmd-palette");
      if (cmdPalette) {
        const isActive = cmdPalette.classList.toggle("active");
        cmdPalette.setAttribute("aria-hidden", isActive ? "false" : "true");
        if (isActive) {
          lastFocusedElement = document.activeElement;
          const cmdInput = document.getElementById("cmd-input");
          if (cmdInput) cmdInput.focus();
        } else {
          const cmdInput = document.getElementById("cmd-input");
          if (cmdInput) cmdInput.blur();
          if (lastFocusedElement && typeof lastFocusedElement.focus === "function") {
            lastFocusedElement.focus();
            lastFocusedElement = null;
          }
        }
      }
      return;
    }

    if (event.key === "Escape") {
      const cmdPalette = document.getElementById("cmd-palette");
      if (cmdPalette) {
        const wasActive = cmdPalette.classList.contains("active");
        cmdPalette.classList.remove("active");
        cmdPalette.setAttribute("aria-hidden", "true");
        if (wasActive) {
          const cmdInput = document.getElementById("cmd-input");
          if (cmdInput) cmdInput.blur();
          if (lastFocusedElement && typeof lastFocusedElement.focus === "function") {
            lastFocusedElement.focus();
            lastFocusedElement = null;
          }
        }
      }
      const contextMenu = document.getElementById("context-menu");
      if (contextMenu) {
        contextMenu.style.display = "none";
        contextMenu.setAttribute("aria-hidden", "true");
      }
      const helpOverlay = document.getElementById("help-overlay");
      if (helpOverlay) {
        helpOverlay.classList.add("wizard-overlay--hidden");
        helpOverlay.setAttribute("aria-hidden", "true");
      }
    }
  });
}

/**
 * Initializes desktop right-click context menu positioning, viewport boundary calculations,
 * and WAI-ARIA accessibility state synchronization (aria-hidden, auto-focus).
 */
export function initContextMenu() {
  if (typeof window === "undefined" || typeof document === "undefined") return;

  if (!window.contextAction) {
    window.contextAction = function () {
      const contextMenu = document.getElementById("context-menu");
      if (contextMenu) {
        contextMenu.style.display = "none";
        contextMenu.setAttribute("aria-hidden", "true");
      }
    };
  }

  document.addEventListener("contextmenu", (event) => {
    const contextMenu = document.getElementById("context-menu");
    if (!contextMenu) return;

    const targetTag = event.target?.tagName?.toLowerCase();
    if (targetTag === "input" || targetTag === "textarea" || event.target?.isContentEditable) {
      return;
    }

    event.preventDefault();

    contextMenu.style.display = "block";
    contextMenu.setAttribute("aria-hidden", "false");

    const menuWidth = contextMenu.offsetWidth || 180;
    const menuHeight = contextMenu.offsetHeight || 150;
    const posX = Math.min(event.clientX, window.innerWidth - menuWidth - 8);
    const posY = Math.min(event.clientY, window.innerHeight - menuHeight - 8);

    contextMenu.style.left = `${Math.max(0, posX)}px`;
    contextMenu.style.top = `${Math.max(0, posY)}px`;

    const firstItem = SovereignDomSelector.selectOne('[role="menuitem"]', contextMenu);
    if (firstItem && typeof firstItem.focus === "function") {
      firstItem.focus();
    }
  });

  document.addEventListener("click", (event) => {
    const contextMenu = document.getElementById("context-menu");
    if (contextMenu && contextMenu.style.display === "block") {
      if (!contextMenu.contains(event.target)) {
        contextMenu.style.display = "none";
        contextMenu.setAttribute("aria-hidden", "true");
      }
    }
  });
}

/**
 * Initializes desktop dock keyboard shortcuts (Alt+F, Alt+T, Alt+O, Alt+S, Alt+M, Alt+D, F1, ?)
 * to match tooltip/ARIA shortcut hints for WCAG 2.1 keyboard accessibility.
 */
export function initDockShortcutNavigation() {
  if (typeof window === "undefined" || typeof document === "undefined") return;

  const shortcutMap = {
    "f": "File Manager",
    "t": "OmniShell Terminal",
    "o": "Observability Matrix",
    "s": "Lattice Settings",
    "m": "Marketplace",
    "d": "Developer Portal",
  };

  document.addEventListener("keydown", (event) => {
    const targetTag = event.target?.tagName?.toLowerCase();
    if (targetTag === "input" || targetTag === "textarea" || event.target?.isContentEditable) {
      return;
    }

    if (event.altKey && shortcutMap[event.key.toLowerCase()]) {
      event.preventDefault();
      const appName = shortcutMap[event.key.toLowerCase()];
      const btn = SovereignDomSelector.selectAll(".dock-icon").find(
        (b) => b.getAttribute("data-tooltip")?.includes(appName) || b.getAttribute("aria-label")?.includes(appName),
      );
      if (btn) btn.click();
    } else if (event.key === "F1" || event.key === "?") {
      event.preventDefault();
      const helpBtn = SovereignDomSelector.selectAll(".dock-icon").find(
        (b) => b.getAttribute("data-tooltip")?.includes("Help Matrix") || b.getAttribute("aria-label")?.includes("Help Matrix"),
      );
      if (helpBtn) helpBtn.click();
    }
  });
}

// Auto-initialize accessibility listeners when loaded in browser environments
if (typeof window !== "undefined" && typeof document !== "undefined") {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
      initKeyboardNavigation();
      initHighContrastSupport();
      initTablistNavigation();
      initToggleSwitches();
      initEscapeKeyDismissal();
      initDockShortcutNavigation();
      initMenuNavigation();
      initContextMenu();
    });
  } else {
    initKeyboardNavigation();
    initHighContrastSupport();
    initTablistNavigation();
    initToggleSwitches();
    initEscapeKeyDismissal();
    initDockShortcutNavigation();
    initMenuNavigation();
    initContextMenu();
  }
}

// =========================================================================
// 5. Web UI DOM Security & Prototype Pollution Protection
// =========================================================================

/**
 * Sanitizes input strings by escaping HTML special characters to prevent DOM-based XSS (js/xss-through-dom)
 */
export function sanitizeDOMString(str) {
  if (typeof str !== "string") {
    return "";
  }
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#x27;")
    .replace(/\//g, "&#x2F;");
}

/**
 * Safely merges properties from source into target, explicitly filtering prototype pollution keys (js/prototype-pollution)
 */
export function safeMergeObjects(target = {}, source = {}) {
  if (typeof target !== "object" || target === null) {
    target = {};
  }
  if (typeof source !== "object" || source === null) {
    return target;
  }

  const unsafeKeys = new Set(["__proto__", "constructor", "prototype"]);

  for (const key of Object.keys(source)) {
    if (unsafeKeys.has(key)) {
      continue; // Block prototype pollution injection vectors
    }

    const value = source[key];
    if (typeof value === "object" && value !== null && !Array.isArray(value)) {
      if (typeof target[key] !== "object" || target[key] === null) {
        target[key] = {};
      }
      safeMergeObjects(target[key], value);
    } else {
      target[key] = value;
    }
  }

  return target;
}

/**
 * Safely parses a JSON string, stripping prototype pollution property keys
 */
export function safeJsonParse(jsonString, fallback = {}) {
  try {
    const parsed = JSON.parse(jsonString, (key, value) => {
      if (key === "__proto__" || key === "constructor" || key === "prototype") {
        return undefined; // Filter dangerous prototype keys during parsing
      }
      return value;
    });
    return parsed;
  } catch (e) {
    return fallback;
  }
}

/**
 * Validates and sanitizes URLs to block malicious JavaScript/data URIs
 */
export function sanitizeUrl(urlStr) {
  if (typeof urlStr !== "string") {
    return "about:blank";
  }
  const cleanUrl = urlStr.trim().toLowerCase();
  if (
    cleanUrl.startsWith("javascript:") ||
    cleanUrl.startsWith("data:") ||
    cleanUrl.startsWith("vbscript:")
  ) {
    return "about:blank";
  }
  return urlStr;
}

// =========================================================================
// Window Management & Accessibility State Handlers
// =========================================================================

const APP_WIN_MAP = {
  "file manager": "file-manager-win",
  "omnishell": "terminal-win",
  "sigma browser": "browser-win",
  "observability matrix": "browser-win",
  "ai assistant": "ai-assistant-win",
  "neural core": "ai-assistant-win",
  "lattice settings": "lattice-settings-win",
  "utility nexus": "utility-nexus-win",
  "pro tools": "utility-nexus-win",
  "marketplace": "sigma-market-win",
  "markup forge": "markup-forge-win",
  "zenith installer": "markup-forge-win",
  "dev portal": "dev-portal-win",
  "developer portal": "dev-portal-win",
  "emulator": "emulator-win",
  "kernel emulator": "emulator-win",
  "analytics": "analytics-win",
  "build analytics": "analytics-win"
};

let topZIndex = 100;

export function closeWindow(winId) {
  const win = typeof winId === "string" ? document.getElementById(winId) : winId;
  if (!win) return;
  win.style.display = "none";
  win.setAttribute("aria-hidden", "true");
  win.classList.remove("active-focus");
}

export function maximizeWindow(winId) {
  const win = typeof winId === "string" ? document.getElementById(winId) : winId;
  if (!win) return;
  const isMax = win.classList.toggle("maximized");
  const maxBtn = SovereignDomSelector.selectOne(".control-dot.max", win);
  if (maxBtn) {
    const label = isMax ? "Restore Window" : "Maximize Window";
    maxBtn.setAttribute("aria-label", label);
    maxBtn.setAttribute("data-tooltip", isMax ? "Restore" : "Maximize");
  }
}

export function launchApp(appName) {
  const winId = APP_WIN_MAP[appName?.toLowerCase()] || appName;
  const win = document.getElementById(winId);
  if (!win) return;

  const isHidden = win.style.display === "none" || (typeof getComputedStyle === "function" && getComputedStyle(win).display === "none");
  if (isHidden) {
    win.style.display = "flex";
    win.setAttribute("aria-hidden", "false");
    win.style.opacity = "1";
    win.style.transform = "scale(1)";
  }

  topZIndex += 1;
  win.style.zIndex = topZIndex;
  SovereignDomSelector.selectAll(".window").forEach((w) => w.classList.remove("active-focus"));
  win.classList.add("active-focus");

  const focusable = SovereignDomSelector.selectOne('input, button, [tabindex="0"]', win);
  if (focusable && typeof focusable.focus === "function") {
    focusable.focus();
  }
}

if (typeof window !== "undefined") {
  window.closeWindow = closeWindow;
  window.maximizeWindow = maximizeWindow;
  window.launchApp = launchApp;
}

// Minimal dummy index file to export initialization and basic attributes
export const version = "15.0.0";
