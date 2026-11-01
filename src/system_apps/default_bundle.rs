// SPDX-License-Identifier: MIT
// SigmaOS V1 Default Application Bundle
// (`src/system_apps/default_bundle.rs`)
//
// Defines the curated set of 24 first-party applications shipped with every
// SigmaOS installation.  The bundle covers all major day-to-day categories and
// provides the MIME-type association table used to populate
// `~/.config/mimeapps.list` on first boot.
//
// Design notes:
//  - All data lives in `&'static` slices so there is no heap allocation at
//    bundle-query time.
//  - `is_essential` marks apps that the OS requires at runtime; the package
//    manager refuses to remove them unless a replacement is registered.
//  - `BundleManager` is a zero-size struct whose methods are all `&self` so it
//    can be used as a singleton without synchronisation overhead.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use thiserror::Error;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors that can occur while managing the default application bundle.
#[derive(Debug, Error)]
pub enum BundleError {
    /// The underlying package manager returned a non-zero exit code.
    #[error("Package installation failed for '{package}': {reason}")]
    InstallFailed { package: &'static str, reason: String },

    /// An essential app was requested to be removed with no replacement.
    #[error("Cannot remove essential app '{app}' without a registered replacement")]
    EssentialRemovalDenied { app: &'static str },

    /// A MIME type was not found in the default association table.
    #[error("No MIME association registered for type '{mime}'")]
    MimeNotFound { mime: String },

    /// The mimeapps.list could not be written to disk.
    #[error("Failed to write mimeapps.list: {0}")]
    IoError(String),
}

// ─────────────────────────────────────────────────────────────────────────────
// AppCategory
// ─────────────────────────────────────────────────────────────────────────────

/// High-level category for a default application.
///
/// Categories are intentionally broad so that a single app can appear in
/// multiple sections of the Settings → Default Applications panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppCategory {
    Browser,
    FileManager,
    Terminal,
    TextEditor,
    Office,
    MediaViewer,
    MediaPlayer,
    Productivity,
    SystemTools,
    Security,
    Settings,
    Communication,
}

// ─────────────────────────────────────────────────────────────────────────────
// DefaultApp
// ─────────────────────────────────────────────────────────────────────────────

/// Metadata record for a single application in the default bundle.
///
/// All fields are `&'static` so the entire bundle can live in read-only memory.
#[derive(Debug, Clone, Copy)]
pub struct DefaultApp {
    /// Human-readable display name shown in the app menu.
    pub name: &'static str,

    /// Package name used with the system package manager (e.g. `apt`, `pacman`).
    pub package_name: &'static str,

    /// `.desktop` entry filename (without directory path).
    pub desktop_entry: &'static str,

    /// One or more functional categories this application belongs to.
    pub categories: &'static [AppCategory],

    /// MIME types this application can handle.
    pub mime_types: &'static [&'static str],

    /// When `true` the package manager refuses to uninstall without a
    /// registered replacement.
    pub is_essential: bool,

    /// Package names of known drop-in replacements accepted by the system.
    pub alternatives: &'static [&'static str],
}

// ─────────────────────────────────────────────────────────────────────────────
// V1 Default Bundle — 24 applications
// ─────────────────────────────────────────────────────────────────────────────

/// The complete, curated V1 application bundle shipped with every SigmaOS ISO.
pub static V1_DEFAULT_BUNDLE: &[DefaultApp] = &[
    // 1. Web browser — essential
    DefaultApp {
        name: "Sigma Browser",
        package_name: "sigma-browser",
        desktop_entry: "sigma-browser.desktop",
        categories: &[AppCategory::Browser],
        mime_types: &[
            "text/html",
            "application/xhtml+xml",
            "x-scheme-handler/http",
            "x-scheme-handler/https",
            "x-scheme-handler/ftp",
        ],
        is_essential: true,
        alternatives: &["firefox", "chromium", "epiphany"],
    },
    // 2. File manager — essential
    DefaultApp {
        name: "Sigma Files",
        package_name: "sigma-files",
        desktop_entry: "sigma-files.desktop",
        categories: &[AppCategory::FileManager],
        mime_types: &["inode/directory", "application/x-gnome-saved-search"],
        is_essential: true,
        alternatives: &["nemo", "nautilus", "thunar", "dolphin"],
    },
    // 3. Terminal emulator — essential
    DefaultApp {
        name: "Sigma Terminal",
        package_name: "sigma-term",
        desktop_entry: "sigma-term.desktop",
        categories: &[AppCategory::Terminal],
        mime_types: &["application/x-terminal"],
        is_essential: true,
        alternatives: &["xterm", "konsole", "gnome-terminal", "alacritty"],
    },
    // 4. Text editor — essential
    DefaultApp {
        name: "Sigma Text",
        package_name: "sigma-text",
        desktop_entry: "sigma-text.desktop",
        categories: &[AppCategory::TextEditor],
        mime_types: &[
            "text/plain",
            "text/x-c",
            "text/x-python",
            "text/x-rust",
            "text/x-shellscript",
            "application/json",
            "application/xml",
        ],
        is_essential: true,
        alternatives: &["gedit", "kate", "mousepad", "xed"],
    },
    // 5. Office suite
    DefaultApp {
        name: "LibreOffice Sigma Edition",
        package_name: "libreoffice-sigma",
        desktop_entry: "libreoffice-startcenter.desktop",
        categories: &[AppCategory::Office, AppCategory::Productivity],
        mime_types: &[
            "application/vnd.oasis.opendocument.text",
            "application/vnd.oasis.opendocument.spreadsheet",
            "application/vnd.oasis.opendocument.presentation",
            "application/msword",
            "application/vnd.ms-excel",
            "application/vnd.ms-powerpoint",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ],
        is_essential: false,
        alternatives: &["onlyoffice-desktopeditors", "wps-office"],
    },
    // 6. PDF viewer
    DefaultApp {
        name: "Sigma PDF Viewer",
        package_name: "sigma-pdfview",
        desktop_entry: "sigma-pdfview.desktop",
        categories: &[AppCategory::Productivity],
        mime_types: &["application/pdf", "application/x-pdf"],
        is_essential: false,
        alternatives: &["evince", "okular", "xreader", "mupdf"],
    },
    // 7. Image viewer
    DefaultApp {
        name: "Sigma Pix",
        package_name: "sigma-pix",
        desktop_entry: "sigma-pix.desktop",
        categories: &[AppCategory::MediaViewer],
        mime_types: &[
            "image/jpeg",
            "image/png",
            "image/gif",
            "image/webp",
            "image/bmp",
            "image/tiff",
            "image/svg+xml",
            "image/heic",
            "image/heif",
            "image/avif",
        ],
        is_essential: false,
        alternatives: &["eog", "gwenview", "shotwell", "xviewer"],
    },
    // 8. Music / audio player
    DefaultApp {
        name: "Sigma Music",
        package_name: "sigma-music",
        desktop_entry: "sigma-music.desktop",
        categories: &[AppCategory::MediaPlayer],
        mime_types: &[
            "audio/mpeg",
            "audio/ogg",
            "audio/flac",
            "audio/x-wav",
            "audio/aac",
            "audio/x-m4a",
            "audio/opus",
            "audio/x-matroska",
        ],
        is_essential: false,
        alternatives: &["rhythmbox", "clementine", "lollypop", "cantata"],
    },
    // 9. Video player
    DefaultApp {
        name: "Sigma Player",
        package_name: "sigma-player",
        desktop_entry: "sigma-player.desktop",
        categories: &[AppCategory::MediaPlayer],
        mime_types: &[
            "video/mp4",
            "video/x-matroska",
            "video/webm",
            "video/ogg",
            "video/avi",
            "video/x-msvideo",
            "video/quicktime",
            "video/x-flv",
            "video/3gpp",
        ],
        is_essential: false,
        alternatives: &["vlc", "mpv", "totem", "celluloid"],
    },
    // 10. Screenshot tool
    DefaultApp {
        name: "Sigma Screenshot",
        package_name: "sigma-screenshot",
        desktop_entry: "sigma-screenshot.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: false,
        alternatives: &["flameshot", "gnome-screenshot", "spectacle", "scrot"],
    },
    // 11. Screen / audio recorder
    DefaultApp {
        name: "Sigma Recorder",
        package_name: "sigma-recorder",
        desktop_entry: "sigma-recorder.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: false,
        alternatives: &["obs-studio", "kazam", "simplescreenrecorder"],
    },
    // 12. Notification daemon — essential
    DefaultApp {
        name: "Sigma Notify",
        package_name: "sigma-notify",
        desktop_entry: "sigma-notify.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: true,
        alternatives: &["dunst", "mako", "notify-osd"],
    },
    // 13. Package manager GUI — essential
    DefaultApp {
        name: "Sigma Packages",
        package_name: "sigma-packages",
        desktop_entry: "sigma-packages.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &["application/x-deb", "application/vnd.debian.binary-package"],
        is_essential: true,
        alternatives: &["synaptic", "pamac", "discover"],
    },
    // 14. System update utility — essential
    DefaultApp {
        name: "Sigma Update",
        package_name: "sigma-update",
        desktop_entry: "sigma-update.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: true,
        alternatives: &["mintupdate", "gnome-software"],
    },
    // 15. Backup & restore
    DefaultApp {
        name: "Sigma Backup",
        package_name: "sigma-backup",
        desktop_entry: "sigma-backup.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: false,
        alternatives: &["timeshift", "deja-dup", "backintime"],
    },
    // 16. Bluetooth manager
    DefaultApp {
        name: "Sigma Bluetooth",
        package_name: "sigma-bluetooth",
        desktop_entry: "sigma-bluetooth.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: false,
        alternatives: &["blueman", "blueberry", "gnome-bluetooth"],
    },
    // 17. Wi-Fi / Network manager — essential
    DefaultApp {
        name: "Sigma Wi-Fi",
        package_name: "sigma-wifi",
        desktop_entry: "sigma-wifi.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: true,
        alternatives: &["nm-applet", "wifi-radar", "connman-gtk"],
    },
    // 18. Audio settings
    DefaultApp {
        name: "Sigma Audio",
        package_name: "sigma-audio",
        desktop_entry: "sigma-audio.desktop",
        categories: &[AppCategory::Settings],
        mime_types: &[],
        is_essential: false,
        alternatives: &["pavucontrol", "paprefs", "alsamixer"],
    },
    // 19. Print & scan manager
    DefaultApp {
        name: "Sigma Print",
        package_name: "sigma-print",
        desktop_entry: "sigma-print.desktop",
        categories: &[AppCategory::SystemTools],
        mime_types: &[],
        is_essential: false,
        alternatives: &["system-config-printer", "cups-pk-helper"],
    },
    // 20. Power management settings
    DefaultApp {
        name: "Sigma Power",
        package_name: "sigma-power",
        desktop_entry: "sigma-power.desktop",
        categories: &[AppCategory::Settings],
        mime_types: &[],
        is_essential: false,
        alternatives: &["xfce4-power-manager", "gnome-power-manager", "tlpui"],
    },
    // 21. Theme & appearance settings
    DefaultApp {
        name: "Sigma Themes",
        package_name: "sigma-themes",
        desktop_entry: "sigma-themes.desktop",
        categories: &[AppCategory::Settings],
        mime_types: &[],
        is_essential: false,
        alternatives: &["gnome-tweaks", "lxappearance", "kvantum-manager"],
    },
    // 22. Security centre — essential
    DefaultApp {
        name: "Sigma Security",
        package_name: "sigma-security",
        desktop_entry: "sigma-security.desktop",
        categories: &[AppCategory::Security],
        mime_types: &[],
        is_essential: true,
        alternatives: &["gufw", "firejail-ui"],
    },
    // 23. System settings hub — essential
    DefaultApp {
        name: "Sigma Settings",
        package_name: "sigma-settings",
        desktop_entry: "sigma-settings.desktop",
        categories: &[AppCategory::Settings],
        mime_types: &[],
        is_essential: true,
        alternatives: &["cinnamon-settings", "gnome-control-center", "system-config"],
    },
    // 24. Calculator
    DefaultApp {
        name: "Sigma Calc",
        package_name: "sigma-calc",
        desktop_entry: "sigma-calc.desktop",
        categories: &[AppCategory::Productivity],
        mime_types: &[],
        is_essential: false,
        alternatives: &["gnome-calculator", "galculator", "qalculate-gtk"],
    },
];

// ─────────────────────────────────────────────────────────────────────────────
// MimeAssociation
// ─────────────────────────────────────────────────────────────────────────────

/// Associates a MIME type with a `.desktop` handler.
#[derive(Debug, Clone, Copy)]
pub struct MimeAssociation {
    /// The MIME type string (e.g. `"text/html"`).
    pub mime_type: &'static str,

    /// The `.desktop` file that handles this MIME type.
    pub app_desktop_entry: &'static str,

    /// When `true` this entry is written to the `[Default Applications]`
    /// section; when `false` it goes in `[Added Associations]`.
    pub is_default: bool,
}

/// Comprehensive MIME association table covering the default bundle.
pub static DEFAULT_MIME_ASSOCIATIONS: &[MimeAssociation] = &[
    // ── Web / markup ────────────────────────────────────────────────────────
    MimeAssociation { mime_type: "text/html",                  app_desktop_entry: "sigma-browser.desktop",  is_default: true  },
    MimeAssociation { mime_type: "application/xhtml+xml",      app_desktop_entry: "sigma-browser.desktop",  is_default: true  },
    MimeAssociation { mime_type: "x-scheme-handler/http",      app_desktop_entry: "sigma-browser.desktop",  is_default: true  },
    MimeAssociation { mime_type: "x-scheme-handler/https",     app_desktop_entry: "sigma-browser.desktop",  is_default: true  },
    MimeAssociation { mime_type: "x-scheme-handler/ftp",       app_desktop_entry: "sigma-browser.desktop",  is_default: false },

    // ── Plain text & code ────────────────────────────────────────────────────
    MimeAssociation { mime_type: "text/plain",                 app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "text/x-c",                   app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "text/x-python",              app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "text/x-rust",                app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "text/x-shellscript",         app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "application/json",           app_desktop_entry: "sigma-text.desktop",     is_default: true  },
    MimeAssociation { mime_type: "application/xml",            app_desktop_entry: "sigma-text.desktop",     is_default: true  },

    // ── PDF & documents ─────────────────────────────────────────────────────
    MimeAssociation { mime_type: "application/pdf",            app_desktop_entry: "sigma-pdfview.desktop",  is_default: true  },
    MimeAssociation { mime_type: "application/x-pdf",          app_desktop_entry: "sigma-pdfview.desktop",  is_default: true  },

    // ── Office documents ─────────────────────────────────────────────────────
    MimeAssociation { mime_type: "application/vnd.oasis.opendocument.text",          app_desktop_entry: "libreoffice-writer.desktop",  is_default: true  },
    MimeAssociation { mime_type: "application/vnd.oasis.opendocument.spreadsheet",   app_desktop_entry: "libreoffice-calc.desktop",    is_default: true  },
    MimeAssociation { mime_type: "application/vnd.oasis.opendocument.presentation",  app_desktop_entry: "libreoffice-impress.desktop", is_default: true  },
    MimeAssociation { mime_type: "application/msword",         app_desktop_entry: "libreoffice-writer.desktop",  is_default: true  },
    MimeAssociation { mime_type: "application/vnd.ms-excel",   app_desktop_entry: "libreoffice-calc.desktop",    is_default: true  },
    MimeAssociation { mime_type: "application/vnd.ms-powerpoint", app_desktop_entry: "libreoffice-impress.desktop", is_default: true },
    MimeAssociation { mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",      app_desktop_entry: "libreoffice-writer.desktop",  is_default: true },
    MimeAssociation { mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",            app_desktop_entry: "libreoffice-calc.desktop",    is_default: true },
    MimeAssociation { mime_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation",   app_desktop_entry: "libreoffice-impress.desktop", is_default: true },

    // ── Images ───────────────────────────────────────────────────────────────
    MimeAssociation { mime_type: "image/jpeg",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/png",                  app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/gif",                  app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/webp",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/bmp",                  app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/tiff",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/svg+xml",              app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/heic",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/heif",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },
    MimeAssociation { mime_type: "image/avif",                 app_desktop_entry: "sigma-pix.desktop",      is_default: true  },

    // ── Audio ────────────────────────────────────────────────────────────────
    MimeAssociation { mime_type: "audio/mpeg",                 app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/ogg",                  app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/flac",                 app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/x-wav",                app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/aac",                  app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/x-m4a",                app_desktop_entry: "sigma-music.desktop",    is_default: true  },
    MimeAssociation { mime_type: "audio/opus",                 app_desktop_entry: "sigma-music.desktop",    is_default: true  },

    // ── Video ────────────────────────────────────────────────────────────────
    MimeAssociation { mime_type: "video/mp4",                  app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/x-matroska",           app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/webm",                 app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/ogg",                  app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/avi",                  app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/x-msvideo",            app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/quicktime",            app_desktop_entry: "sigma-player.desktop",   is_default: true  },
    MimeAssociation { mime_type: "video/x-flv",                app_desktop_entry: "sigma-player.desktop",   is_default: true  },

    // ── Packages (opened by the package manager GUI) ─────────────────────────
    MimeAssociation { mime_type: "application/x-deb",          app_desktop_entry: "sigma-packages.desktop", is_default: true  },
    MimeAssociation { mime_type: "application/vnd.debian.binary-package", app_desktop_entry: "sigma-packages.desktop", is_default: true },

    // ── Directories ──────────────────────────────────────────────────────────
    MimeAssociation { mime_type: "inode/directory",            app_desktop_entry: "sigma-files.desktop",    is_default: true  },
];

// ─────────────────────────────────────────────────────────────────────────────
// BundleManager
// ─────────────────────────────────────────────────────────────────────────────

/// Stateless manager for the V1 default application bundle.
///
/// All methods are `&self` and operate over the compile-time-constant
/// `V1_DEFAULT_BUNDLE` and `DEFAULT_MIME_ASSOCIATIONS` slices.
pub struct BundleManager;

impl BundleManager {
    /// Returns the complete V1 default application bundle.
    #[inline]
    pub fn v1_apps(&self) -> &'static [DefaultApp] {
        V1_DEFAULT_BUNDLE
    }

    /// Iterates over only the essential applications that must always be
    /// present on the system.
    pub fn essential_apps(&self) -> impl Iterator<Item = &'static DefaultApp> {
        V1_DEFAULT_BUNDLE.iter().filter(|app| app.is_essential)
    }

    /// Returns all applications that belong to the given category.
    pub fn apps_for_category(
        &self,
        cat: AppCategory,
    ) -> impl Iterator<Item = &'static DefaultApp> {
        V1_DEFAULT_BUNDLE
            .iter()
            .filter(move |app| app.categories.contains(&cat))
    }

    /// Looks up the `MimeAssociation` for a given MIME type string.
    ///
    /// Returns `None` if the MIME type is not in the default table.
    pub fn mime_handler(&self, mime: &str) -> Option<&'static MimeAssociation> {
        DEFAULT_MIME_ASSOCIATIONS
            .iter()
            .find(|a| a.mime_type == mime)
    }

    /// Triggers package installation for every app in the V1 bundle.
    ///
    /// In production this delegates to the `sigma-packages` back-end.  In the
    /// current implementation the function validates that every listed package
    /// name is non-empty and returns `Ok(())`.  A real back-end hook should be
    /// injected via a trait object before shipping.
    pub fn install_bundle(&self) -> Result<(), BundleError> {
        for app in V1_DEFAULT_BUNDLE {
            if app.package_name.is_empty() {
                return Err(BundleError::InstallFailed {
                    package: app.package_name,
                    reason: "package name must not be empty".into(),
                });
            }
            // TODO(sigma-packages): invoke `sigma-packages install <package_name>`
            // and map non-zero exit codes to BundleError::InstallFailed.
        }
        Ok(())
    }

    /// Generates the content of `~/.config/mimeapps.list` from the default
    /// MIME association table.
    ///
    /// The XDG specification requires two sections:
    ///  - `[Default Applications]` — one entry per MIME type, the preferred app.
    ///  - `[Added Associations]`   — additional capable handlers per type.
    pub fn generate_mimeapps_list(&self) -> String {
        let mut defaults = Vec::new();
        let mut added = Vec::new();

        for assoc in DEFAULT_MIME_ASSOCIATIONS {
            if assoc.is_default {
                defaults.push(format!("{}={}", assoc.mime_type, assoc.app_desktop_entry));
            } else {
                added.push(format!("{}={}", assoc.mime_type, assoc.app_desktop_entry));
            }
        }

        let mut out = String::new();
        out.push_str("[Default Applications]\n");
        for line in &defaults {
            out.push_str(line);
            out.push('\n');
        }
        if !added.is_empty() {
            out.push('\n');
            out.push_str("[Added Associations]\n");
            for line in &added {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_24_apps_present() {
        assert_eq!(
            V1_DEFAULT_BUNDLE.len(),
            24,
            "V1_DEFAULT_BUNDLE must contain exactly 24 applications"
        );
    }

    #[test]
    fn essential_apps_count() {
        let essential: Vec<_> = BundleManager.essential_apps().collect();
        // Browser, Files, Term, Text, Notify, Packages, Update, Wifi, Security, Settings = 10
        assert_eq!(
            essential.len(),
            10,
            "Expected 10 essential apps, got {}",
            essential.len()
        );
    }

    #[test]
    fn mime_lookup_text_html() {
        let assoc = BundleManager
            .mime_handler("text/html")
            .expect("text/html must have a registered handler");
        assert_eq!(assoc.app_desktop_entry, "sigma-browser.desktop");
        assert!(assoc.is_default, "text/html must be a default association");
    }

    #[test]
    fn generate_mimeapps_list_non_empty() {
        let content = BundleManager.generate_mimeapps_list();
        assert!(
            !content.is_empty(),
            "generate_mimeapps_list must produce non-empty output"
        );
        assert!(
            content.contains("[Default Applications]"),
            "output must contain [Default Applications] section"
        );
    }

    #[test]
    fn all_apps_have_non_empty_package_name() {
        for app in V1_DEFAULT_BUNDLE {
            assert!(
                !app.package_name.is_empty(),
                "App '{}' must have a non-empty package_name",
                app.name
            );
        }
    }

    #[test]
    fn install_bundle_succeeds() {
        BundleManager
            .install_bundle()
            .expect("install_bundle must succeed for well-formed bundle");
    }

    #[test]
    fn apps_for_browser_category() {
        let browsers: Vec<_> = BundleManager
            .apps_for_category(AppCategory::Browser)
            .collect();
        assert_eq!(browsers.len(), 1, "exactly one browser expected");
        assert_eq!(browsers[0].package_name, "sigma-browser");
    }

    #[test]
    fn mime_lookup_unknown_returns_none() {
        let result = BundleManager.mime_handler("application/x-unknown-sigma-test");
        assert!(result.is_none(), "unknown MIME type must return None");
    }
}
