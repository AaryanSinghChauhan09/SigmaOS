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

console.log("All Command Palette UX tests passed successfully!");
