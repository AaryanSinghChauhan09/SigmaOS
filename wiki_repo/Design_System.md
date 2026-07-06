# SigmaOS Design System

> The canonical visual language for SigmaOS — every pixel, colour, spacing, and motion principle.

---

## Design Philosophy

### Three principles guide every decision:

1. **Sovereign** — The OS respects you. No dark patterns, no manipulative UI, no ads.

2. **Fluid** — Motion is physics-based, not arbitrary. Every animation has purpose.

3. **Legible** — Content is always readable. Contrast ratios ≥ 4.5:1 (WCAG AA).

---

## Color System

### Base Palette

```
Background:   #07080C  — near-black with blue undertone
Surface:      rgba(31,33,42,0.60)  — glass panel bg (60% opacity)
Border:       rgba(255,255,255,0.09) — subtle glass border
```

### Accent Colors

```
Primary:    #45F3FF  — cyan (actions, links, focus rings)
Secondary:  #A855F7  — purple (secondary actions, tags)
Success:    #34D399  — green (positive states, online)
Warning:    #FBBF24  — amber (caution, degraded)
Error:      #F87171  — red (destructive, offline, critical)
```

### Text Scale

```
Primary text:   #F0F2F8  — readable on all dark surfaces
Secondary text: #9CA3AF  — labels, metadata
Muted:          #6B7280  — placeholders, disabled
Inverse:        #07080C  — text on light surfaces
```

### Semantic Tokens

```
interactive-hover:    rgba(69,243,255,0.08)
interactive-active:   rgba(69,243,255,0.16)
interactive-focus:    rgba(69,243,255,0.24) — also 2px ring
danger-hover:         rgba(248,113,113,0.08)
selection-bg:         rgba(69,243,255,0.20)
```

---

## Typography

### Type Scale

| Token | Size | Weight | Line-height | Use |
|---|---|---|---|---|
| `display-xl` | 48px | 900 | 1.1 | Hero headings |
| `display-lg` | 36px | 800 | 1.15 | Page titles |
| `heading-1` | 28px | 700 | 1.2 | Section titles |
| `heading-2` | 22px | 700 | 1.3 | Card titles |
| `heading-3` | 18px | 600 | 1.35 | Sub-section |
| `body-lg` | 16px | 400 | 1.6 | Primary reading |
| `body` | 14px | 400 | 1.55 | Default UI |
| `body-sm` | 13px | 400 | 1.5 | Captions, metadata |
| `label` | 12px | 600 | 1.4 | Tags, badges, pills |
| `mono` | 13px | 400 | 1.6 | Code, terminal, paths |

### Fonts

```
UI text:    Outfit (variable, wght 300–900)
Code/mono:  JetBrains Mono (variable, wght 400–700)
Fallback:   system-ui, -apple-system, sans-serif
```

### Reading optimisation

- Letter-spacing: `-0.01em` for large headings, `0` for body

- Paragraph max-width: 65 characters (44rem at 14px)

- Optical size on variable fonts enabled

---

## Spacing System

8px base grid. All spacing is a multiple of 4px.

```
space-1:   4px   — micro (icon padding)
space-2:   8px   — xs (tight element gap)
space-3:  12px   — sm (list item gap)
space-4:  16px   — md (default component gap)
space-5:  20px   — lg
space-6:  24px   — xl (section spacing)
space-8:  32px   — 2xl (card padding)
space-10: 40px   — 3xl (page margin on mobile)
space-12: 48px   — 4xl (page margin on desktop)
space-16: 64px   — 5xl (section gap)
```

---

## Elevation (z-layers)

```
z-0:   background, desktop wallpaper
z-10:  base windows
z-20:  floating windows (above tiled)
z-30:  system tray, panel
z-40:  tooltips, popovers
z-50:  modals, dialogs
z-60:  notifications
z-70:  system alerts (critical)
z-80:  screen overlay (onboarding wizard, DND)
z-90:  screen lock
z-100: compositor debug overlay
```

### Shadow scale

```
shadow-sm:  0 1px 3px rgba(0,0,0,0.4)
shadow-md:  0 4px 12px rgba(0,0,0,0.5)
shadow-lg:  0 10px 30px rgba(0,0,0,0.6), 0 0 20px rgba(69,243,255,0.06)
shadow-xl:  0 20px 60px rgba(0,0,0,0.7), 0 0 40px rgba(69,243,255,0.10)
```

---

## Border Radius

```
radius-sm:   6px   — buttons (small), input corners
radius-md:  10px   — buttons (default), cards
radius-lg:  14px   — panels, menus
radius-xl:  18px   — floating windows, modals
radius-2xl: 24px   — full panels, launcher
radius-full: 9999px — pills, badges, avatars, circular icons
```

---

## Iconography

### Grid: 24×24px (4px padding → 16×16 content area)

- **Style**: Outlined, 1.5px stroke weight

- **Corners**: 2px radius on all sharp corners

- **Optical sizing**: icons scaled to 16/20/24/32/48px variants

- **Color**: always uses `currentColor` (inherits text color)

### Icon library (planned)

```
System:      home, settings, search, user, lock, wifi, battery
Actions:     add, close, edit, delete, copy, share, download, upload
Navigation:  chevron-*, arrow-*, back, forward
Files:       file, folder, image, video, code, pdf, archive
Status:      check, warning, error, info, help, star, heart
Apps:        terminal, browser, editor, calculator, calendar, mail
```

---

## Motion Principles

### Duration scale

```
instant:   0ms   — state changes (no animation)
fastest:  80ms   — micro-interactions (toggle state)
fast:    150ms   — hover effects, button press
normal:  250ms   — panel open/close, menu
slow:    350ms   — window transitions
slower:  500ms   — page transitions
slowest: 700ms   — onboarding, first-launch
```

### Easing catalogue

| Token | Curve | Use |
|---|---|---|
| `ease-linear` | t | Progress bars only |
| `ease-out-quad` | t(2-t) | Elements entering screen |
| `ease-in-quad` | t² | Elements leaving screen |
| `ease-in-out-quad` | — | Repositioning |
| `ease-out-cubic` | (t-1)³+1 | Windows opening |
| `ease-out-elastic` | — | Successful actions (spring) |
| `ease-out-bounce` | — | Dragged element released |
| `spring(k,d)` | physics | Any continuous interaction |

### Rules

1. **Enter with ease-out** (decelerates as it arrives = natural)

2. **Exit with ease-in** (accelerates as it leaves = natural)

3. **Position springs** with stiffness=280, damping=28 (responsive feel)

4. **Scale springs** with stiffness=350, damping=35 (snappy)

5. **Respect reduce-motion**: all animations → instant when enabled

---

## Component Specification

### Button

```
States:   default | hover | active | focused | disabled | loading
Sizes:    sm (28px h) | md (36px h) | lg (44px h) | icon (36×36)
Variants: primary | secondary | ghost | danger | link

Default:
  background: rgba(69,243,255,0.08)
  border:     1px solid rgba(69,243,255,0.25)
  color:      #45F3FF
  radius:     radius-md
  padding:    space-2 space-4

Hover:
  background: rgba(69,243,255,0.16)
  box-shadow: 0 0 16px rgba(69,243,255,0.20)

Active:
  background: rgba(69,243,255,0.25)
  transform:  scale(0.98)

Focus ring: 2px solid #45F3FF, offset 2px
```

### Input / TextInput

```
Height: 36px (md), 44px (lg)
Padding: space-3 space-4
Border: 1px solid rgba(255,255,255,0.09)
Background: rgba(255,255,255,0.03)
Radius: radius-md

Focused:
  border-color: #45F3FF
  box-shadow: 0 0 0 3px rgba(69,243,255,0.12)

Error:
  border-color: #F87171
  box-shadow: 0 0 0 3px rgba(248,113,113,0.12)
```

### Card / Panel

```
Background: rgba(31,33,42,0.60)   — glassmorphism
Border: 1px solid rgba(255,255,255,0.09)
Backdrop-blur: 16px
Border-radius: radius-lg (14px)
Padding: space-6 (24px)
Shadow: shadow-md
```

### Toggle (planned)

```
Track: 40×24px, radius-full
  off: rgba(255,255,255,0.15)
  on:  #45F3FF

Thumb: 20×20px circle, white, shadow-sm
  off: translateX(2px)
  on:  translateX(18px) — spring animated

Transition: spring(350, 35) on thumb position
```

---

## Responsive Breakpoints

```
xs:    < 480px   — compact phone
sm:  480–767px   — phone landscape / small tablet
md:  768–1023px  — tablet / small laptop
lg: 1024–1279px  — laptop
xl: 1280–1535px  — desktop
2xl: ≥ 1536px   — large monitor / ultrawide
```

---

## Accessibility Requirements

| Requirement | Target |
|---|---|
| Text contrast (body) | ≥ 4.5:1 (WCAG AA) |
| Text contrast (large) | ≥ 3.0:1 (WCAG AA) |
| Interactive contrast | ≥ 3.0:1 |
| Focus indicator size | ≥ 2px ring |
| Touch target min | 44×44px |
| Animation respect | `prefers-reduced-motion` |
| Colour not sole indicator | ✅ (icon + text + shape) |
| Screen reader support | ARIA roles on all widgets |

---

*See also: [docs/UI_UX_Performance_Plan.md](UI_UX_Performance_Plan.md) · [wiki/UI-UX-Performance](../wiki_repo/UI-UX-Performance.md)*
