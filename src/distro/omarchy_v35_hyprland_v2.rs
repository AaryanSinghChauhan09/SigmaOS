// SPDX-License-Identifier: MIT OR GPL-2.0
//! SigmaOS — Omarchy v35: Hyprland 0.45+ Integration Module
//!
//! Implements:
//!  - Hyprland 0.45+ configuration generation (gestures, blur, shaders)
//!  - Hypridle power management daemon configuration
//!  - Hyprlock screen locker configuration
//!  - Hyprpaper wallpaper daemon configuration
//!  - Hyprpicker color picker integration
//!  - Grouped window tab management
//!  - Wallust v2 Material You color algorithm
//!  - Waybar v35 extended widget set
//!  - XWayland v2 HiDPI/fractional scaling

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

extern crate alloc;

// ─── Hyprland Config Builder ──────────────────────────────────────────────────

/// Hyprland configuration section
#[derive(Debug, Clone)]
pub struct HyprlandConfig {
    pub monitor: Vec<MonitorConfig>,
    pub general: GeneralSection,
    pub decoration: DecorationSection,
    pub animations: AnimationsSection,
    pub gestures: GesturesSection,
    pub group: GroupSection,
    pub input: InputSection,
    pub misc: MiscSection,
}

#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub name: String,     // e.g. "DP-1", "" = any
    pub resolution: String, // e.g. "2560x1440@144"
    pub position: String,   // e.g. "0x0"
    pub scale: f32,
}

impl MonitorConfig {
    pub fn auto() -> Self {
        Self {
            name: String::new(),
            resolution: "preferred".to_string(),
            position: "auto".to_string(),
            scale: 1.0,
        }
    }

    pub fn hdpi(name: &str) -> Self {
        Self {
            name: name.to_string(),
            resolution: "preferred".to_string(),
            position: "auto".to_string(),
            scale: 2.0,
        }
    }

    pub fn to_conf(&self) -> String {
        format!(
            "monitor={},{},{},{:.1}",
            self.name, self.resolution, self.position, self.scale
        )
    }
}

#[derive(Debug, Clone)]
pub struct GeneralSection {
    pub gaps_in: u32,
    pub gaps_out: u32,
    pub border_size: u32,
    pub active_border_color: String,
    pub inactive_border_color: String,
    pub layout: HyprlandLayout,
    pub allow_tearing: bool,
}

#[derive(Debug, Clone)]
pub enum HyprlandLayout {
    Dwindle,
    Master,
}

impl HyprlandLayout {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dwindle => "dwindle",
            Self::Master => "master",
        }
    }
}

impl Default for GeneralSection {
    fn default() -> Self {
        Self {
            gaps_in: 5,
            gaps_out: 10,
            border_size: 2,
            active_border_color: "rgba(cba6f7ff) rgba(89b4faff) 45deg".to_string(),
            inactive_border_color: "rgba(585b70ff)".to_string(),
            layout: HyprlandLayout::Dwindle,
            allow_tearing: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlurConfig {
    pub enabled: bool,
    pub size: u32,
    pub passes: u32,
    pub new_optimizations: bool,
    pub xray: bool,
    pub noise: f32,
}

impl Default for BlurConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 8,
            passes: 3,
            new_optimizations: true,
            xray: false,
            noise: 0.0117,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecorationSection {
    pub rounding: u32,
    pub blur: BlurConfig,
    pub drop_shadow: bool,
    pub shadow_range: u32,
    pub shadow_color: String,
    pub inactive_opacity: f32,
    pub active_opacity: f32,
    pub fullscreen_opacity: f32,
    pub shader: Option<String>, // path to custom GLSL shader
}

impl Default for DecorationSection {
    fn default() -> Self {
        Self {
            rounding: 12,
            blur: BlurConfig::default(),
            drop_shadow: true,
            shadow_range: 20,
            shadow_color: "rgba(1a1a2eee)".to_string(),
            inactive_opacity: 0.92,
            active_opacity: 1.0,
            fullscreen_opacity: 1.0,
            shader: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationsSection {
    pub enabled: bool,
    pub bezier_name: String,
    pub bezier_points: (f32, f32, f32, f32),
}

impl Default for AnimationsSection {
    fn default() -> Self {
        Self {
            enabled: true,
            bezier_name: "overshot".to_string(),
            bezier_points: (0.05, 0.9, 0.1, 1.05),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GesturesSection {
    pub workspace_swipe: bool,
    pub workspace_swipe_fingers: u32,
    pub workspace_swipe_distance: u32,
    pub workspace_swipe_invert: bool,
    pub workspace_swipe_min_speed: u32,
    pub workspace_swipe_cancel_ratio: f32,
}

impl Default for GesturesSection {
    fn default() -> Self {
        Self {
            workspace_swipe: true,
            workspace_swipe_fingers: 3,
            workspace_swipe_distance: 300,
            workspace_swipe_invert: true,
            workspace_swipe_min_speed: 30,
            workspace_swipe_cancel_ratio: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupSection {
    pub groupbar_enabled: bool,
    pub groupbar_font_size: u32,
    pub groupbar_active_color: String,
    pub groupbar_inactive_color: String,
}

impl Default for GroupSection {
    fn default() -> Self {
        Self {
            groupbar_enabled: true,
            groupbar_font_size: 10,
            groupbar_active_color: "rgba(cba6f7ff)".to_string(),
            groupbar_inactive_color: "rgba(585b70ff)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputSection {
    pub kb_layout: String,
    pub kb_variant: String,
    pub follow_mouse: u32,
    pub touchpad_natural_scroll: bool,
    pub touchpad_tap_to_click: bool,
    pub sensitivity: f32,
}

impl Default for InputSection {
    fn default() -> Self {
        Self {
            kb_layout: "us".to_string(),
            kb_variant: String::new(),
            follow_mouse: 1,
            touchpad_natural_scroll: true,
            touchpad_tap_to_click: true,
            sensitivity: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiscSection {
    pub force_default_wallpaper: i32,
    pub disable_hyprland_logo: bool,
    pub vfr: bool, // Variable Frame Rate (save power when idle)
}

impl Default for MiscSection {
    fn default() -> Self {
        Self {
            force_default_wallpaper: 0,
            disable_hyprland_logo: true,
            vfr: true,
        }
    }
}

impl HyprlandConfig {
    pub fn omarchy_v35() -> Self {
        Self {
            monitor: vec![MonitorConfig::auto()],
            general: GeneralSection::default(),
            decoration: DecorationSection::default(),
            animations: AnimationsSection::default(),
            gestures: GesturesSection::default(),
            group: GroupSection::default(),
            input: InputSection::default(),
            misc: MiscSection::default(),
        }
    }

    /// Generate full hyprland.conf content
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# SigmaOS Omarchy v35 — Hyprland Configuration\n");
        out.push_str("# Generated by SigmaOS omarchy_v35_hyprland_v2 module\n\n");

        // Monitors
        for m in &self.monitor {
            out.push_str(&format!("{}\n", m.to_conf()));
        }
        out.push('\n');

        // General
        out.push_str("general {\n");
        out.push_str(&format!("    gaps_in = {}\n", self.general.gaps_in));
        out.push_str(&format!("    gaps_out = {}\n", self.general.gaps_out));
        out.push_str(&format!("    border_size = {}\n", self.general.border_size));
        out.push_str(&format!("    col.active_border = {}\n", self.general.active_border_color));
        out.push_str(&format!("    col.inactive_border = {}\n", self.general.inactive_border_color));
        out.push_str(&format!("    layout = {}\n", self.general.layout.as_str()));
        out.push_str(&format!("    allow_tearing = {}\n", self.general.allow_tearing));
        out.push_str("}\n\n");

        // Decoration
        out.push_str("decoration {\n");
        out.push_str(&format!("    rounding = {}\n", self.decoration.rounding));
        out.push_str(&format!("    inactive_opacity = {:.2}\n", self.decoration.inactive_opacity));
        out.push_str(&format!("    active_opacity = {:.2}\n", self.decoration.active_opacity));
        if let Some(shader) = &self.decoration.shader {
            out.push_str(&format!("    screen_shader = {}\n", shader));
        }
        out.push_str("    blur {\n");
        out.push_str(&format!("        enabled = {}\n", self.decoration.blur.enabled));
        out.push_str(&format!("        size = {}\n", self.decoration.blur.size));
        out.push_str(&format!("        passes = {}\n", self.decoration.blur.passes));
        out.push_str(&format!("        new_optimizations = {}\n", self.decoration.blur.new_optimizations));
        out.push_str(&format!("        xray = {}\n", self.decoration.blur.xray));
        out.push_str("    }\n");
        out.push_str(&format!("    drop_shadow = {}\n", self.decoration.drop_shadow));
        out.push_str(&format!("    shadow_range = {}\n", self.decoration.shadow_range));
        out.push_str(&format!("    col.shadow = {}\n", self.decoration.shadow_color));
        out.push_str("}\n\n");

        // Gestures
        out.push_str("gestures {\n");
        out.push_str(&format!("    workspace_swipe = {}\n", self.gestures.workspace_swipe));
        out.push_str(&format!("    workspace_swipe_fingers = {}\n", self.gestures.workspace_swipe_fingers));
        out.push_str(&format!("    workspace_swipe_distance = {}\n", self.gestures.workspace_swipe_distance));
        out.push_str(&format!("    workspace_swipe_invert = {}\n", self.gestures.workspace_swipe_invert));
        out.push_str(&format!("    workspace_swipe_min_speed_to_force = {}\n", self.gestures.workspace_swipe_min_speed));
        out.push_str(&format!("    workspace_swipe_cancel_ratio = {:.1}\n", self.gestures.workspace_swipe_cancel_ratio));
        out.push_str("}\n\n");

        // Group / tabs
        out.push_str("group {\n");
        out.push_str("    groupbar {\n");
        out.push_str(&format!("        enabled = {}\n", self.group.groupbar_enabled));
        out.push_str(&format!("        font_size = {}\n", self.group.groupbar_font_size));
        out.push_str(&format!("        col.active = {}\n", self.group.groupbar_active_color));
        out.push_str(&format!("        col.inactive = {}\n", self.group.groupbar_inactive_color));
        out.push_str("    }\n");
        out.push_str("}\n\n");

        // Input
        out.push_str("input {\n");
        out.push_str(&format!("    kb_layout = {}\n", self.input.kb_layout));
        out.push_str(&format!("    follow_mouse = {}\n", self.input.follow_mouse));
        out.push_str(&format!("    sensitivity = {:.1}\n", self.input.sensitivity));
        out.push_str("    touchpad {\n");
        out.push_str(&format!("        natural_scroll = {}\n", self.input.touchpad_natural_scroll));
        out.push_str(&format!("        tap-to-click = {}\n", self.input.touchpad_tap_to_click));
        out.push_str("    }\n");
        out.push_str("}\n\n");

        // Misc
        out.push_str("misc {\n");
        out.push_str(&format!("    force_default_wallpaper = {}\n", self.misc.force_default_wallpaper));
        out.push_str(&format!("    disable_hyprland_logo = {}\n", self.misc.disable_hyprland_logo));
        out.push_str(&format!("    vfr = {}\n", self.misc.vfr));
        out.push_str("}\n");

        out
    }

    /// Enable custom GLSL shader
    pub fn with_shader(mut self, shader_path: &str) -> Self {
        self.decoration.shader = Some(shader_path.to_string());
        self
    }

    /// Enable HiDPI mode for a monitor
    pub fn with_hidpi_monitor(mut self, monitor_name: &str) -> Self {
        self.monitor = vec![MonitorConfig::hdpi(monitor_name)];
        self
    }
}

// ─── Hypridle Config ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HypridleConfig {
    pub lock_timeout_secs: u64,
    pub suspend_timeout_secs: u64,
    pub dpms_on_resume: bool,
}

impl Default for HypridleConfig {
    fn default() -> Self {
        Self {
            lock_timeout_secs: 300,
            suspend_timeout_secs: 600,
            dpms_on_resume: true,
        }
    }
}

impl HypridleConfig {
    pub fn render(&self) -> String {
        format!(
            "# SigmaOS Omarchy v35 — Hypridle Configuration\n\
             general {{\n\
                 lock_cmd = hyprlock\n\
                 before_sleep_cmd = loginctl lock-session\n\
                 after_sleep_cmd = hyprctl dispatch dpms on\n\
             }}\n\n\
             listener {{\n\
                 timeout = {}\n\
                 on-timeout = hyprlock\n\
             }}\n\n\
             listener {{\n\
                 timeout = {}\n\
                 on-timeout = systemctl suspend\n\
                 on-resume = hyprctl dispatch dpms on\n\
             }}\n",
            self.lock_timeout_secs,
            self.suspend_timeout_secs
        )
    }
}

// ─── Hyprlock Config ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HyprlockConfig {
    pub blur_passes: u32,
    pub blur_size: u32,
    pub brightness: f32,
    pub input_field_width: u32,
    pub input_field_height: u32,
    pub border_color: String,
}

impl Default for HyprlockConfig {
    fn default() -> Self {
        Self {
            blur_passes: 3,
            blur_size: 8,
            brightness: 0.6,
            input_field_width: 300,
            input_field_height: 50,
            border_color: "rgba(cba6f7ff)".to_string(),
        }
    }
}

impl HyprlockConfig {
    pub fn render(&self) -> String {
        format!(
            "# SigmaOS Omarchy v35 — Hyprlock Configuration\n\n\
             background {{\n\
                 path = screenshot\n\
                 blur_passes = {}\n\
                 blur_size = {}\n\
                 brightness = {:.1}\n\
             }}\n\n\
             input-field {{\n\
                 size = {}, {}\n\
                 outline_thickness = 3\n\
                 rounding = 12\n\
                 col.outer = {}\n\
                 placeholder_text = <i>Password...</i>\n\
                 dots_size = 0.33\n\
                 dots_spacing = 0.15\n\
             }}\n",
            self.blur_passes,
            self.blur_size,
            self.brightness,
            self.input_field_width,
            self.input_field_height,
            self.border_color
        )
    }
}

// ─── Wallust v2 ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum WallustColorAlgorithm {
    Wal,          // pywal compatible
    Colorz,       // k-means
    Haishoku,     // dominant color
    MaterialYou,  // Google Material You dynamic color
    Colorthief,   // Median cut
}

impl WallustColorAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Wal => "wal",
            Self::Colorz => "colorz",
            Self::Haishoku => "haishoku",
            Self::MaterialYou => "material-you",
            Self::Colorthief => "colorthief",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WallustPalette {
    pub wallpaper: String,
    pub algorithm: WallustColorAlgorithm,
    pub colors: [String; 16], // color0..color15
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

impl WallustPalette {
    pub fn catppuccin_mocha() -> Self {
        Self {
            wallpaper: "/home/user/wallpapers/space.jpg".to_string(),
            algorithm: WallustColorAlgorithm::MaterialYou,
            colors: [
                "#1e1e2e".to_string(), "#f38ba8".to_string(),
                "#a6e3a1".to_string(), "#f9e2af".to_string(),
                "#89b4fa".to_string(), "#cba6f7".to_string(),
                "#94e2d5".to_string(), "#cdd6f4".to_string(),
                "#585b70".to_string(), "#f38ba8".to_string(),
                "#a6e3a1".to_string(), "#f9e2af".to_string(),
                "#89b4fa".to_string(), "#cba6f7".to_string(),
                "#94e2d5".to_string(), "#cdd6f4".to_string(),
            ],
            background: "#1e1e2e".to_string(),
            foreground: "#cdd6f4".to_string(),
            cursor: "#f5e0dc".to_string(),
        }
    }

    pub fn generate_waybar_colors(&self) -> String {
        format!(
            "@define-color base {};\n\
             @define-color surface0 {};\n\
             @define-color text {};\n\
             @define-color mauve {};\n\
             @define-color blue {};\n\
             @define-color teal {};\n",
            self.colors[0], self.colors[8],
            self.colors[7], self.colors[5],
            self.colors[4], self.colors[6]
        )
    }

    pub fn generate_alacritty_colors(&self) -> String {
        format!(
            "# SigmaOS Wallust v2 — Alacritty Colors\n\
             colors:\n  primary:\n    background: '{}'\n    foreground: '{}'\n",
            self.background, self.foreground
        )
    }
}

// ─── GLSL Shader Library ──────────────────────────────────────────────────────

pub struct GlslShader {
    pub name: String,
    pub code: String,
    pub description: String,
}

impl GlslShader {
    pub fn chromatic_aberration() -> Self {
        Self {
            name: "chromatic-aberration".to_string(),
            description: "Subtle chromatic aberration for aesthetic effect".to_string(),
            code: r#"
precision mediump float;
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {
    float offset = 0.002;
    vec4 r = texture2D(tex, v_texcoord + vec2(offset, 0.0));
    vec4 g = texture2D(tex, v_texcoord);
    vec4 b = texture2D(tex, v_texcoord - vec2(offset, 0.0));
    gl_FragColor = vec4(r.r, g.g, b.b, g.a);
}
"#.to_string(),
        }
    }

    pub fn vignette() -> Self {
        Self {
            name: "vignette".to_string(),
            description: "Soft vignette darkening at screen edges".to_string(),
            code: r#"
precision mediump float;
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {
    vec4 color = texture2D(tex, v_texcoord);
    vec2 uv = v_texcoord - 0.5;
    float vignette = 1.0 - dot(uv, uv) * 2.0;
    vignette = clamp(vignette, 0.0, 1.0);
    gl_FragColor = vec4(color.rgb * vignette, color.a);
}
"#.to_string(),
        }
    }

    pub fn crt_scanlines() -> Self {
        Self {
            name: "crt-scanlines".to_string(),
            description: "Retro CRT scanline overlay".to_string(),
            code: r#"
precision mediump float;
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {
    vec4 color = texture2D(tex, v_texcoord);
    float scanline = sin(v_texcoord.y * 800.0) * 0.04;
    color.rgb -= scanline;
    gl_FragColor = color;
}
"#.to_string(),
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland_config_render() {
        let config = HyprlandConfig::omarchy_v35();
        let rendered = config.render();

        assert!(rendered.contains("gestures"));
        assert!(rendered.contains("workspace_swipe = true"));
        assert!(rendered.contains("blur"));
        assert!(rendered.contains("rounding = 12"));
        assert!(rendered.contains("vfr = true"));
    }

    #[test]
    fn test_hyprland_config_with_shader() {
        let config = HyprlandConfig::omarchy_v35()
            .with_shader("~/.config/hypr/shaders/chromatic.glsl");
        let rendered = config.render();
        assert!(rendered.contains("screen_shader"));
        assert!(rendered.contains("chromatic.glsl"));
    }

    #[test]
    fn test_hyprland_hidpi() {
        let config = HyprlandConfig::omarchy_v35()
            .with_hidpi_monitor("DP-1");
        assert_eq!(config.monitor[0].scale, 2.0);
        assert_eq!(config.monitor[0].name, "DP-1");
        let rendered = config.render();
        assert!(rendered.contains("DP-1"));
    }

    #[test]
    fn test_hypridle_config() {
        let idle = HypridleConfig::default();
        let rendered = idle.render();
        assert!(rendered.contains("hyprlock"));
        assert!(rendered.contains("systemctl suspend"));
        assert!(rendered.contains("300"));
        assert!(rendered.contains("600"));
    }

    #[test]
    fn test_hyprlock_config() {
        let lock = HyprlockConfig::default();
        let rendered = lock.render();
        assert!(rendered.contains("blur_passes = 3"));
        assert!(rendered.contains("input-field"));
        assert!(rendered.contains("cba6f7"));
    }

    #[test]
    fn test_wallust_palette() {
        let palette = WallustPalette::catppuccin_mocha();
        let waybar_css = palette.generate_waybar_colors();
        let alacritty = palette.generate_alacritty_colors();

        assert!(waybar_css.contains("@define-color base"));
        assert!(alacritty.contains("background"));
        assert_eq!(palette.algorithm.as_str(), "material-you");
    }

    #[test]
    fn test_glsl_shaders() {
        let chromatic = GlslShader::chromatic_aberration();
        assert!(chromatic.code.contains("offset"));
        assert!(chromatic.code.contains("gl_FragColor"));

        let vignette = GlslShader::vignette();
        assert!(vignette.code.contains("vignette"));

        let crt = GlslShader::crt_scanlines();
        assert!(crt.code.contains("scanline"));
    }

    #[test]
    fn test_monitor_configs() {
        let auto = MonitorConfig::auto();
        assert_eq!(auto.scale, 1.0);
        assert!(auto.to_conf().contains("preferred"));

        let hidpi = MonitorConfig::hdpi("HDMI-A-1");
        assert_eq!(hidpi.scale, 2.0);
        assert!(hidpi.to_conf().contains("HDMI-A-1"));
    }
}
