# 🌌 SigmaOS: Comprehensive UI/UX Design Plan & Specification

This document serves as the **Master Blueprint for SigmaOS UI/UX design**. It is engineered specifically for use with **Google Stitch** (or similar gen-AI UI tools) to rapidly create, iterate, and deploy interfaces.

The core philosophy is **Sovereign Cyberpunk**: an aesthetic that is aggressively futuristic, undeniably high-tech, yet ruthlessly optimized to consume absolute minimal system resources. It is entirely customizable and automation-friendly.

---

## 🎭 1. Core Visual Identity: "Sovereign Cyberpunk"

SigmaOS rejects bloated, heavy interfaces. The design must look like a high-end hacker terminal from 2077, but run seamlessly on a 10-year-old laptop or an NCERT educational tablet.

### 🎨 Color Palette (Neural-Neon)

* **Base/Background:** `#030303` (Vantablack) - Saves OLED battery and reduces eye strain.
* **Surface/Panels:** `rgba(10, 15, 20, 0.4)` (Frosted Obsidian)
* **Primary Accent:** `#00FFD2` (Quantum Cyan) - For active states, primary buttons, and borders.
* **Secondary Accent:** `#8A2BE2` (Electric Violet) - For secondary actions, AI interactions, metrics.
* **Alert/Warning:** `#FF0055` (Neural Red) - Errors and critical system warnings.
* **Success/Safe:** `#00FF00` (Terminal Green) - Verifications and safe actions.
* **Text (Primary):** `#E0E0E0` (Off-white)
* **Text (Muted):** `#606060` (Gunmetal)

### 🔤 Typography

* **Primary Font (UI & Code):** `Geist Mono`, `JetBrains Mono`, or `Fira Code`. Monospaced fonts ensure perfect alignment of data structures, logs, and terminal outputs.
* **Headers (Titles):** `Inter` or `Outfit` (Geometric, clean, sans-serif).

### 🧊 Effects & Textures (Ultra-Lightweight)

* **No High-Res Images:** **Zero external images or heavy PNGs/JPGs.** All icons MUST be inline SVGs.
* **Glassmorphism (Optimized):** Use `backdrop-filter: blur(4px)` very sparingly (only on top-level floating modals).
* **Borders & Glows:** The primary aesthetic driver is 1px solid borders (`#00FFD2`) with a subtle CSS `box-shadow` (e.g., `box-shadow: 0 0 5px #00FFD2;`) for active elements.
* **Animations:** Only use `transform` and `opacity` for CSS animations (e.g., sliding panels, fading text). Avoid animating `width`, `height`, or `box-shadow` to prevent layout thrashing and keep GPU usage near zero.

---

## 🚀 2. Design Principles for Google Stitch

When feeding this document to Google Stitch to generate UI components, enforce the following constraints:

1. **Automation & Hook Friendly:** Every actionable element (button, input, tab) MUST have a semantic, predictable HTML id and data attribute (e.g., `id="omni-prompt-btn" data-action="distribute"`). This allows SigmaOS's agentic layers (like Antigravity) to seamlessly interact with the DOM.
2. **Grid/Flexbox Foundations:** The layout must be strictly defined by CSS Grid and Flexbox. Fluidity is mandatory. The UI must scale flawlessly from a smartphone screen up to a 4K ultrawide monitor.
3. **Terminal-Centric Paradigm:** The UI should feel like an evolution of a command-line interface. Inputs should behave like prompt barriers; outputs should stream like standard out.

---

## 🛠️ 3. Absolute OS Component Architectures

Each category requires a meticulously standardized view that prioritizes performance. These define the "Everything Required" for SigmaOS.

### A. The Core Desktop/Workspace (Aura Shell)

* **Taskbar (Bottom/Right edge):** Ultra-thin 32px height/width. No text, only pure cyan SVGs. Displays localized time, battery node, and active Shards.
* **Start/Nexus Menu:** A fullscreen, glassmorphic grid overlay. Applications aren't "icons"; they are live-streaming ASCII widgets or real-time data visualizers (like CPU load or network graph).

### B. System Configuration & Personalization (`Sigma Theme Customizer`)

* **Layout:** Dual-pane split screen.
* **Left Pane (Controls):** Sliders for `border-radius` (sharp vs rounded cyberpunk), Dropdowns for Preset Themes (Midnight, NCERT Academic), and Color Pickers for CSS Variables.
* **Right Pane (Live Preview):** A sandbox component demonstrating the OS style changes in real-time. Since styles are injected via CSS variables, no redraw logic is required—instant performance.
* *Note: The `sigma_theme_customizer.py` backend dynamically generates the `sigma_theme_bundle.css` when changes are saved.*

### C. Intelligence Suite (Omni-Prompt, Auto-Agents)

* **Layout:** Split-pane multiplexer.
* **Input Zone (Bottom/Center):** A glowing, expanding textarea (`#00FFD2` border on focus). Resembles a command prompt (`> [Type command or prompt here...]`).
* **Target Selector:** Inline SVG checkboxes shaped like hex-nodes to select AI targets (Local Llama, ChatGPT, Claude).
* **Output Zone:** Streamed text responses in monospace. Each AI's response is housed in a distinct frosted card.

### D. System Orchestration & Dashboard (Sentinel, Task Manager)

* **Layout:** Masonry or strict Grid of "Shards" (Widgets).
* **Data Visualization:** ASCII-style bar charts instead of heavy canvas libraries. CSS-based progress bars for CPU/RAM usage.
* **Ledger Log:** A continuously scrolling terminal block on the right side of the screen displaying system events and verified Proofs in green monospaced text.

### E. File Explorer (OmniSearch)

* **List View Only:** Tree structures built entirely with `<ul>` and `<li>`. Folders are `[+] DIR_NAME`. Files are `- FILE_NAME.EXT`.
* **Keyboard First:** VIM-like navigation is heavily encouraged. Every element must be tabbable and highlight securely via CSS `:focus-visible` with a heavy solid `#00FFD2` outline.

### F. Terminal & Developer IDE (CodeForge)

* **Editor:** Borderless `textarea` or CodeMirror instance initialized on an empty black void.
* **Gutter:** Line numbers in muted gunmetal `#606060`. Active line highlighted with a very faint `#00FFD2` background (`rgba(0, 255, 210, 0.05)`).
* **Output Scaffold:** A bottom docking terminal that uses WebGL text rendering strictly to avoid DOM node bloat when pumping thousands of lines of logs.

### G. NCERT Virtual Labs (Education & Simulation)

* **Concept:** "Holographic Blueprint."
* **Palette Override:** While maintaining the dark theme, labs use high-contrast cyan grids on the background.
* **Interactive Elements:** Skeuomorphic-but-flat vectors. For a physics lab, a pendulum is a simple white SVG line.

---

## 🧩 4. Component Library for AI Generators

To construct these screens, prompt Stitch to build these exact CSS/React components:

### 1. The 'Sovereign Button'

* **Idle:** Dark transparent background `rgba(0, 255, 210, 0.05)`, text `#00FFD2`, 1px solid border `#00FFD2`. Sharp corners (0px border-radius) or precisely angled (clip-path).
* **Hover:** Background `#00FFD2`, text `#030303`. Fast transition (100ms) on `background-color`.
* **Attributes:** `data-sigma-intent="click"`.

### 2. The 'Data Terminal Input'

* **Idle:** Background absolute black `#000000`, bottom border only `2px solid #333333`.
* **Focus:** Bottom border transitions to `#00FFD2` with a subtle glow. Text color `#E0E0E0`, caret color `#00FFD2`. Prefix with a non-editable `>` symbol.
* **Attributes:** `data-sigma-intent="input"`.

### 3. The 'Shard Grid Panel'

* **Structure:** `display: grid; gap: 1px; background: #333333;`. The 1px gap forms natural, ultra-thin borders between frosted child panels.
* **Child Panels:** `background: #0A0F14; padding: 1.5rem;`
* **Attributes:** `data-sigma-shard-container="true"`.

---

## ⚡ 5. Customization & Future-Proofing (Theme Engine)

SigmaOS uses CSS Variables (`--var-name`) globally. The UI must be fully driven by these variables to allow instant theme swapping. See `sigma_theme_customizer.py`.

```css
:root {
  --sigma-bg: #030303;
  --sigma-surface: rgba(10, 15, 20, 0.4);
  --sigma-border: #333333;
  --sigma-accent-primary: #00FFD2;
  --sigma-accent-secondary: #8A2BE2;
  --sigma-text-main: #E0E0E0;
  --sigma-text-muted: #606060;
  --sigma-font-mono: 'JetBrains Mono', monospace;
  --sigma-glass-blur: 4px;
  --sigma-border-radius: 0px;
}
```

By keeping the styling strictly decoupled from the DOM (meaning no inline styles, no complex utility classes unless cleanly abstracted), the UI is future-proof. If the user wants a "Light Academic Mode" for studying, simply swapping the CSS variables instantly changes the entire OS aesthetic. A Python script controls this OS-wide via dynamic file replacement.

---

## 🤖 6. Integration with Automation Programs

The UI is built to be piloted by an AI or a script.

* **Invisible Hooks:** Elements have `aria-labels` and `data-sigma-intent` attributes describing their function. This completely eliminates UI brittleness.
* An automation crawler can read: `<button data-sigma-intent="execute_prompt" id="btn-exec">Run</button>` and instantly know its purpose without relying on fragile XPath selectors, nested generic `div`s, or dynamic React classes.
* **Feedback Loops:** Every action triggers a visual pulse (a quick CSS `box-shadow` flash) AND dispatches a secure system event to the `Sovereign Interface Layer`, allowing local AI agents to visually and programmatically confirm task success.

---

## 🔮 7. Advanced Google Stitch Directives (Meta-Prompts)

To achieve 100% compliance when using **Google Stitch** to generate your screens, use the following expanded directives depending on the component type:

### A. The Core Desktop Wrapper

**Stitch Prompt:** *"Create a fullscreen web interface matching the 'Sovereign Cyberpunk' theme. The background is `#030303`. Create an ultra-thin 32px taskbar anchored to the bottom using `display: flex`. Ensure the taskbar contains only SVGs (no text). The main screen area should support a glassmorphic dashboard overlay (`rgba(10, 15, 20, 0.4)` with `backdrop-filter: blur(4px)`). Add semantic IDs and `data-sigma-intent` attributes."*

### B. The Terminal / Log Viewer

**Stitch Prompt:** *"Create a developer terminal UI panel. The background must be strictly `#000000` with no borders, except for a 1px solid `#333333` top boundary. The left margin should act as a gutter containing muted `#606060` line numbers. All text must use `Geist Mono` or monospace, colored `#E0E0E0`, and overflowing content must scroll natively seamlessly. Ensure no GPU-heavy layout recalculations occur on log append."*

### C. The Configuration / Settings Panel

**Stitch Prompt:** *"Create a dual-pane split screen configuration menu. The left pane should contain HTML5 range sliders and select dropdowns styled entirely via CSS to match the Sovereign Cyberpunk theme (Cyan `#00FFD2` active tracks). The right pane must be a live-preview module matching the Shard Grid specs. Ensure zero external dependencies. Use semantic data attributes on all sliders like `data-sigma-var='border-radius'`."*

---

**Summary Master Prompt for Google Stitch:**
*"Create a highly responsive, zero-image, CSS-grid-based web interface for an operating system tool. Use a dark 'Sovereign Cyberpunk' theme with an obsidian background, frosted glass panels, and quantum cyan (#00FFD2) glowing borders. Use strictly monospaced fonts (Geist Mono). Ensure all buttons and inputs have semantic IDs and data attributes for AI automation. The design must be extremely lightweight, utilizing only CSS for styling and animations. Build a Shard Grid layout populated with Sovereign Buttons and Data Terminal Inputs."*
