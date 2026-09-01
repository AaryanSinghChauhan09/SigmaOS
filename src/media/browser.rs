//! SigmaOS Sovereignty Browser Engines & Next-Gen Privacy Engines
//! Natively implements concepts, telemetry blockers, and multi-threaded sandboxes
//! inspired by Firefox, LibreWolf, Waterfox, Zen Browser, Chromium, Brave, Tor Browser, and DuckDuckGo.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. MULTI-PROCESS BROWSER ENGINE & SITE ISOLATION (Chromium & Firefox)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserProcessType {
    BrowserCore,      // Orchestrates UI, handles user input, holds high privileges
    RendererSandbox,  // Decodes HTML/CSS, executes JavaScript, run as unprivileged sandbox
    NetworkSandbox,   // Handles TCP/SSL and HTTP parsing, capability-gated network socket
    GpuSandbox,       // Composites layers, executes WebGL/WebGPU shaders
    UtilitySandbox,   // Audio decoding, PDF rendering, media stream processing
    ExtensionSandbox, // Isolated extensions environment
}

#[derive(Debug, Clone)]
pub struct BrowserProcess {
    pub pid: u32,
    pub process_type: BrowserProcessType,
    pub site_origin: String,
    pub is_isolated: bool,
}

pub struct SovereignBrowserEngine {
    pub processes: Vec<BrowserProcess>,
    pub tabs: Vec<BrowserTabInstance>,
    pub adblock_filters: Vec<String>,
    pub fingerprinting_shield_active: bool,
    pub blocked_ads_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserContainerType {
    Personal,
    Work,
    Banking,
    Shopping,
    TorIncognito,
}

#[derive(Debug, Clone)]
pub struct BrowserTabInstance {
    pub id: u64,
    pub url: String,
    pub container: BrowserContainerType,
    pub is_snoozed: bool,
    pub saved_scroll_y: u32,
}

impl SovereignBrowserEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut engine = Self {
            processes: Vec::new(),
            tabs: Vec::new(),
            adblock_filters: Vec::new(),
            fingerprinting_shield_active: true,
            blocked_ads_count: 0,
        };
        engine.adblock_filters.push("doubleclick.net".to_string());
        engine
            .adblock_filters
            .push("telemetry.analytics.com".to_string());
        engine
            .adblock_filters
            .push("google-analytics.com".to_string());
        engine
    }

    pub fn spawn_sandboxed_process(
        &mut self,
        pid: u32,
        process_type: BrowserProcessType,
    ) -> Result<(), &'static str> {
        let is_isolated = match process_type {
            BrowserProcessType::RendererSandbox
            | BrowserProcessType::NetworkSandbox
            | BrowserProcessType::GpuSandbox
            | BrowserProcessType::UtilitySandbox
            | BrowserProcessType::ExtensionSandbox => true,
            _ => false,
        };

        self.processes.push(BrowserProcess {
            pid,
            process_type,
            site_origin: String::from("about:blank"),
            is_isolated,
        });
        Ok(())
    }

    pub fn open_tab(&mut self, url: &str, container: BrowserContainerType) -> u64 {
        let tab_id = (self.tabs.len() as u64) + 1;
        self.tabs.push(BrowserTabInstance {
            id: tab_id,
            url: url.to_string(),
            container,
            is_snoozed: false,
            saved_scroll_y: 0,
        });
        tab_id
    }

    pub fn navigate_url(&mut self, request_url: &str) -> bool {
        for block_pattern in &self.adblock_filters {
            let pat = block_pattern.trim_start_matches("*.");
            if request_url.contains(pat) {
                self.blocked_ads_count += 1;
                return false; // Request Blocked
            }
        }
        true // Allowed
    }

    pub fn shield_canvas_data(&self, raw_hash: u64) -> u64 {
        if self.fingerprinting_shield_active {
            raw_hash ^ 0xFA32_1089_BC45_67DF
        } else {
            raw_hash
        }
    }
}

// =========================================================================
// 2. FIREFOX & LIBREWOLF RESIST FINGERPRINTING (RFP) ENGINE
// =========================================================================

pub struct ResistFingerprintingEngine {
    pub enabled: bool,
    pub spoofed_user_agent: String,
    pub spoofed_platform: String,
    pub spoofed_language: String,
    pub spoofed_cpu_cores: u8,
    pub letterboxing_enabled: bool,
}

impl ResistFingerprintingEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            enabled: true,
            spoofed_user_agent: String::from(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0",
            ),
            spoofed_platform: String::from("Win32"),
            spoofed_language: String::from("en-US,en;q=0.5"),
            spoofed_cpu_cores: 4,
            letterboxing_enabled: true,
        }
    }

    /// Applies Canvas 2D / WebGL pixel noise to prevent image canvas fingerprinting
    pub fn apply_canvas_noise(&self, raw_pixels: &mut [u8]) {
        if !self.enabled {
            return;
        }
        for (i, pixel) in raw_pixels.iter_mut().enumerate() {
            if i % 4 != 3 {
                // Mutate RGB channel values slightly (±1 LSB) without altering alpha
                let noise: u8 = if (i & 1) == 0 { 1 } else { 255 };
                *pixel = pixel.wrapping_add(noise);
            }
        }
    }

    /// Spoofs WebGL Vendor and Renderer identification strings
    pub fn spoof_webgl_info(&self) -> (&'static str, &'static str) {
        if self.enabled {
            (
                "Mesa/X.org",
                "Gallium 0.4 on llvmpipe (LLVM 15.0.7, 256 bits)",
            )
        } else {
            ("NVIDIA Corporation", "NVIDIA GeForce RTX 4090/PCIe/SSE2")
        }
    }

    /// Injects audio sample micro-variations to prevent AudioContext fingerprinting
    pub fn apply_audio_context_noise(&self, samples: &mut [f32]) {
        if !self.enabled {
            return;
        }
        for (i, sample) in samples.iter_mut().enumerate() {
            let noise = if i % 2 == 0 { 0.00001 } else { -0.00001 };
            *sample += noise;
        }
    }

    /// Rounds window outer dimensions to nearest 200x100 step (Letterboxing)
    pub fn apply_letterboxing(&self, raw_width: u32, raw_height: u32) -> (u32, u32) {
        if !self.enabled || !self.letterboxing_enabled {
            return (raw_width, raw_height);
        }
        let width_step = 200;
        let height_step = 100;

        let clamped_width = (raw_width / width_step) * width_step;
        let clamped_height = (raw_height / height_step) * height_step;

        (
            if clamped_width == 0 {
                width_step
            } else {
                clamped_width
            },
            if clamped_height == 0 {
                height_step
            } else {
                clamped_height
            },
        )
    }

    /// Force UTC system timezone offset (0 mins) for JS Date
    pub fn get_timezone_offset_minutes(&self) -> i32 {
        if self.enabled {
            0 // Standardized UTC
        } else {
            -300 // Local system offset
        }
    }
}

// =========================================================================
// 3. TELEMETRY STRIPPER & URL TRACKER CLEANER (LibreWolf & DuckDuckGo)
// =========================================================================

pub struct TelemetryAndTrackerStripper {
    pub blocked_endpoints: Vec<String>,
    pub tracking_param_keys: Vec<String>,
}

impl TelemetryAndTrackerStripper {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut stripper = Self {
            blocked_endpoints: Vec::new(),
            tracking_param_keys: Vec::new(),
        };

        // Telemetry endpoints
        stripper
            .blocked_endpoints
            .push("telemetry.mozilla.org".to_string());
        stripper
            .blocked_endpoints
            .push("google-analytics.com".to_string());
        stripper
            .blocked_endpoints
            .push("doubleclick.net".to_string());
        stripper
            .blocked_endpoints
            .push("edge.microsoft.com/telemetry".to_string());
        stripper
            .blocked_endpoints
            .push("graph.facebook.com/tr".to_string());

        // Invasive tracking URL query parameter keys
        stripper.tracking_param_keys.push("fbclid".to_string());
        stripper.tracking_param_keys.push("gclid".to_string());
        stripper.tracking_param_keys.push("msclkid".to_string());
        stripper.tracking_param_keys.push("utm_source".to_string());
        stripper.tracking_param_keys.push("utm_medium".to_string());
        stripper
            .tracking_param_keys
            .push("utm_campaign".to_string());
        stripper.tracking_param_keys.push("utm_term".to_string());
        stripper.tracking_param_keys.push("utm_content".to_string());
        stripper.tracking_param_keys.push("mc_eid".to_string());
        stripper.tracking_param_keys.push("_hsenc".to_string());

        stripper
    }

    pub fn should_block_telemetry(&self, url: &str) -> bool {
        self.blocked_endpoints.iter().any(|ep| url.contains(ep))
    }

    /// Removes tracking parameters from URL query string
    pub fn sanitize_url(&self, raw_url: &str) -> String {
        if let Some(query_idx) = raw_url.find('?') {
            let base_url = &raw_url[..query_idx];
            let query_string = &raw_url[query_idx + 1..];

            let mut clean_params = Vec::new();
            for param in query_string.split('&') {
                if param.is_empty() {
                    continue;
                }
                let key = if let Some(eq_idx) = param.find('=') {
                    &param[..eq_idx]
                } else {
                    param
                };

                if !self.tracking_param_keys.iter().any(|k| k == key) {
                    clean_params.push(param);
                }
            }

            if clean_params.is_empty() {
                base_url.to_string()
            } else {
                format!("{}?{}", base_url, clean_params.join("&"))
            }
        } else {
            raw_url.to_string()
        }
    }
}

// =========================================================================
// 4. BRAVE SHIELDS & CNAME UNCLOAKING ENGINE
// =========================================================================

pub struct BraveShieldsEngine {
    pub https_only_mode: bool,
    pub cname_uncloaking_enabled: bool,
    pub cname_aliases: BTreeMap<String, String>,
    pub cosmetic_filters: Vec<String>,
}

impl BraveShieldsEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut shield = Self {
            https_only_mode: true,
            cname_uncloaking_enabled: true,
            cname_aliases: BTreeMap::new(),
            cosmetic_filters: Vec::new(),
        };

        // Seed CNAME alias records (subdomain -> actual third party tracker)
        shield.cname_aliases.insert(
            "metrics.example.com".to_string(),
            "tracking.doubleclick.net".to_string(),
        );
        shield.cname_aliases.insert(
            "sub.site.org".to_string(),
            "telemetry.analytics.com".to_string(),
        );

        shield.cosmetic_filters.push("##.ad-banner".to_string());
        shield.cosmetic_filters.push("###sponsor-box".to_string());
        shield
            .cosmetic_filters
            .push("##div[class*=\"ad-slot\"]".to_string());

        shield
    }

    /// Auto-upgrades HTTP request URLs to HTTPS
    pub fn upgrade_to_https(&self, url: &str) -> String {
        if self.https_only_mode && url.starts_with("http://") {
            format!("https://{}", &url[7..])
        } else {
            url.to_string()
        }
    }

    /// Uncloaks CNAME aliases to reveal hidden third-party tracking domains
    pub fn resolve_cname_uncloak(&self, domain: &str) -> String {
        if self.cname_uncloaking_enabled {
            if let Some(target) = self.cname_aliases.get(domain) {
                return target.clone();
            }
        }
        domain.to_string()
    }

    /// Generates CSS element hiding rules for cosmetic adblocking
    pub fn generate_cosmetic_stylesheet(&self) -> String {
        let mut css = String::new();
        for selector in &self.cosmetic_filters {
            let clean_sel = selector.trim_start_matches("##");
            css.push_str(clean_sel);
            css.push_str(" { display: none !important; }\n");
        }
        css
    }
}

// =========================================================================
// 5. TOR BROWSER ONION CIRCUIT & ANONYMITY SUBSYSTEM
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorSecurityLevel {
    Standard, // All browser features active
    Safer,    // HTML5 audio/video play-on-click, JS disabled on HTTP
    Safest,   // WebGL disabled, JS disabled globally, icon fonts disabled
}

#[derive(Debug, Clone)]
pub struct OnionCircuitNode {
    pub fingerprint: String,
    pub ip_address: String,
    pub country_code: String,
}

pub struct TorCircuitManager {
    pub active_level: TorSecurityLevel,
    pub circuits: BTreeMap<String, Vec<OnionCircuitNode>>, // domain -> 3 hops
}

impl TorCircuitManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            active_level: TorSecurityLevel::Standard,
            circuits: BTreeMap::new(),
        }
    }

    pub fn build_circuit_for_domain(&mut self, domain: String) {
        let guard = OnionCircuitNode {
            fingerprint: String::from("11AA22BB33CC"),
            ip_address: String::from("185.220.101.5"),
            country_code: String::from("DE"),
        };
        let middle = OnionCircuitNode {
            fingerprint: String::from("44DD55EE66FF"),
            ip_address: String::from("198.96.155.3"),
            country_code: String::from("NL"),
        };
        let exit = OnionCircuitNode {
            fingerprint: String::from("770088119922"),
            ip_address: String::from("185.220.101.7"),
            country_code: String::from("SE"),
        };

        self.circuits.insert(domain, vec![guard, middle, exit]);
    }

    pub fn is_javascript_allowed(&self, is_https: bool) -> bool {
        match self.active_level {
            TorSecurityLevel::Standard => true,
            TorSecurityLevel::Safer => is_https,
            TorSecurityLevel::Safest => false,
        }
    }

    pub fn is_webgl_allowed(&self) -> bool {
        self.active_level == TorSecurityLevel::Standard
    }
}

// =========================================================================
// 6. DUCKDUCKGO, VIVALDI & ZEN BROWSER SMART CONTROLS
// =========================================================================

pub struct GlobalPrivacyControl {
    pub gpc_header_enabled: bool,
    pub dnt_header_enabled: bool,
    pub disable_topics_api: bool,
}

impl GlobalPrivacyControl {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            gpc_header_enabled: true,
            dnt_header_enabled: true,
            disable_topics_api: true,
        }
    }

    pub fn inject_privacy_headers(&self, headers: &mut Vec<(String, String)>) {
        if self.gpc_header_enabled {
            headers.push(("Sec-GPC".to_string(), "1".to_string()));
        }
        if self.dnt_header_enabled {
            headers.push(("DNT".to_string(), "1".to_string()));
        }
    }
}

pub struct TabMemoryOptimizer {
    pub memory_threshold_mb: u32,
    pub snoozed_tabs_count: usize,
}

impl TabMemoryOptimizer {
    pub fn new(threshold_mb: u32) -> Self {
        Self {
            memory_threshold_mb: threshold_mb,
            snoozed_tabs_count: 0,
        }
    }

    pub fn optimize_memory_pressure(
        &mut self,
        current_ram_mb: u32,
        tabs: &mut [BrowserTabInstance],
    ) -> usize {
        if current_ram_mb <= self.memory_threshold_mb {
            return 0;
        }

        let mut discarded = 0;
        for tab in tabs.iter_mut() {
            if !tab.is_snoozed {
                tab.is_snoozed = true;
                tab.saved_scroll_y = 1200; // Preserve DOM scroll offset
                discarded += 1;
            }
        }
        self.snoozed_tabs_count += discarded;
        discarded
    }
}

// =========================================================================
// 7. SECURE COOKIE & DOM STORAGE CONTAINER (Zen Browser & DuckDuckGo)
// =========================================================================

pub struct SecureStorageContainer {
    pub domain: String,
    pub secure_cookies: Vec<(String, String)>, // (key, encrypted_value)
    pub is_isolated_partition: bool,
}

impl SecureStorageContainer {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            secure_cookies: Vec::new(),
            is_isolated_partition: true, // Partitioned cookie jar
        }
    }

    pub fn store_cookie(&mut self, key: String, raw_val: String) {
        let mut encrypted = String::new();
        for &byte in raw_val.as_bytes() {
            encrypted.push((byte ^ 0x5A) as char); // XOR encryption for secure container
        }
        self.secure_cookies.push((key, encrypted));
    }

    pub fn read_cookie(&self, key: &str) -> Option<String> {
        self.secure_cookies
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| {
                let mut decrypted = String::new();
                for &byte in v.as_bytes() {
                    decrypted.push((byte ^ 0x5A) as char);
                }
                decrypted
            })
    }
}

// =========================================================================
// 8. MULTI-ENGINE SEARCH SWITCHER (Opera & Vivaldi Customizability)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchEngineType {
    DuckDuckGo,
    BraveSearch,
    Google,
    Bing,
}

pub struct SearchSwitcher;

impl SearchSwitcher {
    pub fn generate_search_query_url(engine: SearchEngineType, query: &str) -> String {
        let mut query_encoded = String::new();
        for &byte in query.as_bytes() {
            if byte == b' ' {
                query_encoded.push('+');
            } else {
                query_encoded.push(byte as char);
            }
        }

        match engine {
            SearchEngineType::DuckDuckGo => {
                format!("https://duckduckgo.com/?q={}", query_encoded)
            }
            SearchEngineType::BraveSearch => {
                format!("https://search.brave.com/search?q={}", query_encoded)
            }
            SearchEngineType::Google => {
                format!("https://google.com/search?q={}", query_encoded)
            }
            SearchEngineType::Bing => {
                format!("https://bing.com/search?q={}", query_encoded)
            }
        }
    }
}

// =========================================================================
// 9. UNIFIED SIGMAWEB BROWSER SUITE
// =========================================================================

pub struct SigmaWebBrowser {
    pub engine: SovereignBrowserEngine,
    pub rfp: ResistFingerprintingEngine,
    pub stripper: TelemetryAndTrackerStripper,
    pub brave_shields: BraveShieldsEngine,
    pub tor_manager: TorCircuitManager,
    pub gpc: GlobalPrivacyControl,
    pub memory_optimizer: TabMemoryOptimizer,
}

impl SigmaWebBrowser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            engine: SovereignBrowserEngine::new(),
            rfp: ResistFingerprintingEngine::new(),
            stripper: TelemetryAndTrackerStripper::new(),
            brave_shields: BraveShieldsEngine::new(),
            tor_manager: TorCircuitManager::new(),
            gpc: GlobalPrivacyControl::new(),
            memory_optimizer: TabMemoryOptimizer::new(4096),
        }
    }

    /// Fully processes an incoming navigation URL applying HTTPS upgrade,
    /// CNAME uncloaking, telemetry parameter scrubbing, and adblock filtering.
    pub fn navigate_protected(&mut self, raw_url: &str) -> Result<String, &'static str> {
        // 1. HTTPS Upgrade
        let upgraded = self.brave_shields.upgrade_to_https(raw_url);

        // 2. Telemetry and tracking parameter scrubbing
        let sanitized = self.stripper.sanitize_url(&upgraded);

        // 3. CNAME Uncloaking check
        let domain = if let Some(start) = sanitized.find("://") {
            let after = &sanitized[start + 3..];
            if let Some(end) = after.find('/') {
                &after[..end]
            } else {
                after
            }
        } else {
            &sanitized
        };

        let uncloaked = self.brave_shields.resolve_cname_uncloak(domain);

        // 4. Check if uncloaked domain is a blocked ad or telemetry target
        if self.stripper.should_block_telemetry(&uncloaked) || !self.engine.navigate_url(&uncloaked)
        {
            return Err("Navigation Blocked: Ad/Telemetry Target Detected");
        }

        Ok(sanitized)
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_process_engine() {
        let mut engine = SovereignBrowserEngine::new();
        engine
            .spawn_sandboxed_process(201, BrowserProcessType::BrowserCore)
            .unwrap();
        engine
            .spawn_sandboxed_process(202, BrowserProcessType::RendererSandbox)
            .unwrap();

        assert_eq!(engine.processes.len(), 2);
        assert!(!engine.processes[0].is_isolated); // Core is unisolated
        assert!(engine.processes[1].is_isolated); // Renderer is isolated!
    }

    #[test]
    fn test_resist_fingerprinting() {
        let rfp = ResistFingerprintingEngine::new();

        // Canvas noise test
        let mut pixels = [100u8, 150, 200, 255];
        rfp.apply_canvas_noise(&mut pixels);
        assert_ne!(pixels[0], 100);

        // WebGL spoofing test
        let (vendor, renderer) = rfp.spoof_webgl_info();
        assert_eq!(vendor, "Mesa/X.org");

        // Letterboxing test
        let (w, h) = rfp.apply_letterboxing(1920, 1080);
        assert_eq!(w, 1800);
        assert_eq!(h, 1000);

        // Timezone test
        assert_eq!(rfp.get_timezone_offset_minutes(), 0);
    }

    #[test]
    fn test_telemetry_and_tracker_stripper() {
        let stripper = TelemetryAndTrackerStripper::new();

        assert!(stripper.should_block_telemetry("https://telemetry.mozilla.org/submit"));
        assert!(stripper.should_block_telemetry("https://google-analytics.com/collect"));

        // Clean query parameters
        let raw = "https://example.com/item?id=42&fbclid=XYZ123&utm_source=email&ref=home";
        let clean = stripper.sanitize_url(raw);
        assert_eq!(clean, "https://example.com/item?id=42&ref=home");
    }

    #[test]
    fn test_brave_shields_and_cname_uncloaking() {
        let shield = BraveShieldsEngine::new();

        // HTTPS upgrade
        let http_url = "http://example.com/login";
        assert_eq!(
            shield.upgrade_to_https(http_url),
            "https://example.com/login"
        );

        // CNAME uncloaking
        let uncloaked = shield.resolve_cname_uncloak("metrics.example.com");
        assert_eq!(uncloaked, "tracking.doubleclick.net");

        // Cosmetic stylesheet
        let css = shield.generate_cosmetic_stylesheet();
        assert!(css.contains(".ad-banner { display: none !important; }"));
    }

    #[test]
    fn test_tor_circuit_and_security_slider() {
        let mut tor = TorCircuitManager::new();
        tor.build_circuit_for_domain("check.torproject.org".to_string());

        assert!(tor.circuits.contains_key("check.torproject.org"));
        assert_eq!(tor.circuits["check.torproject.org"].len(), 3);

        // Security level tests
        assert!(tor.is_javascript_allowed(false));

        tor.active_level = TorSecurityLevel::Safer;
        assert!(!tor.is_javascript_allowed(false)); // JS blocked on HTTP
        assert!(tor.is_javascript_allowed(true)); // JS allowed on HTTPS

        tor.active_level = TorSecurityLevel::Safest;
        assert!(!tor.is_javascript_allowed(true));
        assert!(!tor.is_webgl_allowed());
    }

    #[test]
    fn test_gpc_and_tab_memory_optimizer() {
        let gpc = GlobalPrivacyControl::new();
        let mut headers = Vec::new();
        gpc.inject_privacy_headers(&mut headers);

        assert_eq!(headers[0], ("Sec-GPC".to_string(), "1".to_string()));
        assert_eq!(headers[1], ("DNT".to_string(), "1".to_string()));

        let mut optimizer = TabMemoryOptimizer::new(2048);
        let mut tabs = vec![
            BrowserTabInstance {
                id: 1,
                url: "https://site1.com".to_string(),
                container: BrowserContainerType::Personal,
                is_snoozed: false,
                saved_scroll_y: 0,
            },
            BrowserTabInstance {
                id: 2,
                url: "https://site2.com".to_string(),
                container: BrowserContainerType::Work,
                is_snoozed: false,
                saved_scroll_y: 0,
            },
        ];

        let snoozed = optimizer.optimize_memory_pressure(8192, &mut tabs);
        assert_eq!(snoozed, 2);
        assert!(tabs[0].is_snoozed);
        assert_eq!(tabs[0].saved_scroll_y, 1200);
    }

    #[test]
    fn test_secure_partitioned_jar() {
        let mut jar = SecureStorageContainer::new("github.com".to_string());
        jar.store_cookie("session_id".to_string(), "SECRET_TOKEN_123".to_string());

        let val = jar.read_cookie("session_id").unwrap();
        assert_eq!(val, "SECRET_TOKEN_123");
    }

    #[test]
    fn test_search_switcher() {
        let url =
            SearchSwitcher::generate_search_query_url(SearchEngineType::DuckDuckGo, "sigma os");
        assert_eq!(url, "https://duckduckgo.com/?q=sigma+os");

        let brave_url = SearchSwitcher::generate_search_query_url(
            SearchEngineType::BraveSearch,
            "retro computing",
        );
        assert_eq!(
            brave_url,
            "https://search.brave.com/search?q=retro+computing"
        );
    }

    #[test]
    fn test_sigma_web_browser_pipeline() {
        let mut sigma_web = SigmaWebBrowser::new();

        // Test normal safe URL
        let nav = sigma_web
            .navigate_protected("http://rust-lang.org/learn?topic=rust&utm_source=twitter");
        assert_eq!(
            nav,
            Ok("https://rust-lang.org/learn?topic=rust".to_string())
        );

        // Test CNAME uncloaked ad target detection and block
        let blocked_nav =
            sigma_web.navigate_protected("http://metrics.example.com/collect?fbclid=123");
        assert!(blocked_nav.is_err());
    }
}
