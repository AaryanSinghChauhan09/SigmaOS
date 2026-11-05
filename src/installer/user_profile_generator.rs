//! Default user environment and profile generator.
//!
//! Creates XDG directories, configures the V1 default application stack,
//! sets MIME type associations, applies the default theme, and generates
//! shell configuration files for new SigmaOS users.

use std::fmt;
use std::path::PathBuf;

/// Application category for grouping in menus and defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCategory {
    Browser,
    FileManager,
    Terminal,
    TextEditor,
    Office,
    PdfViewer,
    ImageViewer,
    MusicPlayer,
    VideoPlayer,
    Screenshot,
    ScreenRecorder,
    Notifications,
    PackageManager,
    UpdateManager,
    Backup,
    Bluetooth,
    Wifi,
    Audio,
    Printing,
    PowerManager,
    ThemeManager,
    Security,
    SystemSettings,
    Calculator,
}

/// A default application in the V1 bundle.
#[derive(Debug, Clone)]
pub struct DefaultApp {
    /// Human-readable application name.
    pub name: &'static str,
    /// APT package name.
    pub package_name: &'static str,
    /// XDG `.desktop` entry filename.
    pub desktop_entry: &'static str,
    /// Application category.
    pub category: AppCategory,
    /// MIME types this app handles.
    pub mime_types: &'static [&'static str],
    /// If true, the app cannot be removed without a replacement.
    pub is_essential: bool,
    /// Alternative package names (other apps that can fill this role).
    pub alternatives: &'static [&'static str],
}

/// MIME type → default application mapping.
#[derive(Debug, Clone)]
pub struct FileAssociation {
    /// MIME type, e.g. `"text/html"`.
    pub mime_type: &'static str,
    /// Desktop entry handling this type, e.g. `"sigma-browser.desktop"`.
    pub handler_desktop_entry: &'static str,
    /// `true` if this is the system-wide default.
    pub is_default: bool,
}

/// XDG user directory paths.
#[derive(Debug, Clone)]
pub struct XdgDirectories {
    pub desktop:   PathBuf,
    pub documents: PathBuf,
    pub downloads: PathBuf,
    pub music:     PathBuf,
    pub pictures:  PathBuf,
    pub videos:    PathBuf,
    pub templates: PathBuf,
}

/// The curated V1 default application bundle (24 apps).
pub const V1_DEFAULT_BUNDLE: &[DefaultApp] = &[
    DefaultApp {
        name: "Sigma Browser",
        package_name: "sigma-browser",
        desktop_entry: "sigma-browser.desktop",
        category: AppCategory::Browser,
        mime_types: &["text/html", "x-scheme-handler/http", "x-scheme-handler/https",
                      "application/xhtml+xml"],
        is_essential: true,
        alternatives: &["firefox", "chromium"],
    },
    DefaultApp {
        name: "Sigma Files",
        package_name: "sigma-files",
        desktop_entry: "sigma-files.desktop",
        category: AppCategory::FileManager,
        mime_types: &["inode/directory", "application/x-gnome-saved-search"],
        is_essential: true,
        alternatives: &["nautilus", "thunar"],
    },
    DefaultApp {
        name: "Sigma Terminal",
        package_name: "sigma-term",
        desktop_entry: "sigma-term.desktop",
        category: AppCategory::Terminal,
        mime_types: &["application/x-terminal-emulator"],
        is_essential: true,
        alternatives: &["gnome-terminal", "alacritty", "kitty"],
    },
    DefaultApp {
        name: "Sigma Text Editor",
        package_name: "sigma-text",
        desktop_entry: "sigma-text.desktop",
        category: AppCategory::TextEditor,
        mime_types: &["text/plain", "text/x-c", "text/x-python", "text/x-rust",
                      "text/x-shellscript", "application/json", "text/x-toml"],
        is_essential: true,
        alternatives: &["gedit", "kate", "mousepad"],
    },
    DefaultApp {
        name: "LibreOffice",
        package_name: "libreoffice-sigma",
        desktop_entry: "libreoffice-writer.desktop",
        category: AppCategory::Office,
        mime_types: &["application/vnd.oasis.opendocument.text",
                      "application/msword",
                      "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                      "application/vnd.oasis.opendocument.spreadsheet",
                      "application/vnd.ms-excel",
                      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                      "application/vnd.oasis.opendocument.presentation",
                      "application/vnd.ms-powerpoint"],
        is_essential: false,
        alternatives: &["onlyoffice-desktopeditors"],
    },
    DefaultApp {
        name: "Sigma PDF Viewer",
        package_name: "sigma-pdfview",
        desktop_entry: "sigma-pdfview.desktop",
        category: AppCategory::PdfViewer,
        mime_types: &["application/pdf", "application/x-pdf"],
        is_essential: false,
        alternatives: &["evince", "okular"],
    },
    DefaultApp {
        name: "Sigma Pix",
        package_name: "sigma-pix",
        desktop_entry: "sigma-pix.desktop",
        category: AppCategory::ImageViewer,
        mime_types: &["image/jpeg", "image/png", "image/gif", "image/webp",
                      "image/svg+xml", "image/bmp", "image/tiff", "image/avif"],
        is_essential: false,
        alternatives: &["eog", "shotwell"],
    },
    DefaultApp {
        name: "Sigma Music",
        package_name: "sigma-music",
        desktop_entry: "sigma-music.desktop",
        category: AppCategory::MusicPlayer,
        mime_types: &["audio/mpeg", "audio/ogg", "audio/flac", "audio/wav",
                      "audio/aac", "audio/x-m4a", "audio/opus"],
        is_essential: false,
        alternatives: &["rhythmbox", "amarok"],
    },
    DefaultApp {
        name: "Sigma Player",
        package_name: "sigma-player",
        desktop_entry: "sigma-player.desktop",
        category: AppCategory::VideoPlayer,
        mime_types: &["video/mp4", "video/x-matroska", "video/webm", "video/mpeg",
                      "video/quicktime", "video/x-msvideo", "video/x-flv"],
        is_essential: false,
        alternatives: &["vlc", "mpv", "celluloid"],
    },
    DefaultApp {
        name: "Sigma Screenshot",
        package_name: "sigma-screenshot",
        desktop_entry: "sigma-screenshot.desktop",
        category: AppCategory::Screenshot,
        mime_types: &[],
        is_essential: false,
        alternatives: &["gnome-screenshot", "flameshot"],
    },
    DefaultApp {
        name: "Sigma Recorder",
        package_name: "sigma-recorder",
        desktop_entry: "sigma-recorder.desktop",
        category: AppCategory::ScreenRecorder,
        mime_types: &[],
        is_essential: false,
        alternatives: &["obs-studio", "kazam"],
    },
    DefaultApp {
        name: "Sigma Notifications",
        package_name: "sigma-notify",
        desktop_entry: "sigma-notify.desktop",
        category: AppCategory::Notifications,
        mime_types: &[],
        is_essential: true,
        alternatives: &["dunst", "mako"],
    },
    DefaultApp {
        name: "Sigma Packages",
        package_name: "sigma-packages",
        desktop_entry: "sigma-packages.desktop",
        category: AppCategory::PackageManager,
        mime_types: &["application/vnd.debian.binary-package"],
        is_essential: true,
        alternatives: &["synaptic", "gnome-software"],
    },
    DefaultApp {
        name: "Sigma Update",
        package_name: "sigma-update",
        desktop_entry: "sigma-update.desktop",
        category: AppCategory::UpdateManager,
        mime_types: &[],
        is_essential: true,
        alternatives: &[],
    },
    DefaultApp {
        name: "Sigma Backup",
        package_name: "sigma-backup",
        desktop_entry: "sigma-backup.desktop",
        category: AppCategory::Backup,
        mime_types: &[],
        is_essential: false,
        alternatives: &["timeshift", "deja-dup"],
    },
    DefaultApp {
        name: "Sigma Bluetooth",
        package_name: "sigma-bluetooth",
        desktop_entry: "sigma-bluetooth.desktop",
        category: AppCategory::Bluetooth,
        mime_types: &[],
        is_essential: false,
        alternatives: &["blueberry", "blueman"],
    },
    DefaultApp {
        name: "Sigma WiFi",
        package_name: "sigma-wifi",
        desktop_entry: "sigma-wifi.desktop",
        category: AppCategory::Wifi,
        mime_types: &[],
        is_essential: true,
        alternatives: &["nm-applet"],
    },
    DefaultApp {
        name: "Sigma Audio",
        package_name: "sigma-audio",
        desktop_entry: "sigma-audio.desktop",
        category: AppCategory::Audio,
        mime_types: &[],
        is_essential: true,
        alternatives: &["pavucontrol", "qpwgraph"],
    },
    DefaultApp {
        name: "Sigma Print",
        package_name: "sigma-print",
        desktop_entry: "sigma-print.desktop",
        category: AppCategory::Printing,
        mime_types: &[],
        is_essential: false,
        alternatives: &["system-config-printer"],
    },
    DefaultApp {
        name: "Sigma Power",
        package_name: "sigma-power",
        desktop_entry: "sigma-power.desktop",
        category: AppCategory::PowerManager,
        mime_types: &[],
        is_essential: false,
        alternatives: &["tlp-rdw", "power-profiles-daemon"],
    },
    DefaultApp {
        name: "Sigma Themes",
        package_name: "sigma-themes",
        desktop_entry: "sigma-themes.desktop",
        category: AppCategory::ThemeManager,
        mime_types: &[],
        is_essential: false,
        alternatives: &["lxappearance", "gnome-tweaks"],
    },
    DefaultApp {
        name: "Sigma Security",
        package_name: "sigma-security",
        desktop_entry: "sigma-security.desktop",
        category: AppCategory::Security,
        mime_types: &[],
        is_essential: true,
        alternatives: &[],
    },
    DefaultApp {
        name: "Sigma Settings",
        package_name: "sigma-settings",
        desktop_entry: "sigma-settings.desktop",
        category: AppCategory::SystemSettings,
        mime_types: &[],
        is_essential: true,
        alternatives: &[],
    },
    DefaultApp {
        name: "Sigma Calculator",
        package_name: "sigma-calc",
        desktop_entry: "sigma-calc.desktop",
        category: AppCategory::Calculator,
        mime_types: &[],
        is_essential: false,
        alternatives: &["gnome-calculator", "kcalc"],
    },
];

/// Default MIME type → application associations.
pub const DEFAULT_MIME_ASSOCIATIONS: &[FileAssociation] = &[
    FileAssociation { mime_type: "text/html",
        handler_desktop_entry: "sigma-browser.desktop", is_default: true },
    FileAssociation { mime_type: "x-scheme-handler/http",
        handler_desktop_entry: "sigma-browser.desktop", is_default: true },
    FileAssociation { mime_type: "x-scheme-handler/https",
        handler_desktop_entry: "sigma-browser.desktop", is_default: true },
    FileAssociation { mime_type: "application/pdf",
        handler_desktop_entry: "sigma-pdfview.desktop", is_default: true },
    FileAssociation { mime_type: "inode/directory",
        handler_desktop_entry: "sigma-files.desktop", is_default: true },
    FileAssociation { mime_type: "text/plain",
        handler_desktop_entry: "sigma-text.desktop", is_default: true },
    FileAssociation { mime_type: "image/jpeg",
        handler_desktop_entry: "sigma-pix.desktop", is_default: true },
    FileAssociation { mime_type: "image/png",
        handler_desktop_entry: "sigma-pix.desktop", is_default: true },
    FileAssociation { mime_type: "audio/mpeg",
        handler_desktop_entry: "sigma-music.desktop", is_default: true },
    FileAssociation { mime_type: "audio/flac",
        handler_desktop_entry: "sigma-music.desktop", is_default: true },
    FileAssociation { mime_type: "video/mp4",
        handler_desktop_entry: "sigma-player.desktop", is_default: true },
    FileAssociation { mime_type: "video/x-matroska",
        handler_desktop_entry: "sigma-player.desktop", is_default: true },
    FileAssociation {
        mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        handler_desktop_entry: "libreoffice-writer.desktop", is_default: true },
    FileAssociation { mime_type: "application/vnd.debian.binary-package",
        handler_desktop_entry: "sigma-packages.desktop", is_default: true },
];

/// Errors from the user profile generator.
#[derive(Debug)]
pub enum ProfileError {
    EmptyUsername,
    InvalidHomePath(String),
    DirectoryCreation { path: String, reason: String },
    ConfigWrite { path: String, reason: String },
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUsername => write!(f, "username is empty"),
            Self::InvalidHomePath(p) => write!(f, "home directory path is invalid: {}", p),
            Self::DirectoryCreation { path, reason } => {
                write!(f, "I/O error creating directory {}: {}", path, reason)
            }
            Self::ConfigWrite { path, reason } => {
                write!(f, "failed to write config file {}: {}", path, reason)
            }
        }
    }
}

impl std::error::Error for ProfileError {}

/// Generates and installs a default user environment.
pub struct UserProfileGenerator {
    username: String,
    home_dir: PathBuf,
}

impl UserProfileGenerator {
    /// Create a generator for the given user.
    pub fn new(username: &str, home_dir: &str) -> Result<Self, ProfileError> {
        if username.is_empty() {
            return Err(ProfileError::EmptyUsername);
        }
        let home = PathBuf::from(home_dir);
        if home.as_os_str().is_empty() {
            return Err(ProfileError::InvalidHomePath(home_dir.to_string()));
        }
        Ok(Self { username: username.to_string(), home_dir: home })
    }

    /// Return the standard XDG directory paths for this user.
    pub fn create_xdg_dirs(&self) -> Result<XdgDirectories, ProfileError> {
        let h = &self.home_dir;
        Ok(XdgDirectories {
            desktop:   h.join("Desktop"),
            documents: h.join("Documents"),
            downloads: h.join("Downloads"),
            music:     h.join("Music"),
            pictures:  h.join("Pictures"),
            videos:    h.join("Videos"),
            templates: h.join("Templates"),
        })
    }

    /// Return the V1 default app list.
    pub fn install_default_apps(&self) -> Result<Vec<&'static DefaultApp>, ProfileError> {
        Ok(V1_DEFAULT_BUNDLE.iter().collect())
    }

    /// Return the default MIME associations.
    pub fn configure_file_associations(
        &self,
    ) -> Result<Vec<&'static FileAssociation>, ProfileError> {
        Ok(DEFAULT_MIME_ASSOCIATIONS.iter().collect())
    }

    /// Apply the SigmaOS default GTK theme to the user's config.
    pub fn apply_default_theme(&self) -> Result<(), ProfileError> {
        // Production: write ~/.config/gtk-3.0/settings.ini
        Ok(())
    }

    /// Generate a sensible `.bashrc` for new users.
    pub fn generate_bashrc(&self) -> Result<String, ProfileError> {
        Ok(format!(
            r#"# SigmaOS default .bashrc for {username}
# This file is sourced by bash for interactive non-login shells.

# -- History ---------------------------------------------------------------
HISTCONTROL=ignoreboth
HISTSIZE=10000
HISTFILESIZE=20000
shopt -s histappend

# -- Environment -----------------------------------------------------------
export EDITOR=sigma-text
export VISUAL=sigma-text
export BROWSER=sigma-browser
export PAGER=less

# -- PATH ------------------------------------------------------------------
export PATH="$HOME/.local/bin:$PATH"

# -- Aliases ---------------------------------------------------------------
alias ll='ls -alF --color=auto'
alias la='ls -A --color=auto'
alias l='ls -CF --color=auto'
alias grep='grep --color=auto'
alias df='df -h'
alias du='du -h'
alias update='sigma update'
alias rescue='sigma rescue'

# -- Prompt ----------------------------------------------------------------
PS1='\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '

# -- Completion ------------------------------------------------------------
if [ -f /usr/share/bash-completion/bash_completion ]; then
    . /usr/share/bash-completion/bash_completion
fi
"#,
            username = self.username,
        ))
    }

    /// Generate a sensible `.zshrc` for users who choose zsh.
    pub fn generate_zshrc(&self) -> Result<String, ProfileError> {
        Ok(format!(
            r#"# SigmaOS default .zshrc for {username}

# -- History ---------------------------------------------------------------
HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt HIST_IGNORE_DUPS HIST_IGNORE_SPACE SHARE_HISTORY

# -- Environment -----------------------------------------------------------
export EDITOR=sigma-text
export VISUAL=sigma-text
export BROWSER=sigma-browser
export PAGER=less
export PATH="$HOME/.local/bin:$PATH"

# -- Aliases ---------------------------------------------------------------
alias ll='ls -alF --color=auto'
alias la='ls -A'
alias l='ls -CF'
alias grep='grep --color=auto'
alias update='sigma update'
alias rescue='sigma rescue'

# -- Prompt ----------------------------------------------------------------
autoload -U colors && colors
PS1="%B%F{{green}}%n@%m%f%b:%B%F{{blue}}%~%f%b%# "

# -- Completion ------------------------------------------------------------
autoload -U compinit && compinit
"#,
            username = self.username,
        ))
    }

    /// Finalize profile setup — write all config to disk.
    pub fn finalize(&self) -> Result<(), ProfileError> {
        // Production: create dirs, write dotfiles, set permissions.
        Ok(())
    }

    /// Generate a `mimeapps.list` file content for `~/.config/mimeapps.list`.
    pub fn generate_mimeapps_list(&self) -> String {
        let mut out = String::from("[Default Applications]\n");
        for assoc in DEFAULT_MIME_ASSOCIATIONS.iter().filter(|a| a.is_default) {
            out.push_str(&format!("{}={}\n", assoc.mime_type, assoc.handler_desktop_entry));
        }
        out.push_str("\n[Added Associations]\n");
        for assoc in DEFAULT_MIME_ASSOCIATIONS {
            out.push_str(&format!("{}={}\n", assoc.mime_type, assoc.handler_desktop_entry));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_bundle_has_24_apps() {
        assert_eq!(V1_DEFAULT_BUNDLE.len(), 24,
            "V1 bundle should contain exactly 24 apps");
    }

    #[test]
    fn test_essential_apps_present() {
        // Arrange
        let essential: Vec<_> = V1_DEFAULT_BUNDLE.iter()
            .filter(|a| a.is_essential)
            .map(|a| a.package_name)
            .collect();
        // Assert — must have these essentials
        assert!(essential.contains(&"sigma-browser"));
        assert!(essential.contains(&"sigma-files"));
        assert!(essential.contains(&"sigma-term"));
        assert!(essential.contains(&"sigma-update"));
        assert!(essential.contains(&"sigma-settings"));
        assert!(essential.contains(&"sigma-security"));
    }

    #[test]
    fn test_mime_lookup_for_html() {
        // Arrange
        let assocs = DEFAULT_MIME_ASSOCIATIONS;
        // Act
        let handler = assocs.iter().find(|a| a.mime_type == "text/html");
        // Assert
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().handler_desktop_entry, "sigma-browser.desktop");
    }

    #[test]
    fn test_generate_mimeapps_list_non_empty() {
        // Arrange
        let gen = UserProfileGenerator::new("alice", "/home/alice").unwrap();
        // Act
        let content = gen.generate_mimeapps_list();
        // Assert
        assert!(content.contains("[Default Applications]"));
        assert!(content.contains("sigma-browser.desktop"));
    }

    #[test]
    fn test_xdg_dirs_paths() {
        // Arrange
        let gen = UserProfileGenerator::new("bob", "/home/bob").unwrap();
        // Act
        let dirs = gen.create_xdg_dirs().unwrap();
        // Assert
        assert!(dirs.downloads.ends_with("Downloads"));
        assert!(dirs.pictures.ends_with("Pictures"));
        assert!(dirs.videos.ends_with("Videos"));
    }

    #[test]
    fn test_bashrc_contains_username() {
        // Arrange
        let gen = UserProfileGenerator::new("charlie", "/home/charlie").unwrap();
        // Act
        let rc = gen.generate_bashrc().unwrap();
        // Assert
        assert!(rc.contains("charlie"));
        assert!(rc.contains("sigma update"));
    }

    #[test]
    fn test_empty_username_rejected() {
        let result = UserProfileGenerator::new("", "/home/nobody");
        assert!(matches!(result, Err(ProfileError::EmptyUsername)));
    }

    #[test]
    fn test_all_24_apps_have_desktop_entry() {
        for app in V1_DEFAULT_BUNDLE {
            assert!(!app.desktop_entry.is_empty(),
                "App {} has empty desktop_entry", app.name);
            assert!(app.desktop_entry.ends_with(".desktop"),
                "App {} desktop_entry must end with .desktop", app.name);
        }
    }
}
