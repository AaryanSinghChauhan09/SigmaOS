
---

## 🖥️ 16. Per-Application Design Blueprints

Every SigmaOS application follows the Sovereign Cyberpunk system tokens but has its own specific layout, interaction pattern, and visual language. The following sections define each app's design requirements.

---

### 🧠 AI & Intelligence Apps

#### Prompt-o-Matic (Omni-Prompt Distributor)

* **Layout:** Full-height, split view. Top 70% = output pane (multi-column frosted cards, one per AI model). Bottom 30% = input control strip.
* **Input bar:** Single expanding `textarea` with `>` prefix. Glows `#00FFD2` on focus.
* **AI Target Selector:** Horizontal row of pill-shaped toggle buttons. Each pill has an SVG logo of the AI model. Checked = filled `#00FFD2`, unchecked = ghost.
* **Output cards:** Each model gets its own shard. Header = model name in `#8A2BE2`. Body = streamed text in `#E0E0E0` monospace.
* **Key IDs:** `id="omni-input"`, `id="omni-targets"`, `id="omni-output-[model-name]"`.

#### Nexus AI (Local AI Assistant)

* **Layout:** Chat-interface style. Messages alternate left (AI, `#8A2BE2` border) and right (user, `#00FFD2` border).
* **Message bubbles:** No rounded corners. Sharp rectangular blocks with 1px left-border accent.
* **Typing indicator:** Three pulsing dots using `opacity` keyframes only.
* **Input:** Pinned to bottom, full-width `textarea`, `Enter` submits, `Shift+Enter` for newline.

#### AI Studio

* **Layout:** Three vertical panes — Model Library (left), Config Panel (center), Test Console (right).
* **Model cards:** Slim horizontal rows with model name, size badge, and a status dot (green = loaded, muted = idle).
* **Config sliders:** Temperature, context window, top-p — all styled as cyber fader sliders.

---

### 📁 Productivity & Office Apps

#### Writer (Sovereign Word Processor)

* **Layout:** Zen-mode. Centered text canvas (max 75ch wide) on pure `#030303` void.
* **Toolbar:** Collapses to a single thin line. Expands on hover revealing formatting SVG buttons.
* **Status bar:** Bottom edge, shows word count, line, col in `#606060` monospace.
* **No margin lines or rulers** — the void IS the document.

#### CodeForge (IDE)

* **Layout:** Classic 3-column IDE: File tree (left, 200px) → Editor (center, flex-grow) → Output dock (bottom, 200px tall, collapsible).
* **Syntax highlighting:** Colors derived from Sigma palette. Keywords = `#00FFD2`, strings = `#8A2BE2`, comments = `#606060`, numbers = `#FFB800`.
* **Active line:** `rgba(0, 255, 210, 0.04)` full-width highlight.
* **File tree:** Monospace tree using `├─` and `└─` box-drawing characters. Folders have `▶ / ▼` SVG carets.

#### PDF Forge

* **Layout:** Dual pane. Left = source input (drag-drop zone with dashed `#333333` border). Right = live PDF preview in an `<iframe>` or `<canvas>`.
* **Toolbar:** Horizontal strip of action buttons (Merge, Split, Sign, Compress) — all `Sovereign Button` style in a `display: flex` row.

#### Excel Hub / Excel Validator

* **Layout:** Spreadsheet grid. Cells are `<div>` elements in a CSS Grid. Alternating row backgrounds: `#030303` / `rgba(255,255,255,0.02)`.
* **Active cell:** `2px solid #00FFD2` outline.
* **Formula bar:** Fixed top strip with `=` prefix, monospaced.

#### Project Flow (Scrum/Gantt)

* **Layout:** Tab-switcher between Kanban view and Gantt view.
* **Kanban:** CSS Grid columns (To Do, In Progress, Done). Cards are Shard panels with a left-border color coding status.
* **Gantt:** SVG-rendered horizontal timeline. Task bars are pure CSS `background-color` fills on a `<rect>` element.

#### Markdown Viewer / Studio

* **Layout:** Side-by-side. Left = raw editor (monospace, `#E0E0E0`). Right = rendered preview.
* **Preview:** Custom CSS resets so rendered HTML matches the Sigma cyberpunk aesthetic (headers in cyan, code blocks in obsidian `#0A0F14` with monospace).

#### Quantum BI (Business Intelligence)

* **Layout:** Masonry grid of chart widgets. Each widget is a Shard Panel.
* **Charts:** Pure CSS bar/line charts (no Chart.js/D3). Bar fills use `background: linear-gradient(to top, var(--sigma-accent-primary), transparent)`.
* **Filters:** Inline pill-filter tags above chart groups.

#### Vortex Clipboard (Clipboard Manager)

* **Layout:** Compact floating overlay (320px wide, max 480px tall). Pinned to the right edge of screen.
* **Item list:** Scrolling list of clipboard entries. Each entry shows a text preview truncated to 2 lines with `text-overflow: ellipsis`.
* **Actions:** Copy and Delete icon buttons on hover (appear as SVG icons, invisible at rest).

---

### 🔒 Security & Privacy Apps

#### Sentinel (System Monitor)

* **Layout:** Dashboard of 6–8 Shard widgets: CPU, RAM, Disk, Network, GPU, Temperature, Battery, Process List.
* **Metrics:** All numerical. Progress bars use CSS `width` transitions (exception to rule — justified by the continuous-update UX requirement — transitions must be ≤300ms).
* **Process List:** Sortable `<table>` with sticky header. Sorted column shows a `↑ / ↓` SVG arrow.
* **Alert threshold:** When metric exceeds 85%, the Shard's left-border flashes `#FF0055`.

#### Shield (Security Suite)

* **Layout:** Left nav (Scan, Firewall, Vault, Logs). Right = content area.
* **Scan UI:** Pulsing circular SVG loader during scan. Results appear as a scrolling list of threat entries (each with severity badge).
* **Threat badge colors:** `SAFE=#00FF00`, `WARNING=#FFB800`, `CRITICAL=#FF0055`.

#### Vault Keep (Password Manager)

* **Layout:** Two pane. Left = searchable item list. Right = credential detail panel.
* **Item list:** Each row shows service name + a strength meter (CSS bar, color `#00FF00` to `#FF0055` based on entropy).
* **Password field:** Hidden by default (`•••••••`). Reveal on hold of eye-SVG button.
* **Zero external fonts on this screen** — all data is monospace for maximum legibility.

#### Forensic Vault

* **Layout:** Wizard-style. Step 1: Select target. Step 2: Acquire image. Step 3: Analyze. Step 4: Export report.
* **Step indicator:** Horizontal progress bar using `::before` CSS counter.
* **Output:** Chain-of-custody log in a read-only terminal block (`#00FF00` text, `#000000` bg).

#### Aegis Permissions (Permission Manager)

* **Layout:** Permission matrix table. Rows = apps. Columns = permission types. Cells = toggle switches.
* **Toggle switches:** Pure CSS `<input type="checkbox">` styled as sliding switches. On = `#00FFD2`, Off = `#333333`.

---

### 🌐 Communication & Browser Apps

#### OmniBrowser (Secure Browser)

* **Layout:** Standard browser chrome. URL bar at top (full-width, `>_ ` prefix). Tab strip below URL bar.
* **Tabs:** Each tab is a `40px` tall strip. Monospace tab title. Active tab has `#00FFD2` bottom border.
* **Privacy dot:** A green `●` or red `●` SVG dot in the URL bar indicates secure/insecure connection.

#### MeshTalk (Encrypted Chat)

* **Layout:** Contacts sidebar (left, 240px) + Chat view (right, flex-grow).
* **Contact list:** Avatar = first letter of contact name in a circle (pure CSS, `background: #8A2BE2`).
* **Messages:** Alternating alignment with a `2px` left or right accent border.
* **Encryption indicator:** A lock SVG in the chat header. Gold when E2E active.

#### Echo Cast (Broadcasting)

* **Layout:** Preview window (top, 16:9 aspect-ratio locked via `padding-bottom: 56.25%`). Controls below.
* **Controls:** Record, Pause, Stop, Screenshot — large SVG buttons in a flex row.
* **Recording active:** A pulsing red `●` dot in the top-left corner overlaying the preview.

#### Social Hub

* **Layout:** Feed column (center). Trending sidebar (right).
* **Feed cards:** Each post is a Shard Panel. Username in `#00FFD2`, body text in `#E0E0E0`.
* **Reactions:** Inline SVG emoji-like symbols (no actual emoji fonts).

---

### 🎓 NCERT Educational Apps

All NCERT apps share these foundational design rules:

* **Background grid:** Faint `1px` cyan grid lines (`rgba(0, 255, 210, 0.05)`) to evoke a holographic blueprint.
* **Simulation elements:** Pure inline SVG. No canvas. Animated via CSS `transform` only.
* **Control panel:** Always on the right side. HTML5 range sliders + numeric `<input>` fields.
* **Result panel:** Bottom strip showing key metrics in large monospace. Always labeled with units.

#### NCERT Physics Lab

* Pendulum = SVG `<line>` + `<circle>`. Rotation via CSS `rotate()`.
* Projectile = SVG `<path>` animated with `stroke-dashoffset`.
* Ohm's Law = CSS circuit lines, value readouts updating live.

#### NCERT Chemistry Lab

* Beakers = SVG `<rect>` + `<path>` with CSS `height` animation for liquid fill (allowed here — purely visual, no layout impact).
* Periodic Table = CSS Grid, each cell styled by element group color.
* Titration Sim = gradient-fill animated SVG burette.

#### NCERT Math / Maths Visualizer

* Graph canvas = CSS Grid overlay with SVG `<polyline>` for function plots.
* Venn Visualizer = Two overlapping SVG `<circle>` elements with `mix-blend-mode: screen`.
* Geometry tools = SVG protractor + ruler overlays.

#### NCERT Biology Lab

* Cell diagrams = pure SVG with labeled `<path>` shapes.
* Physio Hub = animated SVG flowcharts for body systems.
* Primary Science = simplified, high-contrast icons for younger students, `border-radius: 4px` (soft exception for child-friendliness).

---

### 🎮 Games Design Blueprints

All SigmaOS games use the global `sigma-layout-gaming` body class. The HUD spec from Section 13 applies. Additional per-game rules:

#### Chess (Sovereign Strategist)

* **Board:** Pure CSS Grid (8×8). Light squares = `rgba(255,255,255,0.06)`, dark squares = `#030303`.
* **Pieces:** Inline SVG for each piece type. `stroke="#00FFD2"` for own pieces, `stroke="#8A2BE2"` for opponent.
* **Move highlights:** `box-shadow: inset 0 0 8px #00FFD2` on valid destination squares.
* **Captured pieces:** Display as a strip of small (16px) SVG pieces below the board.

#### Ludo Apex

* **Board:** CSS Grid cross shape. Home zones use one of four accent colors (one per player: Cyan, Violet, Green, Amber).
* **Tokens:** SVG `<circle>` elements with player color fill.
* **Dice:** CSS keyframe `rotate` animation on a square SVG element showing numeric value.

#### Strategic Sovereignty (Strategy/RTS)

* **Layout:** Full-canvas SVG-based hex grid. HUD overlaid at top edge.
* **Unit icons:** Geometric SVG shapes per unit type. Color = player faction.
* **Action menu:** Right-click spawns a radial SVG menu around the unit.

#### Sovereign Serpent (Snake)

* **Grid:** CSS Grid of `div` cells. Active snake = cells with `background: #00FFD2`. Food = `background: #FF0055`.
* **Speed indicator:** Tiny top-right number showing current ticks/second.

#### Sovereign Sudoku

* **Grid:** 9×9 CSS Grid. 3×3 sub-grids separated by `2px solid #333333`. Cell borders `1px solid rgba(255,255,255,0.08)`.
* **Given numbers:** `color: #00FFD2`. User-entered: `color: #E0E0E0`. Conflict: `color: #FF0055`.

#### HyperTrack Runner (Infinite Runner)

* **Viewport:** Pure CSS animated `background-position` scrolling ground. Character = SVG sprite animated via `transform: translateY`.
* **Obstacles:** CSS `div` blocks sliding in from right using `translateX` keyframes. No images.

#### Ludo Apex / Dots and Nodes / Matrix Cross Circle

* All board games share the same grid design principles: CSS Grid board, SVG pieces, `#00FFD2` accent for active/selected states.

#### Chromatic Crush / Color Unblock

* **Grid:** CSS Grid of colored `div` tiles. Colors from the Sigma accent palette.
* **Drag feedback:** Dragging a tile applies a `box-shadow: 0 0 10px currentColor` glow.

#### Nuts and Nodes / Logic Simulator

* **Workspace:** Infinite SVG canvas with pan/zoom via `transform: matrix()`.
* **Nodes:** SVG `<rect>` with rounded corners (4px allowed — mechanical look). Connections = `<path>` bezier curves in `#00FFD2`.
* **Active signal:** `stroke` animates from `#333333` to `#00FFD2` using a CSS `stroke-dashoffset` animation.

#### Jigsaw Puzzle / Spot It / Shell Game

* **Jigsaw:** SVG clip-path pieces. `cursor: grab` on hover, `cursor: grabbing` on drag.
* **Spot It:** Circular SVG card with radially arranged icon SVGs.
* **Shell Game:** Three SVG cups animated via CSS `translateX` keyframes during shuffle.

#### Space Explorer

* **Background:** Pure CSS radial gradient (deep `#030303` to `#0A0050`) with SVG star dots.
* **Planets/objects:** Layered SVG circles with `rotate` animations for orbits.
* **HUD labels:** Floating monospaced text labels connected by `<line>` SVGs.

#### Vidya Quest / Sovereign Sudoku / Lexicon Unleashed (Brain Games)

* **Layout:** Card-based UI. Question card = large centered Shard panel. Answer options = grid of `Sovereign Button` components.
* **Timer:** Linear countdown using only CSS `width` transition on a top-edge progress bar.
* **Score:** Fixed top-right monospace score display.

---

### ⚙️ System Processes & Utilities

#### Startup Orchestrator

* **Layout:** Boot sequence log — fullscreen terminal with scrolling green text on black.
* **Module status:** Each line is `[PASS]` in green or `[FAIL]` in red prefixing module name.

#### Sigma Theme Customizer (Theme Engine)

* **Layout:** Dual pane (see Section 3-B for full spec). 
* **Preset cards:** Horizontal scroll carousel of theme preview cards. Active = `2px solid #00FFD2` border.
* **Color pickers:** Native `<input type="color">` wrapped in a custom CSS shadow to match the dark aesthetic.

#### OmniSearch (File Explorer)

* Full spec in Section 3-E. Additional: search bar at top triggers a real-time filter of the tree. Matched text is highlighted with `background: rgba(0,255,210,0.2)` inline.

#### Titan Capture (Screen Recorder)

* **UI:** Minimal floating toolbar (120px wide, full-height right edge). Record (red), pause, stop, screenshot SVG buttons.
* **Countdown overlay:** Full-screen centered countdown digits in `#00FFD2` massive font (`font-size: 20vw`). `opacity` fades on each count.

#### Energy Core (Power Manager)

* **Layout:** Single-page dashboard. Battery SVG gauge + mode selector buttons.
* **Gauge:** SVG arc that fills proportionally. Green (>50%), Amber (20-50%), Red (<20%).
* **Mode buttons:** One per OS power mode, pill-shaped, active mode filled `#00FFD2`.

#### Net Mapper (Network Scanner)

* **Layout:** Canvas area showing a node-link diagram (SVG). Each discovered device = SVG circle node. Links = `<line>` SVGs.
* **Device type icons:** Geometric SVG inside each node circle (router, phone, PC shapes).
* **Ping latency:** Displayed as a number badge on each link line.

#### Macro Forge (Automation Builder)

* **Layout:** Visual node editor (same SVG canvas approach as Logic Simulator).
* **Trigger nodes:** `#00FFD2` bordered.
* **Action nodes:** `#8A2BE2` bordered.
* **Condition nodes:** `#FFB800` bordered.

#### Stopwatch / Timer

* **Layout:** Centered fullscreen. Large monospace time display (`font-size: clamp(4rem, 15vw, 12rem)`).
* **Start/Stop:** Single large `Sovereign Button`. Lap/Reset below it.
* **Lap list:** Scrollable list below buttons, each lap on one row.

#### Sigma Calculator

* **Layout:** Compact panel (300px × 420px). Numpad grid below display.
* **Display:** Monospace, right-aligned, `font-size: 2.5rem`. Secondary row for the expression history.
* **Buttons:** CSS Grid of `Sovereign Button` tiles. Operators use `#8A2BE2` background fill at idle.

#### Unit Converter / Number Base Converter / OmniConverter

* **Layout:** From/To card pair. Input on the left, result on the right. Arrow SVG in the center.
* **Dropdowns:** Category (Length, Mass, etc.) + Unit selectors — custom CSS-styled `<select>`.

#### Sovereign Forms Hub

* **Layout:** Form builder canvas. Draggable field blocks (Text, Number, Radio, Checkbox, Date).
* **Preview mode:** Toggle button switches to read-only rendered form view.

#### Text Cleaner / Indent Flow / Duplicate Finder

* **Layout:** Input/output split pane. Paste text into left, click action button, result appears in right.
* **Copy button:** Fixed bottom-right Sovereign Button in the output pane.

#### Sovereign Vision / Vision Explorer (Image AI)

* **Layout:** Image drop zone (left, dashed border). Analysis result panel (right, frosted shard).
* **Drop zone:** On drag-over, border changes to `#00FFD2` solid.
* **Result:** Tag pills in `#8A2BE2` frosted backgrounds listing detected objects/labels.

#### Chronos Vault (Time-Lock/Scheduler)

* **Layout:** Calendar grid (CSS Grid). Active dates highlighted `#00FFD2`. Events shown as small color bars below date numbers.
* **Event detail popover:** Frosted glass card appears 8px below clicked date cell.

#### Event Matrix

* **Layout:** Gantt-like horizontal timeline for events. SVG horizontal bars colored by category.

#### Omni ETL Forge (Data Pipeline Builder)

* **Layout:** Node-link canvas (same as Macro Forge). Source, Transform, and Sink node types each have distinct border colors.

#### Spectral Analyzer

* **Layout:** Audio waveform display (SVG `<polyline>` on a dark canvas). Frequency bars below it.
* **Frequency bars:** Pure CSS `height` animation driven by JavaScript, each bar a thin `div` colored `#00FFD2` to `#FF0055` gradient by frequency range.

#### Welcome Guide / Guide Apex

* **Layout:** Centered markdown-rendered card. Progress steps shown as a numbered SVG path at the top.
* **Navigation:** Prev / Next `Sovereign Buttons` at the bottom.

#### Daksha Portal / Advocate Command Center / Sovereign Legal Tracker

* **Layout:** Document-centric. Left sidebar = case/document list. Right = content viewer.
* **Document cards:** Each shows title, date (monospace), and status badge.
* **Status badges:** Pill-shaped, color-coded (Open=Cyan, Pending=Amber, Closed=Green).

#### Sovereign Concierge (Personal Assistant)

* **Layout:** Full-screen card stack. Each card is an actionable suggestion.
* **Card:** Frosted glass panel with a title, context snippet, and action button row.

---

## 📋 Design Blueprint Summary Table

| App / Process | Layout Type | Primary Accent | Special Rule |
| --- | --- | --- | --- |
| Prompt-o-Matic | Split pane | Cyan + Violet | One output card per AI model |
| CodeForge IDE | 3-column IDE | Cyan | Syntax palette uses full token set |
| Writer | Zen centered | Muted | 75ch max width, no toolbars visible |
| Sentinel | Shard Dashboard | Green→Red | Alert threshold at 85% |
| Shield | Nav + Content | Red/Green | Threat severity badge system |
| Vault Keep | List + Detail | Cyan | Monospace-only for credentials |
| Chess | CSS Grid board | Cyan + Violet | SVG pieces, inset move highlights |
| Ludo Apex | Cross CSS Grid | 4-player palette | CSS dice keyframe |
| Sovereign Serpent | Cell Grid | Cyan+Red | Pure CSS cells — no canvas |
| Sudoku | 9×9 CSS Grid | Cyan | Conflict state = Red text |
| NCERT Physics | SVG Lab | Cyan grid | CSS transform animations only |
| NCERT Chemistry | SVG Beakers | Cyan + Amber | Height animation (visual only) |
| Calculator | Compact grid | Violet operators | Right-aligned monospace display |
| Macro Forge | SVG node editor | Cyan/Violet/Amber | 3 node color types |
| Net Mapper | SVG node graph | Cyan links | Ping badge on each link |
| Titan Capture | Floating toolbar | Red (recording) | vw-scale countdown overlay |
| Theme Customizer | Dual pane | Cyan sliders | Live CSS variable injection |

---

**Final Google Stitch Master Prompt (Complete):**
*"Build SigmaOS — a complete operating system UI in HTML/CSS. Use the 'Sovereign Cyberpunk' design system. Token colors: bg=#030303, accent=#00FFD2, secondary=#8A2BE2, alert=#FF0055, success=#00FF00. All icons are inline stroke SVGs. All layouts use CSS Grid or Flexbox only. Animate only 'opacity' and 'transform'. Include semantic IDs and data-sigma-intent on every interactive element. Support 8 layout modes, 5 notification variants, gaming HUD overlay, and 24 CSS design tokens. Implement individual UIs for: Prompt-o-Matic, CodeForge, Writer, Sentinel, Shield, VaultKeep, OmniBrowser, MeshTalk, Chess, Ludo, Sudoku, all NCERT labs (Physics, Chemistry, Math, Biology), Calculator, Macro Forge, Net Mapper, and Titan Capture — each following its per-app blueprint. Entire UI must be zero external image, zero external CSS framework. ARIA labels required on all SVG elements."*
