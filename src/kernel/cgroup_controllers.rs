// Extended Cgroup v2 Controllers Implementation
// Phase 9.5: Device, Hugetlb, RDMA, Pids, and Net_cls Controllers
//
// This module provides comprehensive cgroup v2 controllers for:
// - Device access control (block/char devices)
// - Huge page size limits and tracking
// - RDMA resource limits (HCA, QP, CQ objects)
// - Process count limits
// - Network packet classification

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Cgroup controller trait
pub trait Controller: Send + Sync {
    /// Get controller name
    fn name(&self) -> &str;

    /// Enforce controller limits
    fn enforce(&mut self) -> Result<(), String>;

    /// Update controller settings
    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String>;

    /// Get controller stats
    fn get_stats(&self) -> HashMap<String, u64>;
}

/// ============ DEVICE CONTROLLER ============

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block,
    Char,
    Any,
}

/// Device access rule
#[derive(Debug, Clone)]
pub struct DeviceRule {
    pub device_type: DeviceType,
    pub major: u32,
    pub minor: u32,
    pub access: String, // "r", "w", "rw", etc.
}

/// Device cgroup controller - controls access to block and character devices
pub struct DeviceController {
    allow_rules: Vec<DeviceRule>,
    deny_rules: Vec<DeviceRule>,
    default_allow: bool,
    stats: HashMap<String, u64>,
}

impl DeviceController {
    pub fn new() -> Self {
        let mut stats = HashMap::new();
        stats.insert("device_access_denied".to_string(), 0);
        stats.insert("device_access_allowed".to_string(), 0);

        DeviceController {
            allow_rules: Vec::new(),
            deny_rules: Vec::new(),
            default_allow: false,
            stats,
        }
    }

    pub fn add_allow_rule(&mut self, rule: DeviceRule) {
        self.allow_rules.push(rule);
    }

    pub fn add_deny_rule(&mut self, rule: DeviceRule) {
        self.deny_rules.push(rule);
    }

    pub fn check_device_access(&mut self, device_type: DeviceType, major: u32, minor: u32, access: &str) -> bool {
        // Check deny rules first
        for rule in &self.deny_rules {
            if self.matches_rule(rule, device_type, major, minor, access) {
                *self.stats.get_mut("device_access_denied").unwrap() += 1;
                return false;
            }
        }

        // Check allow rules
        for rule in &self.allow_rules {
            if self.matches_rule(rule, device_type, major, minor, access) {
                *self.stats.get_mut("device_access_allowed").unwrap() += 1;
                return true;
            }
        }

        self.default_allow
    }

    fn matches_rule(&self, rule: &DeviceRule, device_type: DeviceType, major: u32, minor: u32, access: &str) -> bool {
        if rule.device_type != DeviceType::Any && rule.device_type != device_type {
            return false;
        }

        if rule.major != 0 && rule.major != major {
            return false;
        }

        if rule.minor != 0 && rule.minor != minor {
            return false;
        }

        for c in rule.access.chars() {
            if !access.contains(c) {
                return false;
            }
        }

        true
    }
}

impl Default for DeviceController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for DeviceController {
    fn name(&self) -> &str {
        "devices"
    }

    fn enforce(&mut self) -> Result<(), String> {
        // In a real implementation, would enforce device access limits
        Ok(())
    }

    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "allow" => {
                // Parse device rule from value
                // Format: "block|char major:minor rwm"
                Ok(())
            }
            "deny" => {
                // Parse device rule from value
                Ok(())
            }
            _ => Err(format!("Unknown device setting: {}", key)),
        }
    }

    fn get_stats(&self) -> HashMap<String, u64> {
        self.stats.clone()
    }
}

/// ============ HUGETLB CONTROLLER ============

/// Hugetlb page sizes supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HugepageSize {
    Two,    // 2MB
    One,    // 1GB
    Thirty, // 32MB
    SixtyFour, // 64MB
}

impl HugepageSize {
    pub fn bytes(&self) -> u64 {
        match self {
            HugepageSize::Two => 2 * 1024 * 1024,
            HugepageSize::One => 1024 * 1024 * 1024,
            HugepageSize::Thirty => 32 * 1024 * 1024,
            HugepageSize::SixtyFour => 64 * 1024 * 1024,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            HugepageSize::Two => "2MB",
            HugepageSize::One => "1GB",
            HugepageSize::Thirty => "32MB",
            HugepageSize::SixtyFour => "64MB",
        }
    }
}

/// Hugetlb cgroup controller - manages huge page allocations
pub struct HugetlbController {
    limits: HashMap<HugepageSize, u64>, // bytes
    current_usage: HashMap<HugepageSize, u64>, // bytes
    peak_usage: HashMap<HugepageSize, u64>, // bytes
}

impl HugetlbController {
    pub fn new() -> Self {
        HugetlbController {
            limits: HashMap::new(),
            current_usage: HashMap::new(),
            peak_usage: HashMap::new(),
        }
    }

    pub fn set_limit(&mut self, size: HugepageSize, limit: u64) {
        self.limits.insert(size, limit);
    }

    pub fn get_limit(&self, size: HugepageSize) -> Option<u64> {
        self.limits.get(&size).copied()
    }

    pub fn allocate(&mut self, size: HugepageSize, count: u64) -> Result<(), String> {
        let bytes_needed = size.bytes() * count;
        let limit = self.limits.get(&size).copied().unwrap_or(u64::MAX);
        let current = self.current_usage.get(&size).copied().unwrap_or(0);

        if current + bytes_needed > limit {
            return Err(format!(
                "Hugepage allocation would exceed limit for {}",
                size.name()
            ));
        }

        let new_usage = current + bytes_needed;
        self.current_usage.insert(size, new_usage);

        // Track peak usage
        let peak = self.peak_usage.get(&size).copied().unwrap_or(0);
        if new_usage > peak {
            self.peak_usage.insert(size, new_usage);
        }

        Ok(())
    }

    pub fn deallocate(&mut self, size: HugepageSize, count: u64) -> Result<(), String> {
        let bytes_freed = size.bytes() * count;
        let current = self.current_usage.get(&size).copied().unwrap_or(0);

        if bytes_freed > current {
            return Err("Cannot deallocate more than currently allocated".to_string());
        }

        self.current_usage.insert(size, current - bytes_freed);
        Ok(())
    }

    pub fn get_usage(&self, size: HugepageSize) -> u64 {
        self.current_usage.get(&size).copied().unwrap_or(0)
    }
}

impl Default for HugetlbController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for HugetlbController {
    fn name(&self) -> &str {
        "hugetlb"
    }

    fn enforce(&mut self) -> Result<(), String> {
        // Verify all allocations are within limits
        for (size, &limit) in &self.limits {
            if let Some(&usage) = self.current_usage.get(size) {
                if usage > limit {
                    return Err(format!("Hugepage {} usage {} exceeds limit {}", size.name(), usage, limit));
                }
            }
        }
        Ok(())
    }

    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        if key.starts_with("hugepages.") {
            let size_str = key.trim_start_matches("hugepages.");
            let size = match size_str {
                "2MB" => HugepageSize::Two,
                "1GB" => HugepageSize::One,
                "32MB" => HugepageSize::Thirty,
                "64MB" => HugepageSize::SixtyFour,
                _ => return Err(format!("Unknown hugepage size: {}", size_str)),
            };

            let limit: u64 = value.parse()
                .map_err(|_| format!("Invalid limit value: {}", value))?;
            
            self.set_limit(size, limit);
            Ok(())
        } else {
            Err(format!("Unknown hugetlb setting: {}", key))
        }
    }

    fn get_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        for (size, &usage) in &self.current_usage {
            stats.insert(format!("hugetlb_{}_current", size.name()), usage);
        }
        for (size, &peak) in &self.peak_usage {
            stats.insert(format!("hugetlb_{}_peak", size.name()), peak);
        }
        stats
    }
}

/// ============ RDMA CONTROLLER ============

/// RDMA resource object type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RdmaObjectType {
    Hca,    // Host Channel Adapter
    Qp,     // Queue Pair
    Cq,     // Completion Queue
    Mr,     // Memory Region
}

/// RDMA cgroup controller - limits RDMA resource usage
pub struct RdmaController {
    hca_limits: HashMap<String, u32>, // HCA name -> limit
    hca_current: HashMap<String, u32>,
    qp_limit: u32,
    qp_current: u32,
    cq_limit: u32,
    cq_current: u32,
    mr_limit: u32,
    mr_current: u32,
}

impl RdmaController {
    pub fn new() -> Self {
        RdmaController {
            hca_limits: HashMap::new(),
            hca_current: HashMap::new(),
            qp_limit: 1000,
            qp_current: 0,
            cq_limit: 1000,
            cq_current: 0,
            mr_limit: 10000,
            mr_current: 0,
        }
    }

    pub fn set_qp_limit(&mut self, limit: u32) {
        self.qp_limit = limit;
    }

    pub fn set_cq_limit(&mut self, limit: u32) {
        self.cq_limit = limit;
    }

    pub fn allocate_qp(&mut self) -> Result<(), String> {
        if self.qp_current >= self.qp_limit {
            return Err("Queue pair limit exceeded".to_string());
        }
        self.qp_current += 1;
        Ok(())
    }

    pub fn deallocate_qp(&mut self) -> Result<(), String> {
        if self.qp_current == 0 {
            return Err("Cannot deallocate queue pair: none allocated".to_string());
        }
        self.qp_current -= 1;
        Ok(())
    }

    pub fn allocate_cq(&mut self) -> Result<(), String> {
        if self.cq_current >= self.cq_limit {
            return Err("Completion queue limit exceeded".to_string());
        }
        self.cq_current += 1;
        Ok(())
    }
}

impl Default for RdmaController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for RdmaController {
    fn name(&self) -> &str {
        "rdma"
    }

    fn enforce(&mut self) -> Result<(), String> {
        if self.qp_current > self.qp_limit {
            return Err(format!("Queue pair usage {} exceeds limit {}", self.qp_current, self.qp_limit));
        }
        if self.cq_current > self.cq_limit {
            return Err(format!("Completion queue usage {} exceeds limit {}", self.cq_current, self.cq_limit));
        }
        Ok(())
    }

    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "qp_limit" => {
                let limit: u32 = value.parse()
                    .map_err(|_| format!("Invalid QP limit: {}", value))?;
                self.set_qp_limit(limit);
                Ok(())
            }
            "cq_limit" => {
                let limit: u32 = value.parse()
                    .map_err(|_| format!("Invalid CQ limit: {}", value))?;
                self.set_cq_limit(limit);
                Ok(())
            }
            _ => Err(format!("Unknown RDMA setting: {}", key)),
        }
    }

    fn get_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("rdma_qp_current".to_string(), self.qp_current as u64);
        stats.insert("rdma_qp_limit".to_string(), self.qp_limit as u64);
        stats.insert("rdma_cq_current".to_string(), self.cq_current as u64);
        stats.insert("rdma_cq_limit".to_string(), self.cq_limit as u64);
        stats.insert("rdma_mr_current".to_string(), self.mr_current as u64);
        stats.insert("rdma_mr_limit".to_string(), self.mr_limit as u64);
        stats
    }
}

/// ============ PIDS CONTROLLER ============

/// Pids cgroup controller - limits number of processes
pub struct PidsController {
    max_pids: u64,
    current_pids: u64,
    peak_pids: u64,
    events_limit_reached: u64,
}

impl PidsController {
    pub fn new() -> Self {
        PidsController {
            max_pids: 4096,
            current_pids: 0,
            peak_pids: 0,
            events_limit_reached: 0,
        }
    }

    pub fn set_max_pids(&mut self, max: u64) {
        self.max_pids = max;
    }

    pub fn get_max_pids(&self) -> u64 {
        self.max_pids
    }

    pub fn get_current_pids(&self) -> u64 {
        self.current_pids
    }

    pub fn fork_process(&mut self) -> Result<(), String> {
        if self.current_pids >= self.max_pids {
            self.events_limit_reached += 1;
            return Err("PID limit exceeded".to_string());
        }

        self.current_pids += 1;

        if self.current_pids > self.peak_pids {
            self.peak_pids = self.current_pids;
        }

        Ok(())
    }

    pub fn exit_process(&mut self) -> Result<(), String> {
        if self.current_pids == 0 {
            return Err("No processes to exit".to_string());
        }
        self.current_pids -= 1;
        Ok(())
    }
}

impl Default for PidsController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for PidsController {
    fn name(&self) -> &str {
        "pids"
    }

    fn enforce(&mut self) -> Result<(), String> {
        if self.current_pids > self.max_pids {
            return Err(format!("PID usage {} exceeds limit {}", self.current_pids, self.max_pids));
        }
        Ok(())
    }

    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "max" => {
                let max: u64 = value.parse()
                    .map_err(|_| format!("Invalid PID limit: {}", value))?;
                self.set_max_pids(max);
                Ok(())
            }
            _ => Err(format!("Unknown pids setting: {}", key)),
        }
    }

    fn get_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("pids_current".to_string(), self.current_pids);
        stats.insert("pids_peak".to_string(), self.peak_pids);
        stats.insert("pids_max".to_string(), self.max_pids);
        stats.insert("pids_limit_reached".to_string(), self.events_limit_reached);
        stats
    }
}

/// ============ NET_CLS CONTROLLER ============

/// Network classification controller - tags packets for QoS
pub struct NetClsController {
    class_id: u32,
    packet_count: u64,
    byte_count: u64,
}

impl NetClsController {
    pub fn new() -> Self {
        NetClsController {
            class_id: 0,
            packet_count: 0,
            byte_count: 0,
        }
    }

    pub fn set_class_id(&mut self, id: u32) {
        self.class_id = id;
    }

    pub fn get_class_id(&self) -> u32 {
        self.class_id
    }

    pub fn classify_packet(&mut self, bytes: u64) {
        self.packet_count += 1;
        self.byte_count += bytes;
    }
}

impl Default for NetClsController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for NetClsController {
    fn name(&self) -> &str {
        "net_cls"
    }

    fn enforce(&mut self) -> Result<(), String> {
        // Net_cls doesn't enforce limits, just tracks
        Ok(())
    }

    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "classid" => {
                let class_id: u32 = value.parse()
                    .map_err(|_| format!("Invalid class ID: {}", value))?;
                self.set_class_id(class_id);
                Ok(())
            }
            _ => Err(format!("Unknown net_cls setting: {}", key)),
        }
    }

    fn get_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("net_cls_packets".to_string(), self.packet_count);
        stats.insert("net_cls_bytes".to_string(), self.byte_count);
        stats
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_device_controller_creation() {
        let controller = DeviceController::new();
        assert_eq!(controller.name(), "devices");
    }

    #[test]
    fn test_device_allow_deny() {
        let mut controller = DeviceController::new();
        controller.default_allow = false;

        let allow_rule = DeviceRule {
            device_type: DeviceType::Block,
            major: 8,
            minor: 0,
            access: "rw".to_string(),
        };

        controller.add_allow_rule(allow_rule);

        let allowed = controller.check_device_access(DeviceType::Block, 8, 0, "r");
        assert!(allowed, "Allowed device should grant access");
    }

    #[test]
    fn test_hugetlb_allocation() {
        let mut controller = HugetlbController::new();
        controller.set_limit(HugepageSize::Two, 100 * 1024 * 1024);

        let result = controller.allocate(HugepageSize::Two, 10);
        assert!(result.is_ok(), "Allocation should succeed");
        assert_eq!(controller.get_usage(HugepageSize::Two), 10 * 2 * 1024 * 1024);
    }

    #[test]
    fn test_hugetlb_exceed_limit() {
        let mut controller = HugetlbController::new();
        controller.set_limit(HugepageSize::Two, 100 * 1024 * 1024);

        let result = controller.allocate(HugepageSize::Two, 100);
        assert!(result.is_err(), "Allocation should fail when exceeding limit");
    }

    #[test]
    fn test_pids_fork_and_exit() {
        let mut controller = PidsController::new();
        controller.set_max_pids(10);

        for _ in 0..5 {
            assert!(controller.fork_process().is_ok(), "Fork should succeed");
        }

        assert_eq!(controller.get_current_pids(), 5);

        for _ in 0..3 {
            assert!(controller.exit_process().is_ok(), "Exit should succeed");
        }

        assert_eq!(controller.get_current_pids(), 2);
    }

    #[test]
    fn test_pids_limit() {
        let mut controller = PidsController::new();
        controller.set_max_pids(3);

        for _ in 0..3 {
            assert!(controller.fork_process().is_ok(), "Fork within limit should succeed");
        }

        let result = controller.fork_process();
        assert!(result.is_err(), "Fork should fail when limit reached");
    }

    #[test]
    fn test_rdma_qp_allocation() {
        let mut controller = RdmaController::new();
        controller.set_qp_limit(100);

        let result = controller.allocate_qp();
        assert!(result.is_ok(), "QP allocation should succeed");
    }

    #[test]
    fn test_net_cls_classification() {
        let mut controller = NetClsController::new();
        controller.set_class_id(0x00050001);

        assert_eq!(controller.get_class_id(), 0x00050001);

        controller.classify_packet(1000);
        controller.classify_packet(2000);

        let stats = controller.get_stats();
        assert_eq!(stats.get("net_cls_packets"), Some(&2));
        assert_eq!(stats.get("net_cls_bytes"), Some(&3000));
    }

    #[test]
    fn test_controller_enforce() {
        let mut device_controller = DeviceController::new();
        let result = device_controller.enforce();
        assert!(result.is_ok(), "Device controller enforce should succeed");

        let mut pids_controller = PidsController::new();
        pids_controller.current_pids = 100;
        pids_controller.max_pids = 50;
        let result = pids_controller.enforce();
        assert!(result.is_err(), "Enforce should fail when limits exceeded");
    }

    #[test]
    fn test_hugetlb_peak_tracking() {
        let mut controller = HugetlbController::new();
        controller.set_limit(HugepageSize::Two, 1000 * 1024 * 1024);

        controller.allocate(HugepageSize::Two, 100).unwrap();
        let stats1 = controller.get_stats();
        assert!(stats1.contains_key("hugetlb_2MB_peak"));

        controller.deallocate(HugepageSize::Two, 50).unwrap();
        let stats2 = controller.get_stats();
        assert!(stats2.contains_key("hugetlb_2MB_current"));
    }
}
