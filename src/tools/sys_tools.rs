//! Sovereign BSD/Linux-style System Administration and Diagnostics Tools (sys_tools)
//! Implements high-fidelity tcpdump, ncdu disk analysis, and kernel sysctl runtime tuners.
use alloc::format;
extern crate alloc;

use alloc::string::{String, ToString};
use crate::klib::{Vec, HashMap};

// ==========================================
// 1. SovereignTcpDump Packet Sniffer Tool
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketProtocol {
    Arp,
    Ipv4,
    Tcp,
    Udp,
    Icmp,
}

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub protocol: PacketProtocol,
    pub src_ip: String,
    pub dest_ip: String,
    pub src_port: u16,
    pub dest_port: u16,
    pub payload_len: usize,
}

pub struct SovereignTcpDump {
    pub interface_name: String,
    pub filter_ip: Option<String>,
    pub filter_port: Option<u16>,
}

impl SovereignTcpDump {
    pub fn new(interface_name: &str) -> Self {
        Self {
            interface_name: interface_name.to_string(),
            filter_ip: None,
            filter_port: None,
        }
    }

    /// Captures a raw packet and produces a descriptive tcpdump formatting string
    pub fn sniff_packet(&self, packet: &RawPacket) -> Option<String> {
        // Apply dynamic IP filtering
        if let Some(ref f_ip) = self.filter_ip {
            if &packet.src_ip != f_ip && &packet.dest_ip != f_ip {
                return None; // Filtered out
            }
        }
        // Apply dynamic port filtering
        if let Some(f_port) = self.filter_port {
            if packet.src_port != f_port && packet.dest_port != f_port {
                return None; // Filtered out
            }
        }

        Some(format!(
            "[{}] {:?} {}:{} > {}:{}: len {} bytes",
            self.interface_name,
            packet.protocol,
            packet.src_ip,
            packet.src_port,
            packet.dest_ip,
            packet.dest_port,
            packet.payload_len
        ))
    }
}

// ==========================================
// 2. SovereignNcdu Disk Usage Analyzer
// ==========================================

#[derive(Debug, Clone)]
pub struct DiskNode {
    pub name: String,
    pub is_directory: bool,
    pub size_kb: u64,
    pub children: Vec<DiskNode>,
}

impl DiskNode {
    pub fn file(name: &str, size_kb: u64) -> Self {
        Self {
            name: name.to_string(),
            is_directory: false,
            size_kb,
            children: Vec::new(),
        }
    }

    pub fn directory(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_directory: true,
            size_kb: 0,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: DiskNode) {
        if self.is_directory {
            self.children.push(node);
            self.recalculate_directory_size();
        }
    }

    pub fn recalculate_directory_size(&mut self) {
        let mut total = 0;
        for child in &self.children {
            total += child.size_kb;
        }
        self.size_kb = total;
    }
}

pub struct SovereignNcdu {
    pub root_node: DiskNode,
}

impl SovereignNcdu {
    pub fn new(root_node: DiskNode) -> Self {
        Self { root_node }
    }

    /// Ranks directory subnodes based on space consumed (largest first)
    pub fn get_ranked_disk_consumers(&self) -> Vec<(String, u64)> {
        let mut consumers = Vec::new();
        for child in &self.root_node.children {
            consumers.push((child.name.clone(), child.size_kb));
        }
        let n = consumers.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                if consumers[j].1 < consumers[j+1].1 {
                    let temp = consumers[j].clone();
                    consumers[j] = consumers[j+1].clone();
                    consumers[j+1] = temp;
                }
            }
        }
        consumers
    }
}

// ==========================================
// 3. SovereignSysctl Kernel Tuning Engine
// ==========================================

pub struct SovereignSysctl {
    pub kernel_parameters: HashMap<String, String>,
}

impl SovereignSysctl {
    pub fn new() -> Self {
        let mut params = HashMap::new();
        params.insert("net.ipv4.ip_forward".to_string(), "0".to_string());
        params.insert("kernel.sched_latency_ns".to_string(), "12000000".to_string());
        params.insert("vm.swappiness".to_string(), "60".to_string());
        Self {
            kernel_parameters: params,
        }
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.kernel_parameters.get(key)
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if !self.kernel_parameters.contains_key(key) {
            return Err("Sysctl parameter key does not exist");
        }
        // Basic swappiness range validation
        if key == "vm.swappiness" {
            let num = value.parse::<u32>().map_err(|_| "Invalid number format")?;
            if num > 100 {
                return Err("Swappiness cannot exceed 100");
            }
        }
        self.kernel_parameters.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

impl Default for SovereignSysctl {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_tcpdump() {
        let mut dump = SovereignTcpDump::new("eth0");
        let packet = RawPacket {
            protocol: PacketProtocol::Tcp,
            src_ip: "192.168.1.5".to_string(),
            dest_ip: "10.0.0.1".to_string(),
            src_port: 443,
            dest_port: 8080,
            payload_len: 256,
        };

        let output = dump.sniff_packet(&packet).unwrap();
        assert!(output.contains("eth0"));
        assert!(output.contains("Tcp"));

        // Setup filter
        dump.filter_ip = Some("192.168.1.100".to_string());
        assert!(dump.sniff_packet(&packet).is_none()); // Filtered
    }

    #[test]
    fn test_sovereign_ncdu() {
        let mut root = DiskNode::directory("/");
        root.add_child(DiskNode::file("etc", 50));
        root.add_child(DiskNode::file("usr", 500));
        root.add_child(DiskNode::file("home", 1200));

        let ncdu = SovereignNcdu::new(root);
        let ranked = ncdu.get_ranked_disk_consumers();
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0].0, "home");
        assert_eq!(ranked[0].1, 1200);
        assert_eq!(ranked[2].0, "etc");
    }

    #[test]
    fn test_sovereign_sysctl() {
        let mut sysctl = SovereignSysctl::new();
        assert_eq!(sysctl.get_parameter("net.ipv4.ip_forward").unwrap(), "0");

        assert!(sysctl.set_parameter("net.ipv4.ip_forward", "1").is_ok());
        assert_eq!(sysctl.get_parameter("net.ipv4.ip_forward").unwrap(), "1");

        // Error validations
        assert!(sysctl.set_parameter("vm.swappiness", "150").is_err());
        assert!(sysctl.set_parameter("invalid.key", "1").is_err());
    }
}
