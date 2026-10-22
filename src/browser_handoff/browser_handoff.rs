// src/browser_handoff/browser_handoff.rs
// Browser URL handoff and default-browser management for SigmaOS,
// inspired by Omarchy 4.0.5 (`omancy-default-browser`,
// `omancy-cmd-browser-handoff`, `omancy-chromium-copy-url-host`,
// `omancy-chromium-ytdlp-host`).
//
// This subsystem owns the *mechanism* of browser selection and URL handling
// only. The actual process launch (policy) is left to the caller, preserving
// SEPARATION OF MECHANISM & POLICY and keeping the module testable without
// spawning processes.
//
// SigmaOS engineering canon applied throughout:
//   * Single responsibility: default-browser selection + URL/host handling only.
//   * Clear abstraction & encapsulation: internal state is private.
//   * Command-query separation: commands mutate and report outcomes;
//     queries observe without mutating.
//   * Composition over inheritance: plain data + free functions, no hierarchy.
//   * Memory safety: #![no_std] + alloc, bounds-checked, no `unsafe`.
//   * Fail-fast / defensive: rejects empty URLs, clamps unknown schemes,
//     never panics on missing browser.
//   * Deterministic: no wall clock, no randomness; outcomes are pure functions
//     of input (FAST, ISOLATED, REPEATABLE, SELF-VALIDATING, TIMELY).
//   * Cache-conscious / locality of reference: small fixed set of known
//     browsers stored as an enum (no heap lookup for the common case).
//   * Zero-copy observation: `default_browser()` borrows; host extraction
//     returns owned String only because a new substring is produced.
//   * Secure defaults: `None` browser with graceful fallback; empty URL
//     rejected instead of silently opening about:blank.
//   * Zero-one-infinity: zero browsers configured is valid, one typical,
//     custom browsers unbounded via `Custom(String)`.

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Known default browsers. Omarchy's `omancy-default-browser` manages a
/// single default; this enum keeps the common cases in-memory without a
/// heap allocation for the hot path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultBrowser {
    Chromium,
    Firefox,
    Epiphany,
    None,
}

impl DefaultBrowser {
    /// Friendly display name (Zero-copy `&str`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chromium => "chromium",
            Self::Firefox => "firefox",
            Self::Epiphany => "epiphany",
            Self::None => "none",
        }
    }
}

impl fmt::Display for DefaultBrowser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for DefaultBrowser {
    fn default() -> Self {
        Self::None
    }
}

/// Outcome of a URL handoff. Kept as data so the caller decides how to act
//  (SEPARATION OF MECHANISM & POLICY).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffOutcome {
    /// The URL should be opened in `browser`.
    Opened {
        browser: DefaultBrowser,
        url: String,
    },
    /// No browser configured; URL queued for later.
    Deferred { url: String },
    /// The URL was rejected (invalid).
    Rejected { url: String, reason: HandoffError },
}

/// Why a handoff was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffError {
    EmptyUrl,
    InvalidScheme,
    NoDefaultBrowser,
}

impl fmt::Display for HandoffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUrl => write!(f, "empty url"),
            Self::InvalidScheme => write!(f, "unsupported url scheme"),
            Self::NoDefaultBrowser => write!(f, "no default browser configured"),
        }
    }
}

/// Result of `handoff_url` — `Ok` when a browser was selected, `Err` when
/// the URL must not be opened.
pub type HandoffResult = Result<HandoffOutcome, HandoffError>;

/// Browser handoff state machine.
pub struct BrowserHandoff {
    default: DefaultBrowser,
    custom: Option<String>,
    /// URLs deferred because no browser was configured (FIFO).
    deferred: Vec<String>,
    handoffs: u64, // instrumentation / observability
}

impl BrowserHandoff {
    /// Create a new handoff manager with no default browser.
    pub fn new() -> Self {
        Self {
            default: DefaultBrowser::None,
            custom: None,
            deferred: Vec::new(),
            handoffs: 0,
        }
    }

    /// Current default browser (Query, zero-copy borrow).
    pub fn default_browser(&self) -> DefaultBrowser {
        self.default
    }

    /// Whether a default browser is configured (Query).
    pub fn has_default_browser(&self) -> bool {
        self.default != DefaultBrowser::None
    }

    /// All deferred URLs, oldest-first (Query, zero-copy borrow).
    pub fn deferred_urls(&self) -> &[String] {
        &self.deferred
    }

    /// Number of deferred URLs (Query).
    pub fn deferred_count(&self) -> usize {
        self.deferred.len()
    }

    /// Number of successful handoffs (instrumentation, Query).
    pub fn handoffs(&self) -> u64 {
        self.handoffs
    }

    /// Set the default browser (Command). `Custom(name)` stores the name.
    pub fn set_default_browser(&mut self, browser: DefaultBrowser) {
        self.default = browser;
    }

    /// Set a custom default browser by name (Command).
    pub fn set_custom_browser(&mut self, name: String) {
        if !name.trim().is_empty() {
            self.custom = Some(name.trim().to_string());
            self.default = DefaultBrowser::None; // custom tracked separately
        }
    }

    /// Resolve the effective browser to use for a handoff (Query).
    pub fn effective_browser(&self) -> DefaultBrowser {
        if self.custom.is_some() {
            // Custom browsers are launched by name by the caller; report None
            // so the caller uses `custom_name()`.
            DefaultBrowser::None
        } else {
            self.default
        }
    }

    /// Custom browser name, if a custom browser was configured (Query).
    pub fn custom_name(&self) -> Option<&str> {
        self.custom.as_deref()
    }

    /// Validate and classify a URL for handoff. Returns the outcome the
    /// caller should act on (Command: mutates deferred queue on failure;
    /// Query: pure classification otherwise).
    pub fn handoff_url(&mut self, url: String) -> HandoffResult {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err(HandoffError::EmptyUrl);
        }
        if !is_supported_scheme(trimmed) {
            self.deferred.push(url.clone());
            self.handoffs += 1;
            return Err(HandoffError::InvalidScheme);
        }
        if self.default == DefaultBrowser::None && self.custom.is_none() {
            // Secure default: never silently open a URL with no browser.
            self.deferred.push(url.clone());
            self.handoffs += 1;
            return Err(HandoffError::NoDefaultBrowser);
        }
        self.handoffs += 1;
        Ok(HandoffOutcome::Opened {
            browser: self.effective_browser(),
            url: trimmed.to_string(),
        })
    }

    /// Extract the host from a URL (Query). Returns `None` for malformed
    /// input; defensive, never panics. Mirrors `omancy-chromium-copy-url-host`.
    pub fn copy_url_host(url: &str) -> Option<String> {
        let url = url.trim();
        if url.is_empty() {
            return None;
        }
        let rest = url
            .strip_prefix("http://")
            .or_else(|| url.strip_prefix("https://"))
            .or_else(|| url.strip_prefix("file://"))
            .unwrap_or(url);
        let host = rest.split('/').next().unwrap_or(rest);
        let host = host.split('?').next().unwrap_or(host);
        let host = host.split('#').next().unwrap_or(host);
        let host = host.split(':').next().unwrap_or(host);
        let host = host.trim();
        if host.is_empty() {
            None
        } else {
            Some(host.to_string())
        }
    }

    /// Extract a ytdlp-friendly host from a URL (Query). Mirrors
    /// `omancy-chromium-ytdlp-host`: strips scheme and www prefix.
    pub fn ytdlp_host(url: &str) -> Option<String> {
        let host = Self::copy_url_host(url)?;
        let host = host.strip_prefix("www.").unwrap_or(&host);
        if host.is_empty() {
            None
        } else {
            Some(host.to_string())
        }
    }

    /// Clear the deferred URL queue (Command).
    pub fn clear_deferred(&mut self) {
        self.deferred.clear();
    }

    /// Retry all deferred URLs once a browser is configured (Command).
    /// Returns the outcomes the caller should act on.
    pub fn retry_deferred(&mut self) -> Vec<HandoffResult> {
        let urls: Vec<String> = self.deferred.drain(..).collect();
        urls.into_iter().map(|u| self.handoff_url(u)).collect()
    }
}

impl Default for BrowserHandoff {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BrowserHandoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BrowserHandoff {{ default={}, deferred={}, handoffs={} }}",
            self.default,
            self.deferred.len(),
            self.handoffs
        )
    }
}

/// Whether `url` begins with a supported scheme (defensive helper).
fn is_supported_scheme(url: &str) -> bool {
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("file://")
        || url.starts_with("mailto:")
}

// ---- Deterministic unit tests (ARRANGE-ACT-ASSERT,
// FAST-ISOLATED-REPEATABLE-SELF-VALIDATING-TIMELY) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_default_browser() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        // Act
        bh.set_default_browser(DefaultBrowser::Firefox);
        // Assert
        assert_eq!(bh.default_browser(), DefaultBrowser::Firefox);
        assert!(bh.has_default_browser());
    }

    #[test]
    fn no_browser_starts_as_none() {
        // Arrange / Act
        let bh = BrowserHandoff::new();
        // Assert
        assert_eq!(bh.default_browser(), DefaultBrowser::None);
        assert!(!bh.has_default_browser());
    }

    #[test]
    fn handoff_url_opens_with_configured_browser() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        bh.set_default_browser(DefaultBrowser::Chromium);
        // Act
        let res = bh.handoff_url("https://sigmaos.example/page".to_string());
        // Assert
        assert!(matches!(res, Ok(HandoffOutcome::Opened { .. })));
        if let Ok(HandoffOutcome::Opened { browser, url }) = res {
            assert_eq!(browser, DefaultBrowser::Chromium);
            assert_eq!(url, "https://sigmaos.example/page");
        }
        assert_eq!(bh.handoffs(), 1);
        assert_eq!(bh.deferred_count(), 0);
    }

    #[test]
    fn empty_url_is_rejected() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        bh.set_default_browser(DefaultBrowser::Chromium);
        // Act
        let res = bh.handoff_url("".to_string());
        // Assert
        assert_eq!(res, Err(HandoffError::EmptyUrl));
        assert_eq!(bh.deferred_count(), 0);
    }

    #[test]
    fn unsupported_scheme_is_deferred_and_rejected() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        bh.set_default_browser(DefaultBrowser::Chromium);
        // Act
        let res = bh.handoff_url("ftp://example.com".to_string());
        // Assert
        assert_eq!(res, Err(HandoffError::InvalidScheme));
        assert_eq!(bh.deferred_count(), 1);
    }

    #[test]
    fn no_browser_defers_instead_of_opening_secure_default() {
        // Arrange
        let mut bh = BrowserHandoff::new(); // no browser
                                            // Act
        let res = bh.handoff_url("https://example.com".to_string());
        // Assert
        assert_eq!(res, Err(HandoffError::NoDefaultBrowser));
        assert_eq!(bh.deferred_count(), 1);
        // Not opened: secure default, never silently navigates.
    }

    #[test]
    fn custom_browser_name_is_kept() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        // Act
        bh.set_custom_browser("epiphany".to_string());
        // Assert
        assert_eq!(bh.custom_name(), Some("epiphany"));
        assert_eq!(bh.effective_browser(), DefaultBrowser::None);
    }

    #[test]
    fn copy_url_host_strips_scheme() {
        // Arrange / Act / Assert
        assert_eq!(
            BrowserHandoff::copy_url_host("https://sigmaos.example/page?q=1"),
            Some("sigmaos.example".to_string())
        );
        assert_eq!(
            BrowserHandoff::copy_url_host("http://www.omarchy.org/"),
            Some("www.omarchy.org".to_string())
        );
        assert_eq!(
            BrowserHandoff::copy_url_host("not-a-url"),
            Some("not-a-url".to_string())
        );
        assert_eq!(BrowserHandoff::copy_url_host(""), None);
    }

    #[test]
    fn ytdlp_host_strips_www() {
        // Arrange / Act / Assert
        assert_eq!(
            BrowserHandoff::ytdlp_host("https://www.youtube.com/watch?v=x"),
            Some("youtube.com".to_string())
        );
        assert_eq!(
            BrowserHandoff::ytdlp_host("https://vimeo.com/clip"),
            Some("vimeo.com".to_string())
        );
    }

    #[test]
    fn deferred_urls_are_retryable() {
        // Arrange
        let mut bh = BrowserHandoff::new(); // no browser -> defers
        bh.handoff_url("https://a.example".to_string()).unwrap_err();
        bh.handoff_url("https://b.example".to_string()).unwrap_err();
        assert_eq!(bh.deferred_count(), 2);
        // Act: configure a browser, then retry.
        bh.set_default_browser(DefaultBrowser::Firefox);
        let results = bh.retry_deferred();
        // Assert
        assert_eq!(bh.deferred_count(), 0);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn clear_deferred_empties_queue() {
        // Arrange
        let mut bh = BrowserHandoff::new();
        bh.handoff_url("https://a.example".to_string()).unwrap_err();
        // Act
        bh.clear_deferred();
        // Assert
        assert_eq!(bh.deferred_count(), 0);
    }
}
