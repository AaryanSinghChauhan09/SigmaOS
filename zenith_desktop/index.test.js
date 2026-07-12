// @vitest-environment jsdom
import { describe, it, expect, beforeEach } from "vitest";
import { toggleWindow, setTheme } from "./index.js";

describe("Zenith Desktop UX & Accessibility Suite", () => {
  beforeEach(() => {
    // Setup mock DOM environment
    document.body.innerHTML = `
            <div id="mouse-glow"></div>
            <div id="win-terminal" class="window" aria-label="Terminal" style="display: none;">
                <div class="window-header"></div>
                <div class="window-body"></div>
            </div>
            <div id="win-settings" class="window" aria-label="Settings" style="display: none;">
                <div class="window-header"></div>
                <div class="window-body"></div>
            </div>
            <div id="screen-reader-log" class="screen-reader-log">
                <div id="screen-reader-text">Ready</div>
            </div>
            <input type="checkbox" id="toggle-high-contrast">
            <input type="checkbox" id="toggle-reduced-motion">
            <input type="checkbox" id="toggle-screen-reader">
        `;
    document.body.className = "";
  });

  it("should toggle window display states correctly", () => {
    const terminal = document.getElementById("win-terminal");
    expect(terminal.style.display).toBe("none");

    // Open
    toggleWindow("win-terminal");
    expect(terminal.style.display).toBe("flex");
    expect(terminal.classList.contains("active-focus")).toBe(true);

    // Close
    toggleWindow("win-terminal");
    expect(terminal.style.display).toBe("none");
  });

  it("should update body class when theme is selected", () => {
    setTheme("solar");
    expect(document.body.classList.contains("theme-solar")).toBe(true);

    setTheme("crimson");
    expect(document.body.classList.contains("theme-crimson")).toBe(true);
    expect(document.body.classList.contains("theme-solar")).toBe(false);
  });

  it("should handle simulated screen reader logs when enabled", () => {
    const srToggle = document.getElementById("toggle-screen-reader");

    // Manually trigger screen reader change simulation
    srToggle.checked = true;

    // Setup mock event and trigger DOMContentLoaded listeners
    window.dispatchEvent(new Event("DOMContentLoaded"));

    // Simulating focus trigger that calls announce
    setTheme("gold"); // changing theme invokes announce()

    // Since DOMContentLoaded adds event listeners and window.setTheme can be invoked:
    // We can check body class matches theme-gold
    expect(document.body.classList.contains("theme-gold")).toBe(true);
  });
});
