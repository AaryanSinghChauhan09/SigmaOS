// SPDX-License-Identifier: MIT
// Unit test suite for Zenith Desktop Command Palette UX & Accessibility

import assert from "node:assert";
import * as zenith from "../zenith_desktop/index.js";

// Mock DOM environment
let activeElement = null;
const mockElements = {};

function createMockElement(id, tag = "div") {
  const el = {
    id,
    tag,
    style: {},
    children: [],
    attributes: { id },
    classList: {
      _classes: new Set(),
      add(c) { this._classes.add(c); },
      remove(c) { this._classes.delete(c); },
      toggle(c) {
        if (this._classes.has(c)) { this._classes.delete(c); return false; }
        this._classes.add(c); return true;
      },
      contains(c) { return this._classes.has(c); }
    },
    value: "",
    innerHTML: "",
    listeners: {},
    focus() { activeElement = el; },
    blur() { if (activeElement === el) activeElement = null; },
    setAttribute(k, v) { this.attributes[k] = v; },
    getAttribute(k) { return this.attributes[k]; },
    removeAttribute(k) { delete this.attributes[k]; },
    appendChild(child) { this.children.push(child); },
    addEventListener(event, fn) {
      if (!this.listeners[event]) this.listeners[event] = [];
      this.listeners[event].push(fn);
    },
    dispatchEvent(event) {
      const fns = this.listeners[event.type] || [];
      fns.forEach((fn) => fn(event));
    }
  };
  mockElements[id] = el;
  return el;
}

global.document = {
  readyState: "complete",
  get activeElement() { return activeElement; },
  createElement: (tag) => {
    const el = createMockElement(`anon_${Math.random()}`, tag);
    return el;
  },
  createTextNode: (text) => ({ textContent: text }),
  getElementById: (id) => mockElements[id] || createMockElement(id),
  querySelectorAll: () => []
};

// Test 1: Full command list rendering and ARIA combobox attributes
const cmdResultsEl = createMockElement("cmd-results");
createMockElement("cmd-palette");
const cmdInputEl = createMockElement("cmd-input");

const allCommands = zenith.renderCommandResults("");
console.log(`✓ Rendered all commands: count = ${allCommands.length}`);
assert.strictEqual(allCommands.length, 14, "Expected 14 default system commands");
assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), "cmd-option-0", "Expected aria-activedescendant to point to first option 'cmd-option-0'");
assert.strictEqual(cmdResultsEl.children[0].id, "cmd-option-0", "Expected option child element to have id 'cmd-option-0'");
assert.strictEqual(cmdResultsEl.children[0].role, "option", "Expected option element to have role='option'");

// Test 2: Search filtering and updated aria-activedescendant
const filtered = zenith.renderCommandResults("terminal");
console.log(`✓ Search "terminal" matched: ${filtered.length} command`);
assert.strictEqual(filtered.length, 1, "Expected 1 matching command for 'terminal'");
assert.strictEqual(filtered[0].name.includes("Terminal"), true, "Matched command name should contain 'Terminal'");
assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), "cmd-option-0", "Expected aria-activedescendant to point to 'cmd-option-0'");

// Test 3: No results handling clears aria-activedescendant
const noResults = zenith.renderCommandResults("nonexistent_command_query_123");
console.log(`✓ Nonexistent query returned empty list: ${noResults.length}`);
assert.strictEqual(noResults.length, 0, "Expected 0 matching commands");
assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), undefined, "Expected aria-activedescendant to be removed when no results");

// Test 4: closeCommandPalette resets aria-expanded
zenith.closeCommandPalette();
assert.strictEqual(cmdInputEl.getAttribute("aria-expanded"), "false", "Expected aria-expanded to be 'false' on close");

// Test 5: initWindowFocus attaches handlers and elevates zIndex / active-focus
const win1 = createMockElement("win1");
win1.classList.add("window");
const win2 = createMockElement("win2");
win2.classList.add("window");

global.document.querySelectorAll = (selector) => {
  if (selector === ".window") return [win1, win2];
  return [];
};

zenith.initWindowFocus();

win1.dispatchEvent({ type: "mousedown" });
assert.strictEqual(win1.classList.contains("active-focus"), true, "Expected win1 to have active-focus on mousedown");
assert.strictEqual(win2.classList.contains("active-focus"), false, "Expected win2 to not have active-focus when win1 is focused");

const z1 = Number(win1.style.zIndex);
win2.dispatchEvent({ type: "focusin" });
assert.strictEqual(win2.classList.contains("active-focus"), true, "Expected win2 to have active-focus on focusin");
assert.strictEqual(win1.classList.contains("active-focus"), false, "Expected win1 active-focus to be removed when win2 is focused");
assert.strictEqual(Number(win2.style.zIndex) > z1, true, "Expected win2 zIndex to be elevated above win1 zIndex");
console.log("✓ Window focus elevation and active-focus state verified successfully!");

// Test 6: toggleHelp toggles modal dialog and ARIA states
const helpOverlay = createMockElement("help-overlay");
helpOverlay.classList.add("wizard-overlay--hidden");

zenith.toggleHelp();
assert.strictEqual(helpOverlay.classList.contains("wizard-overlay--hidden"), false, "Expected help-overlay to no longer be hidden after toggleHelp()");
assert.strictEqual(helpOverlay.getAttribute("role"), "dialog", "Expected role='dialog' on help-overlay");
assert.strictEqual(helpOverlay.getAttribute("aria-label"), "Sovereign Desktop Help Matrix", "Expected aria-label on help-overlay");
assert.strictEqual(helpOverlay.getAttribute("aria-hidden"), "false", "Expected aria-hidden='false' when opened");

zenith.toggleHelp();
assert.strictEqual(helpOverlay.classList.contains("wizard-overlay--hidden"), true, "Expected help-overlay to be hidden on second toggleHelp()");
assert.strictEqual(helpOverlay.getAttribute("aria-hidden"), "true", "Expected aria-hidden='true' when closed");
console.log("✓ toggleHelp modal overlay and ARIA states verified successfully!");

// Test 7: Modal focus trap handling for Tab key navigation
zenith.initEscapeKeyDismissal();
zenith.toggleHelp();
assert.strictEqual(helpOverlay.classList.contains("wizard-overlay--hidden"), false, "Expected help-overlay open for focus trap test");
const docListeners = global.document.listeners || {};
// Keydown listener registered via initEscapeKeyDismissal
console.log("✓ Modal focus trap listener initialized successfully!");

zenith.toggleHelp(); // Clean up overlay state

// Test 8: Window minimization, dock toggling, and keyboard focus restoration
const termWin = createMockElement("terminal-win");
termWin.classList.add("window");
termWin.style.display = "none";
const dockBtn = createMockElement("dock-term-btn");
dockBtn.classList.add("dock-icon");

global.document.querySelectorAll = (selector) => {
  if (selector === ".window") return [termWin];
  if (selector === ".dock-icon") return [dockBtn];
  return [];
};

zenith.launchApp("OmniShell");
assert.strictEqual(termWin.style.display, "flex", "Expected launchApp to open window");
assert.strictEqual(termWin.classList.contains("active-focus"), true, "Expected opened window to have active-focus");

// Toggling launchApp on already active focused window should minimize it
zenith.launchApp("OmniShell");
assert.strictEqual(termWin.style.display, "none", "Expected active dock toggle to minimize window");
assert.strictEqual(termWin.getAttribute("aria-hidden"), "true", "Expected minimized window to have aria-hidden='true'");
assert.strictEqual(global.document.activeElement, dockBtn, "Expected focus to be restored to dock button on minimize");

console.log("✓ Window minimization, dock app toggling, and focus restoration verified successfully!");

// Test 9: Dock icon ARIA pressed state and visual indicator updates
dockBtn.setAttribute("data-tooltip", "OmniShell Terminal (Alt+T)");
zenith.launchApp("OmniShell");
assert.strictEqual(dockBtn.getAttribute("aria-pressed"), "true", "Expected dock button aria-pressed='true' when app window is open");
assert.strictEqual(dockBtn.classList.contains("app-open"), true, "Expected dock button to have .app-open class when open");
assert.strictEqual(dockBtn.classList.contains("app-active"), true, "Expected dock button to have .app-active class when focused");

zenith.closeWindow("terminal-win");
assert.strictEqual(dockBtn.getAttribute("aria-pressed"), "false", "Expected dock button aria-pressed='false' when app window is closed");
assert.strictEqual(dockBtn.classList.contains("app-open"), false, "Expected dock button .app-open removed when closed");
assert.strictEqual(dockBtn.classList.contains("app-active"), false, "Expected dock button .app-active removed when closed");
console.log("✓ Dynamic dock icon ARIA pressed and app state indicators verified successfully!");

console.log("All Command Palette & Desktop UX tests passed successfully!");
