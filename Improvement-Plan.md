# SigmaOS Master Improvement Plan

> Canonical source: [docs/Master_Improvement_Plan.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Master_Improvement_Plan.md)

---

## Overview

| Area | Current | v1.0 Target | v2.0 Target |
|---|---|---|---|
| Design System | ✅ Defined | All widgets use tokens | Live theming |
| Applications | 0 installable | 8 Tier-1 apps | 20+ apps |
| UI/UX | Core engine ✅ | Full widget set | Wayland compat |
| Performance | No boot yet | Boot < 2s, 60fps | Boot < 1s, 120fps |
| OOP Patterns | 20+ patterns ✅ | Type-safe errors | Async/await |

---

## Design Improvements

1. **Design tokens** → `sigma_design_tokens.rs` (colours/spacing/radius/motion as constants)

2. **Icon system** → 100+ 24×24 icons, SVG-to-bitmap rasterizer

3. **Typography engine** → PSF bitmap → TrueType rasterizer

4. **Dark/Light adaptive** → all components implement `Themed` trait

5. **Consistency audit** → WCAG AA contrast, 8px grid, hover/active/focus on all elements

## Application Plan

**Tier 1 (v0.1)**: terminal, files, editor, settings, launcher
**Tier 2 (v1.0)**: browser, PDF, notes, calendar, contacts, calculator
**Tier 3 (v1.5)**: media player, image viewer, email, chat, video calls
**Tier 4 (v2.0)**: office suite, vector/raster editor, IDE, DAW

Each app uses the **SigmaApp trait** (Model-View-Intent pattern).

## UI/UX Improvements

**Missing widgets**: Toggle, Slider, ListView, DropDown, ProgressBar, Modal, Tooltip, TabBar, TreeView
**UX flows**: Onboarding wizard, Quick Settings panel, App Switcher, Global Search, Screen Lock
**Accessibility**: TTS audio, screen magnifier, voice control, braille display
**Mobile**: Breakpoints 480/1024px, bottom nav, split-view, adaptive layout

## Performance Targets

| Metric | v0.1 | v1.0 | v2.0 |
|---|---|---|---|
| Boot to desktop | < 2.5s | < 1.5s | < 1.0s |
| Frame time | < 16ms | < 8ms | < 4ms |
| Input latency | < 10ms | < 5ms | < 2ms |
| Idle RAM | < 512MB | < 256MB | < 128MB |

## OOP Improvements

1. **Error hierarchy** — typed errors with context (not just `IoError`)

2. **Capability-typed API** — phantom types prevent privilege escalation at compile time

3. **Plugin system** — `Plugin` trait + `PluginManager` for extensibility

4. **Reactive state** — MVI (Model-View-Intent) pattern for all app state

5. **Async/await** — `sigma-async` no_std runtime for I/O-bound code

## Implementation Schedule

| Sprint | Duration | Focus |
|---|---|---|
| 1 | Month 1–2 | Design tokens, damage tracking, core widgets, terminal MVP |
| 2 | Month 2–3 | Onboarding, quick settings, font rendering, app switcher |
| 3 | Month 3–4 | sigma-files, sigma-edit, sigma-calc, screenshots |
| 4 | Month 4–6 | GPU compositing, boot < 2.5s, online pkg registry |
| 5 | Month 6–9 | Accessibility TTS, mobile UI, RPi4 ARM64, public alpha |

---

## All Planning Documents

| Document | What it covers |
|---|---|
| [Master Improvement Plan](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Master_Improvement_Plan.md) | This overview |
| [Design System](Design-System) | Colours, typography, spacing, motion |
| [UI/UX Performance Plan](UI-UX-Performance) | Animation, renderer, input, panel, settings, a11y |
| [App Development Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/App_Development_Roadmap.md) | App specs, app framework, distribution |
| [Performance Targets](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Performance_Targets.md) | Concrete benchmarks with measurement methods |
| [OOP Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/OOP_Architecture.md) | OOP patterns in Rust/Zig/Nim/SPARK |
| [Adoption Strategy](Adoption-Strategy) | Community, marketing, enterprise |
| [ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ROADMAP.md) | Phase 1–4 execution plan |
