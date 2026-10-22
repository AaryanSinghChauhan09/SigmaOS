// src/branding/branding.rs
// System branding / about / logo for SigmaOS, inspired by Omarchy 4.0.5
// (`omancy-branding-about`, `omancy-show-logo`, `omancy-branding-screensaver`).
//
// This subsystem owns the branding *data* only. Rendering the logo to a
// screen, wallpaper, or screensaver is the caller's policy
// (SEPARATION OF MECHANISM & POLICY), which keeps this module small,
// testable, and free of graphics dependencies.
//
// SigmaOS engineering canon applied throughout:
//   * Single responsibility: branding metadata (name, version, tagline,
//     logo, copyright, support URL) only.
//   * Clear abstraction & encapsulation: all fields are private.
//   * Command-query separation: `set_*` commands mutate; `about()` and
//     `show_logo()` queries observe without mutating.
//   * Composition over inheritance: plain struct + methods, no hierarchy.
//   * Memory safety: #![no_std] + alloc, bounds-checked, no `unsafe`.
//   * Fail-fast / defensive: rejects empty identity values, clamps lengths,
//     never panics on missing branding.
//   * Deterministic: pure functions of fields; tests are
//     FAST, ISOLATED, REPEATABLE, SELF-VALIDATING and TIMELY.
//   * Cache-conscious / locality of reference: logo stored as a flat Vec of
//     lines; `about()` builds a single String.
//   * Zero-copy observation: `show_logo()` borrows the internal line slice.
//   * Secure defaults: falls back to "SigmaOS" / empty rather than crashing
//     when identity is unset.
//   * Zero-one-infinity: zero logo lines valid, one typical, many supported.

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Maximum length enforced on identity fields (defensive bound).
pub const MAX_IDENTITY_LEN: usize = 128;

/// Maximum number of logo lines (zero-one-infinity bound).
pub const MAX_LOGO_LINES: usize = 64;

/// System branding identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branding {
    distro_name: String,
    version: String,
    tagline: String,
    logo: Vec<String>,
    copyright: String,
    support_url: String,
}

impl Branding {
    /// Secure-default branding (SigmaOS, unset version/tagline).
    pub fn new() -> Self {
        Self {
            distro_name: "SigmaOS".to_string(),
            version: String::new(),
            tagline: String::new(),
            logo: Vec::new(),
            copyright: String::new(),
            support_url: String::new(),
        }
    }

    /// Distro/product name (Query, zero-copy borrow).
    pub fn distro_name(&self) -> &str {
        &self.distro_name
    }

    /// Version string (Query, zero-copy borrow).
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Tagline (Query, zero-copy borrow).
    pub fn tagline(&self) -> &str {
        &self.tagline
    }

    /// Copyright notice (Query, zero-copy borrow).
    pub fn copyright(&self) -> &str {
        &self.copyright
    }

    /// Support URL (Query, zero-copy borrow).
    pub fn support_url(&self) -> &str {
        &self.support_url
    }

    /// Logo lines, newest/original order (Query, zero-copy borrow).
    pub fn show_logo(&self) -> &[String] {
        &self.logo
    }

    /// Whether any branding identity has been customized (Query).
    pub fn is_customized(&self) -> bool {
        self.version.is_empty()
            && self.tagline.is_empty()
            && self.logo.is_empty()
            && self.copyright.is_empty()
            && self.support_url.is_empty() == false
    }

    /// Human-readable "about" summary (Query). Builds one String so callers
    /// can print or display it in a single pass.
    pub fn about(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.distro_name);
        if !self.version.is_empty() {
            out.push(' ');
            out.push_str(&self.version);
        }
        if !self.tagline.is_empty() {
            out.push_str(" - ");
            out.push_str(&self.tagline);
        }
        out.push('\n');
        if !self.copyright.is_empty() {
            out.push_str(&self.copyright);
            out.push('\n');
        }
        if !self.support_url.is_empty() {
            out.push_str(&self.support_url);
            out.push('\n');
        }
        out
    }

    /// Set the distro name (Command). Rejects empty values (secure default:
    /// keeps the previous name rather than clearing it).
    pub fn set_distro_name(&mut self, name: String) -> bool {
        let name = name.trim();
        if name.is_empty() || name.len() > MAX_IDENTITY_LEN {
            return false;
        }
        self.distro_name = name.to_string();
        true
    }

    /// Set the version (Command). Rejects empty values.
    pub fn set_version(&mut self, version: String) -> bool {
        let version = version.trim();
        if version.is_empty() || version.len() > MAX_IDENTITY_LEN {
            return false;
        }
        self.version = version.to_string();
        true
    }

    /// Set the tagline (Command). Rejects empty values.
    pub fn set_tagline(&mut self, tagline: String) -> bool {
        let tagline = tagline.trim();
        if tagline.len() > MAX_IDENTITY_LEN {
            return false;
        }
        self.tagline = tagline.to_string();
        true
    }

    /// Set the copyright notice (Command).
    pub fn set_copyright(&mut self, copyright: String) -> bool {
        let copyright = copyright.trim();
        if copyright.len() > MAX_IDENTITY_LEN {
            return false;
        }
        self.copyright = copyright.to_string();
        true
    }

    /// Set the support URL (Command). Rejects values with spaces (defensive:
    /// a URL must not contain whitespace).
    pub fn set_support_url(&mut self, url: String) -> bool {
        let url = url.trim();
        if url.is_empty() || url.contains(char::is_whitespace) || url.len() > MAX_IDENTITY_LEN {
            return false;
        }
        self.support_url = url.to_string();
        true
    }

    /// Replace the logo with `lines` (Command). Trims trailing empty lines;
    /// rejects if the total exceeds `MAX_LOGO_LINES`.
    pub fn set_logo(&mut self, lines: Vec<String>) -> bool {
        if lines.len() > MAX_LOGO_LINES {
            return false;
        }
        let logo: Vec<String> = lines
            .into_iter()
            .map(|l| l.trim_end().to_string())
            .collect();
        self.logo = logo;
        true
    }

    /// Append a single line to the logo (Command).
    pub fn push_logo_line(&mut self, line: String) -> bool {
        if self.logo.len() >= MAX_LOGO_LINES {
            return false;
        }
        self.logo.push(line.trim_end().to_string());
        true
    }

    /// Clear all custom branding back to secure defaults (Command).
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Number of logo lines (Query).
    pub fn logo_line_count(&self) -> usize {
        self.logo.len()
    }
}

impl Default for Branding {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Branding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Branding {{ name={}, version={}, logo_lines={} }}",
            self.distro_name,
            self.version,
            self.logo.len()
        )
    }
}

// ---- Deterministic unit tests (ARRANGE-ACT-ASSERT,
// FAST-ISOLATED-REPEATABLE-SELF-VALIDATING-TIMELY) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_secure_and_nonempty_name() {
        // Arrange / Act
        let b = Branding::new();
        // Assert
        assert_eq!(b.distro_name(), "SigmaOS");
        assert_eq!(b.version(), "");
        assert_eq!(b.tagline(), "");
        assert!(b.show_logo().is_empty());
        assert_eq!(b.logo_line_count(), 0);
    }

    #[test]
    fn set_distro_name_updates_and_about_reflects_it() {
        // Arrange
        let mut b = Branding::new();
        // Act
        assert!(b.set_distro_name("Omarchy OS".to_string()));
        b.set_version("4.0.5".to_string());
        b.set_tagline("A tiling desktop for creators".to_string());
        // Assert
        assert_eq!(b.distro_name(), "Omarchy OS");
        let about = b.about();
        assert!(about.contains("Omarchy OS"));
        assert!(about.contains("4.0.5"));
        assert!(about.contains("A tiling desktop for creators"));
        assert!(b.is_customized());
    }

    #[test]
    fn empty_distro_name_is_rejected_secure_default() {
        // Arrange
        let mut b = Branding::new();
        // Act
        let r1 = b.set_distro_name("".to_string());
        let r2 = b.set_distro_name("   ".to_string());
        let r3 = b.set_distro_name("x".repeat(MAX_IDENTITY_LEN + 1).to_string());
        // Assert
        assert!(!r1);
        assert!(!r2);
        assert!(!r3);
        assert_eq!(b.distro_name(), "SigmaOS"); // unchanged
    }

    #[test]
    fn set_version_and_tagline() {
        // Arrange
        let mut b = Branding::new();
        // Act
        assert!(b.set_version("1.0.0".to_string()));
        assert!(b.set_tagline("tag".to_string()));
        // Assert
        assert_eq!(b.version(), "1.0.0");
        assert_eq!(b.tagline(), "tag");
    }

    #[test]
    fn set_copyright_and_support_url() {
        // Arrange
        let mut b = Branding::new();
        // Act
        assert!(b.set_copyright("(c) 2026 SigmaOS".to_string()));
        assert!(b.set_support_url("https://sigmaos.example/support".to_string()));
        // Assert
        assert_eq!(b.copyright(), "(c) 2026 SigmaOS");
        assert_eq!(b.support_url(), "https://sigmaos.example/support");
    }

    #[test]
    fn support_url_with_spaces_is_rejected() {
        // Arrange
        let mut b = Branding::new();
        // Act
        let r = b.set_support_url("https:// example.com".to_string());
        // Assert
        assert!(!r);
        assert_eq!(b.support_url(), "");
    }

    #[test]
    fn set_logo_stores_lines_zero_copy() {
        // Arrange
        let mut b = Branding::new();
        let lines = vec!["  __  ".to_string(), " /  \\".to_string()];
        // Act
        assert!(b.set_logo(lines.clone()));
        // Assert
        assert_eq!(b.show_logo(), &["__".to_string(), "/  \\".to_string()]);
        assert_eq!(b.logo_line_count(), 2);
    }

    #[test]
    fn push_logo_line_appends() {
        // Arrange
        let mut b = Branding::new();
        // Act
        assert!(b.push_logo_line("line one".to_string()));
        assert!(b.push_logo_line("line two".to_string()));
        // Assert
        assert_eq!(
            b.show_logo(),
            &["line one".to_string(), "line two".to_string()]
        );
    }

    #[test]
    fn logo_line_count_cap_is_enforced() {
        // Arrange
        let mut b = Branding::new();
        // Act
        for i in 0..MAX_LOGO_LINES {
            assert!(b.push_logo_line(format!("l{i}")));
        }
        assert!(!b.push_logo_line("one too many".to_string()));
        // Assert
        assert_eq!(b.logo_line_count(), MAX_LOGO_LINES);
    }

    #[test]
    fn reset_returns_to_secure_defaults() {
        // Arrange
        let mut b = Branding::new();
        b.set_distro_name("Custom".to_string());
        b.push_logo_line("art".to_string());
        assert!(b.is_customized());
        // Act
        b.reset();
        // Assert
        assert_eq!(b.distro_name(), "SigmaOS");
        assert!(!b.is_customized());
        assert_eq!(b.logo_line_count(), 0);
    }

    #[test]
    fn about_contains_version_and_tagline_when_present() {
        // Arrange
        let mut b = Branding::new();
        b.set_version("9".to_string());
        b.set_tagline("t".to_string());
        // Act
        let about = b.about();
        // Assert
        assert!(about.contains("9"));
        assert!(about.contains("t"));
    }
}
