// SigmaOS System Service Innovations Engine
// Inspired by Phoronix, ItsFOSS, 9to5Linux, Geeky-Gadgets, HWBusters, and InfoWorld
// Zero-dependency, #![no_std] compliant native Rust implementations

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. LINUX NEWS & PRESS TECH FEEDS AGGREGATOR SERVICE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechNewsFeedItem {
    pub portal_name: String,
    pub article_title: String,
    pub category: String,
    pub url: String,
}

pub struct LinuxNewsPressTechFeedsEngine {
    pub feed_items: Vec<TechNewsFeedItem>,
}

impl LinuxNewsPressTechFeedsEngine {
    pub fn new() -> Self {
        Self {
            feed_items: Vec::new(),
        }
    }

    pub fn add_article(&mut self, portal: &str, title: &str, category: &str, url: &str) {
        self.feed_items.push(TechNewsFeedItem {
            portal_name: portal.to_string(),
            article_title: title.to_string(),
            category: category.to_string(),
            url: url.to_string(),
        });
    }

    pub fn get_articles_by_portal(&self, portal: &str) -> Vec<&TechNewsFeedItem> {
        self.feed_items
            .iter()
            .filter(|i| i.portal_name.eq_ignore_ascii_case(portal))
            .collect()
    }
}

impl Default for LinuxNewsPressTechFeedsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. HARDWARE POWER & TELEMETRY HEALTH DIAGNOSTIC SERVICE (HWBusters)
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct PowerTelemetryReading {
    pub sensor_id: String,
    pub voltage_v: f32,
    pub current_a: f32,
    pub temp_celsius: f32,
}

pub struct SystemHealthDiagnosticService {
    pub max_allowed_temp_c: f32,
    pub readings: BTreeMap<String, PowerTelemetryReading>,
}

impl SystemHealthDiagnosticService {
    pub fn new(max_temp: f32) -> Self {
        Self {
            max_allowed_temp_c: max_temp,
            readings: BTreeMap::new(),
        }
    }

    pub fn update_sensor(&mut self, reading: PowerTelemetryReading) {
        self.readings.insert(reading.sensor_id.clone(), reading);
    }

    /// Validates system health and flags thermal throttling risks
    pub fn audit_thermal_health(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        for (id, reading) in &self.readings {
            if reading.temp_celsius > self.max_allowed_temp_c {
                warnings.push(format!(
                    "THERMAL WARNING: Sensor {} exceeds limit at {:.1}°C",
                    id, reading.temp_celsius
                ));
            }
        }
        warnings
    }
}

// =========================================================================
// 3. TECH MEDIA BENCHMARK COMPARATOR SERVICE (Phoronix)
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkScore {
    pub suite_name: String,
    pub score: f64,
    pub unit: String,
}

pub struct TechMediaBenchmarkAggregator {
    pub suite_results: BTreeMap<String, BenchmarkScore>,
}

impl TechMediaBenchmarkAggregator {
    pub fn new() -> Self {
        Self {
            suite_results: BTreeMap::new(),
        }
    }

    pub fn record_score(&mut self, suite: &str, score: f64, unit: &str) {
        self.suite_results.insert(
            suite.to_string(),
            BenchmarkScore {
                suite_name: suite.to_string(),
                score,
                unit: unit.to_string(),
            },
        );
    }

    pub fn compare_score(&self, suite: &str, baseline: f64) -> Option<f64> {
        self.suite_results
            .get(suite)
            .map(|res| ((res.score - baseline) / baseline) * 100.0)
    }
}

impl Default for TechMediaBenchmarkAggregator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. ZERO-TRUST SERVICE HARDENING SERVICE (InfoWorld / LinuxFoundation)
// =========================================================================

pub struct ZeroTrustServiceHardeningEngine {
    pub allowed_capabilities: Vec<String>,
    pub sandbox_active: bool,
}

impl ZeroTrustServiceHardeningEngine {
    pub fn new() -> Self {
        Self {
            allowed_capabilities: Vec::new(),
            sandbox_active: false,
        }
    }

    pub fn grant_capability(&mut self, cap: &str) {
        if !self.allowed_capabilities.contains(&cap.to_string()) {
            self.allowed_capabilities.push(cap.to_string());
        }
    }

    pub fn lock_service_sandbox(&mut self) {
        self.sandbox_active = true;
    }

    pub fn authorize_action(&self, cap: &str) -> bool {
        if !self.sandbox_active {
            return true;
        }
        self.allowed_capabilities.contains(&cap.to_string())
    }
}

impl Default for ZeroTrustServiceHardeningEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_news_feed_engine() {
        let mut news = LinuxNewsPressTechFeedsEngine::new();
        news.add_article(
            "Phoronix",
            "Linux 6.12 Benchmarks Released",
            "Kernel",
            "https://phoronix.com/6.12",
        );
        news.add_article(
            "ItsFOSS",
            "Top 10 Terminal Emulators",
            "Software",
            "https://itsfoss.com/terminals",
        );

        let phoronix_news = news.get_articles_by_portal("Phoronix");
        assert_eq!(phoronix_news.len(), 1);
        assert_eq!(phoronix_news[0].article_title, "Linux 6.12 Benchmarks Released");
    }

    #[test]
    fn test_system_health_diagnostic_service() {
        let mut health = SystemHealthDiagnosticService::new(85.0);
        health.update_sensor(PowerTelemetryReading {
            sensor_id: "CPU_CORE_0".to_string(),
            voltage_v: 1.2,
            current_a: 45.0,
            temp_celsius: 90.5,
        });

        let warnings = health.audit_thermal_health();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("THERMAL WARNING"));
    }

    #[test]
    fn test_benchmark_aggregator() {
        let mut bench = TechMediaBenchmarkAggregator::new();
        bench.record_score("KernelBuild", 120.0, "seconds");

        // Lower is better or percentage improvement
        let delta = bench.compare_score("KernelBuild", 100.0).unwrap();
        assert_eq!(delta, 20.0);
    }

    #[test]
    fn test_zero_trust_hardening() {
        let mut zt = ZeroTrustServiceHardeningEngine::new();
        zt.grant_capability("CAP_NET_BIND_SERVICE");
        zt.lock_service_sandbox();

        assert!(zt.authorize_action("CAP_NET_BIND_SERVICE"));
        assert!(!zt.authorize_action("CAP_SYS_ADMIN"));
    }
}
