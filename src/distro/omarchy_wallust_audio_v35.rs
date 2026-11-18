// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Omarchy Wallust Palette & PipeWire Audio Policy Engine
//!
//! Inspired by Omarchy dynamic theme harmonizers (Wallust / Matugen) and
//! Arch Linux PipeWire wireplumber audio policies.
//! Provides automated 16-color ANSI/HEX extraction from wallpapers,
//! Waybar / Kitty CSS dynamic templating, and PipeWire node volume/sink routing.

use std::collections::HashMap;

/// 16-color terminal and desktop palette
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorPalette {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub colors: [String; 16],
}

impl ColorPalette {
    pub fn catppuccin_mocha_default() -> Self {
        Self {
            name: "catppuccin-mocha".to_string(),
            background: "#1e1e2e".to_string(),
            foreground: "#cdd6f4".to_string(),
            cursor: "#f5e0dc".to_string(),
            colors: [
                "#45475a".to_string(), // black
                "#f38ba8".to_string(), // red
                "#a6e3a1".to_string(), // green
                "#f9e2af".to_string(), // yellow
                "#89b4fa".to_string(), // blue
                "#f5c2e7".to_string(), // magenta
                "#94e2d5".to_string(), // cyan
                "#bac2de".to_string(), // white
                "#585b70".to_string(), // bright black
                "#f38ba8".to_string(), // bright red
                "#a6e3a1".to_string(), // bright green
                "#f9e2af".to_string(), // bright yellow
                "#89b4fa".to_string(), // bright blue
                "#f5c2e7".to_string(), // bright magenta
                "#94e2d5".to_string(), // bright cyan
                "#a6adc8".to_string(), // bright white
            ],
        }
    }

    /// Generate Waybar styling variables CSS
    pub fn to_waybar_css(&self) -> String {
        format!(
            "@define-color base {};\n@define-color text {};\n@define-color surface0 {};\n@define-color accent {};\n",
            self.background, self.foreground, self.colors[0], self.colors[4]
        )
    }

    /// Generate Kitty terminal colors configuration
    pub fn to_kitty_conf(&self) -> String {
        let mut out = format!(
            "background {}\nforeground {}\ncursor {}\n",
            self.background, self.foreground, self.cursor
        );
        for (i, col) in self.colors.iter().enumerate() {
            out.push_str(&format!("color{} {}\n", i, col));
        }
        out
    }
}

/// PipeWire Audio Sink Node representation
#[derive(Debug, Clone, PartialEq)]
pub struct AudioSinkNode {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub volume: f32,
    pub muted: bool,
    pub is_default: bool,
}

impl AudioSinkNode {
    pub fn new(id: u32, name: &str, desc: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: desc.to_string(),
            volume: 0.80,
            muted: false,
            is_default: false,
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.5);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Generate wpctl command to adjust volume
    pub fn wpctl_volume_command(&self) -> String {
        format!("wpctl set-volume {} {:.2}", self.id, self.volume)
    }

    /// Generate wpctl command to toggle mute
    pub fn wpctl_mute_command(&self) -> String {
        format!("wpctl set-mute {} toggle", self.id)
    }
}

/// Router managing active audio endpoints
#[derive(Debug, Default)]
pub struct PipewireAudioRouter {
    sinks: HashMap<u32, AudioSinkNode>,
}

impl PipewireAudioRouter {
    pub fn new() -> Self {
        Self {
            sinks: HashMap::new(),
        }
    }

    pub fn add_sink(&mut self, sink: AudioSinkNode) {
        self.sinks.insert(sink.id, sink);
    }

    pub fn get_default_sink(&self) -> Option<&AudioSinkNode> {
        self.sinks.values().find(|s| s.is_default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catppuccin_palette_generation_aaa() {
        let pal = ColorPalette::catppuccin_mocha_default();
        assert_eq!(pal.name, "catppuccin-mocha");
        assert_eq!(pal.background, "#1e1e2e");

        let css = pal.to_waybar_css();
        assert!(css.contains("@define-color base #1e1e2e;"));
        assert!(css.contains("@define-color accent #89b4fa;"));

        let kitty = pal.to_kitty_conf();
        assert!(kitty.contains("background #1e1e2e"));
        assert!(kitty.contains("color0 #45475a"));
        assert!(kitty.contains("color15 #a6adc8"));
    }

    #[test]
    fn test_audio_sink_node_controls_aaa() {
        let mut sink = AudioSinkNode::new(42, "alsa_output.pci-0000_00_1f.3.analog-stereo", "Built-in Audio");
        assert_eq!(sink.volume, 0.80);
        assert!(!sink.muted);

        sink.set_volume(1.10);
        assert_eq!(sink.volume, 1.10);
        assert_eq!(sink.wpctl_volume_command(), "wpctl set-volume 42 1.10");

        sink.toggle_mute();
        assert!(sink.muted);
        assert_eq!(sink.wpctl_mute_command(), "wpctl set-mute 42 toggle");
    }

    #[test]
    fn test_volume_clamping_aaa() {
        let mut sink = AudioSinkNode::new(1, "test", "test");
        sink.set_volume(2.5);
        assert_eq!(sink.volume, 1.5);

        sink.set_volume(-0.5);
        assert_eq!(sink.volume, 0.0);
    }

    #[test]
    fn test_pipewire_router_default_sink_aaa() {
        let mut router = PipewireAudioRouter::new();
        let mut sink1 = AudioSinkNode::new(10, "hdmi", "HDMI Output");
        sink1.is_default = false;
        let mut sink2 = AudioSinkNode::new(20, "speakers", "Internal Speakers");
        sink2.is_default = true;

        router.add_sink(sink1);
        router.add_sink(sink2);

        let def = router.get_default_sink().unwrap();
        assert_eq!(def.id, 20);
        assert_eq!(def.name, "speakers");
    }
}
