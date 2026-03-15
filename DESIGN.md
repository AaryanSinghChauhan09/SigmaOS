# 🌌 SigmaOS: Comprehensive UI/UX Design Plan & Specification

This document serves as the **Master Blueprint for SigmaOS UI/UX design**. It is engineered specifically for use with **Google Stitch** (or similar gen-AI UI tools) to rapidly create, iterate, and deploy interfaces. 

The core philosophy is **Sovereign Cyberpunk**: an aesthetic that is aggressively futuristic, undeniably high-tech, yet ruthlessly optimized to consume absolute minimal system resources.

---

## 🎭 1. Core Visual Identity: "Sovereign Cyberpunk"

SigmaOS rejects bloated, heavy interfaces. The design must look like a high-end hacker terminal from 2077, but run seamlessly on a 10-year-old laptop or an NCERT educational tablet.

### 🎨 Color Palette (Neural-Neon)
*   **Base/Background:** `#030303` (Vantablack) - Saves OLED battery and reduces eye strain.
*   **Surface/Panels:** `rgba(10, 15, 20, 0.4)` (Frosted Obsidian)
*   **Primary Accent:** `#00FFD2` (Quantum Cyan) - For active states, primary buttons, and borders.
*   **Secondary Accent:** `#8A2BE2` (Electric Violet) - For secondary actions, AI interactions, metrics.
*   **Alert/Warning:** `#FF0055` (Neural Red) - Errors and critical system warnings.
*   **Success/Safe:** `#00FF00` (Terminal Green) - Verifications and safe actions.
*   **Text (Primary):** `#E0E0E0` (Off-white)
*   **Text (Muted):** `#606060` (Gunmetal)

### 🔤 Typography
*   **Primary Font (UI & Code):** `Geist Mono`, `JetBrains Mono`, or `Fira Code`. Monospaced fonts ensure perfect alignment of data structures, logs, and terminal outputs.
*   **Headers (Titles):** `Inter` or `Outfit` (Geometric, clean, sans-serif).

### 🧊 Effects & Textures (Ultra-Lightweight)
*   **No High-Res Images:** **Zero external images or heavy PNGs/JPGs.** All icons MUST be inline SVGs.
*   **Glassmorphism (Optimized):** Use `backdrop-filter: blur(4px)` very sparingly (only on top-level floating modals).
*   **Borders & Glows:** The primary aesthetic driver is 1px solid borders (`#00FFD2`) with a subtle CSS `box-shadow` (e.g., `box-shadow: 0 0 5px #00FFD2;`) for active elements.
*   **Animations:** Only use `transform` and `opacity` for CSS animations (e.g., sliding panels, fading text). Avoid animating `width`, `height`, or `box-shadow` to prevent layout thrashing and keep GPU usage near zero.

---

## 🚀 2. Design Principles for Google Stitch

When feeding this document to Google Stitch to generate UI components, enforce the following constraints:

1.  **Automation & Hook Friendly:** Every actionable element (button, input, tab) MUST have a semantic, predictable HTML id and data attribute (e.g., `id="omni-prompt-btn" data-action="distribute"`). This allows SigmaOS's agentic layers (like Antigravity) to seamlessly interact with the DOM.
2.  **Grid/Flexbox Foundations:** The layout must be strictly defined by CSS Grid and Flexbox. Fluidity is mandatory. The UI must scale flawlessly from a smartphone screen up to a 4K ultrawide monitor.
3.  **Terminal-Centric Paradigm:** The UI should feel like an evolution of a command-line interface. Inputs should behave like prompt barriers; outputs should stream like standard out.

---

## 🛠️ 3. Application-Specific UI/UX Architectures

SigmaOS contains over 100+ sharded modules. Here is the design blueprint for the major categories:

### A. Intelligence Suite & AI Interfacing (e.g., Omni-Prompt Distributor)
*   **Layout:** Split-pane multiplexer. 
*   **Input Zone (Bottom/Center):** A glowing, expanding textarea (`#00FFD2` border on focus). Resembles a command prompt (`> [Type command or prompt here...]`).
*   **Target Selector:** Inline SVG checkboxes shaped like hex-nodes to select AI targets (Local Llama, ChatGPT, Claude).
*   **Output Zone:** Streamed text responses in monospace. Each AI's response is housed in a distinct frosted card.

### B. NCERT Virtual Labs (Education & Simulation)
*   **Concept:** "Holographic Blueprint."
*   **Palette Override:** While maintaining the dark theme, labs use high-contrast cyan grids on the background.
*   **Interactive Elements:** Skeuomorphic-but-flat vectors. For a physics lab, a pendulum is a simple white SVG line glowing cyan; for chemistry, beakers are outlined shapes with CSS-animated fluid heights.
*   **Controls:** Slider inputs that look like mixing desk faders. Numeric inputs with up/down micro-buttons.

### C. System Orchestration & Dashboard (e.g., Sentinel, Task Manager)
*   **Layout:** Masonry or strict Grid of "Shards" (Widgets).
*   **Data Visualization:** ASCII-style bar charts instead of heavy canvas libraries. CSS-based progress bars for CPU/RAM usage.
*   **Ledger Log:** A continuously scrolling terminal block on the right side of the screen displaying system events and verified Proofs in green monospaced text.

### D. Productivity Suite (CodeForge, Writer)
*   **Layout:** Zen-mode maximized.
*   **Sidebar:** Collapsible, ultra-thin left sidebar for file trees. Folders are represented by simple bracketing `[+] folder_name`.
*   **Editor:** Borderless text area. Line numbers in muted gunmetal `#606060`. Active line highlighted with a very faint `#00FFD2` background (`rgba(0, 255, 210, 0.05)`).

---

## ⚡ 4. Customization & Future-Proofing (Theme Engine)

SigmaOS uses CSS Variables (`--var-name`) globally. The UI must be fully driven by these variables to allow instant theme swapping.

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
}
```

By keeping the styling strictly decoupled from the DOM (meaning no inline styles, no complex utility classes unless cleanly abstracted), the UI is future-proof. If the user wants a "Light Academic Mode" for studying, simply swapping the CSS variables instantly changes the entire OS aesthetic.

---

## 🤖 5. Integration with Automation Programs

The UI is built to be piloted by an AI or a script. 
*   **Invisible Hooks:** Elements have `aria-labels` and `data-sigma-intent` attributes describing their function. 
*   An automation crawler can read: `<button data-sigma-intent="execute_prompt" id="btn-exec">Run</button>` and instantly know its purpose without relying on fragile XPath selectors.
*   **Feedback Loops:** Every action triggers a visual pulse (a quick CSS `box-shadow` flash) AND dispatches a secure system event to the `Sovereign Interface Layer`, allowing local AI agents to visually and programmatically confirm task success.

---
**Summary for Google Stitch Prompting:**
*"Create a highly responsive, zero-image, CSS-grid-based web interface for an operating system tool. Use a dark 'Sovereign Cyberpunk' theme with an obsidian background, frosted glass panels, and quantum cyan (#00FFD2) glowing borders. Use strictly monospaced fonts (Geist Mono). Ensure all buttons and inputs have semantic IDs and data attributes for AI automation. The design must be extremely lightweight, utilizing only CSS for styling and animations."*
