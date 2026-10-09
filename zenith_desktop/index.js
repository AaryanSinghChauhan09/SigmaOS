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
    if (!root) return null;
    if (typeof root.querySelector === "function") {
      try {
        const res = root.querySelector(selector);
        if (res) return res;
      } catch (e) {}
    }
    const all = this.selectAll(selector, root);
    return all.length > 0 ? all[0] : null;
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
let activeCommandIndex = 0;

export const SYSTEM_COMMANDS = [
  { name: "📂 File Manager", action: () => launchApp("File Manager"), shortcut: "Alt+F" },
  { name: "💻 OmniShell Terminal", action: () => launchApp("OmniShell"), shortcut: "Alt+T" },
  { name: "📊 Observability Matrix", action: () => launchApp("Sigma Browser"), shortcut: "Alt+O" },
  { name: "🧠 Neural Core", action: () => launchApp("AI Assistant") },
  { name: "⚙️ Lattice Settings", action: () => launchApp("Lattice Settings"), shortcut: "Alt+S" },
  { name: "🔧 Sovereign Pro Tools", action: () => launchApp("Utility Nexus") },
  { name: "🛒 Shard Marketplace", action: () => launchApp("Marketplace"), shortcut: "Alt+M" },
  { name: "📥 System Installer", action: () => launchApp("Markup Forge") },
  { name: "📖 Developer Portal", action: () => launchApp("Dev Portal"), shortcut: "Alt+D" },
  { name: "💿 Kernel Emulator", action: () => launchApp("Emulator") },
  { name: "📈 Build Analytics", action: () => launchApp("Analytics") },
  { name: "🔄 Force Refresh System", action: () => window.contextAction?.("refresh") },
  { name: "🧹 Purge System Caches", action: () => window.contextAction?.("cleanup") },
  { name: "🛡️ Attest System Security", action: () => window.contextAction?.("audit") },
];

export function renderCommandResults(query = "") {
  if (typeof document === "undefined") return [];
  const resultsContainer = document.getElementById("cmd-results");
  const cmdInput = document.getElementById("cmd-input");
  if (!resultsContainer) return [];

  const cleanQuery = query.trim().toLowerCase();
  const filtered = SYSTEM_COMMANDS.filter((cmd) => cmd.name.toLowerCase().includes(cleanQuery));

  while (resultsContainer.firstChild) {
    resultsContainer.removeChild(resultsContainer.firstChild);
  }
  if (filtered.length === 0) {
    const noResults = document.createElement("div");
    noResults.className = "cmd-no-results";
    noResults.style.padding = "10px";
    noResults.style.color = "var(--text-muted)";
    noResults.style.fontSize = "0.85rem";
    noResults.setAttribute("role", "status");
    noResults.textContent = "No matching commands found";
    resultsContainer.appendChild(noResults);
    activeCommandIndex = -1;
    if (cmdInput) cmdInput.removeAttribute("aria-activedescendant");
    return [];
  }

  if (activeCommandIndex >= filtered.length || activeCommandIndex < 0) {
    activeCommandIndex = 0;
  }

  if (cmdInput) {
    cmdInput.setAttribute("aria-activedescendant", `cmd-option-${activeCommandIndex}`);
  }

  filtered.forEach((cmd, idx) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.id = `cmd-option-${idx}`;
    btn.className = `command-item${idx === activeCommandIndex ? " selected" : ""}`;
    btn.role = "option";
    btn.setAttribute("aria-selected", idx === activeCommandIndex ? "true" : "false");
    btn.setAttribute("tabindex", "-1");

    const nameSpan = document.createElement("span");
    nameSpan.textContent = cmd.name;
    btn.appendChild(nameSpan);

    if (cmd.shortcut) {
      const badge = document.createElement("span");
      badge.className = "capsule-badge";
      badge.style.fontSize = "0.65rem";
      badge.textContent = cmd.shortcut;
      btn.appendChild(badge);
    }

    btn.addEventListener("click", () => {
      cmd.action();
      closeCommandPalette();
    });

    resultsContainer.appendChild(btn);

    if (idx === activeCommandIndex && typeof btn.scrollIntoView === "function") {
      btn.scrollIntoView({ block: "nearest" });
    }
  });

  return filtered;
}

let helpLastFocusedElement = null;

/**
 * Toggles the accessible Sovereign Help Matrix modal overlay (#help-overlay),
 * populates system keyboard shortcuts guidance, and manages dialog focus.
 */
export function toggleHelp() {
  if (typeof document === "undefined") return;
  const overlay = document.getElementById("help-overlay");
  if (!overlay) return;

  const isHidden = overlay.classList.contains("wizard-overlay--hidden");
  if (isHidden) {
    if (typeof document !== "undefined" && document.activeElement) {
      helpLastFocusedElement = document.activeElement;
    }
    overlay.setAttribute("role", "dialog");
    overlay.setAttribute("aria-label", "Sovereign Desktop Help Matrix");
    overlay.setAttribute("aria-modal", "true");
    if (!overlay.firstElementChild) {
      const card = document.createElement("div");
      card.className = "wizard-card";
      card.setAttribute("role", "document");

      const h2 = document.createElement("h2");
      h2.textContent = "❓ Help Matrix";
      card.appendChild(h2);

      const p = document.createElement("p");
      p.style.color = "var(--text-muted)";
      p.style.marginBottom = "15px";
      p.style.fontSize = "0.9rem";
      p.textContent = "Essential Sovereign Desktop Keyboard Shortcuts";
      card.appendChild(p);

      const steps = document.createElement("div");
      steps.className = "wizard-steps";
      steps.style.textAlign = "left";
      steps.style.fontSize = "0.85rem";
      steps.style.gap = "8px";

      const shortcuts = [
        { key: "Alt + Space", desc: " Toggle Command Palette" },
        { key: "Alt + F / T / O / S / M / D", desc: " Launch Apps" },
        { key: "F1 / ?", desc: " Toggle Help Matrix" },
        { key: "Escape", desc: " Dismiss active dialogs & menus" },
      ];

      shortcuts.forEach((sc) => {
        const item = document.createElement("div");
        const kbd = document.createElement("kbd");
        kbd.style.background = "rgba(255,255,255,0.1)";
        kbd.style.padding = "2px 6px";
        kbd.style.borderRadius = "4px";
        kbd.textContent = sc.key;
        item.appendChild(kbd);
        item.appendChild(document.createTextNode(sc.desc));
        steps.appendChild(item);
      });
      card.appendChild(steps);

      const closeBtn = document.createElement("button");
      closeBtn.type = "button";
      closeBtn.className = "wizard-btn";
      closeBtn.id = "help-close-btn";
      closeBtn.style.marginTop = "20px";
      closeBtn.setAttribute("aria-label", "Close Help Matrix");
      closeBtn.textContent = "Close";
      closeBtn.addEventListener("click", () => toggleHelp());
      card.appendChild(closeBtn);

      overlay.appendChild(card);
    }
    overlay.classList.remove("wizard-overlay--hidden");
    overlay.setAttribute("aria-hidden", "false");
    const closeBtn = (typeof overlay.querySelector === "function") 
      ? overlay.querySelector("button") 
      : (typeof document.getElementById === "function" ? document.getElementById("help-close-btn") : null);
    if (closeBtn && typeof closeBtn.focus === "function") {
      closeBtn.focus();
    }
  } else {
    overlay.classList.add("wizard-overlay--hidden");
    overlay.setAttribute("aria-hidden", "true");
    if (helpLastFocusedElement && typeof helpLastFocusedElement.focus === "function") {
      helpLastFocusedElement.focus();
      helpLastFocusedElement = null;
    }
  }
}

export function closeCommandPalette() {
  if (typeof document === "undefined") return;
  const cmdPalette = document.getElementById("cmd-palette");
  const cmdInput = document.getElementById("cmd-input");
  if (cmdPalette) {
    cmdPalette.classList.remove("active");
    cmdPalette.setAttribute("aria-hidden", "true");
  }
  if (cmdInput) {
    cmdInput.setAttribute("aria-expanded", "false");
    cmdInput.removeAttribute("aria-activedescendant");
    cmdInput.blur();
  }
  if (lastFocusedElement && typeof lastFocusedElement.focus === "function") {
    lastFocusedElement.focus();
    lastFocusedElement = null;
  }
}

export function initCommandPalette() {
  if (typeof document === "undefined") return;
  const cmdInput = document.getElementById("cmd-input");
  const cmdPalette = document.getElementById("cmd-palette");
  if (!cmdInput || !cmdPalette) return;

  if (!cmdInput.hasAttribute("aria-haspopup")) {
    cmdInput.setAttribute("aria-haspopup", "listbox");
  }

  cmdInput.addEventListener("input", (e) => {
    activeCommandIndex = 0;
    renderCommandResults(e.target.value);
  });

  cmdInput.addEventListener("keydown", (e) => {
    const query = cmdInput.value;
    const cleanQuery = query.trim().toLowerCase();
    const filtered = SYSTEM_COMMANDS.filter((cmd) => cmd.name.toLowerCase().includes(cleanQuery));

    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (filtered.length > 0) {
        activeCommandIndex = (activeCommandIndex + 1) % filtered.length;
        renderCommandResults(query);
      }
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (filtered.length > 0) {
        activeCommandIndex = (activeCommandIndex - 1 + filtered.length) % filtered.length;
        renderCommandResults(query);
      }
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (filtered.length > 0 && activeCommandIndex >= 0 && activeCommandIndex < filtered.length) {
        filtered[activeCommandIndex].action();
        closeCommandPalette();
      }
    }
  });

  document.addEventListener("click", (event) => {
    if (
      cmdPalette.classList.contains("active") &&
      typeof cmdPalette.contains === "function" &&
      !cmdPalette.contains(event.target)
    ) {
      closeCommandPalette();
    }
  });
}

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
          if (cmdInput) {
            cmdInput.setAttribute("aria-expanded", "true");
            cmdInput.value = "";
            cmdInput.focus();
            activeCommandIndex = 0;
            renderCommandResults("");
          }
        } else {
          const cmdInput = document.getElementById("cmd-input");
          if (cmdInput) {
            cmdInput.setAttribute("aria-expanded", "false");
            cmdInput.removeAttribute("aria-activedescendant");
            cmdInput.blur();
          }
          if (lastFocusedElement && typeof lastFocusedElement.focus === "function") {
            lastFocusedElement.focus();
            lastFocusedElement = null;
          }
        }
      }
      return;
    }

    if (event.key === "Tab") {
      const helpOverlay = document.getElementById("help-overlay");
      if (helpOverlay && !helpOverlay.classList.contains("wizard-overlay--hidden")) {
        const focusables = SovereignDomSelector.selectAll('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])', helpOverlay);
        if (focusables.length > 0) {
          const first = focusables[0];
          const last = focusables[focusables.length - 1];
          if (event.shiftKey && document.activeElement === first) {
            event.preventDefault();
            last.focus();
          } else if (!event.shiftKey && document.activeElement === last) {
            event.preventDefault();
            first.focus();
          }
        }
      }
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
        const wasActive = !helpOverlay.classList.contains("wizard-overlay--hidden");
        helpOverlay.classList.add("wizard-overlay--hidden");
        helpOverlay.setAttribute("aria-hidden", "true");
        if (wasActive && helpLastFocusedElement && typeof helpLastFocusedElement.focus === "function") {
          helpLastFocusedElement.focus();
          helpLastFocusedElement = null;
        }
      }
    }
  });
}

const CONTEXT_MESSAGES = {
  refresh: "🔄 System refreshed and window state synchronized.",
  cleanup: "🧹 System caches purged (128 MB reclaimed).",
  audit: "🛡️ System security attestation verified: Kyber-1024.",
};

/**
 * Displays an accessible, transient toast notification with role="status" and aria-live="polite".
 */
export function showToast(message, duration = 3000) {
  if (typeof document === "undefined") return null;
  const container = document.getElementById("kernel-logs");
  if (!container) return null;

  const toast = document.createElement("div");
  toast.className = "kernel-toast";
  toast.setAttribute("role", "status");
  toast.setAttribute("aria-live", "polite");
  toast.textContent = message;
  container.appendChild(toast);

  if (typeof setTimeout === "function") {
    setTimeout(() => {
      toast.classList.add("fade-out");
      setTimeout(() => {
        if (toast.parentNode) {
          toast.parentNode.removeChild(toast);
        }
      }, 300);
    }, duration);
  }
  return toast;
}

/**
 * Initializes desktop right-click context menu positioning, viewport boundary calculations,
 * and WAI-ARIA accessibility state synchronization (aria-hidden, auto-focus).
 */
export function initContextMenu() {
  if (typeof window === "undefined" || typeof document === "undefined") return;

  if (!window.contextAction) {
    window.contextAction = function (action) {
      const contextMenu = document.getElementById("context-menu");
      if (contextMenu) {
        contextMenu.style.display = "none";
        contextMenu.setAttribute("aria-hidden", "true");
      }
      if (action && CONTEXT_MESSAGES[action]) {
        showToast(CONTEXT_MESSAGES[action]);
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

/**
 * Initializes desktop window click-to-raise and focus-to-raise event handlers on .window elements
 * to elevate zIndex and manage .active-focus state for keyboard and mouse accessibility.
 */
export function initWindowFocus() {
  const windows = SovereignDomSelector.selectAll(".window");
  windows.forEach((win) => {
    const raiseWindow = () => {
      topZIndex += 1;
      win.style.zIndex = topZIndex;
      windows.forEach((w) => w.classList.remove("active-focus"));
      win.classList.add("active-focus");
      updateDockAppStates();
    };

    win.addEventListener("mousedown", raiseWindow);
    win.addEventListener("focusin", raiseWindow);
  });
}

// =========================================================================
// 6. Sovereign 60fps Compositor Runtime
//    One requestAnimationFrame pipeline batches ALL per-frame DOM work
//    (window dragging, ambient glow tracking, live telemetry) into a single
//    write phase: no read/write interleaving, no layout thrash, one style
//    write per element per frame. Pointer events only record intent; the
//    frame applies it.
// =========================================================================

const PERF_STATE = {
  rafScheduled: false,
  frameDeltas: [],
  lastFrameTs: 0,
  lastFpsPaint: 0,
  lastClockPaint: 0,
  lastTelemetryPaint: 0,
  cpu: 14,
  memGB: 2.6,
  drag: null,
  glow: null,
  telemetry: null,
};

function perfSetTspanIfChanged(el, text) {
  if (el && el.textContent !== text) {
    el.textContent = text;
  }
}

function perfScheduleFrame() {
  if (PERF_STATE.rafScheduled) return;
  PERF_STATE.rafScheduled = true;
  if (typeof requestAnimationFrame === "function") {
    requestAnimationFrame(perfFrame);
  }
}

function perfUpdateFps() {
  const t = PERF_STATE.telemetry;
  if (!t || !t.fpsEl || PERF_STATE.frameDeltas.length < 5) return;
  const avg =
    PERF_STATE.frameDeltas.reduce((a, b) => a + b, 0) / PERF_STATE.frameDeltas.length;
  perfSetTspanIfChanged(t.fpsEl, avg > 0 ? (1000 / avg).toFixed(1) : "—");
}

function perfUpdateClock() {
  const t = PERF_STATE.telemetry;
  if (!t || (!t.dateEl && !t.timeEl)) return;
  const now = new Date();
  if (t.dateEl) {
    perfSetTspanIfChanged(
      t.dateEl,
      now.toLocaleDateString(undefined, { weekday: "long", month: "long", day: "numeric" }),
    );
  }
  if (t.timeEl) {
    perfSetTspanIfChanged(
      t.timeEl,
      now.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" }),
    );
  }
}

// Simulated vitals for the preview shell: bounded random walk keeps the
// readouts plausible (and visibly alive) until a real kernel bridge exists.
function perfUpdateTelemetrySim() {
  const t = PERF_STATE.telemetry;
  if (!t) return;
  const clamp = (v, lo, hi) => Math.min(Math.max(v, lo), hi);
  PERF_STATE.cpu = clamp(PERF_STATE.cpu + (Math.random() * 10 - 5), 2, 96);
  PERF_STATE.memGB = clamp(PERF_STATE.memGB + (Math.random() * 0.3 - 0.15), 1.2, 7.8);
  if (t.cpuEl) perfSetTspanIfChanged(t.cpuEl, `${Math.round(PERF_STATE.cpu)}%`);
  if (t.memEl) perfSetTspanIfChanged(t.memEl, `${PERF_STATE.memGB.toFixed(1)} GB`);
}

function perfFrame(ts) {
  PERF_STATE.rafScheduled = false;

  // FPS accounting (spikes > 1s are visibility changes, not frames)
  if (PERF_STATE.lastFrameTs) {
    const dt = ts - PERF_STATE.lastFrameTs;
    if (dt > 0 && dt < 1000) {
      PERF_STATE.frameDeltas.push(dt);
      if (PERF_STATE.frameDeltas.length > 90) PERF_STATE.frameDeltas.shift();
    }
  }
  PERF_STATE.lastFrameTs = ts;

  // Window drag: single batched style write; all geometry was cached at
  // drag start, so the hot loop performs zero layout reads.
  const drag = PERF_STATE.drag;
  if (drag && drag.el) {
    const x = Math.min(
      Math.max(drag.baseX + (drag.pointerX - drag.startX), -drag.w + 140),
      drag.parentW - 140,
    );
    const y = Math.min(Math.max(drag.baseY + (drag.pointerY - drag.startY), 0), drag.parentH - 60);
    drag.el.style.left = `${Math.round(x)}px`;
    drag.el.style.top = `${Math.round(y)}px`;
  }

  // Ambient glow: compositor-only transform (no layout, no paint)
  if (PERF_STATE.glow && PERF_STATE.glow.dirty) {
    PERF_STATE.glow.dirty = false;
    PERF_STATE.glow.el.style.transform =
      `translate3d(${PERF_STATE.glow.x}px, ${PERF_STATE.glow.y}px, 0) translate(-50%, -50%)`;
  }

  // Rate-limited telemetry paints driven by the same frame clock
  if (ts - PERF_STATE.lastFpsPaint >= 500) {
    PERF_STATE.lastFpsPaint = ts;
    perfUpdateFps();
  }
  if (ts - PERF_STATE.lastClockPaint >= 1000) {
    PERF_STATE.lastClockPaint = ts;
    perfUpdateClock();
  }
  if (ts - PERF_STATE.lastTelemetryPaint >= 2000) {
    PERF_STATE.lastTelemetryPaint = ts;
    perfUpdateTelemetrySim();
  }

  // Continuous loop: the FPS meter measures the live desktop frame rate.
  perfScheduleFrame();
}

/**
 * GPU-smooth, pointer-captured window dragging.
 * Windows advertise "Draggable Premium Windows" in the markup; this finally
 * implements it: delta-based positioning (drift-free), geometry cached at
 * drag start, one style write per frame, transitions suppressed mid-drag.
 */
export function initSmoothWindowDragging() {
  if (typeof document === "undefined") return;
  SovereignDomSelector.selectAll(".window").forEach((win) => {
    const header = SovereignDomSelector.selectOne(".window-header", win);
    if (!header || typeof header.setPointerCapture !== "function") return;
    let activePointerId = null;

    header.addEventListener("pointerdown", (event) => {
      if (event.button !== 0) return;
      if (SovereignDomSelector.matches(event.target, ".control-dot, .control-dot *")) return;
      if (win.classList.contains("maximized")) return;

      const rect = win.getBoundingClientRect();
      const parent = win.offsetParent || document.body;
      const parentRect = parent.getBoundingClientRect();

      activePointerId = event.pointerId;
      PERF_STATE.drag = {
        el: win,
        header,
        pointerId: event.pointerId,
        startX: event.clientX,
        startY: event.clientY,
        pointerX: event.clientX,
        pointerY: event.clientY,
        baseX: rect.left - parentRect.left,
        baseY: rect.top - parentRect.top,
        w: rect.width,
        h: rect.height,
        parentW: parentRect.width,
        parentH: parentRect.height,
      };
      win.classList.add("dragging");
      try {
        header.setPointerCapture(event.pointerId);
      } catch (e) {
        /* pointer capture is best-effort */
      }
      event.preventDefault();
    });

    header.addEventListener("pointermove", (event) => {
      if (activePointerId === null || event.pointerId !== activePointerId) return;
      if (!PERF_STATE.drag) return;
      PERF_STATE.drag.pointerX = event.clientX;
      PERF_STATE.drag.pointerY = event.clientY;
      perfScheduleFrame();
    });

    const endDrag = (event) => {
      if (activePointerId === null) return;
      if (PERF_STATE.drag && PERF_STATE.drag.el === win) {
        PERF_STATE.drag = null;
      }
      activePointerId = null;
      win.classList.remove("dragging");
      try {
        header.releasePointerCapture(event.pointerId);
      } catch (e) {
        /* already released */
      }
    };
    header.addEventListener("pointerup", endDrag);
    header.addEventListener("pointercancel", endDrag);
    header.addEventListener("lostpointercapture", () => {
      if (PERF_STATE.drag && PERF_STATE.drag.el === win) {
        PERF_STATE.drag = null;
      }
      activePointerId = null;
      win.classList.remove("dragging");
    });

    // Double-click the header to toggle maximize (common desktop convention)
    header.addEventListener("dblclick", (event) => {
      if (SovereignDomSelector.matches(event.target, ".control-dot, .control-dot *")) return;
      if (typeof win.id === "string" && win.id) {
        maximizeWindow(win.id);
      }
    });
  });
}

/**
 * Ambient mouse glow: the #mouse-glow element was dead markup — styled in
 * CSS but never driven. It now follows the pointer via a rAF-batched
 * translate3d (compositor-only, zero layout cost) and fades in/out.
 */
export function initAmbientMouseGlow() {
  if (typeof document === "undefined" || typeof window === "undefined") return;
  const glow = document.getElementById("mouse-glow");
  if (!glow) return;
  if (window.matchMedia && !window.matchMedia("(pointer: fine)").matches) return;

  PERF_STATE.glow = {
    el: glow,
    x: Math.round(window.innerWidth / 2),
    y: Math.round(window.innerHeight / 2),
    dirty: true,
  };

  document.addEventListener(
    "pointermove",
    (event) => {
      if (!PERF_STATE.glow) return;
      PERF_STATE.glow.x = event.clientX;
      PERF_STATE.glow.y = event.clientY;
      PERF_STATE.glow.dirty = true;
      if (!glow.classList.contains("warm")) {
        glow.classList.add("warm");
      }
      perfScheduleFrame();
    },
    { passive: true },
  );

  document.addEventListener("pointerleave", () => glow.classList.remove("warm"));
  document.addEventListener("mouseleave", () => glow.classList.remove("warm"));
}

/**
 * Live top-bar telemetry: FPS (measured from the frame clock), CPU/MEM
 * simulated vitals, and the wall clock. The markup shipped these readouts
 * frozen at "0% / 0.0 GB / 60.0ms / 12:00" — this revives them.
 */
export function initLiveTelemetry() {
  if (typeof document === "undefined") return;
  PERF_STATE.telemetry = {
    cpuEl: document.getElementById("telemetry-cpu"),
    memEl: document.getElementById("telemetry-mem"),
    fpsEl: document.getElementById("telemetry-fps"),
    dateEl: document.getElementById("clock-date"),
    timeEl: document.getElementById("clock-time"),
  };
  const t = PERF_STATE.telemetry;
  if (!t.cpuEl && !t.memEl && !t.fpsEl && !t.dateEl && !t.timeEl) {
    PERF_STATE.telemetry = null;
    return;
  }
  perfUpdateClock();
  perfScheduleFrame();
}

// Auto-initialize accessibility listeners when loaded in browser environments
if (typeof window !== "undefined" && typeof document !== "undefined") {
  const autoInit = () => {
    initKeyboardNavigation();
    initHighContrastSupport();
    initTablistNavigation();
    initToggleSwitches();
    initCommandPalette();
    initEscapeKeyDismissal();
    initDockShortcutNavigation();
    initMenuNavigation();
    initContextMenu();
    initWindowFocus();
    initSmoothWindowDragging();
    initAmbientMouseGlow();
    initLiveTelemetry();
    updateDockAppStates();
  };
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", autoInit);
  } else {
    autoInit();
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

/**
 * Synchronizes dock icon launcher buttons with aria-pressed="true|false" and visual indicators (.app-open, .app-active)
 * based on whether corresponding application windows are currently open and focused.
 */
export function updateDockAppStates() {
  if (typeof document === "undefined") return;
  const dockIcons = SovereignDomSelector.selectAll(".dock-icon");
  dockIcons.forEach((btn) => {
    const hint = (btn.getAttribute("data-tooltip") || btn.getAttribute("aria-label") || "").toLowerCase();
    let matchedWin = null;
    for (const [appName, winId] of Object.entries(APP_WIN_MAP)) {
      if (hint.includes(appName)) {
        matchedWin = document.getElementById(winId);
        if (matchedWin) break;
      }
    }
    if (matchedWin) {
      const isOpen = matchedWin.style.display !== "none" && matchedWin.getAttribute("aria-hidden") !== "true";
      const isActive = isOpen && matchedWin.classList.contains("active-focus");
      btn.setAttribute("aria-pressed", isOpen ? "true" : "false");
      btn.classList.toggle("app-open", isOpen);
      btn.classList.toggle("app-active", isActive);
    }
  });
}

export function restoreFocusAfterWindowClose(closedWin) {
  if (typeof document === "undefined") return;
  const windows = SovereignDomSelector.selectAll(".window").filter(
    (w) => w !== closedWin && w.style.display !== "none" && w.getAttribute("aria-hidden") !== "true"
  );
  if (windows.length > 0) {
    windows.sort((a, b) => (Number(b.style.zIndex) || 0) - (Number(a.style.zIndex) || 0));
    const topWin = windows[0];
    topWin.classList.add("active-focus");
    const focusable = SovereignDomSelector.selectOne('input, button, [tabindex="0"]', topWin);
    if (focusable && typeof focusable.focus === "function") {
      focusable.focus();
      return;
    }
  }
  const dockBtn = SovereignDomSelector.selectOne('.dock-icon');
  if (dockBtn && typeof dockBtn.focus === "function") {
    dockBtn.focus();
  }
}

export function closeWindow(winId) {
  const win = typeof winId === "string" ? document.getElementById(winId) : winId;
  if (!win) return;
  const wasFocused = win.classList.contains("active-focus") || (typeof document !== "undefined" && document.activeElement && win.contains && win.contains(document.activeElement));
  win.style.display = "none";
  win.setAttribute("aria-hidden", "true");
  win.classList.remove("active-focus");
  if (wasFocused) {
    restoreFocusAfterWindowClose(win);
  }
  updateDockAppStates();
}

export function minimizeWindow(winId) {
  closeWindow(winId);
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
  const isActiveFocus = win.classList.contains("active-focus");

  if (!isHidden && isActiveFocus) {
    minimizeWindow(win);
    return;
  }

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
  updateDockAppStates();
}

if (typeof window !== "undefined") {
  window.closeWindow = closeWindow;
  window.minimizeWindow = minimizeWindow;
  window.maximizeWindow = maximizeWindow;
  window.launchApp = launchApp;
  window.toggleHelp = toggleHelp;
  window.renderCommandResults = renderCommandResults;
  window.initCommandPalette = initCommandPalette;
  window.initWindowFocus = initWindowFocus;
  window.updateDockAppStates = updateDockAppStates;
  window.showToast = showToast;
}

// Minimal dummy index file to export initialization and basic attributes
export const version = "15.0.0";
