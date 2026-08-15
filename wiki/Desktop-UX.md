# SigmaOS Desktop & UX

## Overview

SigmaOS provides a comprehensive desktop environment called Zenith with modern UX features including accessibility support, touch/gesture input, theme customization, a unified control center, and an onboarding wizard for new users.

## Zenith Desktop Environment

Zenith is the native desktop environment for SigmaOS, designed with a focus on accessibility, customization, and user experience.

### Architecture

**Implementation:** `desktop/zenith_*.rs`

The Zenith desktop environment is built with a modular architecture:
- **Accessibility:** Screen reader, magnifier, high-contrast themes, keyboard navigation
- **Touch:** Multi-touch tracking and gesture recognition
- **Theme Engine:** Comprehensive theming system with presets and custom themes
- **Control Center:** Unified settings management
- **Onboarding:** First-time user setup wizard

## Accessibility Features

### Overview

SigmaOS includes comprehensive accessibility features to ensure the desktop environment is usable by everyone, including users with visual, motor, or cognitive disabilities.

**Implementation:** `desktop/zenith_accessibility.rs`

### Screen Reader

The screen reader provides text-to-speech functionality for visually impaired users.

#### Screen Reader Features
```rust
pub struct ScreenReader {
    pub enabled: bool,
    pub voice: String,
    pub rate: f32, // 0.5 to 2.0
    pub pitch: f32, // 0.5 to 2.0
    pub volume: f32, // 0.0 to 1.0
}
```

#### C-ABI Functions
```rust
pub fn speak(&self, text: &str);
pub fn stop(&self);
pub fn set_voice(&mut self, voice: &str);
pub fn set_rate(&mut self, rate: f32);
pub fn set_pitch(&mut self, pitch: f32);
pub fn set_volume(&mut self, volume: f32);
```

#### Usage
```rust
let screen_reader = ScreenReader::new();
screen_reader.set_voice("en-US");
screen_reader.set_rate(1.0);
screen_reader.speak("Welcome to SigmaOS");
```

### High Contrast Themes

Zenith includes WCAG AAA compliant high-contrast themes for users with visual impairments.

#### Theme Types
```rust
pub enum Theme {
    Default,
    HighContrast,
    HighContrastDark,
    HighContrastLight,
}
```

#### WCAG AAA Compliance
All high-contrast themes meet WCAG AAA contrast ratio requirements (7:1 minimum contrast).

#### Color Palette
```rust
pub struct ThemeColors {
    pub background: [u8; 4],
    pub foreground: [u8; 4],
    pub accent: [u8; 4],
    pub border: [u8; 4],
    pub selection: [u8; 4],
}
```

#### Contrast Validation
```rust
pub fn check_wcag_aaa_contrast(&self, fg: [u8; 4], bg: [u8; 4]) -> bool {
    let fg_luminance = self.luminance(fg);
    let bg_luminance = self.luminance(bg);
    let contrast_ratio = (lighter + 0.05) / (darker + 0.05);
    contrast_ratio >= 7.0
}
```

### Screen Magnifier

The magnifier provides zoom functionality for users with low vision.

#### Magnifier Features
```rust
pub struct Magnifier {
    pub enabled: bool,
    pub zoom_level: f32, // 2.0 to 16.0
    pub x: i32,
    pub y: i32,
    pub follow_cursor: bool,
}
```

#### Zoom Levels
- Minimum: 2x
- Maximum: 16x
- Default: 2x
- Cursor following: Optional

#### C-ABI Functions
```rust
pub fn set_magnifier_enabled(&mut self, enabled: bool);
pub fn set_magnifier_zoom(&mut self, zoom: f32);
pub fn magnifier_zoom_in(&mut self);
pub fn magnifier_zoom_out(&mut self);
pub fn set_magnifier_position(&mut self, x: i32, y: i32);
```

### Keyboard Navigation

Keyboard navigation allows users to navigate the desktop without a mouse.

#### Keyboard Navigation Features
```rust
pub struct KeyboardNav {
    pub enabled: bool,
    pub focus_ring_visible: bool,
    pub focus_ring_color: [u8; 4],
    pub focus_ring_width: u32,
    pub tab_navigation: bool,
}
```

#### Focus Ring
- Customizable color (default: bright green)
- Adjustable width (default: 3px)
- Tab navigation support

#### C-ABI Functions
```rust
pub fn set_keyboard_nav_enabled(&mut self, enabled: bool);
pub fn set_focus_ring_visible(&mut self, visible: bool);
pub fn set_focus_ring_color(&mut self, color: [u8; 4]);
pub fn set_focus_ring_width(&mut self, width: u32);
```

### Reduced Motion

Reduced motion mode disables animations for users who experience motion sickness or vestibular disorders.

#### C-ABI Functions
```rust
pub fn set_reduced_motion(&mut self, enabled: bool);
pub fn is_reduced_motion(&self) -> bool;
```

## Touch and Gesture Support

### Overview

SigmaOS provides comprehensive touch and gesture support for tablets and touch-enabled devices.

**Implementation:** `desktop/zenith_touch.rs`

### Touch Point Tracking

#### Touch Point Structure
```rust
pub struct TouchPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 to 1.0
    pub major_axis: f32,
    pub minor_axis: f32,
    pub orientation: f32, // radians
    pub active: bool,
}
```

### Touch Events

#### Event Types
```rust
pub enum TouchEventType {
    Down,
    Move,
    Up,
    Cancel,
}
```

#### Touch Event Structure
```rust
pub struct TouchEvent {
    pub event_type: TouchEventType,
    pub points: Vec<TouchPoint>,
    pub timestamp: u64,
}
```

### Gesture Recognition

#### Supported Gestures
```rust
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Swipe,
    Pinch,
    Rotate,
    Pan,
    Scroll,
}
```

#### Gesture Event Structure
```rust
pub struct GestureEvent {
    pub gesture_type: GestureType,
    pub center_x: f32,
    pub center_y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub velocity: f32,
    pub timestamp: u64,
}
```

### Gesture Configuration

#### Gesture Thresholds
```rust
pub struct GestureConfig {
    pub tap_timeout: u64, // milliseconds
    pub double_tap_timeout: u64,
    pub long_press_timeout: u64,
    pub swipe_threshold: f32,
    pub pinch_threshold: f32,
    pub rotate_threshold: f32,
    pub scroll_threshold: f32,
    pub min_velocity: f32,
}
```

#### Default Values
- Tap timeout: 300ms
- Double-tap timeout: 500ms
- Long-press timeout: 500ms
- Swipe threshold: 50px
- Pinch threshold: 10px
- Rotate threshold: 0.1 radians
- Scroll threshold: 10px
- Minimum velocity: 100px/s

### Touch Manager

#### Touch Manager Structure
```rust
pub struct TouchManager {
    pub state: TouchState,
    pub config: GestureConfig,
    pub enabled: bool,
    pub multi_touch_enabled: bool,
}
```

#### C-ABI Functions
```rust
pub fn init(&mut self);
pub fn process_event(&mut self, event: &TouchEvent) -> Vec<GestureEvent>;
pub fn set_enabled(&mut self, enabled: bool);
pub fn set_multi_touch_enabled(&mut self, enabled: bool);
pub fn set_config(&mut self, config: GestureConfig);
```

### Touch Event Handler

#### Trait Definition
```rust
pub trait TouchEventHandler {
    fn on_touch_event(&mut self, event: &TouchEvent);
    fn on_gesture(&mut self, gesture: &GestureEvent);
}
```

#### Default Handler
```rust
pub struct DefaultTouchHandler {
    pub touch_manager: TouchManager,
}
```

## Theme Engine

### Overview

The Zenith theme engine provides comprehensive customization options for the desktop appearance.

**Implementation:** `modules/ui/zenith/zenith_theme_engine.rs`

### Theme Modes

```rust
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
    Custom,
}
```

### Color Palette

#### Palette Structure
```rust
pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub surface: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub border: Color,
    pub divider: Color,
}
```

### Typography

#### Typography Structure
```rust
pub struct Typography {
    pub font_family: [u8; 64],
    pub font_size_base: u32,
    pub font_size_small: u32,
    pub font_size_large: u32,
    pub font_size_h1: u32,
    pub font_size_h2: u32,
    pub font_size_h3: u32,
    pub line_height: f32,
    pub letter_spacing: f32,
}
```

### Spacing System

#### Spacing Structure
```rust
pub struct Spacing {
    pub unit: u32,
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
}
```

#### Default Values
- unit: 4px
- xs: 4px
- sm: 8px
- md: 16px
- lg: 24px
- xl: 32px

### Border Radius

#### Border Radius Structure
```rust
pub struct BorderRadius {
    pub none: u32,
    pub small: u32,
    pub medium: u32,
    pub large: u32,
    pub full: u32,
}
```

#### Default Values
- none: 0px
- small: 4px
- medium: 8px
- large: 16px
- full: 9999px

### Shadows

#### Shadow Structure
```rust
pub struct Shadows {
    pub small: Color,
    pub medium: Color,
    pub large: Color,
    pub xlarge: Color,
}
```

### Animation Settings

#### Animation Structure
```rust
pub struct AnimationSettings {
    pub duration_fast: u32,
    pub duration_normal: u32,
    pub duration_slow: u32,
    pub easing_default: [u8; 32],
    pub easing_in: [u8; 32],
    pub easing_out: [u8; 32],
    pub easing_in_out: [u8; 32],
}
```

#### Default Durations
- fast: 150ms
- normal: 300ms
- slow: 500ms

### Theme Management

#### C-ABI Functions
```c
int zenith_theme_init(void);
void zenith_theme_load_shard(void);
void zenith_theme_load_industrial(void);
int zenith_theme_set_mode(ThemeMode mode);
const Theme* zenith_theme_get_current(void);
int zenith_theme_save_custom(const uint8_t* name);
int zenith_theme_load_saved(uint32_t index);
int zenith_theme_set_custom_color(uint32_t index, Color color);
Color zenith_theme_get_custom_color(uint32_t index);
int zenith_theme_set_font_family(const uint8_t* font_family);
int zenith_theme_set_font_size(uint32_t size_type, uint32_t size);
void personalization_sync_ui(void);
```

### Preset Themes

#### Light Theme
- Background: White (#FFFFFF)
- Surface: Light Gray (#F5F5F5)
- Primary: Blue (#0078D7)
- Text: Dark Gray (#212121)

#### Dark Theme
- Background: Dark Gray (#121212)
- Surface: Medium Gray (#1E1E1E)
- Primary: Light Blue (#90CAF9)
- Text: White (#FFFFFF)

#### Shard Theme
- Background: Dark Blue (#0A0A14)
- Surface: Darker Blue (#14141E)
- Primary: Cyan (#00FFFF)
- Secondary: Magenta (#FF00FF)

#### Industrial Theme
- Background: Light Gray (#FAFAFA)
- Surface: White (#FFFFFF)
- Primary: Blue (#1976D2)
- Secondary: Gray (#424242)

### Theme Export/Import

#### JSON Export
```c
int zenith_theme_export_json(uint8_t* buffer, uint32_t buffer_size, uint32_t* bytes_written);
```

#### JSON Import
```c
int zenith_theme_import_json(const uint8_t* json, uint32_t json_len);
```

## Control Center

### Overview

The unified Control Center provides centralized access to all system settings.

**Implementation:** `desktop/zenith_control_center.rs`

### Panels

#### Available Panels
```rust
pub enum PanelType {
    Network,
    Display,
    Sound,
    Bluetooth,
    WiFi,
    Power,
    Storage,
    Accessibility,
    Security,
    Accounts,
    Updates,
    About,
}
```

### Quick Settings

#### Default Quick Settings
- WiFi toggle
- Bluetooth toggle
- Do Not Disturb toggle
- Brightness slider
- Volume slider

### Setting Types

```rust
pub enum SettingType {
    Toggle,
    Slider,
    Select,
    Text,
    Color,
    Action,
}
```

### Control Center State

```rust
pub struct ControlCenterState {
    pub panels: Vec<Panel>,
    pub current_panel: Option<PanelType>,
    pub settings: HashMap<String, SettingItem>,
    pub quick_settings: Vec<SettingItem>,
    pub notifications_enabled: bool,
    pub do_not_disturb: bool,
}
```

### Control Center Functions

```rust
pub fn init(&mut self);
pub fn open_panel(&mut self, panel_type: PanelType);
pub fn close_panel(&mut self);
pub fn get_current_panel(&self) -> Option<PanelType>;
pub fn get_panel_settings(&self, panel_type: PanelType) -> Vec<SettingItem>;
pub fn update_setting(&mut self, id: &str, value: SettingValue) -> bool;
pub fn get_setting(&self, id: &str) -> Option<SettingValue>;
pub fn update_quick_setting(&mut self, id: &str, value: SettingValue) -> bool;
pub fn get_quick_setting(&self, id: &str) -> Option<SettingValue>;
pub fn toggle_do_not_disturb(&mut self);
pub fn set_notifications_enabled(&mut self, enabled: bool);
pub fn search_settings(&self, query: &str) -> Vec<SettingItem>;
pub fn reset_to_defaults(&mut self);
```

### Control Center Manager

```rust
pub struct ControlCenterManager {
    pub state: ControlCenterState,
    pub visible: bool,
}
```

#### Manager Functions
```rust
pub fn init(&mut self);
pub fn show(&mut self);
pub fn hide(&mut self);
pub fn toggle(&mut self);
pub fn is_visible(&self) -> bool;
```

## Onboarding Wizard

### Overview

The onboarding wizard guides new users through initial system setup.

**Implementation:** `desktop/zenith_onboarding.rs`

### Onboarding Steps

```rust
pub enum OnboardingStep {
    Welcome,
    Language,
    Region,
    Keyboard,
    Network,
    Privacy,
    Account,
    Theme,
    Accessibility,
    Complete,
}
```

### User Configuration

```rust
pub struct UserConfig {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub password: String,
    pub language: String,
    pub region: String,
    pub timezone: String,
    pub keyboard_layout: String,
}
```

### System Configuration

```rust
pub struct SystemConfig {
    pub theme_mode: String,
    pub auto_updates: bool,
    pub telemetry_enabled: bool,
    pub crash_reports: bool,
    pub accessibility_enabled: bool,
    pub screen_reader_enabled: bool,
    pub high_contrast_enabled: bool,
}
```

### Onboarding Wizard Functions

```rust
pub fn init(&mut self);
pub fn get_current_page(&self) -> Option<&OnboardingPage>;
pub fn next_step(&mut self) -> bool;
pub fn previous_step(&mut self) -> bool;
pub fn skip_step(&mut self) -> bool;
pub fn jump_to_step(&mut self, step: OnboardingStep) -> bool;
pub fn set_language(&mut self, language: &str);
pub fn set_region(&mut self, region: &str);
pub fn set_timezone(&mut self, timezone: &str);
pub fn set_keyboard_layout(&mut self, layout: &str);
pub fn set_username(&mut self, username: &str);
pub fn set_full_name(&mut self, full_name: &str);
pub fn set_email(&mut self, email: &str);
pub fn set_password(&mut self, password: &str);
pub fn set_theme_mode(&mut self, theme_mode: &str);
pub fn set_auto_updates(&mut self, enabled: bool);
pub fn set_telemetry(&mut self, enabled: bool);
pub fn set_crash_reports(&mut self, enabled: bool);
pub fn set_accessibility_enabled(&mut self, enabled: bool);
pub fn set_screen_reader_enabled(&mut self, enabled: bool);
pub fn set_high_contrast_enabled(&mut self, enabled: bool);
pub fn validate_current_step(&self) -> bool;
pub fn get_progress(&self) -> u8;
pub fn is_complete(&self) -> bool;
pub fn skip_all(&mut self);
pub fn was_skipped(&self) -> bool;
pub fn apply_configurations(&self) -> bool;
```

### Onboarding Manager

```rust
pub struct OnboardingManager {
    pub wizard: OnboardingWizard,
    pub first_boot: bool,
}
```

#### Manager Functions
```rust
pub fn init(&mut self);
pub fn start_onboarding(&mut self);
pub fn complete_onboarding(&mut self) -> bool;
pub fn is_first_boot(&self) -> bool;
```

## Configuration

### Accessibility Configuration

Example accessibility configuration:
```rust
let mut a11y = AccessibilityManager::new();
a11y.init();
a11y.set_theme(Theme::HighContrast);
a11y.set_magnifier_enabled(true);
a11y.set_magnifier_zoom(3.0);
a11y.set_keyboard_nav_enabled(true);
a11y.set_screen_reader_enabled(true);
a11y.set_reduced_motion(true);
```

### Touch Configuration

Example touch configuration:
```rust
let mut touch = TouchManager::new();
touch.init();
touch.set_enabled(true);
touch.set_multi_touch_enabled(true);
let config = GestureConfig {
    tap_timeout: 300,
    swipe_threshold: 50.0,
    ..Default::default()
};
touch.set_config(config);
```

### Theme Configuration

Example theme configuration:
```rust
zenith_theme_init();
zenith_theme_set_mode(ThemeMode::Dark);
zenith_theme_set_custom_color(0, Color { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF });
zenith_theme_set_font_family(b"Roboto");
zenith_theme_set_font_size(0, 16); // base size
personalization_sync_ui();
```

### Control Center Configuration

Example control center usage:
```rust
let mut cc = ControlCenterManager::new();
cc.init();
cc.show();
cc.state.open_panel(PanelType::Display);
cc.state.update_setting("display_brightness", SettingValue::Float(0.8));
cc.state.toggle_do_not_disturb();
```

### Onboarding Configuration

Example onboarding usage:
```rust
let mut onboarding = OnboardingManager::new();
onboarding.init();
onboarding.start_onboarding();
onboarding.wizard.set_language("en_US");
onboarding.wizard.set_region("US");
onboarding.wizard.set_username("user");
onboarding.wizard.set_full_name("User Name");
onboarding.wizard.set_password("password");
onboarding.wizard.next_step();
onboarding.complete_onboarding();
```

## Performance Optimization

### Accessibility Performance
- Screen reader uses efficient TTS caching
- Magnifier uses hardware-accelerated scaling
- Focus ring rendering optimized for minimal overhead

### Touch Performance
- Touch point tracking uses efficient data structures
- Gesture recognition uses incremental calculations
- Velocity calculation optimized for real-time detection

### Theme Performance
- Theme changes use efficient diff algorithm
- Color palette updates are batched
- Font changes use lazy loading

## Troubleshooting

### Accessibility Issues
- Check if screen reader is enabled
- Verify TTS engine is installed
- Check magnifier zoom level
- Verify keyboard navigation is enabled

### Touch Issues
- Check if touch is enabled
- Verify gesture thresholds
- Check multi-touch support
- Verify touch driver is loaded

### Theme Issues
- Check if theme engine is initialized
- Verify theme mode is set correctly
- Check custom colors are valid
- Verify font is installed

### Control Center Issues
- Check if control center is visible
- Verify panel is loaded
- Check setting values are valid
- Verify search query format

### Onboarding Issues
- Check if it's first boot
- Verify step validation
- Check configuration values
- Verify configuration application

## References

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Material Design Guidelines](https://material.io/design)
- [Kernel Architecture](Kernel-Architecture.md)
- [Security Documentation](Security.md)
- [Networking Documentation](Networking.md)

## License

All SigmaOS desktop and UX components are licensed under MIT License. See [LICENSE](../LICENSE) for details.
