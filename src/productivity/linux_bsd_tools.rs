extern crate alloc;
// SigmaOS Standard CLI Utilities & Diagnostic Tools (Linux/BSD/Windows Parity)
// Implements top/htop, ifconfig/ip, and ping equivalents inside the microkernel ecosystem.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// =========================================================================
// 1. TOP / HTOP (System Resource & Process Task Monitor)
// =========================================================================

#[derive(Debug, Clone)]
pub struct ProcessTaskInfo {
    pub pid: usize,
    pub name: String,
    pub cpu_usage_pct: f32,
    pub memory_rss_bytes: usize,
    pub state: String,
}

pub struct TopCommand {
    pub processes: Vec<ProcessTaskInfo>,
}

impl TopCommand {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn register_process(&mut self, pid: usize, name: &str, cpu: f32, rss: usize, state: &str) {
        self.processes.push(ProcessTaskInfo {
            pid,
            name: name.to_string(),
            cpu_usage_pct: cpu,
            memory_rss_bytes: rss,
            state: state.to_string(),
        });
    }

    /// Simulates sorting processes by CPU or Memory RSS (htop sorting)
    pub fn sort_by_cpu(&mut self) {
        self.processes
            .sort_by(|a, b| b.cpu_usage_pct.partial_cmp(&a.cpu_usage_pct).unwrap());
    }

    pub fn sort_by_memory(&mut self) {
        self.processes
            .sort_by(|a, b| b.memory_rss_bytes.cmp(&a.memory_rss_bytes));
    }
}

// =========================================================================
// 2. IFCONFIG / IP (Network Interface & Link Configuration)
// =========================================================================

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub mac_address: String,
    pub mtu: usize,
    pub is_up: bool,
}

pub struct IfconfigCommand {
    pub interfaces: BTreeMap<String, NetworkInterface>,
}

impl IfconfigCommand {
    pub fn new() -> Self {
        let mut ic = Self {
            interfaces: BTreeMap::new(),
        };
        // Seed default loopback interface
        ic.set_interface("lo0", "127.0.0.1", "00:00:00:00:00:00", 16384, true);
        ic
    }

    pub fn set_interface(&mut self, name: &str, ip: &str, mac: &str, mtu: usize, up: bool) {
        self.interfaces.insert(
            name.to_string(),
            NetworkInterface {
                name: name.to_string(),
                ip_address: ip.to_string(),
                mac_address: mac.to_string(),
                mtu,
                is_up: up,
            },
        );
    }

    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }
}

// =========================================================================
// 3. PING (ICMP Network Packet Echo Diagnostics)
// =========================================================================

#[derive(Debug, Clone)]
pub struct PingResult {
    pub host: String,
    pub packets_transmitted: usize,
    pub packets_received: usize,
    pub average_latency_ms: f32,
}

pub struct PingCommand {
    pub ping_history: Vec<PingResult>,
}

impl PingCommand {
    pub fn new() -> Self {
        Self {
            ping_history: Vec::new(),
        }
    }

    /// Simulates sending ICMP Echo Request packets and tracking average latency
    pub fn execute_ping(&mut self, host: &str, count: usize) -> PingResult {
        let mut rtt_sum = 0.0f32;
        let mut seed = 42u64;

        // Simulate network latency calculations using a linear congruential generator
        for _ in 0..count {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let latency = 5.0f32 + (seed % 45) as f32; // Latency between 5ms and 50ms
            rtt_sum += latency;
        }

        let result = PingResult {
            host: host.to_string(),
            packets_transmitted: count,
            packets_received: count, // 100% success rate in simulation
            average_latency_ms: rtt_sum / (count as f32),
        };

        self.ping_history.push(result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htop_sorting() {
        let mut top = TopCommand::new();
        top.register_process(1, "init", 0.5, 4096, "Running");
        top.register_process(50, "sigma-office", 12.5, 65536, "Sleeping");
        top.register_process(100, "zenith-compositor", 45.2, 131072, "Running");

        top.sort_by_cpu();
        assert_eq!(top.processes[0].pid, 100); // zenith-compositor has highest CPU

        top.sort_by_memory();
        assert_eq!(top.processes[0].pid, 100); // zenith-compositor has highest memory RSS
    }

    #[test]
    fn test_ifconfig_setup() {
        let mut ifconfig = IfconfigCommand::new();
        ifconfig.set_interface("eth0", "192.168.1.15", "aa:bb:cc:dd:ee:ff", 1500, true);

        let eth0 = ifconfig.get_interface("eth0").unwrap();
        assert_eq!(eth0.ip_address, "192.168.1.15");
        assert_eq!(eth0.mtu, 1500);
        assert!(eth0.is_up);
    }

    #[test]
    fn test_ping_diagnostics() {
        let mut ping = PingCommand::new();
        let report = ping.execute_ping("google.com", 4);

        assert_eq!(report.host, "google.com");
        assert_eq!(report.packets_transmitted, 4);
        assert_eq!(report.packets_received, 4);
        assert!(report.average_latency_ms > 0.0);
    }
}
