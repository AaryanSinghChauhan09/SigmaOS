// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS System Monitor - Hardware monitoring module

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, ProcessorExt, CpuExt, DiskExt, NetworkExt, ProcessExt};
use crate::control_center::{HardwareStatus, NetworkStatus};

/// System Monitor for real-time hardware monitoring
pub struct SystemMonitor {
    system: System,
    monitor_interval: u64,
    last_update: Instant,
}

impl SystemMonitor {
    /// Create a new System Monitor
    pub fn new(monitor_interval: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut system = System::new_all();
        system.refresh_all();
        
        Ok(Self {
            system,
            monitor_interval,
            last_update: Instant::now(),
        })
    }

    /// Update system information
    pub fn update(&mut self) {
        if self.last_update.elapsed() >= Duration::from_secs(self.monitor_interval) {
            self.system.refresh_all();
            self.last_update = Instant::now();
        }
    }

    /// Get current hardware status
    pub fn get_hardware_status(&self) -> HardwareStatus {
        let cpu_usage = self.get_cpu_usage();
        let cpu_temp = self.get_cpu_temperature();
        let (memory_usage, memory_total) = self.get_memory_info();
        let (disk_usage, disk_total) = self.get_disk_info();
        let (gpu_usage, gpu_temp) = self.get_gpu_info();
        let network_status = self.get_network_status();

        HardwareStatus {
            cpu_usage,
            cpu_temperature: cpu_temp,
            memory_usage,
            memory_total,
            disk_usage,
            disk_total,
            gpu_usage,
            gpu_temperature: gpu_temp,
            network_status,
        }
    }

    /// Get CPU usage percentage
    fn get_cpu_usage(&self) -> f32 {
        let total_cpu = self.system.global_cpu_info();
        total_cpu.cpu_usage()
    }

    /// Get CPU temperature (if available)
    fn get_cpu_temperature(&self) -> f32 {
        // Try to read from Linux thermal zones
        if let Ok(temp) = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(temp_millidegrees) = temp.trim().parse::<i32>() {
                return temp_millidegrees as f32 / 1000.0;
            }
        }
        // Fallback to placeholder
        45.0
    }

    /// Get memory usage information
    fn get_memory_info(&self) -> (f32, u64) {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let usage_percent = if total_memory > 0 {
            (used_memory as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };
        (usage_percent, total_memory)
    }

    /// Get disk usage information
    fn get_disk_info(&self) -> (f32, u64) {
        let disks = self.system.disks();
        if let Some(disk) = disks.first() {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space.saturating_sub(available_space);
            let usage_percent = if total_space > 0 {
                (used_space as f32 / total_space as f32) * 100.0
            } else {
                0.0
            };
            (usage_percent, total_space)
        } else {
            (0.0, 0)
        }
    }

    /// Get GPU information (if available)
    fn get_gpu_info(&self) -> (Option<f32>, Option<f32>) {
        // Try NVIDIA GPU first
        #[cfg(feature = "nvidia-gpu")]
        {
            if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                if let Ok(device_count) = nvml.device_count() {
                    if device_count > 0 {
                        if let Ok(device) = nvml.device_by_index(0) {
                            if let Ok(usage) = device.utilization_rates() {
                                if let Ok(temp) = device.temperature(nvml_wrapper::TemperatureSensor::Gpu) {
                                    return (Some(usage.gpu as f32), Some(temp as f32));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Try AMD GPU
        #[cfg(feature = "amd-gpu")]
        {
            if let Ok(gpu_info) = amdgpu::get_gpu_info() {
                return (Some(gpu_info.usage), Some(gpu_info.temperature));
            }
        }

        // Fallback to reading from sysfs
        if let Ok(temp) = std::fs::read_to_string("/sys/class/hwmon/hwmon0/temp1_input") {
            if let Ok(temp_millidegrees) = temp.trim().parse::<i32>() {
                return (None, Some(temp_millidegrees as f32 / 1000.0));
            }
        }

        (None, None)
    }

    /// Get network status
    fn get_network_status(&self) -> NetworkStatus {
        let networks = self.system.networks();
        let (interface, ip_address, upload_speed, download_speed) = 
            networks.iter().next().map(|(name, data)| {
                (
                    name.clone(),
                    self.get_ip_address(name),
                    data.total_received() as f64,
                    data.total_transmitted() as f64,
                )
            }).unwrap_or_else(|| {
                (
                    "unknown".to_string(),
                    "0.0.0.0".to_string(),
                    0.0,
                    0.0,
                )
            });

        NetworkStatus {
            connected: !interface.is_empty() && interface != "unknown",
            interface,
            ip_address,
            upload_speed,
            download_speed,
        }
    }

    /// Get IP address for a network interface
    fn get_ip_address(&self, interface: &str) -> String {
        // Try to read IP address from /proc/net/if_inetaddr or use iproute2
        // For now, return a placeholder
        "192.168.1.100".to_string()
    }

    /// Get detailed process information
    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        self.system.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            })
            .collect()
    }

    /// Get system uptime in seconds
    pub fn get_uptime(&self) -> u64 {
        self.system.uptime()
    }
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SystemMonitor::new(5);
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_hardware_status() {
        let monitor = SystemMonitor::new(5).unwrap();
        let status = monitor.get_hardware_status();
        assert!(status.cpu_usage >= 0.0 && status.cpu_usage <= 100.0);
        assert!(status.memory_usage >= 0.0 && status.memory_usage <= 100.0);
    }

    #[test]
    fn test_update() {
        let mut monitor = SystemMonitor::new(1).unwrap();
        monitor.update();
        let status = monitor.get_hardware_status();
        assert!(status.cpu_usage >= 0.0);
    }
}
