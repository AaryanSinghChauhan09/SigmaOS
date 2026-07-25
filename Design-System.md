# SigmaOS Design System

> The visual language of SigmaOS — colours, typography, spacing, motion.
> Full spec: [docs/Design_System.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Design_System.md)

---

## Core Principles

1. **Sovereign** — No dark patterns, no ads, no manipulation
2. **Fluid** — Physics-based motion, every animation has purpose
3. **Legible** — Contrast ≥ 4.5:1 (WCAG AA), always readable

---

## Colour Palette

| Token | Value | Use |
|---|---|---|
| `bg` | `#07080C` | Desktop background |
| `surface` | `rgba(31,33,42,0.60)` | Glass panels |
| `border` | `rgba(255,255,255,0.09)` | Panel borders |
| `accent` | `#45F3FF` | Actions, focus, links |
| `accent2` | `#A855F7` | Secondary actions |
| `success` | `#34D399` | Positive states |
| `warning` | `#FBBF24` | Caution |
| `error` | `#F87171` | Destructive, offline |
| `text` | `#F0F2F8` | Primary text |
| `muted` | `#6B7280` | Labels, disabled |

---

## Type Scale (key entries)

| Token | Size | Weight | Use |
|---|---|---|---|
| `display-lg` | 36px | 800 | Page titles |
| `heading-2` | 22px | 700 | Card titles |
| `body` | 14px | 400 | Default UI |
| `mono` | 13px | 400 | Code, terminal |

**Fonts**: Outfit (UI) + JetBrains Mono (code)

---

## Spacing & Radius

8px base grid: `4 / 8 / 12 / 16 / 24 / 32 / 48 / 64px`

Corner radius: `6 / 10 / 14 / 18 / 24px / full`

---

## Motion Tokens

| Token | Duration | Easing | Use |
|---|---|---|---|
| `micro` | 80ms | ease-out-quad | Toggle, hover |
| `fast` | 150ms | ease-out-quad | Button press |
| `normal` | 250ms | ease-out-cubic | Panel open/close |
| `slow` | 350ms | ease-out-cubic | Window transitions |
| `spring` | physics | stiffness=280, damp=28 | Drag, position |

---

## Component Quick Ref

**Button** — 36px h, 10px radius, cyan border + bg tint, 2px focus ring  
**Input** — 36px h, 1px border, 3px glow on focus  
**Card** — glassmorphism (60% opacity + 16px blur + 14px radius)  
**Toggle** — 40×24px, spring-animated thumb  

---

*Full specification: [docs/Design_System.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Design_System.md)*
