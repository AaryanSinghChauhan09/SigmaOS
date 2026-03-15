# SigmaOS: Master UI/UX Design Blueprint

This document is the **single source of truth** for all SigmaOS design decisions. It is structured for use
with **Google Stitch** or any generative AI UI tool, and is simultaneously the canonical reference for
developers, automation scripts, and AI agents building or customising the OS.

**Core Philosophy — Sovereign Cyberpunk:** Aggressively futuristic, high-tech, yet ruthlessly optimised
for minimum resource consumption. Fully automatable, deeply customisable, and personally adaptable.

---

## 1. Core Visual Identity

### 1.1 Color Palette — Neural-Neon

| Token | Hex Value | Usage |
| --- | --- | --- |
| `--sigma-bg` | `#030303` | Vantablack background — saves OLED power, reduces eye strain |
| `--sigma-surface` | `rgba(10, 15, 20, 0.4)` | Frosted Obsidian — panel/card background |
| `--sigma-border` | `#333333` | Default border |
| `--sigma-accent-primary` | `#00FFD2` | Quantum Cyan — active states, primary actions, glows |
| `--sigma-accent-secondary` | `#8A2BE2` | Electric Violet — AI interactions, secondary actions |
| `--sigma-alert` | `#FF0055` | Neural Red — errors, critical events |
| `--sigma-success` | `#00FF00` | Terminal Green — verified, safe states |
| `--sigma-warning` | `#FFB800` | Amber — anomalies, caution |
| `--sigma-text-main` | `#E0E0E0` | Off-white primary text |
| `--sigma-text-muted` | `#606060` | Gunmetal secondary/hint text |

### 1.2 Typography

- **Monospace (UI, Code, Data):** `Geist Mono`, `JetBrains Mono`, or `Fira Code`
- **Sans-serif (Headings only):** `Inter` or `Outfit`
- All data, logs, metrics, passwords, and terminal output **must** use the monospace stack.

### 1.3 Effects and Textures

- **No external images.** Zero PNGs, JPGs, or WebP assets. All icons are inline SVGs.
- **Glassmorphism:** `backdrop-filter: blur(4px)` — used sparingly, only on top-level floating modals.
- **Borders and Glows:** `1px solid #00FFD2` with `box-shadow: 0 0 5px #00FFD2` for active elements.
- **Animations:** Only `transform` and `opacity`. Never animate `width`, `height`, `margin`, or `padding`.

---

## 2. Full System-Wide Design Token Map

All tokens are overrideable at `:root` scope. The Theme Engine writes these dynamically.

```css
:root {
  /* Color */
  --sigma-bg: #030303;
  --sigma-surface: rgba(10, 15, 20, 0.4);
  --sigma-border: #333333;
  --sigma-accent-primary: #00FFD2;
  --sigma-accent-secondary: #8A2BE2;
  --sigma-alert: #FF0055;
  --sigma-success: #00FF00;
  --sigma-warning: #FFB800;
  --sigma-text-main: #E0E0E0;
  --sigma-text-muted: #606060;

  /* Typography */
  --sigma-font-mono: 'JetBrains Mono', monospace;
  --sigma-font-sans: 'Inter', sans-serif;
  --sigma-font-size-base: 14px;

  /* Effects */
  --sigma-glass-blur: 4px;
  --sigma-border-radius: 0px;          /* 0 = sharp cyberpunk; increase for soft mode */
  --sigma-transition-speed: 150ms;

  /* Layout */
  --sigma-taskbar-size: 32px;
  --sigma-sidebar-width: 240px;
  --sigma-panel-gap: 1px;
  --sigma-shard-padding: 1.5rem;
  --sigma-layout-cols: 12;
  --sigma-content-max-width: 1800px;

  /* Scrollbar */
  --sigma-scrollbar-width: 4px;
  --sigma-scrollbar-color: #333333;
}
```

---

## 3. Automation, Customisation and Personalisation Architecture

This section defines how every design element connects to automation scripts, AI agents, and user
personalisation workflows.

### 3.1 Automation Hooks

Every interactive element MUST carry both `id` and `data-sigma-intent` attributes:

```html
<button id="btn-exec-prompt"
        data-sigma-intent="execute_prompt"
        data-sigma-target="ai-output-pane"
        aria-label="Execute prompt across all selected AI targets">
  Run
</button>
```

Rules:
- `id` must be globally unique, kebab-case, semantically descriptive.
- `data-sigma-intent` describes the action in verb-noun form (e.g., `open_file`, `toggle_theme`).
- `data-sigma-target` (optional) points to the affected element's `id`.
- All SVG icons need `aria-label`. All form inputs need `aria-describedby` pointing to a label.

An AI crawler (or Antigravity agent) can traverse the DOM using only these attributes:

```js
// Example: AI agent clicks the run button
document.querySelector('[data-sigma-intent="execute_prompt"]').click();
```

### 3.2 Personalisation Layer — CSS Variable Injection

The Theme Engine (`sigma_theme_customizer.py`) reads a user JSON profile and writes a theme bundle:

```json
{
  "theme_name": "Midnight Academic",
  "overrides": {
    "--sigma-accent-primary": "#00BFFF",
    "--sigma-border-radius": "4px",
    "--sigma-font-size-base": "15px",
    "--sigma-taskbar-size": "40px"
  }
}
```

The Python backend merges overrides into `sigma_theme_bundle.css` at `:root` scope. The OS reloads
styles without a page refresh. No redraw logic required.

### 3.3 Customisation Presets

| Preset Name | Description | Key Token Changes |
| --- | --- | --- |
| `Sovereign Dark` | Default — Vantablack + Cyan | No changes from `:root` |
| `Midnight Academic` | Study mode — softer blue accent | `--sigma-accent-primary: #00BFFF`, `--sigma-border-radius: 4px` |
| `NCERT Holograph` | Education — bright cyan grid | `--sigma-bg: #010A14`, grid overlay enabled |
| `Blade Runner Red` | Alternate accent — red neon | `--sigma-accent-primary: #FF0055`, `--sigma-accent-secondary: #FF6600` |
| `Ghost Protocol` | Stealth — ultra-muted palette | `--sigma-accent-primary: #444444`, `--sigma-text-muted: #333333` |
| `Solar Flare` | Light academic mode | `--sigma-bg: #F5F5F0`, `--sigma-text-main: #1A1A1A` |
| `Violet Storm` | Deep violet AI mode | `--sigma-bg: #08000F`, `--sigma-accent-primary: #BF5FFF` |

### 3.4 Per-User Profile Schema

```json
{
  "user_profile": {
    "layout_mode": "sigma-layout-default",
    "theme_preset": "Sovereign Dark",
    "custom_tokens": {},
    "font_scale": 1.0,
    "reduce_motion": false,
    "taskbar_position": "bottom",
    "sidebar_collapsed": false,
    "notification_level": "all"
  }
}
```

All profile fields map directly to CSS tokens or `data-sigma-*` body attributes, ensuring full
programmatic control by both the user and automation agents.

---

## 4. Component Library

### 4.1 Sovereign Button

| State | Style |
| --- | --- |
| Idle | `background: rgba(0,255,210,0.05)`, `color: #00FFD2`, `border: 1px solid #00FFD2` |
| Hover | `background: #00FFD2`, `color: #030303` — `100ms` transition |
| Active | `transform: scale(0.97)` — instant feedback |
| Disabled | `opacity: 0.3`, `cursor: not-allowed` |
| Danger | Idle border = `#FF0055`, hover fill = `#FF0055` |

Required attributes: `id="[action]-btn"`, `data-sigma-intent="[verb_noun]"`, `aria-label="[description]"`.

### 4.2 Data Terminal Input

| State | Style |
| --- | --- |
| Idle | `background: #000000`, bottom border only `2px solid #333333` |
| Focus | Bottom border `#00FFD2`, `caret-color: #00FFD2`, text `#E0E0E0` |

Prefix with a non-editable `>` glyph. Required attributes: `data-sigma-intent="input"`.

### 4.3 Shard Grid Panel

```css
.sigma-shard-container {
  display: grid;
  gap: var(--sigma-panel-gap);       /* 1px — creates natural borders */
  background: var(--sigma-border);
}

.sigma-shard {
  background: #0A0F14;
  padding: var(--sigma-shard-padding);
}
```

Required: `data-sigma-shard-container="true"` on the parent.

### 4.4 Toggle Switch (Pure CSS)

```css
.sigma-toggle { appearance: none; width: 36px; height: 20px;
  background: #333333; border-radius: 10px; cursor: pointer;
  transition: background var(--sigma-transition-speed); }
.sigma-toggle:checked { background: var(--sigma-accent-primary); }
.sigma-toggle::before { content: ''; position: absolute;
  width: 14px; height: 14px; background: white; border-radius: 50%;
  transition: transform var(--sigma-transition-speed); }
.sigma-toggle:checked::before { transform: translateX(16px); }
```

### 4.5 Status Badge

```html
<span class="sigma-badge" data-sigma-status="safe" aria-label="Status: Safe">SAFE</span>
```

```css
.sigma-badge[data-sigma-status="safe"]     { color: #00FF00; border-color: #00FF00; }
.sigma-badge[data-sigma-status="warning"]  { color: #FFB800; border-color: #FFB800; }
.sigma-badge[data-sigma-status="critical"] { color: #FF0055; border-color: #FF0055; }
```

### 4.6 Toast Notification

```css
.sigma-toast {
  position: fixed;
  bottom: calc(var(--sigma-taskbar-size) + 1rem);
  right: 1rem;
  width: 320px;
  border-left: 3px solid var(--sigma-accent-primary);
  background: var(--sigma-surface);
  padding: 0.75rem 1rem;
  font-family: var(--sigma-font-mono);
  animation: sigma-toast-in 200ms ease-out forwards;
}

@keyframes sigma-toast-in {
  from { transform: translateX(calc(100% + 1rem)); opacity: 0; }
  to   { transform: translateX(0); opacity: 1; }
}
```

Variants: `data-sigma-toast-type="info|success|warning|critical|agent"`.

---

## 5. Layout System

### 5.1 Workspace Layout Modes

Apply by setting `data-sigma-layout` on `<body>`:

| Layout Token | Description | Ideal Use Case |
| --- | --- | --- |
| `default` | 12-column Shard Grid | General productivity |
| `focus` | Single maximised panel, no sidebar | Deep work / coding |
| `split` | 50/50 horizontal dual pane | Side-by-side comparing |
| `terminal` | Full-screen terminal, minimal chrome | SSH / developer sessions |
| `dashboard` | Masonry grid of live-data shards | System monitoring |
| `edu` | Large content area, nav on left | NCERT Virtual Labs |
| `gaming` | Fullscreen canvas, collapsed taskbar | Gaming mode |
| `minimal` | Borderless, no decorations | Presentation / stealth |

### 5.2 Responsive Breakpoints (CSS Only)

```css
/* Mobile / Educational Device */
@media (max-width: 768px) {
  :root {
    --sigma-sidebar-width: 0px;
    --sigma-shard-padding: 0.75rem;
    --sigma-layout-cols: 4;
    --sigma-font-size-base: 13px;
  }
}

/* Standard Laptop */
@media (min-width: 769px) and (max-width: 1440px) {
  :root {
    --sigma-sidebar-width: 200px;
    --sigma-layout-cols: 8;
    --sigma-font-size-base: 14px;
  }
}

/* Desktop / 1440p */
@media (min-width: 1441px) and (max-width: 2560px) {
  :root {
    --sigma-sidebar-width: 260px;
    --sigma-layout-cols: 12;
    --sigma-font-size-base: 15px;
  }
}

/* Ultrawide / 4K+ */
@media (min-width: 2561px) {
  :root {
    --sigma-sidebar-width: 340px;
    --sigma-layout-cols: 24;
    --sigma-shard-padding: 2.5rem;
    --sigma-font-size-base: 17px;
  }
}
```

### 5.3 Gaming HUD Overlay

```css
body[data-sigma-layout="gaming"] .sigma-taskbar { height: 0; overflow: hidden; }

body[data-sigma-layout="gaming"] .sigma-hud {
  position: fixed;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: var(--sigma-font-mono);
  font-size: 10px;
  color: var(--sigma-text-muted);
  pointer-events: none;
  z-index: 9999;
}
```

HUD contains: FPS counter, CPU+RAM 2px progress bar, OS mode label. All `pointer-events: none`.

---

## 6. Performance Guidelines — 144Hz / Zero-Jitter

- **Animate only:** `opacity` and `transform`. Never `width`, `height`, `margin`, `box-shadow` in transitions.
- **Timing:** `transition: all 150ms cubic-bezier(0.2, 0.8, 0.2, 1)` — sharp and mechanical.
- **State changes (buttons, toggles):** 50ms max response.
- **Glassmorphism:** Use `will-change: transform` only on actively transitioning modals. Remove after animation.
- **Scrollbars:** Custom 4px scrollbars via `scrollbar-width: thin; scrollbar-color: var(--sigma-scrollbar-color) transparent;`.

---

## 7. Accessibility and Machine Readability

- Every SVG icon: `aria-label="[description]"` and `role="img"`.
- All inputs: `aria-describedby` pointing to their visible label.
- `:focus-visible` outline: `2px solid var(--sigma-accent-primary)` — prominent, consistent everywhere.
- Logical `tabindex` — tab order must follow reading order; every app must be fully keyboard-navigable.
- For AI agents: `document.querySelectorAll('[data-sigma-intent]')` returns every interactive surface.

---

## 8. SVG and Icon Directives

- **Stroke over fill.** Use `stroke="currentColor"` with `stroke-width="1.5"`.
- **Crisp geometry.** Geometric shapes, sharp angles, no organic curves.
- **24×24 viewport** as default. Scale using CSS `width`/`height`, never `transform: scale`.
- **No external URLs.** All `<svg>` tags are inline in HTML/JSX.

---

## 9. Notification and Alert System

| Variant | Border Color | Auto-dismiss |
| --- | --- | --- |
| `info` | `#00FFD2` | 3s |
| `success` | `#00FF00` | 2s |
| `warning` | `#FFB800` | 5s (manual dismiss) |
| `critical` | `#FF0055` | Persistent until ACK |
| `agent` | `#8A2BE2` | 4s |

All notifications: bottom-right anchored, 320px wide, `translateX` entry animation, `opacity` exit.

---

## 10. Core OS Shell Design

### 10.1 Taskbar (Aura Shell)

- 32px thin bar, anchored to `var(--sigma-taskbar-position, bottom)`.
- Contents: Clock (monospace), Battery SVG arc gauge, Active Shard indicators, Mode label.
- No labels — SVG icons only unless accessibility mode is active.

### 10.2 Nexus Menu (App Launcher)

- Full-screen glass overlay on trigger.
- Apps displayed as a CSS Grid of 64×64 tiles. Each tile = app SVG icon + monospace name below.
- Live data widgets: CPU sparkline, RAM bar, Network status — rendered as ASCII art in tiles.
- Search bar at top: Data Terminal Input style, `id="nexus-search"`, filters tiles in real-time via DOM attribute toggle.

---

## 11. Per-Application Design Blueprints

---

### AI and Intelligence Apps

#### Prompt-o-Matic (Omni-Prompt Distributor)

- **Layout:** Vertical split — top 70% output pane, bottom 30% input strip.
- **Input bar:** Expanding `textarea` with `>` prefix. Glows `#00FFD2` on focus.
  - `id="omni-input"`, `data-sigma-intent="compose_prompt"`
- **AI Target Selector:** Horizontal row of pill toggles. Checked = filled `#00FFD2`.
  - `id="omni-targets"`, `data-sigma-intent="toggle_ai_target"`
- **Output cards:** One frosted shard per AI target. Header in `#8A2BE2`, streamed body in `#E0E0E0` monospace.
  - `id="omni-output-[model-slug]"`, `data-sigma-intent="display_response"`

#### Nexus AI (Local AI Assistant)

- **Layout:** Chat view. AI messages left (`#8A2BE2` left border), user messages right (`#00FFD2` right border).
- Sharp rectangular message blocks — no border-radius.
- Typing indicator: three `opacity`-pulsing dots, `aria-label="AI is typing"`.
- Input pinned to bottom, `Enter` submits, `Shift+Enter` newline.

#### AI Studio (Model Manager)

- **Layout:** Three vertical panes — Model Library | Config Panel | Test Console.
- Model cards: slim rows with name, size badge, status dot (green=loaded, muted=idle).
- Config faders: `temperature`, `context_window`, `top_p` — cyber slider style.

---

### Productivity and Office Apps

#### Writer (Sovereign Word Processor)

- **Layout:** Zen mode. Centered canvas, `max-width: 75ch`, on pure `#030303` void.
- Toolbar collapses to 1px line, expands on hover. SVG formatting icons only.
- Status bar (bottom): word count, line, column in `#606060` monospace.
- `id="writer-canvas"`, `data-sigma-intent="write_document"`.

#### CodeForge (IDE)

- **Layout:** 3-column — File Tree (200px) | Editor (flex) | Output Dock (200px, collapsible bottom).
- Syntax colors: keywords=`#00FFD2`, strings=`#8A2BE2`, comments=`#606060`, numbers=`#FFB800`.
- Active line: `rgba(0, 255, 210, 0.04)` full-width.
- File tree: box-drawing chars (`├─`, `└─`), SVG `▶/▼` fold carets.

#### PDF Forge

- **Layout:** Dual pane — drop zone (left, dashed `#333333` border) | live PDF preview (right).
- Toolbar: Merge, Split, Sign, Compress — all Sovereign Buttons in `display: flex` row.
- Drop zone: border transitions to `#00FFD2` solid on drag-over.

#### Excel Hub and Excel Validator

- Spreadsheet: `<div>` CSS Grid cells. Alternating rows `#030303` / `rgba(255,255,255,0.02)`.
- Active cell: `2px solid #00FFD2` outline.
- Formula bar: fixed top strip, `=` prefix, monospaced, `id="formula-bar"`.

#### Project Flow (Scrum / Gantt)

- Tab-switcher: Kanban | Gantt.
- Kanban: CSS Grid columns, cards = Shard panels with left-border status colors.
- Gantt: SVG horizontal timeline — task bars as CSS `background-color` fills on `<rect>`.

#### Markdown Viewer / Studio

- Side-by-side: raw monospace editor (left) | rendered preview (right).
- Preview CSS resets: headers in `#00FFD2`, code blocks in `#0A0F14` with monospace.

#### Quantum BI (Business Intelligence)

- Masonry Shard Grid of chart widgets.
- Charts: pure CSS bars with `background: linear-gradient(to top, var(--sigma-accent-primary), transparent)`.
- Filters: inline pill-tag row above chart groups.

#### Vortex Clipboard

- Compact floating overlay, 320px wide, pinned right edge.
- Clipboard entries: 2-line text preview with `text-overflow: ellipsis`.
- Copy / Delete SVG icons appear on row hover only.

#### Sovereign Forms Hub

- Form builder canvas: draggable field blocks (Text, Number, Radio, Checkbox, Date).
- Preview toggle switches between builder and read-only rendered form.

#### Project Flow / Chronos Vault / Event Matrix

- Calendar: CSS Grid. Active dates: `#00FFD2` highlight. Event bars below date numbers.
- Gantt/Event Matrix: SVG horizontal bars, category-color coded.

#### Spectral Analyzer

- Waveform: SVG `<polyline>`. Frequency bars: thin `<div>` elements, `#00FFD2` to `#FF0055` gradient.

---

### Security and Privacy Apps

#### Sentinel (System Monitor)

- Dashboard of 6-8 Shards: CPU, RAM, Disk, Network, GPU, Temperature, Battery, Process List.
- Metrics: progress bars use `width` transition, max 300ms. Exception warranted by live data UX.
- Process List: sortable `<table>`, sticky header, sorted column shows `↑/↓` SVG arrow.
- Alert: Shard left-border flashes `#FF0055` when metric exceeds 85%.

#### Shield (Security Suite)

- Left nav: Scan | Firewall | Vault | Logs. Right = content area.
- Scan: pulsing circular SVG loader. Results: scrolling threat list with severity badges.
- Badges: `SAFE=#00FF00`, `WARNING=#FFB800`, `CRITICAL=#FF0055`.

#### Vault Keep (Password Manager)

- Two pane: searchable item list (left) | credential detail (right).
- Strength meter: CSS width bar, `#00FF00` to `#FF0055` per entropy score.
- Password: hidden by default. Reveal on hold of eye SVG button.
- Monospace-only — zero decorative fonts on this screen.

#### Forensic Vault

- Wizard: Select Target → Acquire Image → Analyze → Export Report.
- Step indicator: horizontal progress bar via `::before` CSS counter.
- Output: read-only terminal block, `#00FF00` text on `#000000`.

#### Aegis Permissions

- Permission matrix: rows = apps, columns = permission types, cells = CSS Toggle Switches.
- On = `#00FFD2`, Off = `#333333`.

#### Chronos Vault (Scheduler / Time-Lock)

- Calendar CSS Grid. Event detail: frosted popover 8px below clicked date.

---

### Communication and Browser Apps

#### OmniBrowser

- URL bar full-width, `>_ ` prefix, `id="browser-url"`.
- Tabs: 40px strips, monospace title, active = `#00FFD2` bottom border.
- Privacy dot: green/red SVG `●` in URL bar for connection security.

#### MeshTalk (Encrypted Chat)

- Contacts sidebar (240px). Chat view flex-grows.
- Avatar: first letter in a CSS circle, `background: #8A2BE2`.
- Messages: alternating alignment, `2px` accent border.
- Lock SVG in chat header — gold fill when E2E encryption active.

#### Echo Cast (Broadcasting)

- Preview: 16:9 via `padding-bottom: 56.25%` trick.
- Controls: Record, Pause, Stop, Screenshot — large flex SVG buttons.
- Recording active: pulsing red `●` SVG dot overlaid top-left.

#### Social Hub

- Feed (center) + Trending sidebar (right).
- Post cards: Shard panels. Username in `#00FFD2`, body `#E0E0E0`.
- Reactions: inline geometric SVGs only — no emoji fonts.

#### Email Agent / Email Disco

- Three-pane: Folder tree (left) | Message list (center) | Reading pane (right).
- Unread count: `#00FFD2` badge on folder names.
- Reading pane: monospace-first, HTML email rendered in sanitised `<iframe>`.

---

### NCERT Educational Apps

All NCERT apps share:

- Background: faint `1px` cyan grid `rgba(0, 255, 210, 0.05)` — holographic blueprint feel.
- Simulations: pure inline SVG, animated via CSS `transform` only.
- Control panel: right side — HTML5 range sliders + numeric inputs.
- Result strip: bottom — key metrics in large monospace with units.

#### NCERT Physics Lab

- Pendulum: SVG `<line>` + `<circle>`, rotation via CSS `rotate()`.
- Projectile: SVG `<path>` with `stroke-dashoffset` animation.
- Ohm's Law: CSS circuit diagram, live value readouts.
- Ray Optics: SVG lens shapes, `<line>` rays.

#### NCERT Chemistry Lab

- Beakers: SVG `<rect>` + `<path>`, liquid fill via CSS `height` animation (visual only, no layout impact).
- Periodic Table: CSS Grid, each cell styled by element group color token.
- Titration: gradient-fill animated SVG burette.
- Chem Balancer: equation input field + live coefficient display.

#### NCERT Math and Maths Visualizer

- Grapher: CSS Grid overlay + SVG `<polyline>` for plots.
- Venn: overlapping SVG `<circle>` with `mix-blend-mode: screen`.
- Geometry tools: SVG protractor and ruler overlays.
- Primary Maths: simplified large-target buttons, `border-radius: 4px` (child-friendly exception).

#### NCERT Biology Lab

- Cell diagrams: SVG `<path>` shapes with inline labels.
- Physio Hub: animated SVG flowcharts for body systems.
- Primary Science: high-contrast icons, soft edges for younger learners.

#### NCERT Logic Circuit / Omni Simulator

- Infinite SVG canvas (pan/zoom via `transform: matrix()`).
- Logic gates: SVG `<rect>` nodes. Connections: `<path>` bezier curves in `#00FFD2`.
- Active signal: `stroke-dashoffset` animation from `#333333` to `#00FFD2`.

---

### Games Design Blueprints

All games: `data-sigma-layout="gaming"` on `<body>`. Gaming HUD active (Section 5.3).

#### Chess (Sovereign Strategist)

- Board: 8×8 CSS Grid. Light = `rgba(255,255,255,0.06)`, dark = `#030303`.
- Pieces: inline SVG. Own = `stroke="#00FFD2"`, opponent = `stroke="#8A2BE2"`.
- Valid moves: `box-shadow: inset 0 0 8px #00FFD2`.
- Captured pieces: 16px SVG strip below board.

#### Ludo Apex

- Board: CSS Grid cross shape. Player home zones: Cyan, Violet, Green, Amber.
- Tokens: SVG `<circle>` with player color fill.
- Dice: square SVG with CSS `rotate` keyframe animation on roll.

#### Strategic Sovereignty (Strategy / RTS)

- Full SVG hex grid canvas. HUD overlaid at top.
- Units: geometric SVGs per type, faction-colored.
- Right-click: radial SVG context menu around unit.

#### Sovereign Serpent (Snake)

- CSS Grid of `<div>` cells. Snake = `#00FFD2`, food = `#FF0055`.
- Speed displayed as ticks/second in top-right HUD.

#### Sovereign Sudoku

- 9×9 CSS Grid. Sub-grid dividers: `2px solid #333333`. Cell borders: `1px solid rgba(255,255,255,0.08)`.
- Given: `#00FFD2`, user-entered: `#E0E0E0`, conflict: `#FF0055`.

#### HyperTrack Runner (Infinite Runner)

- Ground: CSS `background-position` scrolling. Character: SVG `translateY` animation.
- Obstacles: `<div>` blocks via `translateX` keyframes. Zero images.

#### Chromatic Crush and Color Unblock

- Colored CSS Grid tiles. Drag = `box-shadow: 0 0 10px currentColor` glow.

#### Nuts and Nodes and Logic Simulator

- Infinite SVG canvas. Nodes: SVG `<rect>`. Connections: bezier `<path>` in `#00FFD2`.
- Active signal: `stroke-dashoffset` pulse animation.

#### Jigsaw Puzzle / Spot It / Shell Game

- Jigsaw: SVG `clip-path` pieces. `cursor: grab` on hover.
- Spot It: circular SVG card with radial icon arrangement.
- Shell Game: three SVG cups animated via `translateX` shuffle.

#### Space Explorer

- Background: CSS radial gradient `#030303` to `#0A0050`. SVG star dots.
- Planets: layered SVG `<circle>` with `rotate` orbit animations.
- Labels: monospaced `<text>` elements connected by `<line>` SVGs.

#### AetherGlow / Orion Vanguard / Soil vs Mutants / Silent Sentinel

- All: `data-sigma-layout="gaming"`. SVG sprite characters animated via CSS keyframes.
- UI overlays: 10px monospaced HUD text, `pointer-events: none`.

#### Vidya Quest / Lexicon Unleashed / Crowd Flow Legends (Brain / Word Games)

- Card-based. Question card = large Shard panel. Answers = Sovereign Button grid.
- Timer: CSS `width` transition countdown bar across top edge.
- Score: fixed top-right monospace display.

---

### System Utilities and Processes

#### Startup Orchestrator

- Fullscreen terminal. Scrolling green text on black.
- Each line: `[PASS]` in `#00FF00` or `[FAIL]` in `#FF0055` prefixing module name.

#### Theme Engine (Sigma Theme Customizer)

- Dual pane: Controls (left) | Live Preview (right).
- Controls: range sliders, dropdowns, `<input type="color">` all CSS-styled.
- Preset carousel: horizontal scroll of theme preview cards. Active = `2px solid #00FFD2`.
- Live preview updates instantly via CSS variable injection — no redraw.

#### OmniSearch (File Explorer)

- Full-height tree via `<ul>/<li>`. Folders: `[+] DIR_NAME`. Files: `- FILE.EXT`.
- Search bar (top): real-time tree filter. Matches highlighted `background: rgba(0,255,210,0.2)`.
- VIM-style keyboard navigation. All nodes have `tabindex="0"`.

#### Titan Capture (Screen Recorder)

- Floating toolbar: 120px wide, right-edge pinned. Record (red), Pause, Stop, Screenshot SVG buttons.
- Countdown overlay: `font-size: 20vw` digits in `#00FFD2`. `opacity` fades each count.
- `data-sigma-intent="start_recording"` on record button.

#### Energy Core (Power Manager)

- SVG arc battery gauge + mode selector pills.
- Gauge: green > 50%, amber 20-50%, red < 20%.
- Mode pills: active = filled `#00FFD2`. `data-sigma-intent="switch_power_mode"`.

#### Net Mapper (Network Scanner)

- SVG node graph: devices = circle nodes, connections = `<line>` SVGs.
- Device type: geometric SVG icon inside each node (router, phone, PC).
- Ping latency: number badge on each link.

#### Macro Forge (Automation Builder)

- SVG node editor. Three node types with distinct borders:
  - Trigger nodes: `#00FFD2`
  - Action nodes: `#8A2BE2`
  - Condition nodes: `#FFB800`

#### Stopwatch / Timer

- Fullscreen centered. `font-size: clamp(4rem, 15vw, 12rem)` monospace time display.
- Single Sovereign Button (Start/Stop). Lap / Reset below.
- Lap list: scrollable, one row per lap, monospace.

#### Sigma Calculator

- Compact 300×420px panel. Numpad CSS Grid below display.
- Display: monospace, right-aligned, `font-size: 2.5rem`. Expression history above.
- Operators: idle background `#8A2BE2` for visual distinction.

#### Unit Converter / OmniConverter / Number Base Converter

- From/To card pair with SVG arrow center.
- Category and unit: custom CSS-styled `<select>` dropdowns.

#### Sovereign Vision / Vision Explorer

- Drop zone (left, dashed) | Analysis shard (right).
- Drag-over: border solid `#00FFD2`. Drop: AI tags appear as `#8A2BE2` pills.

#### Text Cleaner / Indent Flow / Duplicate Finder

- Input/output split pane. Action button between them.
- Copy button: fixed bottom-right Sovereign Button in output pane.

#### Advocate Command Center / Sovereign Legal Tracker / Diksha Portal

- Document-centric: case/doc list (left) | content viewer (right).
- Status badges: Open=Cyan, Pending=Amber, Closed=Green.

#### Sovereign Concierge (Personal Assistant)

- Full-screen card stack. Each card: frosted glass, title, context snippet, action button row.
- Cards dismissible via `translateX` swipe animation.

#### Package Weaver / Repo Sync / Startup Orchestrator

- Terminal-style output. Scrollable log with `[INFO]`, `[WARN]`, `[ERROR]` prefixes color-coded.

#### Board Hub / Context Engine / Omni Lens

- Shard Dashboard layout. Real-time data tiles.

---

## 12. Design Blueprint Quick-Reference Table

| App / Module | Layout Type | Primary Accent | Special Rule |
| --- | --- | --- | --- |
| Prompt-o-Matic | Vertical split | Cyan + Violet | One output shard per AI model |
| Nexus AI | Chat interface | Violet (AI), Cyan (user) | Sharp rectangles, no border-radius |
| AI Studio | 3-pane vertical | Cyan | Cyber fader sliders |
| Writer | Zen centered | Muted | 75ch max, toolbar collapses |
| CodeForge | 3-column IDE | Cyan | Full syntax palette |
| PDF Forge | Dual pane | Cyan | Drag-drop zone |
| Excel Hub | Spreadsheet Grid | Cyan | Formula bar with `=` prefix |
| Project Flow | Kanban + Gantt tabs | Status-mapped | SVG Gantt bars |
| Quantum BI | Masonry shards | Cyan gradient | Pure CSS charts |
| Sentinel | Shard Dashboard | Green to Red | 85% alert threshold |
| Shield | Nav + Content | Red / Green | Threat badge system |
| Vault Keep | List + Detail | Cyan | Monospace-only |
| Forensic Vault | Wizard steps | Green | Chain-of-custody log |
| OmniBrowser | Browser chrome | Cyan | Privacy dot SVG |
| MeshTalk | Sidebar + Chat | Violet + Cyan | Lock SVG for E2E |
| Chess | 8×8 CSS Grid | Cyan + Violet | SVG pieces, inset glow |
| Ludo Apex | Cross CSS Grid | 4-player palette | CSS dice roll animation |
| Sovereign Serpent | Cell grid | Cyan + Red | Pure CSS, no canvas |
| Sovereign Sudoku | 9×9 CSS Grid | Cyan | Conflict = Red |
| HyperTrack Runner | CSS scroll | Cyan | translateX obstacles |
| Logic Simulator | SVG canvas | Cyan | signal dashoffset animation |
| NCERT Physics | SVG lab | Cyan grid | transform animations only |
| NCERT Chemistry | SVG beakers | Cyan + Amber | Height animation (visual-only) |
| NCERT Maths | SVG grapher | Cyan | Venn mix-blend-mode |
| NCERT Biology | SVG diagrams | Cyan | Labeled path shapes |
| Calculator | Compact Grid | Violet operators | Right-aligned display |
| Macro Forge | SVG node editor | Cyan/Violet/Amber | 3 node color types |
| Net Mapper | SVG graph | Cyan | Ping badges on links |
| Titan Capture | Floating toolbar | Red | 20vw countdown overlay |
| Theme Customizer | Dual pane | Cyan sliders | Live CSS variable injection |
| Energy Core | Single dashboard | Green/Amber/Red | SVG arc gauge |
| Stopwatch | Fullscreen center | Cyan | clamp() font size |

---

## 13. Google Stitch Prompt Library

### Master Prompt (Full OS)

```
Build SigmaOS — a complete operating system UI in HTML/CSS.

Design system: Sovereign Cyberpunk.
- bg=#030303, accent=#00FFD2, secondary=#8A2BE2, alert=#FF0055, success=#00FF00
- All icons: inline stroke SVGs, stroke-width=1.5, stroke="currentColor"
- All layouts: CSS Grid or Flexbox only
- Animate only 'opacity' and 'transform' — never width/height/margin
- Every interactive element: unique id, data-sigma-intent attribute, aria-label
- 8 layout modes on data-sigma-layout body attribute
- 5 notification toast variants
- Gaming HUD overlay (pointer-events none, top-right, 10px monospace)
- 24 CSS design tokens on :root scope

Implement individual UIs for: Prompt-o-Matic, Nexus AI, AI Studio, Writer, CodeForge,
PDF Forge, Excel Hub, Project Flow, Quantum BI, Sentinel, Shield, Vault Keep,
OmniBrowser, MeshTalk, all NCERT Labs (Physics, Chemistry, Math, Biology),
Chess, Ludo, Sudoku, Sovereign Serpent, Calculator, Macro Forge, Net Mapper,
Theme Customizer, Titan Capture, Energy Core, Stopwatch — each per its blueprint.

Zero external images. Zero external CSS frameworks. ARIA required on all SVGs.
Fully keyboard navigable. Tab order follows reading order.
```

### Module-Specific Prompts

**Sovereign Button:**
*"CSS button: idle = rgba(0,255,210,0.05) bg, #00FFD2 text, 1px solid #00FFD2 border, 0px radius. Hover = #00FFD2 bg, #030303 text, 100ms transition on background-color only. Active = scale(0.97). Required: id and data-sigma-intent attributes."*

**Data Terminal Input:**
*"Input field: #000000 bg, bottom-border only (2px solid #333333 idle, #00FFD2 on focus). Prefix non-editable '>' glyph. Caret color #00FFD2. Font: JetBrains Mono."*

**Shard Grid Panel:**
*"CSS Grid container with 1px gap and background #333333 — the gap becomes natural borders. Child panels: #0A0F14 bg, 1.5rem padding. Container attr: data-sigma-shard-container='true'."*

**NCERT Lab (any subject):**
*"Educational simulation UI: dark bg (#030303) with faint 1px cyan grid (rgba(0,255,210,0.05)). SVG simulation viewport left or center, control panel (range sliders + number inputs) on right, results strip bottom. Animate SVG elements using CSS transform only. Holographic blueprint aesthetic."*

**Gaming HUD:**
*"Overlay div: position fixed, top-right corner, pointer-events none, z-index 9999. Contains: FPS number (JetBrains Mono 10px, #606060), 2px progress bar for resource load (linear-gradient #00FF00 to #FF0055). Invisible to game mouse events."*

---

*Document version: 2026-03-15 | SigmaOS Design System v4 | All sections linted and validated.*
