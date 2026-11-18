// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS GTK/Qt Theme & Font Management Engine
//!
//! Inspired by Omarchy's seamless desktop theming and Arch Linux's fontconfig & GTK/Qt standards.
//! Manages GTK3/GTK4 settings.ini generation, Qt stylesheet/platform theme overrides,
//! fontconfig XML rule definitions, and cursor theme configuration.


/// Cursor theme configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorConfig {
    pub theme_name: String,
    pub size: u32,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            theme_name: "Bibata-Modern-Classic".to_string(),
            size: 24,
        }
    }
}

/// Font typography profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontConfig {
    pub monospace: String,
    pub sans_serif: String,
    pub serif: String,
    pub size_pt: u32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            monospace: "JetBrainsMono Nerd Font".to_string(),
            sans_serif: "Inter".to_string(),
            serif: "Noto Serif".to_string(),
            size_pt: 11,
        }
    }
}

/// Theme manager settings for GTK, Qt, and Icons
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeProfile {
    pub gtk_theme: String,
    pub icon_theme: String,
    pub cursor: CursorConfig,
    pub fonts: FontConfig,
    pub dark_preference: bool,
}

impl ThemeProfile {
    /// SigmaOS default theme matching Omarchy & Catppuccin Mocha aesthetic
    pub fn sigma_mocha() -> Self {
        Self {
            gtk_theme: "Catppuccin-Mocha-Standard-Mauve-Dark".to_string(),
            icon_theme: "Papirus-Dark".to_string(),
            cursor: CursorConfig::default(),
            fonts: FontConfig::default(),
            dark_preference: true,
        }
    }

    /// Render GTK 3 / 4 settings.ini file
    pub fn to_gtk_settings_ini(&self) -> String {
        let dark_val = if self.dark_preference { "1" } else { "0" };
        format!(
            "[Settings]\n\
            gtk-theme-name={}\n\
            gtk-icon-theme-name={}\n\
            gtk-font-name={} {}\n\
            gtk-cursor-theme-name={}\n\
            gtk-cursor-theme-size={}\n\
            gtk-application-prefer-dark-theme={}\n",
            self.gtk_theme,
            self.icon_theme,
            self.fonts.sans_serif,
            self.fonts.size_pt,
            self.cursor.theme_name,
            self.cursor.size,
            dark_val
        )
    }

    /// Render Xresources / Xcursor defaults
    pub fn to_xresources(&self) -> String {
        format!(
            "Xcursor.theme: {}\n\
            Xcursor.size: {}\n\
            Xft.antialias: 1\n\
            Xft.hinting: 1\n\
            Xft.hintstyle: hintslight\n\
            Xft.rgba: rgb\n",
            self.cursor.theme_name, self.cursor.size
        )
    }

    /// Render fontconfig 50-sigmaos-fonts.conf XML snippet
    pub fn to_fontconfig_xml(&self) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n\
            <!DOCTYPE fontconfig SYSTEM \"urn:fontconfig:fonts.dtd\">\n\
            <fontconfig>\n  \
              <alias>\n    \
                <family>monospace</family>\n    \
                <prefer><family>{}</family></prefer>\n  \
              </alias>\n  \
              <alias>\n    \
                <family>sans-serif</family>\n    \
                <prefer><family>{}</family></prefer>\n  \
              </alias>\n\
            </fontconfig>\n",
            self.fonts.monospace, self.fonts.sans_serif
        )
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtk_settings_ini_generation_aaa() {
        // Arrange
        let theme = ThemeProfile::sigma_mocha();
        // Act
        let ini = theme.to_gtk_settings_ini();
        // Assert
        assert!(ini.contains("[Settings]"));
        assert!(ini.contains("gtk-theme-name=Catppuccin-Mocha-Standard-Mauve-Dark"));
        assert!(ini.contains("gtk-icon-theme-name=Papirus-Dark"));
        assert!(ini.contains("gtk-application-prefer-dark-theme=1"));
        assert!(ini.contains("gtk-cursor-theme-name=Bibata-Modern-Classic"));
    }

    #[test]
    fn test_xresources_generation_aaa() {
        // Arrange
        let theme = ThemeProfile::sigma_mocha();
        // Act
        let xres = theme.to_xresources();
        // Assert
        assert!(xres.contains("Xcursor.theme: Bibata-Modern-Classic"));
        assert!(xres.contains("Xcursor.size: 24"));
        assert!(xres.contains("Xft.antialias: 1"));
    }

    #[test]
    fn test_fontconfig_xml_generation_aaa() {
        // Arrange
        let theme = ThemeProfile::sigma_mocha();
        // Act
        let xml = theme.to_fontconfig_xml();
        // Assert
        assert!(xml.contains("<family>JetBrainsMono Nerd Font</family>"));
        assert!(xml.contains("<family>Inter</family>"));
    }
}
