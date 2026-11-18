// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Wayland Environment Variable & Toolchain Export Engine
//!
//! Inspired by Omarchy's comprehensive environment configuration (`hyprland.conf` envs)
//! and Arch Linux's `/etc/environment` & `/etc/profile.d/` standard exports.
//! Sets up GTK/Qt Wayland backends, hardware video acceleration (VA-API),
//! Clutter, SDL, and default editor/terminal variables for a seamless desktop experience.

use std::collections::BTreeMap;

/// Environment variable definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
    pub description: String,
}

impl EnvironmentVariable {
    pub fn new(key: &str, value: &str, desc: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            description: desc.to_string(),
        }
    }

    /// Render to bash export line
    pub fn to_sh_export(&self) -> String {
        format!("export {}=\"{}\"", self.key, self.value)
    }

    /// Render to Hyprland `env = ` line
    pub fn to_hyprland_env(&self) -> String {
        format!("env = {},{}", self.key, self.value)
    }
}

/// System-wide Wayland environment profile
#[derive(Debug, Default)]
pub struct WaylandEnvProfile {
    variables: BTreeMap<String, EnvironmentVariable>,
}

impl WaylandEnvProfile {
    pub fn omarchy_v35_defaults() -> Self {
        let mut prof = Self { variables: BTreeMap::new() };

        // Toolkit Wayland forcing
        prof.insert("GDK_BACKEND", "wayland,x11,*", "Force GTK to use Wayland backend");
        prof.insert("QT_QPA_PLATFORM", "wayland;xcb", "Force Qt to use Wayland platform");
        prof.insert("QT_QPA_PLATFORMTHEME", "qt5ct", "Ensure Qt applications adhere to system styling");
        prof.insert("QT_AUTO_SCREEN_SCALE_FACTOR", "1", "Enable automatic Qt HiDPI scaling");
        prof.insert("QT_WAYLAND_DISABLE_WINDOWDECORATION", "1", "Disable client-side window decoration in Qt");
        prof.insert("SDL_VIDEODRIVER", "wayland", "Force SDL games and applications onto Wayland");
        prof.insert("CLUTTER_BACKEND", "wayland", "Force Clutter toolkit onto Wayland");

        // XDG Desktop Specs
        prof.insert("XDG_CURRENT_DESKTOP", "Hyprland", "Identify desktop session for portal routing");
        prof.insert("XDG_SESSION_TYPE", "wayland", "Identify session as native Wayland");
        prof.insert("XDG_SESSION_DESKTOP", "Hyprland", "Session desktop manager");

        // Hardware Acceleration
        prof.insert("NVD_BACKEND", "direct", "Direct hardware acceleration for NVIDIA VA-API");
        prof.insert("LIBVA_DRIVER_NAME", "nvidia", "Default VA-API driver name");

        // Default Userspace Tools
        prof.insert("EDITOR", "nvim", "Default system text editor");
        prof.insert("VISUAL", "nvim", "Default visual editor");
        prof.insert("TERMINAL", "kitty", "Default hardware-accelerated terminal");
        prof.insert("BROWSER", "firefox", "Default web browser");

        prof
    }

    pub fn insert(&mut self, key: &str, val: &str, desc: &str) {
        self.variables.insert(key.to_string(), EnvironmentVariable::new(key, val, desc));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|e| e.value.as_str())
    }

    /// Render to shell script (/etc/profile.d/sigmaos-wayland.sh)
    pub fn to_sh_profile(&self) -> String {
        let mut out = String::from("#!/bin/sh\n# SigmaOS Wayland Environment Profile\n\n");
        for var in self.variables.values() {
            out.push_str(&format!("{} # {}\n", var.to_sh_export(), var.description));
        }
        out
    }

    /// Render to hyprland.conf environment lines
    pub fn to_hyprland_conf(&self) -> String {
        let mut out = String::from("# SigmaOS Hyprland Environment Directives\n\n");
        for var in self.variables.values() {
            out.push_str(&format!("{}\n", var.to_hyprland_env()));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_variable_rendering_aaa() {
        let var = EnvironmentVariable::new("GDK_BACKEND", "wayland,x11,*", "GTK Wayland");
        assert_eq!(var.to_sh_export(), "export GDK_BACKEND=\"wayland,x11,*\"");
        assert_eq!(var.to_hyprland_env(), "env = GDK_BACKEND,wayland,x11,*");
    }

    #[test]
    fn test_omarchy_defaults_profile_aaa() {
        let prof = WaylandEnvProfile::omarchy_v35_defaults();
        assert_eq!(prof.get("GDK_BACKEND"), Some("wayland,x11,*"));
        assert_eq!(prof.get("QT_QPA_PLATFORM"), Some("wayland;xcb"));
        assert_eq!(prof.get("XDG_CURRENT_DESKTOP"), Some("Hyprland"));
        assert_eq!(prof.get("EDITOR"), Some("nvim"));
        assert_eq!(prof.get("TERMINAL"), Some("kitty"));
    }

    #[test]
    fn test_sh_profile_rendering_aaa() {
        let prof = WaylandEnvProfile::omarchy_v35_defaults();
        let sh = prof.to_sh_profile();
        assert!(sh.contains("export GDK_BACKEND=\"wayland,x11,*\""));
        assert!(sh.contains("export XDG_SESSION_TYPE=\"wayland\""));
        assert!(sh.contains("export EDITOR=\"nvim\""));
    }

    #[test]
    fn test_hyprland_conf_rendering_aaa() {
        let prof = WaylandEnvProfile::omarchy_v35_defaults();
        let conf = prof.to_hyprland_conf();
        assert!(conf.contains("env = GDK_BACKEND,wayland,x11,*"));
        assert!(conf.contains("env = XDG_CURRENT_DESKTOP,Hyprland"));
    }
}
