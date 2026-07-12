# SigmaOS Zenith Desktop Specification

## Overview

Zenith Desktop is the native, Wayland-first desktop environment for SigmaOS. Built with performance and modern aesthetics in mind, Zenith features a hardware-accelerated compositor, native screen reading APIs, and deep localization with out-of-the-box support for major Indic languages.

### Key Features

- **Wayland-First**: Native Wayland compositor with no X11 dependencies
- **Hardware Acceleration**: Vulkan-based rendering for maximum performance
- **Accessibility**: Built-in screen reader and accessibility APIs
- **Indic Languages**: Full support for Indian languages and input methods
- **Modern Design**: Clean, modern UI with consistent design language
- **Performance**: Optimized for low-latency and smooth animations
- **Extensible**: Plugin system for custom widgets and extensions

## Architecture

### Compositor Flow

```
 [User Input (Wayland Events)]
               │
               ▼
   [Zenith Wayland Compositor] ◄──► [Accessibility screen reader daemon]
               │
               ▼
   [Indic Language Input IM] (IBus/Fcitx API)
               │
               ▼
   [Vulkan Desktop Layout Renderer]
```

### Component Architecture

```
┌─────────────────────────────────────────┐
│         Zenith Desktop Environment      │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Compositor│ Shell    │ Panel        │ │
│  └──────────┴──────────┴──────────────┘ │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Window   │ Input    │ Accessibility │ │
│  │ Manager  │ Method   │ Services     │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Wayland Protocol Layer          │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Vulkan Rendering Engine          │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│              GPU Hardware               │
└─────────────────────────────────────────┘
```

## Configuration

### Compositor Configuration

**File**: `/etc/zenith/compositor.conf`

```toml
[compositor]
renderer = "vulkan"
vsync = true
scaling = "hidpi"
max_fps = 144
tearing = false

[display]
scale = 1.0
refresh_rate = 144
color_profile = "srgb"
hdr = false

[accessibility]
screen_reader = true
default_locale = "hi_IN" # Hindi (India)
input_method = "ibus-m17n"
high_contrast = false
large_text = false

[performance]
gpu_acceleration = true
memory_limit = "2GB"
thread_count = 4
```

### Shell Configuration

**File**: `/etc/zenith/shell.conf`

```toml
[shell]
theme = "zenith-dark"
icon_theme = "zenith-icons"
font = "Inter 11"
panel_position = "bottom"
panel_height = 48

[desktop]
wallpaper = "/usr/share/zenith/wallpapers/default.jpg"
show_icons = true
grid_size = 64
sort_method = "name"

[windows]
border_width = 2
shadow = true
blur = true
animations = true
```

## Technical Implementation

### Compositor Core

```rust
// userland/apps/zenith-compositor/src/compositor.rs
use ash::vk;
use ash::Device;
use ash::extensions::khr::Swapchain;

pub struct ZenithCompositor {
    pub vk_device: Device,
    pub swapchain: Swapchain,
    pub screen_reader_active: bool,
    pub accessibility_service: AccessibilityService,
    pub input_method: InputMethodManager,
}

impl ZenithCompositor {
    pub fn new() -> Result<Self, CompositorError> {
        let vk_device = Self::init_vulkan()?;
        let swapchain = Self::create_swapchain(&vk_device)?;
        let accessibility_service = AccessibilityService::new()?;
        let input_method = InputMethodManager::new("ibus-m17n")?;

        Ok(Self {
            vk_device,
            swapchain,
            screen_reader_active: false,
            accessibility_service,
            input_method,
        })
    }

    pub fn draw_desktop_elements(&self) -> Result<(), CompositorError> {
        // GPU accelerated composite rendering of panels, taskbars and windows
        self.render_panels()?;
        self.render_windows()?;
        self.render_overlays()?;

        if self.screen_reader_active {
            self.announce_accessibility_focus()?;
        }

        Ok(())
    }

    pub fn handle_wayland_event(&mut self, event: WaylandEvent) -> Result<(), CompositorError> {
        match event {
            WaylandEvent::Keyboard(input) => {
                self.input_method.handle_input(input)?;
            }
            WaylandEvent::Pointer(motion) => {
                self.handle_pointer_motion(motion)?;
            }
            WaylandEvent::Touch(touch) => {
                self.handle_touch(touch)?;
            }
        }
        Ok(())
    }

    pub fn toggle_screen_reader(&mut self) -> Result<(), CompositorError> {
        self.screen_reader_active = !self.screen_reader_active;
        if self.screen_reader_active {
            self.accessibility_service.enable()?;
        } else {
            self.accessibility_service.disable()?;
        }
        Ok(())
    }
}
```

### Window Manager

```rust
// userland/apps/zenith-shell/src/window_manager.rs
pub struct WindowManager {
    windows: Vec<Window>,
    focused_window: Option<usize>,
    layout_manager: LayoutManager,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            windows: Vec::new(),
            focused_window: None,
            layout_manager: LayoutManager::new(),
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
        self.layout_manager.relayout(&mut self.windows);
    }

    pub fn focus_window(&mut self, window_id: usize) {
        self.focused_window = Some(window_id);
        self.windows[window_id].set_focused(true);

        // Announce to screen reader
        if let Some(ref mut sr) = self.screen_reader {
            sr.announce_window_focus(&self.windows[window_id]);
        }
    }

    pub fn close_window(&mut self, window_id: usize) {
        self.windows.remove(window_id);
        self.layout_manager.relayout(&mut self.windows);
    }
}
```

### Accessibility Service

```rust
// userland/apps/zenith-compositor/src/accessibility.rs
pub struct AccessibilityService {
    screen_reader: ScreenReader,
    braille_display: Option<BrailleDisplay>,
    high_contrast_mode: bool,
}

impl AccessibilityService {
    pub fn new() -> Result<Self, AccessibilityError> {
        let screen_reader = ScreenReader::new()?;
        let braille_display = BrailleDisplay::detect()?;

        Ok(Self {
            screen_reader,
            braille_display,
            high_contrast_mode: false,
        })
    }

    pub fn enable(&mut self) -> Result<(), AccessibilityError> {
        self.screen_reader.enable()?;
        if let Some(ref mut bd) = self.braille_display {
            bd.enable()?;
        }
        Ok(())
    }

    pub fn announce_window_focus(&self, window: &Window) {
        let text = format!("Window focused: {}", window.title());
        self.screen_reader.speak(&text);
    }

    pub fn announce_text_change(&self, text: &str) {
        self.screen_reader.speak(text);
    }
}
```

### Input Method Integration

```rust
// userland/apps/zenith-compositor/src/input_method.rs
pub struct InputMethodManager {
    backend: InputMethodBackend,
    current_locale: String,
}

impl InputMethodManager {
    pub fn new(backend: &str) -> Result<Self, InputMethodError> {
        let backend = match backend {
            "ibus" => InputMethodBackend::IBus,
            "fcitx" => InputMethodBackend::Fcitx,
            _ => return Err(InputMethodError::InvalidBackend),
        };

        Ok(Self {
            backend,
            current_locale: "en_US".to_string(),
        })
    }

    pub fn set_locale(&mut self, locale: &str) -> Result<(), InputMethodError> {
        self.current_locale = locale.to_string();
        self.backend.set_locale(locale)?;
        Ok(())
    }

    pub fn handle_input(&self, input: InputEvent) -> Result<String, InputMethodError> {
        match self.backend {
            InputMethodBackend::IBus => self.handle_ibus_input(input),
            InputMethodBackend::Fcitx => self.handle_fcitx_input(input),
        }
    }
}
```

## Indic Language Support

### Supported Languages

- **Hindi (hi_IN)**: Devanagari script
- **Bengali (bn_IN)**: Bengali script
- **Tamil (ta_IN)**: Tamil script
- **Telugu (te_IN)**: Telugu script
- **Marathi (mr_IN)**: Devanagari script
- **Gujarati (gu_IN)**: Gujarati script
- **Kannada (kn_IN)**: Kannada script
- **Malayalam (ml_IN)**: Malayalam script
- **Punjabi (pa_IN)**: Gurmukhi script
- **Urdu (ur_IN)**: Perso-Arabic script

### Input Methods

**IBus Integration**:
```toml
[input_method]
engine = "ibus"
layouts = ["hi", "bn", "ta", "te"]
candidates = 10
preedit = true
```

**Fcitx Integration**:
```toml
[input_method]
engine = "fcitx"
layouts = ["hi", "bn", "ta", "te"]
candidates = 10
preedit = true
```

### Font Configuration

```toml
[fonts]
default = "Noto Sans"
size = 11
hinting = "full"
antialiasing = true

[indic_fonts]
hindi = "Noto Sans Devanagari"
bengali = "Noto Sans Bengali"
tamil = "Noto Sans Tamil"
telugu = "Noto Sans Telugu"
```

## Accessibility Features

### Screen Reader

**Features**:
- Text-to-speech synthesis
- Window focus announcements
- Text change notifications
- Keyboard navigation support
- Braille display support

**Configuration**:
```toml
[screen_reader]
enabled = true
voice = "en-US"
rate = 1.0
pitch = 1.0
volume = 1.0
```

### High Contrast Mode

**Features**:
- Increased contrast colors
- Larger text
- Focus indicators
- Customizable color schemes

**Configuration**:
```toml
[high_contrast]
enabled = false
theme = "high-contrast"
text_scale = 1.2
```

### Keyboard Navigation

**Features**:
- Full keyboard support
- Keyboard shortcuts
- Focus indicators
- Tab navigation

**Shortcuts**:
- `Alt+Tab`: Switch windows
- `Super+D`: Show desktop
- `Super+L`: Lock screen
- `Ctrl+Alt+T`: Open terminal
- `Super+E`: Open file manager

## Performance Optimization

### GPU Acceleration

**Vulkan Features**:
- Hardware-accelerated rendering
- Multi-threaded command buffers
- Descriptor set management
- Pipeline caching

**Configuration**:
```toml
[vulkan]
validation = false
synchronization = true
pipeline_cache = true
descriptor_cache = true
```

### Memory Management

**Features**:
- Texture compression
- Memory pooling
- Resource recycling
- Lazy loading

**Configuration**:
```toml
[memory]
texture_compression = true
pool_size = "512MB"
lazy_loading = true
```

## Theming

### Theme System

**Structure**:
```
/usr/share/zenith/themes/
├── zenith-dark/
│   ├── theme.conf
│   ├── colors.conf
│   └── widgets/
├── zenith-light/
│   ├── theme.conf
│   ├── colors.conf
│   └── widgets/
└── custom/
    ├── theme.conf
    ├── colors.conf
    └── widgets/
```

**Theme Configuration**:
```toml
[theme]
name = "Zenith Dark"
version = "1.0"
author = "SigmaOS Team"

[colors]
background = "#1e1e1e"
foreground = "#ffffff"
primary = "#3b82f6"
secondary = "#10b981"
accent = "#f59e0b"
error = "#ef4444"
warning = "#f59e0b"
success = "#10b981"
```

### Icon Themes

**Supported Themes**:
- Zenith Icons
- Adwaita
- Papirus
- Numix

**Configuration**:
```toml
[icons]
theme = "zenith-icons"
size = 24
symbolic = true
```

## Plugin System

### Plugin Architecture

```rust
// userland/apps/zenith-shell/src/plugin.rs
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&mut self) -> Result<(), PluginError>;
    fn shutdown(&mut self) -> Result<(), PluginError>;
    fn handle_event(&mut self, event: &Event) -> Result<(), PluginError>;
}
```

### Plugin Examples

**System Monitor Plugin**:
```rust
pub struct SystemMonitorPlugin {
    cpu_usage: f32,
    memory_usage: f32,
}

impl Plugin for SystemMonitorPlugin {
    fn name(&self) -> &str {
        "System Monitor"
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), PluginError> {
        match event {
            Event::Timer => self.update_stats(),
            _ => Ok(()),
        }
    }
}
```

## Best Practices

### Development

1. **Modular Design**: Keep components independent
2. **Clear APIs**: Define clear interfaces
3. **Testing**: Comprehensive testing
4. **Documentation**: Document all public APIs

### Performance

1. **Profiling**: Profile regularly
2. **Optimization**: Optimize hot paths
3. **Memory**: Minimize memory usage
4. **GPU**: Utilize GPU efficiently

### Accessibility

1. **Screen Reader**: Ensure screen reader compatibility
2. **Keyboard**: Full keyboard support
3. **High Contrast**: Support high contrast mode
4. **Localization**: Full localization support

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Core Wayland compositor protocols
- Vulkan backend rendering
- Basic window management
- Panel and taskbar

### Phase 2 (Months 3-6)
- IBus integration for Indic languages
- Input method support
- Layout engines
- Font configuration

### Phase 3 (Months 6-9)
- Integrated speech synthesizer
- Screen reading engine
- Braille display support
- High contrast mode

### Phase 4 (Months 9-12)
- Gestural accessibility control
- Multi-display support
- Color profiles
- Plugin system

## References

- [Wayland Protocol](https://wayland.freedesktop.org/)
- [Vulkan Specification](https://www.khronos.org/vulkan/)
- [IBus Documentation](https://ibus.github.io/docs/)
- [Accessibility Guidelines](https://www.w3.org/WAI/)
- [GNOME HIG](https://developer.gnome.org/hig/)
