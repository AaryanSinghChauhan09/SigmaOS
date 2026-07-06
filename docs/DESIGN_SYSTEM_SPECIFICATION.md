# SigmaOS Design System Specification

## Executive Summary

This document defines the unified design system for SigmaOS, ensuring consistency across all applications and components. The design system provides a cohesive visual language, component library, and guidelines for building SigmaOS applications.

## Design Philosophy

### Core Principles:

- **Consistency**: Uniform appearance and behavior across all applications

- **Clarity**: Clear visual hierarchy and intuitive interactions

- **Efficiency**: Fast, responsive, and performant

- **Accessibility**: Inclusive design for all users

- **Polish**: macOS-like attention to detail

## Color System

### Primary Colors

```rust
pub struct PrimaryColors {
    pub primary: String,      // #3B82F6 - Blue
    pub primary_dark: String, // #2563EB
    pub primary_light: String, // #60A5FA
}
```

### Semantic Colors

```rust
pub struct SemanticColors {
    pub success: String,      // #10B981 - Green
    pub warning: String,      // #F59E0B - Amber
    pub error: String,       // #EF4444 - Red
    pub info: String,        // #3B82F6 - Blue
}
```

### Neutral Colors

```rust
pub struct NeutralColors {
    pub gray_50: String,     // #F9FAFB
    pub gray_100: String,    // #F3F4F6
    pub gray_200: String,    // #E5E7EB
    pub gray_300: String,    // #D1D5DB
    pub gray_400: String,    // #9CA3AF
    pub gray_500: String,    // #6B7280
    pub gray_600: String,    // #4B5563
    pub gray_700: String,    // #374151
    pub gray_800: String,    // #1F2937
    pub gray_900: String,    // #111827
}
```

### Theme Support

```rust
pub enum Theme {
    Light,
    Dark,
    Auto,
}

pub struct ThemeColors {
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
}
```

## Typography System

### Font Families

```rust
pub struct FontFamilies {
    pub sans: String,        // "Inter", system-ui, sans-serif
    pub mono: String,        // "JetBrains Mono", monospace
    pub display: String,     // "Inter", system-ui, sans-serif
}
```

### Font Scale

```rust
pub struct FontScale {
    pub xs: FontSize,        // 12px
    pub sm: FontSize,        // 14px
    pub base: FontSize,      // 16px
    pub lg: FontSize,        // 18px
    pub xl: FontSize,        // 20px
    pub xl2: FontSize,       // 24px
    pub xl3: FontSize,       // 30px
    pub xl4: FontSize,       // 36px
    pub xl5: FontSize,       // 48px
    pub xl6: FontSize,       // 60px
}

pub struct FontSize {
    pub size: u32,           // Pixel size
    pub line_height: f32,    // Line height ratio
    pub letter_spacing: f32, // Letter spacing in em
}
```

### Font Weights

```rust
pub enum FontWeight {
    Light = 300,
    Regular = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
}
```

## Spacing System

### Spacing Scale

```rust
pub struct SpacingScale {
    pub px: u32,             // 0px
    pub xxs: u32,            // 4px
    pub xs: u32,             // 8px
    pub sm: u32,             // 12px
    pub md: u32,             // 16px
    pub lg: u32,             // 24px
    pub xl: u32,             // 32px
    pub xl2: u32,            // 48px
    pub xl3: u32,            // 64px
    pub xl4: u32,            // 96px
    pub xl5: u32,            // 128px
}
```

## Component Library

### Button Component

```rust
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Destructive,
}

pub enum ButtonSize {
    Small,
    Medium,
    Large,
}
```

### Input Component

```rust
pub struct Input {
    pub placeholder: String,
    pub value: String,
    pub disabled: bool,
    pub error: Option<String>,
    pub size: InputSize,
}

pub enum InputSize {
    Small,
    Medium,
    Large,
}
```

### Card Component

```rust
pub struct Card {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub content: String,
    pub actions: Vec<Action>,
    pub elevation: u32,
}
```

### Modal Component

```rust
pub struct Modal {
    pub title: String,
    pub content: String,
    pub actions: Vec<ModalAction>,
    pub size: ModalSize,
    pub closable: bool,
}

pub enum ModalSize {
    Small,
    Medium,
    Large,
    Fullscreen,
}
```

### Dropdown Component

```rust
pub struct Dropdown {
    pub trigger: String,
    pub items: Vec<DropdownItem>,
    pub placement: Placement,
}

pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}
```

## Layout System

### Container

```rust
pub struct Container {
    pub max_width: u32,
    pub padding: u32,
    pub center: bool,
}
```

### Grid

```rust
pub struct Grid {
    pub columns: u32,
    pub gap: u32,
    pub responsive: bool,
}
```

### Flex

```rust
pub struct Flex {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub gap: u32,
    pub wrap: bool,
}

pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
```

## Animation System

### Easing Functions

```rust
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
}
```

### Duration Scale

```rust
pub struct DurationScale {
    pub fast: u32,      // 150ms
    pub normal: u32,    // 300ms
    pub slow: u32,      // 500ms
}
```

## Accessibility Guidelines

### WCAG 2.1 AA Compliance

- **Color Contrast**: Minimum 4.5:1 for normal text, 3:1 for large text

- **Focus Indicators**: Visible focus states for all interactive elements

- **Keyboard Navigation**: Full keyboard accessibility

- **Screen Reader Support**: Proper ARIA labels and roles

- **Text Scaling**: Support for 200% text zoom

### Accessibility Features

```rust
pub struct AccessibilityFeatures {
    pub high_contrast_mode: bool,
    pub reduced_motion: bool,
    pub screen_reader_support: bool,
    pub keyboard_navigation: bool,
    pub text_scaling: bool,
}
```

## Design Tokens

### Token Structure

```rust
pub struct DesignTokens {
    pub colors: ColorTokens,
    pub typography: TypographyTokens,
    pub spacing: SpacingTokens,
    pub borders: BorderTokens,
    pub shadows: ShadowTokens,
    pub animations: AnimationTokens,
}
```

### Token Usage

```rust
// Example token usage
let primary_color = tokens.colors.primary;
let spacing_md = tokens.spacing.md;
let font_size_base = tokens.typography.base.size;
```

## Component Generator

### Generator API

```rust
pub struct ComponentGenerator {
    pub tokens: DesignTokens,
}

impl ComponentGenerator {
    pub fn generate_button(&self, config: ButtonConfig) -> String;
    pub fn generate_input(&self, config: InputConfig) -> String;
    pub fn generate_card(&self, config: CardConfig) -> String;
    pub fn generate_modal(&self, config: ModalConfig) -> String;
}
```

## Implementation Guidelines

### File Structure

```
sigma-design-system/
├── tokens/
│   ├── colors.rs
│   ├── typography.rs
│   ├── spacing.rs
│   └── borders.rs
├── components/
│   ├── button.rs
│   ├── input.rs
│   ├── card.rs
│   └── modal.rs
├── themes/
│   ├── light.rs
│   ├── dark.rs
│   └── auto.rs
├── utils/
│   ├── generator.rs
│   └── validator.rs
└── guidelines/
    ├── ui_guidelines.md
    ├── ux_principles.md
    └── accessibility.md
```

### Usage Example

```rust
use sigma_design_system::{DesignTokens, Button, Theme};

fn main() {
    let tokens = DesignTokens::new(Theme::Dark);
    let button = Button::new()
        .variant(ButtonVariant::Primary)
        .size(ButtonSize::Medium)
        .build(&tokens);

    // Render button
}
```

## Success Criteria

- **100% design consistency** across all applications

- **WCAG 2.1 AA compliance** for accessibility

- **50+ components** in component library

- **Light/Dark theme support** with auto-switching

- **Design token system** for easy customization

- **Component generator** for developer productivity

## References

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

- [Material Design Guidelines](https://material.io/design)

- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)

---

**Document Version**: 1.0
**Last Updated**: 2026-07-05
**Status**: Draft for Implementation
**Next Review**: 2026-07-12
