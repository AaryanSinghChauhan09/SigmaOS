// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Tech Media Innovations Engine
// Inspired by ItsFOSS, 9to5Linux, Geeky-Gadgets, Linux.com, KDnuggets, HWBusters,
// ITDaily, HowToGeek, Linux.org, InfoWorld, LinuxFoundation, MakeUseOf, PCWorld, MarkTechPost,
// WindowsLatest, TechSpot, TheNewStack, TechPowerUp, WindowsCentral, Phoronix, TechCrunch,
// XDA-Developers, ZDNet, OpenSourceForU, PCMag, LinuxTeck, Appuals, DistroWatch.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. Linux & Open Source Press Feed Aggregator
// Inspired by ItsFOSS, 9to5Linux, Geeky-Gadgets, Linux.com, KDnuggets, HWBusters,
// ITDaily, HowToGeek, Linux.org, InfoWorld, LinuxFoundation, MakeUseOf, PCWorld,
// MarkTechPost, WindowsLatest, TechSpot, TheNewStack, TechPowerUp, WindowsCentral,
// Phoronix, TechCrunch, XDA-Developers, ZDNet, OpenSourceForU, PCMag, LinuxTeck,
// Appuals, DistroWatch
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechMediaArticleFeed {
    pub title: String,
    pub portal: String,
    pub domain: String,
    pub url: String,
    pub category: String,
    pub timestamp_epoch: u64,
}

#[derive(Debug, Clone)]
pub struct LinuxPressFeedEngine {
    pub articles: Vec<TechMediaArticleFeed>,
}

impl LinuxPressFeedEngine {
    pub fn new() -> Self {
        let sample_articles = vec![
            TechMediaArticleFeed {
                title: "Linux Kernel 6.12 LTS Released with Real-time PREEMPT_RT Support".to_string(),
                portal: "9to5Linux".to_string(),
                domain: "9to5linux.com".to_string(),
                url: "https://9to5linux.com/linux-kernel-6-12-lts-released".to_string(),
                category: "Kernel".to_string(),
                timestamp_epoch: 1730000000,
            },
            TechMediaArticleFeed {
                title: "Top 10 Zero-Dependency Rust Tools for Systems Software".to_string(),
                portal: "ItsFOSS".to_string(),
                domain: "itsfoss.com".to_string(),
                url: "https://itsfoss.com/zero-dependency-rust-tools".to_string(),
                category: "OpenSource".to_string(),
                timestamp_epoch: 1730000100,
            },
            TechMediaArticleFeed {
                title: "SigmaOS Architecture Analysis: Surpassing Legacy Monolithic Kernels".to_string(),
                portal: "TechCrunch".to_string(),
                domain: "techcrunch.com".to_string(),
                url: "https://techcrunch.com/sigmaos-sovereign-kernel".to_string(),
                category: "Enterprise".to_string(),
                timestamp_epoch: 1730000200,
            },
            TechMediaArticleFeed {
                title: "DistroWatch Review: Sovereign Operating System Performance Benchmarks".to_string(),
                portal: "DistroWatch".to_string(),
                domain: "distrowatch.com".to_string(),
                url: "https://distrowatch.com/sigmaos-review".to_string(),
                category: "DistroReview".to_string(),
                timestamp_epoch: 1730000300,
            },
            TechMediaArticleFeed {
                title: "DIY Microcontroller Prototyping and Hardware Hacks".to_string(),
                portal: "Geeky-Gadgets".to_string(),
                domain: "geeky-gadgets.com".to_string(),
                url: "https://geeky-gadgets.com/diy-microcontroller-hacks".to_string(),
                category: "Hardware".to_string(),
                timestamp_epoch: 1730000400,
            },
            TechMediaArticleFeed {
                title: "Enterprise IT Uptime and Cloud Infrastructure Benchmark Report".to_string(),
                portal: "ITDaily".to_string(),
                domain: "itdaily.com".to_string(),
                url: "https://itdaily.com/enterprise-cloud-uptime-report".to_string(),
                category: "Enterprise".to_string(),
                timestamp_epoch: 1730000500,
            },
            TechMediaArticleFeed {
                title: "How to Optimize Linux Kernel Parameters for Low Latency".to_string(),
                portal: "HowToGeek".to_string(),
                domain: "howtogeek.com".to_string(),
                url: "https://howtogeek.com/optimize-linux-kernel-low-latency".to_string(),
                category: "Guides".to_string(),
                timestamp_epoch: 1730000600,
            },
            TechMediaArticleFeed {
                title: "WebAssembly Runtimes in Modern Cloud Native Architectures".to_string(),
                portal: "TheNewStack".to_string(),
                domain: "thenewstack.io".to_string(),
                url: "https://thenewstack.io/wasm-cloud-native-architectures".to_string(),
                category: "CloudNative".to_string(),
                timestamp_epoch: 1730000700,
            },
            TechMediaArticleFeed {
                title: "Linux Foundation Announces New Sovereign OS Working Group".to_string(),
                portal: "Linux.com".to_string(),
                domain: "linux.com".to_string(),
                url: "https://linux.com/news/sovereign-os-working-group".to_string(),
                category: "Community".to_string(),
                timestamp_epoch: 1730000800,
            },
            TechMediaArticleFeed {
                title: "Fixing Linux Boot and Systemd Initialization Failures".to_string(),
                portal: "Appuals".to_string(),
                domain: "appuals.com".to_string(),
                url: "https://appuals.com/fix-linux-boot-failures".to_string(),
                category: "Troubleshooting".to_string(),
                timestamp_epoch: 1730000900,
            },
            TechMediaArticleFeed {
                title: "Power Supply Voltage Stability and Thermal Ratings Explained".to_string(),
                portal: "HWBusters".to_string(),
                domain: "hwbusters.com".to_string(),
                url: "https://hwbusters.com/psu-voltage-stability".to_string(),
                category: "Hardware".to_string(),
                timestamp_epoch: 1730001000,
            },
            TechMediaArticleFeed {
                title: "Deep Learning Model Quantization Techniques in 2025".to_string(),
                portal: "KDnuggets".to_string(),
                domain: "kdnuggets.com".to_string(),
                url: "https://kdnuggets.com/deep-learning-quantization-2025".to_string(),
                category: "AI".to_string(),
                timestamp_epoch: 1730001100,
            },
            TechMediaArticleFeed {
                title: "Zero-Trust Security Policies for Modern Linux Kernels".to_string(),
                portal: "InfoWorld".to_string(),
                domain: "infoworld.com".to_string(),
                url: "https://infoworld.com/zero-trust-linux-kernels".to_string(),
                category: "Security".to_string(),
                timestamp_epoch: 1730001200,
            },
            TechMediaArticleFeed {
                title: "Linux Foundation Open Source Security Mandates".to_string(),
                portal: "LinuxFoundation".to_string(),
                domain: "linuxfoundation.org".to_string(),
                url: "https://linuxfoundation.org/open-source-security-mandates".to_string(),
                category: "Governance".to_string(),
                timestamp_epoch: 1730001300,
            },
            TechMediaArticleFeed {
                title: "Best Lightweight Linux Desktop Environments for Old PCs".to_string(),
                portal: "MakeUseOf".to_string(),
                domain: "makeuseof.com".to_string(),
                url: "https://makeuseof.com/best-lightweight-linux-desktops".to_string(),
                category: "Desktop".to_string(),
                timestamp_epoch: 1730001400,
            },
            TechMediaArticleFeed {
                title: "Next-Gen CPU Power Delivery & Thermal Throttling Analysis".to_string(),
                portal: "PCWorld".to_string(),
                domain: "pcworld.com".to_string(),
                url: "https://pcworld.com/cpu-thermal-throttling-analysis".to_string(),
                category: "Hardware".to_string(),
                timestamp_epoch: 1730001500,
            },
            TechMediaArticleFeed {
                title: "State-of-the-Art SLM Architectures for Edge Inference".to_string(),
                portal: "MarkTechPost".to_string(),
                domain: "marktechpost.com".to_string(),
                url: "https://marktechpost.com/slm-edge-inference-2025".to_string(),
                category: "AI".to_string(),
                timestamp_epoch: 1730001600,
            },
            TechMediaArticleFeed {
                title: "Cross-Platform Subsystem Compatibility in Modern OS Design".to_string(),
                portal: "WindowsLatest".to_string(),
                domain: "windowslatest.com".to_string(),
                url: "https://windowslatest.com/cross-platform-subsystem-compatibility".to_string(),
                category: "Subsystems".to_string(),
                timestamp_epoch: 1730001700,
            },
            TechMediaArticleFeed {
                title: "GPU Memory Bandwidth Benchmarks: DDR5 vs HBM3e".to_string(),
                portal: "TechSpot".to_string(),
                domain: "techspot.com".to_string(),
                url: "https://techspot.com/gpu-memory-bandwidth-benchmarks".to_string(),
                category: "Hardware".to_string(),
                timestamp_epoch: 1730001800,
            },
            TechMediaArticleFeed {
                title: "GPU Power Draw & Voltage Scaling Breakdown".to_string(),
                portal: "TechPowerUp".to_string(),
                domain: "techpowerup.com".to_string(),
                url: "https://techpowerup.com/gpu-power-draw-scaling".to_string(),
                category: "Hardware".to_string(),
                timestamp_epoch: 1730001900,
            },
            TechMediaArticleFeed {
                title: "Seamless Mobile to Desktop Ecosystem Interoperability".to_string(),
                portal: "WindowsCentral".to_string(),
                domain: "windowscentral.com".to_string(),
                url: "https://windowscentral.com/ecosystem-interoperability".to_string(),
                category: "Ecosystem".to_string(),
                timestamp_epoch: 1730002000,
            },
            TechMediaArticleFeed {
                title: "Linux Phoronix Test Suite 10.8 Performance Results".to_string(),
                portal: "Phoronix".to_string(),
                domain: "phoronix.com".to_string(),
                url: "https://phoronix.com/phoronix-test-suite-results".to_string(),
                category: "Benchmarks".to_string(),
                timestamp_epoch: 1730002100,
            },
            TechMediaArticleFeed {
                title: "Android APEX Modules and Kernel Treble Innovations".to_string(),
                portal: "XDA-Developers".to_string(),
                domain: "xda-developers.com".to_string(),
                url: "https://xda-developers.com/android-apex-kernel-treble".to_string(),
                category: "Mobile".to_string(),
                timestamp_epoch: 1730002200,
            },
            TechMediaArticleFeed {
                title: "Enterprise Linux Server Migration Strategies".to_string(),
                portal: "ZDNet".to_string(),
                domain: "zdnet.com".to_string(),
                url: "https://zdnet.com/enterprise-linux-migration-strategies".to_string(),
                category: "Enterprise".to_string(),
                timestamp_epoch: 1730002300,
            },
            TechMediaArticleFeed {
                title: "Open Source Tooling and Sysadmin Utilities Guide".to_string(),
                portal: "OpenSourceForU".to_string(),
                domain: "opensourceforu.com".to_string(),
                url: "https://opensourceforu.com/open-source-sysadmin-guide".to_string(),
                category: "OpenSource".to_string(),
                timestamp_epoch: 1730002400,
            },
            TechMediaArticleFeed {
                title: "The Best Modern Operating Systems Evaluated".to_string(),
                portal: "PCMag".to_string(),
                domain: "pcmag.com".to_string(),
                url: "https://pcmag.com/best-operating-systems-evaluated".to_string(),
                category: "Reviews".to_string(),
                timestamp_epoch: 1730002500,
            },
            TechMediaArticleFeed {
                title: "Hardening Linux Systems with Firewall and SSH Rules".to_string(),
                portal: "LinuxTeck".to_string(),
                domain: "linuxteck.com".to_string(),
                url: "https://linuxteck.com/hardening-linux-firewall-ssh".to_string(),
                category: "Security".to_string(),
                timestamp_epoch: 1730002600,
            },
            TechMediaArticleFeed {
                title: "Linux Kernel Memory Allocators: Buddy vs Slab vs Sovereign".to_string(),
                portal: "Linux.org".to_string(),
                domain: "linux.org".to_string(),
                url: "https://linux.org/kernel-memory-allocators-comparison".to_string(),
                category: "Kernel".to_string(),
                timestamp_epoch: 1730002700,
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

    pub fn get_articles_by_domain(&self, domain_name: &str) -> Vec<TechMediaArticleFeed> {
        self.articles
            .iter()
            .filter(|a| a.domain.eq_ignore_ascii_case(domain_name))
            .cloned()
            .collect()
    }

    pub fn get_articles_by_category(&self, category: &str) -> Vec<TechMediaArticleFeed> {
        self.articles
            .iter()
            .filter(|a| a.category.eq_ignore_ascii_case(category))
            .cloned()
            .collect()
    }

    pub fn search_articles(&self, query: &str) -> Vec<TechMediaArticleFeed> {
        let q = query.to_lowercase();
        self.articles
            .iter()
            .filter(|a| {
                a.title.to_lowercase().contains(&q)
                    || a.portal.to_lowercase().contains(&q)
                    || a.category.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    pub fn get_latest_news(&self) -> Vec<TechMediaArticleFeed> {
        let mut sorted = self.articles.clone();
        sorted.sort_by(|a, b| b.timestamp_epoch.cmp(&a.timestamp_epoch));
        sorted
    }
}

impl Default for LinuxPressFeedEngine {
    fn default() -> Self {
        Self::new()
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
// 6. Geeky-Gadgets Hardware Prototyping & Peripheral Spec Evaluator
// Inspired by Geeky-Gadgets
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct PeripheralDeviceSpec {
    pub device_name: String,
    pub bus_type: String, // e.g. "I2C", "SPI", "USB-C", "PCIe"
    pub clock_freq_khz: u32,
    pub power_consumption_mw: f32,
    pub max_bandwidth_mbps: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GeekyGadgetsTechReviewEngine {
    pub peripherals: Vec<PeripheralDeviceSpec>,
}

impl GeekyGadgetsTechReviewEngine {
    pub fn new() -> Self {
        let peripherals = vec![
            PeripheralDeviceSpec {
                device_name: "Sigma-OLED-Display-128x64".to_string(),
                bus_type: "I2C".to_string(),
                clock_freq_khz: 400,
                power_consumption_mw: 15.0,
                max_bandwidth_mbps: 0.4,
            },
            PeripheralDeviceSpec {
                device_name: "Sigma-NVMe-Gen5-SSD".to_string(),
                bus_type: "PCIe".to_string(),
                clock_freq_khz: 3200000,
                power_consumption_mw: 6500.0,
                max_bandwidth_mbps: 14000.0,
            },
        ];
        Self { peripherals }
    }

    pub fn register_peripheral(&mut self, spec: PeripheralDeviceSpec) {
        self.peripherals.push(spec);
    }

    pub fn evaluate_bus_efficiency(&self, bus_type: &str) -> Vec<PeripheralDeviceSpec> {
        self.peripherals
            .iter()
            .filter(|p| p.bus_type.eq_ignore_ascii_case(bus_type))
            .cloned()
            .collect()
    }

    pub fn calculate_total_power_mw(&self) -> f32 {
        self.peripherals.iter().map(|p| p.power_consumption_mw).sum()
    }
}

// ============================================================================
// 7. ITDaily Enterprise IT & SLA Uptime Governor
// Inspired by ITDaily
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct EnterpriseServiceSlaRecord {
    pub service_id: String,
    pub target_uptime_pct: f64,
    pub current_uptime_pct: f64,
    pub mean_time_between_failures_hrs: f64,
    pub mean_time_to_recovery_mins: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ItDailyEnterpriseItGovernor {
    pub sla_records: BTreeMap<String, EnterpriseServiceSlaRecord>,
}

impl ItDailyEnterpriseItGovernor {
    pub fn new() -> Self {
        let mut sla_records = BTreeMap::new();
        sla_records.insert(
            "kernel_core".to_string(),
            EnterpriseServiceSlaRecord {
                service_id: "kernel_core".to_string(),
                target_uptime_pct: 99.999,
                current_uptime_pct: 99.9999,
                mean_time_between_failures_hrs: 87600.0,
                mean_time_to_recovery_mins: 0.01,
            },
        );
        sla_records.insert(
            "network_mesh".to_string(),
            EnterpriseServiceSlaRecord {
                service_id: "network_mesh".to_string(),
                target_uptime_pct: 99.99,
                current_uptime_pct: 99.995,
                mean_time_between_failures_hrs: 43800.0,
                mean_time_to_recovery_mins: 0.5,
            },
        );
        Self { sla_records }
    }

    pub fn check_all_slas_met(&self) -> bool {
        self.sla_records.values().all(|r| r.current_uptime_pct >= r.target_uptime_pct)
    }

    pub fn add_service_sla(&mut self, record: EnterpriseServiceSlaRecord) {
        self.sla_records.insert(record.service_id.clone(), record);
    }
}

// ============================================================================
// 8. HowToGeek Guide & Troubleshooting Walkthrough Engine
// Inspired by HowToGeek
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemGuideStep {
    pub step_number: u32,
    pub description: String,
    pub terminal_command: String,
    pub expected_output_keyword: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowToGeekGuide {
    pub guide_id: String,
    pub title: String,
    pub category: String,
    pub steps: Vec<SystemGuideStep>,
}

#[derive(Debug, Clone, Default)]
pub struct HowToGeekGuideSystemEngine {
    pub guides: BTreeMap<String, HowToGeekGuide>,
}

impl HowToGeekGuideSystemEngine {
    pub fn new() -> Self {
        let mut guides = BTreeMap::new();
        guides.insert(
            "low_latency_kernel".to_string(),
            HowToGeekGuide {
                guide_id: "low_latency_kernel".to_string(),
                title: "How to Tune SigmaOS Kernel for Ultra-Low Latency".to_string(),
                category: "Kernel".to_string(),
                steps: vec![
                    SystemGuideStep {
                        step_number: 1,
                        description: "Check current kernel scheduler mode".to_string(),
                        terminal_command: "sigma-ctl sched get".to_string(),
                        expected_output_keyword: "PREEMPT_RT".to_string(),
                    },
                    SystemGuideStep {
                        step_number: 2,
                        description: "Enable CPU governor performance profile".to_string(),
                        terminal_command: "sigma-power set performance".to_string(),
                        expected_output_keyword: "OK".to_string(),
                    },
                ],
            },
        );
        Self { guides }
    }

    pub fn get_guide(&self, guide_id: &str) -> Option<&HowToGeekGuide> {
        self.guides.get(guide_id)
    }

    pub fn register_guide(&mut self, guide: HowToGeekGuide) {
        self.guides.insert(guide.guide_id.clone(), guide);
    }
}

// ============================================================================
// 9. TheNewStack Cloud Native & WebAssembly Engine
// Inspired by TheNewStack
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudNativeWorkloadSpec {
    pub workload_id: String,
    pub runtime: String, // e.g., "Wasm", "OCI", "Containerd"
    pub replica_count: u32,
    pub is_zero_trust_isolated: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TheNewStackCloudNativeEngine {
    pub workloads: Vec<CloudNativeWorkloadSpec>,
}

impl TheNewStackCloudNativeEngine {
    pub fn new() -> Self {
        let workloads = vec![
            CloudNativeWorkloadSpec {
                workload_id: "wasm_microservice_auth".to_string(),
                runtime: "Wasm".to_string(),
                replica_count: 5,
                is_zero_trust_isolated: true,
            },
            CloudNativeWorkloadSpec {
                workload_id: "oci_container_api".to_string(),
                runtime: "OCI".to_string(),
                replica_count: 3,
                is_zero_trust_isolated: true,
            },
        ];
        Self { workloads }
    }

    pub fn deploy_workload(&mut self, workload: CloudNativeWorkloadSpec) {
        self.workloads.push(workload);
    }

    pub fn verify_wasm_workloads(&self) -> bool {
        self.workloads
            .iter()
            .filter(|w| w.runtime == "Wasm")
            .all(|w| w.is_zero_trust_isolated)
    }
}

// ============================================================================
// 10. Linux.com & LinuxFoundation Open Source Community Engine
// Inspired by Linux.com, LinuxFoundation
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxCommunityEvent {
    pub event_name: String,
    pub organizer: String, // e.g. "Linux Foundation"
    pub event_type: String, // e.g. "Summit", "Webinar", "Hackathon"
    pub location: String,
}

#[derive(Debug, Clone, Default)]
pub struct LinuxDotComCommunityNewsEngine {
    pub events: Vec<LinuxCommunityEvent>,
}

impl LinuxDotComCommunityNewsEngine {
    pub fn new() -> Self {
        let events = vec![
            LinuxCommunityEvent {
                event_name: "Open Source Summit 2025".to_string(),
                organizer: "Linux Foundation".to_string(),
                event_type: "Summit".to_string(),
                location: "Global / Hybrid".to_string(),
            },
            LinuxCommunityEvent {
                event_name: "Sovereign Kernel Architecture Hackathon".to_string(),
                organizer: "SigmaOS Foundation".to_string(),
                event_type: "Hackathon".to_string(),
                location: "Online".to_string(),
            },
        ];
        Self { events }
    }

    pub fn get_events_by_organizer(&self, organizer: &str) -> Vec<LinuxCommunityEvent> {
        self.events
            .iter()
            .filter(|e| e.organizer.eq_ignore_ascii_case(organizer))
            .cloned()
            .collect()
    }
}

// ============================================================================
// 11. Appuals System Error Diagnosis & Fix Recipe Engine
// Inspired by Appuals
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppualsFixRecipe {
    pub error_code: String,
    pub symptom: String,
    pub solution_steps: Vec<String>,
    pub automated_remediation_cmd: String,
}

#[derive(Debug, Clone, Default)]
pub struct AppualsTroubleshootingEngine {
    pub recipes: BTreeMap<String, AppualsFixRecipe>,
}

impl AppualsTroubleshootingEngine {
    pub fn new() -> Self {
        let mut recipes = BTreeMap::new();
        recipes.insert(
            "ERR_MEM_CGROUP_OOM".to_string(),
            AppualsFixRecipe {
                error_code: "ERR_MEM_CGROUP_OOM".to_string(),
                symptom: "Process terminated due to memory pressure in cgroup v2".to_string(),
                solution_steps: vec![
                    "Inspect cgroup memory limits with 'cat /sys/fs/cgroup/memory.max'".to_string(),
                    "Trigger OOM score recalculation using 'sigma-memcg balance'".to_string(),
                ],
                automated_remediation_cmd: "sigma-memcg balance --force".to_string(),
            },
        );
        Self { recipes }
    }

    pub fn diagnose_and_fix(&self, error_code: &str) -> Option<&AppualsFixRecipe> {
        self.recipes.get(error_code)
    }

    pub fn register_recipe(&mut self, recipe: AppualsFixRecipe) {
        self.recipes.insert(recipe.error_code.clone(), recipe);
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
    pub geeky_gadgets: GeekyGadgetsTechReviewEngine,
    pub itdaily_sla: ItDailyEnterpriseItGovernor,
    pub howtogeek_guides: HowToGeekGuideSystemEngine,
    pub thenewstack_cloud: TheNewStackCloudNativeEngine,
    pub linux_dot_com: LinuxDotComCommunityNewsEngine,
    pub appuals_troubleshoot: AppualsTroubleshootingEngine,
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
            geeky_gadgets: GeekyGadgetsTechReviewEngine::new(),
            itdaily_sla: ItDailyEnterpriseItGovernor::new(),
            howtogeek_guides: HowToGeekGuideSystemEngine::new(),
            thenewstack_cloud: TheNewStackCloudNativeEngine::new(),
            linux_dot_com: LinuxDotComCommunityNewsEngine::new(),
            appuals_troubleshoot: AppualsTroubleshootingEngine::new(),
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

        // Verify Geeky-Gadgets
        let geeky_ok = self.geeky_gadgets.calculate_total_power_mw() > 0.0;

        // Verify ITDaily SLAs
        let sla_ok = self.itdaily_sla.check_all_slas_met();

        // Verify HowToGeek Guides
        let htg_ok = self.howtogeek_guides.get_guide("low_latency_kernel").is_some();

        // Verify TheNewStack Cloud Native
        let tns_ok = self.thenewstack_cloud.verify_wasm_workloads();

        // Verify Linux.com Events
        let ldc_ok = !self.linux_dot_com.get_events_by_organizer("Linux Foundation").is_empty();

        // Verify Appuals Troubleshooting
        let appuals_ok = self.appuals_troubleshoot.diagnose_and_fix("ERR_MEM_CGROUP_OOM").is_some();

        feeds_ok
            && telemetry_ok
            && bench_ok
            && ai_ok
            && sec_ok
            && bridge_ok
            && geeky_ok
            && sla_ok
            && htg_ok
            && tns_ok
            && ldc_ok
            && appuals_ok
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
        assert_eq!(engine.get_articles_by_domain("geeky-gadgets.com").len(), 1);
        assert_eq!(engine.get_articles_by_category("Kernel").len(), 2);

        engine.add_article(TechMediaArticleFeed {
            title: "New Phoronix Benchmarking Suite Release".to_string(),
            portal: "Phoronix".to_string(),
            domain: "phoronix.com".to_string(),
            url: "https://phoronix.com/news".to_string(),
            category: "Hardware".to_string(),
            timestamp_epoch: 1730009900,
        });

        let news = engine.get_latest_news();
        assert_eq!(news[0].portal, "Phoronix");

        let search_res = engine.search_articles("microcontroller");
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].portal, "Geeky-Gadgets");
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
    fn test_geeky_gadgets_engine() {
        let mut geeky = GeekyGadgetsTechReviewEngine::new();
        assert_eq!(geeky.evaluate_bus_efficiency("I2C").len(), 1);
        assert!(geeky.calculate_total_power_mw() > 0.0);

        geeky.register_peripheral(PeripheralDeviceSpec {
            device_name: "Test-Sensor".to_string(),
            bus_type: "SPI".to_string(),
            clock_freq_khz: 1000,
            power_consumption_mw: 5.0,
            max_bandwidth_mbps: 1.0,
        });
        assert_eq!(geeky.evaluate_bus_efficiency("SPI").len(), 1);
    }

    #[test]
    fn test_itdaily_sla_engine() {
        let mut gov = ItDailyEnterpriseItGovernor::new();
        assert!(gov.check_all_slas_met());

        gov.add_service_sla(EnterpriseServiceSlaRecord {
            service_id: "test_failing_service".to_string(),
            target_uptime_pct: 99.9,
            current_uptime_pct: 95.0,
            mean_time_between_failures_hrs: 100.0,
            mean_time_to_recovery_mins: 60.0,
        });
        assert!(!gov.check_all_slas_met());
    }

    #[test]
    fn test_howtogeek_and_thenewstack_engines() {
        let htg = HowToGeekGuideSystemEngine::new();
        let g = htg.get_guide("low_latency_kernel");
        assert!(g.is_some());
        assert_eq!(g.unwrap().steps.len(), 2);

        let mut tns = TheNewStackCloudNativeEngine::new();
        assert!(tns.verify_wasm_workloads());
        tns.deploy_workload(CloudNativeWorkloadSpec {
            workload_id: "wasm_analytics".to_string(),
            runtime: "Wasm".to_string(),
            replica_count: 2,
            is_zero_trust_isolated: true,
        });
        assert!(tns.verify_wasm_workloads());
    }

    #[test]
    fn test_linux_dot_com_and_appuals_engines() {
        let ldc = LinuxDotComCommunityNewsEngine::new();
        assert_eq!(ldc.get_events_by_organizer("Linux Foundation").len(), 1);

        let appuals = AppualsTroubleshootingEngine::new();
        let recipe = appuals.diagnose_and_fix("ERR_MEM_CGROUP_OOM");
        assert!(recipe.is_some());
        assert_eq!(recipe.unwrap().automated_remediation_cmd, "sigma-memcg balance --force");
    }

    #[test]
    fn test_sovereign_tech_media_master_suite() {
        let mut suite = SovereignTechMediaMasterSuite::new();
        assert!(suite.synthesize_and_verify_all());
    }
}
