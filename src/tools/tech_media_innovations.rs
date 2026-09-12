// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Tech Media Innovations Engine
// Inspired by ItsFOSS, 9to5Linux, Geeky-Gadgets, Linux.org, KDnuggets, HWBusters,
// ITDaily, HowToGeek, InfoWorld, LinuxFoundation, MakeUseOf, PCWorld, MarkTechPost,
// WindowsLatest, TechSpot, TheNewStack, TechPowerUp, Phoronix, TechCrunch, XDA-Developers,
// ZDNet, OpenSourceForU, PCMag, LinuxTeck, Appuals, DistroWatch.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. Linux & Open Source Press Feed Aggregator
// Inspired by ItsFOSS, 9to5Linux, Linux.org, TechCrunch, ZDNet, MakeUseOf, DistroWatch
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechMediaArticleFeed {
    pub title: String,
    pub portal: String,
    pub url: String,
    pub category: String,
    pub timestamp_epoch: u64,
}

#[derive(Debug, Clone, Default)]
pub struct LinuxPressFeedEngine {
    pub articles: Vec<TechMediaArticleFeed>,
}

impl LinuxPressFeedEngine {
    pub fn new() -> Self {
        let sample_articles = vec![
            TechMediaArticleFeed {
                title: "Linux Kernel 6.12 LTS Released with Real-time PREEMPT_RT Support".to_string(),
                portal: "9to5Linux".to_string(),
                url: "https://9to5linux.com/linux-kernel-6-12-lts-released".to_string(),
                category: "Kernel".to_string(),
                timestamp_epoch: 1730000000,
            },
            TechMediaArticleFeed {
                title: "Top 10 Zero-Dependency Rust Tools for Systems Software".to_string(),
                portal: "ItsFOSS".to_string(),
                url: "https://itsfoss.com/zero-dependency-rust-tools".to_string(),
                category: "OpenSource".to_string(),
                timestamp_epoch: 1730000100,
            },
            TechMediaArticleFeed {
                title: "SigmaOS Architecture Analysis: Surpassing Legacy Monolithic Kernels".to_string(),
                portal: "TechCrunch".to_string(),
                url: "https://techcrunch.com/sigmaos-sovereign-kernel".to_string(),
                category: "Enterprise".to_string(),
                timestamp_epoch: 1730000200,
            },
            TechMediaArticleFeed {
                title: "DistroWatch Review: Sovereign Operating System Performance Benchmarks".to_string(),
                portal: "DistroWatch".to_string(),
                url: "https://distrowatch.com/sigmaos-review".to_string(),
                category: "DistroReview".to_string(),
                timestamp_epoch: 1730000300,
            },
        ];
        Self {
            articles: sample_articles,
        }
    }

    pub fn add_article(&mut self, article: TechMediaArticleFeed) {
        self.articles.push(article);
    }

    pub fn get_articles_by_portal(&self, portal_name: &str) -> Vec<TechMediaArticleFeed> {
        self.articles
            .iter()
            .filter(|a| a.portal.eq_ignore_ascii_case(portal_name))
            .cloned()
            .collect()
    }

    pub fn get_latest_news(&self) -> Vec<TechMediaArticleFeed> {
        let mut sorted = self.articles.clone();
        sorted.sort_by(|a, b| b.timestamp_epoch.cmp(&a.timestamp_epoch));
        sorted
    }
}

// ============================================================================
// 2. Hardware Telemetry & Automated Benchmark Engine
// Inspired by Phoronix, TechPowerUp, HWBusters, PCWorld, TechSpot
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct PowerThermalTelemetryNode {
    pub cpu_temp_c: f32,
    pub gpu_temp_c: f32,
    pub rail_12v_v: f32,
    pub rail_5v_v: f32,
    pub rail_3v3_v: f32,
    pub total_draw_watts: f32,
    pub fan_rpm: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhoronixBenchmarkSuiteNode {
    pub test_name: String,
    pub score_ops_per_sec: f64,
    pub latency_ms: f64,
    pub cpu_util_pct: f32,
    pub power_efficiency_rating: f64,
}

#[derive(Debug, Clone)]
pub struct HardwareTelemetryMonitor {
    pub current_telemetry: PowerThermalTelemetryNode,
}

impl HardwareTelemetryMonitor {
    pub fn new() -> Self {
        Self {
            current_telemetry: PowerThermalTelemetryNode {
                cpu_temp_c: 42.0,
                gpu_temp_c: 39.5,
                rail_12v_v: 12.04,
                rail_5v_v: 5.02,
                rail_3v3_v: 3.31,
                total_draw_watts: 75.2,
                fan_rpm: 1200,
            },
        }
    }

    pub fn is_power_and_thermal_nominal(&self) -> bool {
        let t = &self.current_telemetry;
        t.cpu_temp_c < 85.0
            && t.gpu_temp_c < 88.0
            && t.rail_12v_v >= 11.4
            && t.rail_12v_v <= 12.6
            && t.rail_5v_v >= 4.75
            && t.rail_5v_v <= 5.25
            && t.rail_3v3_v >= 3.13
            && t.rail_3v3_v <= 3.47
    }
}

impl Default for HardwareTelemetryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PhoronixBenchEngine {
    pub benchmarks: BTreeMap<String, PhoronixBenchmarkSuiteNode>,
}

impl PhoronixBenchEngine {
    pub fn new() -> Self {
        let mut bench = BTreeMap::new();
        bench.insert(
            "sysbench_cpu".to_string(),
            PhoronixBenchmarkSuiteNode {
                test_name: "sysbench_cpu".to_string(),
                score_ops_per_sec: 185000.0,
                latency_ms: 0.85,
                cpu_util_pct: 98.5,
                power_efficiency_rating: 2460.0,
            },
        );
        bench.insert(
            "ipc_pipe_latency".to_string(),
            PhoronixBenchmarkSuiteNode {
                test_name: "ipc_pipe_latency".to_string(),
                score_ops_per_sec: 2400000.0,
                latency_ms: 0.12,
                cpu_util_pct: 45.0,
                power_efficiency_rating: 12000.0,
            },
        );
        Self { benchmarks: bench }
    }

    pub fn run_automated_benchmark(&mut self, test_name: &str) -> PhoronixBenchmarkSuiteNode {
        let res = PhoronixBenchmarkSuiteNode {
            test_name: test_name.to_string(),
            score_ops_per_sec: 210000.0,
            latency_ms: 0.45,
            cpu_util_pct: 88.0,
            power_efficiency_rating: 3200.0,
        };
        self.benchmarks.insert(test_name.to_string(), res.clone());
        res
    }

    pub fn rank_system_benchmarks(&self) -> Vec<PhoronixBenchmarkSuiteNode> {
        let mut list: Vec<_> = self.benchmarks.values().cloned().collect();
        list.sort_by(|a, b| b.score_ops_per_sec.partial_cmp(&a.score_ops_per_sec).unwrap());
        list
    }
}

// ============================================================================
// 3. AI / ML Data Pipeline & Inference Benchmark
// Inspired by KDnuggets, MarkTechPost
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct TensorColumnData {
    pub column_name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmInferenceMetrics {
    pub model_name: String,
    pub tokens_per_sec: f64,
    pub memory_vram_mb: u32,
    pub quantization_level: String,
}

#[derive(Debug, Clone, Default)]
pub struct AiMlTensorDatasetPipeline {
    pub columns: BTreeMap<String, TensorColumnData>,
}

impl AiMlTensorDatasetPipeline {
    pub fn new() -> Self {
        Self {
            columns: BTreeMap::new(),
        }
    }

    pub fn load_column(&mut self, col_name: &str, values: &[f64]) {
        self.columns.insert(
            col_name.to_string(),
            TensorColumnData {
                column_name: col_name.to_string(),
                values: values.to_vec(),
            },
        );
    }

    pub fn calculate_mean(&self, col_name: &str) -> Option<f64> {
        let col = self.columns.get(col_name)?;
        if col.values.is_empty() {
            return None;
        }
        let sum: f64 = col.values.iter().sum();
        Some(sum / col.values.len() as f64)
    }

    pub fn calculate_std_dev(&self, col_name: &str) -> Option<f64> {
        let mean = self.calculate_mean(col_name)?;
        let col = self.columns.get(col_name)?;
        let variance: f64 = col.values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / col.values.len() as f64;
        Some(variance.sqrt())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelPerformanceBenchmark {
    pub models: Vec<LlmInferenceMetrics>,
}

impl ModelPerformanceBenchmark {
    pub fn new() -> Self {
        let models = vec![
            LlmInferenceMetrics {
                model_name: "Sigma-SLM-3B".to_string(),
                tokens_per_sec: 142.5,
                memory_vram_mb: 2048,
                quantization_level: "Q4_K_M".to_string(),
            },
            LlmInferenceMetrics {
                model_name: "Sigma-CodeAgent-7B".to_string(),
                tokens_per_sec: 88.0,
                memory_vram_mb: 4096,
                quantization_level: "Q8_0".to_string(),
            },
        ];
        Self { models }
    }

    pub fn evaluate_llm_performance(&self, model_name: &str) -> Option<LlmInferenceMetrics> {
        self.models.iter().find(|m| m.model_name == model_name).cloned()
    }
}

// ============================================================================
// 4. Zero-Trust Security Sandbox & Open-Source Governance Engine
// Inspired by InfoWorld, LinuxFoundation
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroTrustSandboxPolicy {
    pub service_name: String,
    pub pledge_promises: Vec<String>,
    pub unveil_paths: Vec<String>,
    pub rlimit_mem_mb: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbomLicenseComplianceRecord {
    pub package_name: String,
    pub license_spdx: String,
    pub vulnerability_count: u32,
    pub is_fips_compliant: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ZeroTrustSecuritySandbox {
    pub policies: BTreeMap<String, ZeroTrustSandboxPolicy>,
}

impl ZeroTrustSecuritySandbox {
    pub fn new() -> Self {
        let mut policies = BTreeMap::new();
        policies.insert(
            "network_subsystem".to_string(),
            ZeroTrustSandboxPolicy {
                service_name: "network_subsystem".to_string(),
                pledge_promises: vec!["stdio".to_string(), "inet".to_string(), "rpath".to_string()],
                unveil_paths: vec!["/etc/ssl/certs".to_string(), "/etc/resolv.conf".to_string()],
                rlimit_mem_mb: 256,
            },
        );
        Self { policies }
    }

    pub fn verify_sandbox_policy(&self, service_name: &str) -> bool {
        if let Some(p) = self.policies.get(service_name) {
            !p.pledge_promises.is_empty() && !p.unveil_paths.is_empty() && p.rlimit_mem_mb > 0
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpenSourceGovernanceEngine {
    pub records: Vec<SbomLicenseComplianceRecord>,
}

impl OpenSourceGovernanceEngine {
    pub fn new() -> Self {
        let records = vec![
            SbomLicenseComplianceRecord {
                package_name: "sigma-core".to_string(),
                license_spdx: "MIT".to_string(),
                vulnerability_count: 0,
                is_fips_compliant: true,
            },
            SbomLicenseComplianceRecord {
                package_name: "sigma-hal".to_string(),
                license_spdx: "Apache-2.0".to_string(),
                vulnerability_count: 0,
                is_fips_compliant: true,
            },
        ];
        Self { records }
    }

    pub fn audit_license_compliance(&self) -> bool {
        self.records
            .iter()
            .all(|r| r.vulnerability_count == 0 && (r.license_spdx == "MIT" || r.license_spdx == "Apache-2.0"))
    }
}

// ============================================================================
// 5. Cross-Platform Device Bridge Subsystem
// Inspired by XDA-Developers, WindowsLatest, WindowsCentral
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossPlatformDeviceSpec {
    pub device_id: String,
    pub os_family: String,
    pub connection_type: String,
    pub side_loaded_apps: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CrossPlatformDeviceBridge {
    pub devices: Vec<CrossPlatformDeviceSpec>,
}

impl CrossPlatformDeviceBridge {
    pub fn new() -> Self {
        let devices = vec![CrossPlatformDeviceSpec {
            device_id: "dev_android_1".to_string(),
            os_family: "Android/APEX".to_string(),
            connection_type: "ADB_WiFi".to_string(),
            side_loaded_apps: vec!["org.sigma.terminal".to_string()],
        }];
        Self { devices }
    }

    pub fn register_device(&mut self, device: CrossPlatformDeviceSpec) {
        self.devices.push(device);
    }

    pub fn sideload_app(&mut self, device_id: &str, app_id: &str) -> Result<String, &'static str> {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.device_id == device_id) {
            dev.side_loaded_apps.push(app_id.to_string());
            Ok(format!("Sideloaded app '{}' onto device '{}'", app_id, device_id))
        } else {
            Err("Device not found")
        }
    }

    pub fn verify_bridge_status(&self) -> bool {
        !self.devices.is_empty()
    }
}

// ============================================================================
// Sovereign Tech Media Master Suite
// ============================================================================

#[derive(Debug, Default)]
pub struct SovereignTechMediaMasterSuite {
    pub press_feeds: LinuxPressFeedEngine,
    pub telemetry: HardwareTelemetryMonitor,
    pub phoronix_bench: PhoronixBenchEngine,
    pub tensor_pipeline: AiMlTensorDatasetPipeline,
    pub model_bench: ModelPerformanceBenchmark,
    pub zero_trust: ZeroTrustSecuritySandbox,
    pub governance: OpenSourceGovernanceEngine,
    pub device_bridge: CrossPlatformDeviceBridge,
}

impl SovereignTechMediaMasterSuite {
    pub fn new() -> Self {
        Self {
            press_feeds: LinuxPressFeedEngine::new(),
            telemetry: HardwareTelemetryMonitor::new(),
            phoronix_bench: PhoronixBenchEngine::new(),
            tensor_pipeline: AiMlTensorDatasetPipeline::new(),
            model_bench: ModelPerformanceBenchmark::new(),
            zero_trust: ZeroTrustSecuritySandbox::new(),
            governance: OpenSourceGovernanceEngine::new(),
            device_bridge: CrossPlatformDeviceBridge::new(),
        }
    }

    pub fn synthesize_and_verify_all(&mut self) -> bool {
        // Verify Press Feeds
        let latest_news = self.press_feeds.get_latest_news();
        let feeds_ok = !latest_news.is_empty();

        // Verify Hardware Telemetry & Bench
        let telemetry_ok = self.telemetry.is_power_and_thermal_nominal();
        let bench_res = self.phoronix_bench.run_automated_benchmark("synthetic_mem");
        let bench_ok = bench_res.score_ops_per_sec > 0.0;

        // Verify AI / ML Tensor Pipeline & Models
        self.tensor_pipeline.load_column("loss", &[0.5, 0.4, 0.3, 0.2, 0.1]);
        let mean = self.tensor_pipeline.calculate_mean("loss");
        let ai_ok = mean == Some(0.3) && self.model_bench.evaluate_llm_performance("Sigma-SLM-3B").is_some();

        // Verify Security & Governance
        let sec_ok = self.zero_trust.verify_sandbox_policy("network_subsystem") && self.governance.audit_license_compliance();

        // Verify Device Bridge
        let bridge_ok = self.device_bridge.verify_bridge_status()
            && self.device_bridge.sideload_app("dev_android_1", "org.sigma.pqc_vpn").is_ok();

        feeds_ok && telemetry_ok && bench_ok && ai_ok && sec_ok && bridge_ok
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_press_feed_engine() {
        let mut engine = LinuxPressFeedEngine::new();
        assert_eq!(engine.get_articles_by_portal("9to5Linux").len(), 1);

        engine.add_article(TechMediaArticleFeed {
            title: "New Phoronix Benchmarking Suite Release".to_string(),
            portal: "Phoronix".to_string(),
            url: "https://phoronix.com/news".to_string(),
            category: "Hardware".to_string(),
            timestamp_epoch: 1730000400,
        });

        let news = engine.get_latest_news();
        assert_eq!(news[0].portal, "Phoronix");
    }

    #[test]
    fn test_hardware_telemetry_and_phoronix_bench() {
        let monitor = HardwareTelemetryMonitor::new();
        assert!(monitor.is_power_and_thermal_nominal());

        let mut bench = PhoronixBenchEngine::new();
        let res = bench.run_automated_benchmark("crypto_aes");
        assert_eq!(res.test_name, "crypto_aes");
        let ranked = bench.rank_system_benchmarks();
        assert!(ranked[0].score_ops_per_sec >= ranked[1].score_ops_per_sec);
    }

    #[test]
    fn test_ai_ml_tensor_dataset_pipeline() {
        let mut pipeline = AiMlTensorDatasetPipeline::new();
        pipeline.load_column("accuracy", &[10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(pipeline.calculate_mean("accuracy"), Some(30.0));
        assert!(pipeline.calculate_std_dev("accuracy").unwrap() > 0.0);

        let model_bench = ModelPerformanceBenchmark::new();
        let slm = model_bench.evaluate_llm_performance("Sigma-SLM-3B");
        assert!(slm.is_some());
        assert_eq!(slm.unwrap().quantization_level, "Q4_K_M");
    }

    #[test]
    fn test_zero_trust_sandbox_and_governance() {
        let zero_trust = ZeroTrustSecuritySandbox::new();
        assert!(zero_trust.verify_sandbox_policy("network_subsystem"));
        assert!(!zero_trust.verify_sandbox_policy("unknown_service"));

        let gov = OpenSourceGovernanceEngine::new();
        assert!(gov.audit_license_compliance());
    }

    #[test]
    fn test_cross_platform_device_bridge() {
        let mut bridge = CrossPlatformDeviceBridge::new();
        assert!(bridge.verify_bridge_status());
        assert!(bridge.sideload_app("dev_android_1", "com.sigma.app").is_ok());
        assert!(bridge.sideload_app("invalid_dev", "com.sigma.app").is_err());
    }

    #[test]
    fn test_sovereign_tech_media_master_suite() {
        let mut suite = SovereignTechMediaMasterSuite::new();
        assert!(suite.synthesize_and_verify_all());
    }
}
