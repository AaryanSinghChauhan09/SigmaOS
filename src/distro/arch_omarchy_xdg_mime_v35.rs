// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS XDG MIME Type Associations & Default Application Router
//!
//! Inspired by Arch Linux's xdg-utils / mimeapps.list specifications and
//! Omarchy's automated file opener associations (yazi / nvim / mpv / imv).
//! Maps MIME types to default handler desktop applications, evaluates
//! fallback handler chains, and serializes mimeapps.list.

use std::collections::HashMap;

/// An association entry mapping a MIME type to desktop application handlers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MimeAssociation {
    pub mime_type: String,
    pub default_app: String,
    pub fallback_apps: Vec<String>,
}

impl MimeAssociation {
    pub fn new(mime: &str, default: &str) -> Self {
        Self {
            mime_type: mime.to_string(),
            default_app: default.to_string(),
            fallback_apps: Vec::new(),
        }
    }

    pub fn with_fallback(mut self, fallback: &str) -> Self {
        self.fallback_apps.push(fallback.to_string());
        self
    }
}

/// MIME database and resolver manager
#[derive(Debug, Default)]
pub struct MimeRegistry {
    associations: HashMap<String, MimeAssociation>,
}

impl MimeRegistry {
    pub fn omarchy_defaults() -> Self {
        let mut reg = Self { associations: HashMap::new() };

        // Web & URLs
        reg.register(MimeAssociation::new("text/html", "firefox.desktop").with_fallback("chromium.desktop"));
        reg.register(MimeAssociation::new("x-scheme-handler/http", "firefox.desktop"));
        reg.register(MimeAssociation::new("x-scheme-handler/https", "firefox.desktop"));

        // Text & Code
        reg.register(MimeAssociation::new("text/plain", "nvim.desktop").with_fallback("kitty.desktop"));
        reg.register(MimeAssociation::new("text/x-rust", "nvim.desktop"));
        reg.register(MimeAssociation::new("application/json", "nvim.desktop"));

        // Media
        reg.register(MimeAssociation::new("image/png", "imv.desktop"));
        reg.register(MimeAssociation::new("image/jpeg", "imv.desktop"));
        reg.register(MimeAssociation::new("video/mp4", "mpv.desktop"));
        reg.register(MimeAssociation::new("video/x-matroska", "mpv.desktop"));
        reg.register(MimeAssociation::new("audio/flac", "mpv.desktop"));

        // Directories & Archives
        reg.register(MimeAssociation::new("inode/directory", "yazi.desktop"));
        reg.register(MimeAssociation::new("application/zip", "yazi.desktop"));

        reg
    }

    pub fn register(&mut self, assoc: MimeAssociation) {
        self.associations.insert(assoc.mime_type.clone(), assoc);
    }

    pub fn get_handler(&self, mime: &str) -> Option<&str> {
        self.associations.get(mime).map(|a| a.default_app.as_str())
    }

    /// Render to standard mimeapps.list format
    pub fn to_mimeapps_list(&self) -> String {
        let mut out = String::from("[Default Applications]\n");
        let mut sorted_keys: Vec<_> = self.associations.keys().collect();
        sorted_keys.sort();

        for k in sorted_keys {
            if let Some(a) = self.associations.get(k) {
                out.push_str(&format!("{}={}\n", a.mime_type, a.default_app));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_handler_lookup_aaa() {
        let reg = MimeRegistry::omarchy_defaults();
        assert_eq!(reg.get_handler("text/html"), Some("firefox.desktop"));
        assert_eq!(reg.get_handler("text/plain"), Some("nvim.desktop"));
        assert_eq!(reg.get_handler("video/mp4"), Some("mpv.desktop"));
        assert_eq!(reg.get_handler("inode/directory"), Some("yazi.desktop"));
    }

    #[test]
    fn test_mimeapps_list_serialization_aaa() {
        let reg = MimeRegistry::omarchy_defaults();
        let rendered = reg.to_mimeapps_list();

        assert!(rendered.contains("[Default Applications]"));
        assert!(rendered.contains("text/html=firefox.desktop"));
        assert!(rendered.contains("video/mp4=mpv.desktop"));
        assert!(rendered.contains("inode/directory=yazi.desktop"));
    }
}
