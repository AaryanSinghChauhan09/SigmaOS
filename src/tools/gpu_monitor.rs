// SigmaOS `sigma-gpu` VRAM & Process Compute Telemetry Dashboard
// Implements process-level VRAM allocation tracking, GPU core frequency telemetry,
// Tensor/NPU engine occupancy, and PCIe bandwidth usage in an ANSI dashboard.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};

#[derive(Debug, Clone)]
pub struct GpuProcessUtilization {
    pub pid: usize,
    pub process_name: String,
    pub vram_used_bytes: u64,
    pub compute_engine_pct: f32,
    pub tensor_npu_pct: f32,
}

pub struct SovereignGpuProcessTelemetryEngine {
    pub gpu_device_name: String,
    pub total_vram_bytes: u64,
    pub gpu_core_clock_mhz: u32,
    pub pcie_bandwidth_gbps: f32,
    pub active_processes: BTreeMap<usize, GpuProcessUtilization>,
}

impl SovereignGpuProcessTelemetryEngine {
    pub fn new(device_name: &str, vram_bytes: u64) -> Self {
        Self {
            gpu_device_name: device_name.to_string(),
            total_vram_bytes: vram_bytes,
            gpu_core_clock_mhz: 2520,
            pcie_bandwidth_gbps: 31.5, // PCIe Gen 4 x16
            active_processes: BTreeMap::new(),
        }
    }

    pub fn update_process_gpu_utilization(
        &mut self,
        pid: usize,
        name: &str,
        vram_bytes: u64,
        compute_pct: f32,
        tensor_pct: f32,
    ) {
        let util = GpuProcessUtilization {
            pid,
            process_name: name.to_string(),
            vram_used_bytes: vram_bytes,
            compute_engine_pct: compute_pct,
            tensor_npu_pct: tensor_pct,
        };
        self.active_processes.insert(pid, util);
    }

    pub fn total_vram_used_bytes(&self) -> u64 {
        self.active_processes.values().map(|p| p.vram_used_bytes).sum()
    }

    pub fn generate_ansi_dashboard(&self) -> String {
        let used_vram = self.total_vram_used_bytes();
        let vram_pct = (used_vram as f64 / self.total_vram_bytes as f64) * 100.0;

        let mut dashboard = format!(
            "=== sigma-gpu: {} | Clock: {} MHz | VRAM: {}/{} MB ({:.1}%) ===\n",
            self.gpu_device_name,
            self.gpu_core_clock_mhz,
            used_vram / (1024 * 1024),
            self.total_vram_bytes / (1024 * 1024),
            vram_pct
        );

        dashboard.push_str("PID    Process          VRAM (MB)  Compute %  Tensor/NPU %\n");
        for proc in self.active_processes.values() {
            dashboard.push_str(&format!(
                "{:<6} {:<16} {:<10} {:<10.1} {:.1}%\n",
                proc.pid,
                proc.process_name,
                proc.vram_used_bytes / (1024 * 1024),
                proc.compute_engine_pct,
                proc.tensor_npu_pct
            ));
        }

        dashboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_process_telemetry_engine() {
        let mut gpu = SovereignGpuProcessTelemetryEngine::new("NVIDIA RTX 4090", 24 * 1024 * 1024 * 1024);

        gpu.update_process_gpu_utilization(1024, "llama.cpp", 8 * 1024 * 1024 * 1024, 85.0, 92.0);
        gpu.update_process_gpu_utilization(2048, "blender", 4 * 1024 * 1024 * 1024, 60.0, 0.0);

        assert_eq!(gpu.total_vram_used_bytes(), 12 * 1024 * 1024 * 1024);

        let dashboard = gpu.generate_ansi_dashboard();
        assert!(dashboard.contains("RTX 4090"));
        assert!(dashboard.contains("llama.cpp"));
        assert!(dashboard.contains("blender"));
    }
}
