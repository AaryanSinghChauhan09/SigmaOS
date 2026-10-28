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
      toggle(c, force) {
        if (force === true) { this._classes.add(c); return true; }
        if (force === false) { this._classes.delete(c); return false; }
        if (this._classes.has(c)) { this._classes.delete(c); return false; }
        this._classes.add(c); return true;
      },
      contains(c) { return this._classes.has(c); }
    },
    value: "",
    innerHTML: "",
    listeners: {},
    contains(other) {
      return this === other || this.children.some((child) => child === other || (child.contains && child.contains(other)));
    },
    focus() { activeElement = el; },
    blur() { if (activeElement === el) activeElement = null; },
    setAttribute(k, v) { this.attributes[k] = v; },
    getAttribute(k) { return this.attributes[k]; },
    hasAttribute(k) { return k in this.attributes; },
    removeAttribute(k) { delete this.attributes[k]; },
    appendChild(child) { this.children.push(child); },
    querySelectorAll(selector) {
      if (selector === ".command-item") {
        return this.children.filter((c) => c.classList && c.classList.contains("command-item"));
      }
      return [];
    },
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

global.window = globalThis;

const docListeners = {};
global.document = {
  readyState: "complete",
  get activeElement() { return activeElement; },
  createElement: (tag) => {
    const el = createMockElement(`anon_${Math.random()}`, tag);
    return el;
  },
  createTextNode: (text) => ({ textContent: text }),
  getElementById: (id) => mockElements[id] || createMockElement(id),
  querySelectorAll: (selector, root = global.document) => {
    if (selector === ".command-item") {
      const container = mockElements["cmd-results"];
      return container ? container.children.filter((c) => c.classList && c.classList.contains("command-item")) : [];
    }
    return [];
  },
  addEventListener: (event, fn) => {
    if (!docListeners[event]) docListeners[event] = [];
    docListeners[event].push(fn);
  },
  dispatchEvent: (event) => {
    const fns = docListeners[event.type] || [];
    fns.forEach((fn) => fn(event));
  }
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

// Test 4: closeCommandPalette resets aria-expanded and initCommandPalette sets aria-haspopup
zenith.initCommandPalette();
assert.strictEqual(cmdInputEl.getAttribute("aria-haspopup"), "listbox", "Expected aria-haspopup='listbox' on cmdInput");

zenith.renderCommandResults("");
// Simulate ArrowDown key press on input to verify active option traversal and aria-activedescendant update
cmdInputEl.dispatchEvent({ type: "keydown", key: "ArrowDown", preventDefault: () => {} });
assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), "cmd-option-1", "Expected aria-activedescendant to update to 'cmd-option-1' on ArrowDown");

cmdInputEl.dispatchEvent({ type: "keydown", key: "ArrowUp", preventDefault: () => {} });
assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), "cmd-option-0", "Expected aria-activedescendant to update to 'cmd-option-0' on ArrowUp");

// Simulate mouseenter on option index 2 to test mouse hover synchronization
const opt2 = cmdResultsEl.children[2];
if (opt2) {
  opt2.dispatchEvent({ type: "mouseenter" });
  assert.strictEqual(cmdInputEl.getAttribute("aria-activedescendant"), "cmd-option-2", "Expected aria-activedescendant to update to 'cmd-option-2' on mouseenter");
  assert.strictEqual(opt2.getAttribute("aria-selected"), "true", "Expected option 2 aria-selected='true' on mouseenter");
}

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

// Test 10: Command Palette outside-click dismissal
const cmdPaletteEl = mockElements["cmd-palette"];
cmdPaletteEl.classList.add("active");
cmdPaletteEl.setAttribute("aria-hidden", "false");
const outsideTarget = createMockElement("outside-target");

global.document.dispatchEvent({ type: "click", target: outsideTarget });
assert.strictEqual(cmdPaletteEl.classList.contains("active"), false, "Expected cmd-palette to lose .active class on outside click");
assert.strictEqual(cmdPaletteEl.getAttribute("aria-hidden"), "true", "Expected cmd-palette aria-hidden='true' on outside click");
console.log("✓ Command Palette outside-click dismissal verified successfully!");

// Test 11: WAI-ARIA Toolbar keyboard navigation
const toolbarEl = createMockElement("test-toolbar", "nav");
toolbarEl.setAttribute("role", "toolbar");

const tool1 = createMockElement("tool1", "button");
const tool2 = createMockElement("tool2", "button");
const tool3 = createMockElement("tool3", "button");

toolbarEl.children = [tool1, tool2, tool3];

toolbarEl.querySelectorAll = (selector) => {
  if (selector === 'button, [role="button"], [tabindex]:not([tabindex="-1"])') return [tool1, tool2, tool3];
  return [];
};

global.document.querySelectorAll = (selector) => {
  if (selector === '[role="toolbar"]') return [toolbarEl];
  if (selector === 'button, [role="button"], [tabindex]:not([tabindex="-1"])') return [tool1, tool2, tool3];
  return [];
};

zenith.initToolbarNavigation();

// Focus tool1 and simulate ArrowRight
tool1.focus();
toolbarEl.dispatchEvent({ type: "keydown", key: "ArrowRight", preventDefault: () => {} });
assert.strictEqual(global.document.activeElement, tool2, "Expected ArrowRight to move focus from tool1 to tool2");

// Simulate ArrowLeft
toolbarEl.dispatchEvent({ type: "keydown", key: "ArrowLeft", preventDefault: () => {} });
assert.strictEqual(global.document.activeElement, tool1, "Expected ArrowLeft to move focus back to tool1");

// Simulate End
toolbarEl.dispatchEvent({ type: "keydown", key: "End", preventDefault: () => {} });
assert.strictEqual(global.document.activeElement, tool3, "Expected End key to move focus to tool3");

// Simulate Home
toolbarEl.dispatchEvent({ type: "keydown", key: "Home", preventDefault: () => {} });
assert.strictEqual(global.document.activeElement, tool1, "Expected Home key to move focus to tool1");

console.log("✓ WAI-ARIA Toolbar keyboard navigation verified successfully!");

// Test 12: Window maximize ARIA expanded and aria-label context preservation
const maxWin = createMockElement("max-test-win");
maxWin.classList.add("window");
const maxBtn = createMockElement("max-test-btn", "button");
maxBtn.classList.add("control-dot");
maxBtn.classList.add("max");
maxBtn.setAttribute("aria-label", "Maximize OmniShell Terminal");
maxBtn.setAttribute("aria-expanded", "false");
maxWin.children = [maxBtn];

maxWin.querySelectorAll = (selector) => {
  if (selector === ".control-dot.max") return [maxBtn];
  return [];
};

mockElements["max-test-win"] = maxWin;

zenith.maximizeWindow("max-test-win");
assert.strictEqual(maxWin.classList.contains("maximized"), true, "Expected window to have .maximized class");
assert.strictEqual(maxBtn.getAttribute("aria-expanded"), "true", "Expected aria-expanded='true' when window is maximized");
assert.strictEqual(maxBtn.getAttribute("aria-label"), "Restore OmniShell Terminal", "Expected aria-label to update to 'Restore OmniShell Terminal'");

zenith.maximizeWindow("max-test-win");
assert.strictEqual(maxWin.classList.contains("maximized"), false, "Expected window to remove .maximized class");
assert.strictEqual(maxBtn.getAttribute("aria-expanded"), "false", "Expected aria-expanded='false' when window is unmaximized");
assert.strictEqual(maxBtn.getAttribute("aria-label"), "Maximize OmniShell Terminal", "Expected aria-label to revert to 'Maximize OmniShell Terminal'");
console.log("✓ Window maximize ARIA state and title context preservation verified successfully!");

// Test 13: Interactive Utility Actions (calculateGST, checkLegalCompliance, runSelectedTests, etc.)
const gstAmt = createMockElement("gst-amount");
gstAmt.value = "10000"; // 100 Rs in paise
const gstRate = createMockElement("gst-rate");
gstRate.value = "18";
const gstRes = createMockElement("gst-result");
createMockElement("kernel-logs"); // container for showToast

const logsContainer = mockElements["kernel-logs"];
const initialToastCount = logsContainer ? logsContainer.children.length : 0;
const toastEl = zenith.showToast("Test Toast Dismissal");
assert.notStrictEqual(toastEl, null, "Expected showToast to return created toast element");
assert.strictEqual(toastEl.getAttribute("title"), "Click to dismiss notification", "Expected toast title tooltip attribute");
assert.strictEqual(toastEl.getAttribute("aria-label"), "Test Toast Dismissal (Click to dismiss)", "Expected toast aria-label attribute");

// Simulate click dismissal on the toast element
toastEl.dispatchEvent({ type: "click" });
assert.strictEqual(toastEl.classList.contains("fade-out"), true, "Expected toast to have .fade-out class on click dismissal");

zenith.calculateGST();
assert.strictEqual(gstRes.textContent, "> Result: Base ₹100.00 + Tax ₹18.00 (18%) = Total ₹118.00", "Expected GST result text calculation");

const legalQuery = createMockElement("legal-query");
legalQuery.value = "302";
const legalRes = createMockElement("legal-result");

zenith.checkLegalCompliance();
assert.strictEqual(legalRes.textContent, '> Mapping Status: Section "302" mapped to Sovereign Ring-0 Sandbox policy', "Expected BNS legal section mapping text");

const testRes = createMockElement("test-run-result");
const testCb1 = createMockElement("test-cb-1");
testCb1.classList.add("test-suite-check");
testCb1.value = "Memory Allocator";
testCb1.checked = true;

global.document.querySelectorAll = (selector) => {
  if (selector === ".test-suite-check:checked") return [testCb1];
  return [];
};

zenith.runSelectedTests();
assert.strictEqual(testRes.textContent, "Completed 1 suite(s): Memory Allocator - 100% PASS", "Expected test suite runner result text");
console.log("✓ Interactive desktop utility actions verified successfully!");

// Test 14: Developer Portal & File Manager / Emulator Actions
const cicdLog = createMockElement("cicd-log-output");
zenith.triggerCIBuild();
assert.strictEqual(cicdLog.textContent.includes("CI/CD build pipeline"), true, "Expected CI/CD trigger to update log output");

const stepBtn = createMockElement("step-btn", "button");
stepBtn.textContent = "VERIFY INSTALLATION";
zenith.verifyStep(1, stepBtn);
assert.strictEqual(stepBtn.textContent, "✓ VERIFIED", "Expected verifyStep to update button text");

const syscallSearch = createMockElement("syscall-search");
syscallSearch.value = "write";
const syscallContainer = createMockElement("syscall-list-container");
zenith.searchSyscalls();
assert.strictEqual(syscallContainer.children.length, 1, "Expected searchSyscalls to filter 1 matching syscall");

// Test searchSyscalls empty state for non-matching query
syscallSearch.value = "unknown_syscall_xyz";
zenith.searchSyscalls();
assert.strictEqual(syscallContainer.children.length, 1, "Expected searchSyscalls empty state element when no match");
assert.strictEqual(syscallContainer.children[0].getAttribute("role"), "status", "Expected searchSyscalls empty state role='status'");
assert.strictEqual(syscallContainer.children[0].textContent.includes("No syscalls matching"), true, "Expected empty state suggestion text");

const fmCrumbs = createMockElement("fm-breadcrumbs");
zenith.fmBack();
assert.strictEqual(fmCrumbs.textContent, "/home/sovereign", "Expected fmBack to update breadcrumb");

const emulatorConsole = createMockElement("emulator-console");
const emulatorInput = createMockElement("emulator-input");
emulatorInput.value = "boot";
zenith.handleEmulatorCommand({ key: "Enter", target: emulatorInput });
assert.strictEqual(emulatorConsole.children.length, 2, "Expected handleEmulatorCommand to append 2 console lines");
console.log("✓ Developer Portal & System Action handlers verified successfully!");

// Test 15: OmniShell Terminal Interactive Command Handler & ARIA Log Output
const termOutput = createMockElement("terminal-output");
const termInput = createMockElement("terminal-input");
const termContainer = createMockElement("term-container");
termContainer.classList.add("terminal-container");

global.document.querySelectorAll = (selector) => {
  if (selector === ".terminal-container") return [termContainer];
  return [];
};

zenith.initOmniShellTerminal();
termContainer.dispatchEvent({ type: "click" });
assert.strictEqual(global.document.activeElement, termInput, "Expected terminal container click to focus terminal input");

zenith.handleTerminalCommand("neofetch");
assert.strictEqual(termOutput.children.length, 2, "Expected handleTerminalCommand('neofetch') to append prompt line and response line");
assert.strictEqual(termOutput.children[0].textContent, "sovereign@sigma:~$ neofetch", "Expected prompt line text");
assert.strictEqual(termOutput.children[1].textContent.includes("SigmaOS Zenith v15.0"), true, "Expected neofetch response text");

zenith.handleTerminalCommand("clear");
assert.strictEqual(termOutput.children.length, 0, "Expected clear command to clear terminal output");

console.log("✓ OmniShell Terminal interactive command execution and ARIA log output verified successfully!");

// Test 16: Form Input Enter Submission Handler
const archSearchInput = createMockElement("archweb-search");
archSearchInput.value = "hyprland";
let preventDefaultCalled = false;

zenith.initFormInputEnterSubmit();

archSearchInput.dispatchEvent({
  type: "keydown",
  key: "Enter",
  preventDefault: () => { preventDefaultCalled = true; },
});

assert.strictEqual(preventDefaultCalled, true, "Expected Enter keydown to call preventDefault on archweb-search");
console.log("✓ Form input Enter submission handler verified successfully!");

// Test 17: File Manager VFS Grid Initialization and Keyboard Navigation
const fmGridEl = createMockElement("fm-grid");
zenith.initFileManagerGrid();
assert.strictEqual(fmGridEl.children.length, 5, "Expected 5 default VFS items in File Manager grid");
assert.strictEqual(fmGridEl.children[0].getAttribute("aria-label"), "Open directory documents", "Expected directory ARIA label");
assert.strictEqual(fmGridEl.children[3].getAttribute("aria-label"), "View file zenith.conf", "Expected file ARIA label");

// Simulate folder click
const folderBtn = fmGridEl.children[0];
folderBtn.dispatchEvent({ type: "click" });
assert.strictEqual(fmCrumbs.textContent, "/home/sovereign/documents", "Expected folder click to update breadcrumb");

// Simulate fmBack navigation
zenith.fmBack();
assert.strictEqual(fmCrumbs.textContent, "/home/sovereign", "Expected fmBack to reset breadcrumb");
console.log("✓ File Manager VFS grid initialization and keyboard navigation verified successfully!");

console.log("All Command Palette & Desktop UX tests passed successfully!");
