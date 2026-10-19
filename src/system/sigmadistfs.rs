// SigmaDistFS - Distributed Filesystem Support for SigmaOS
// Inspired by Ceph, GlusterFS, and MooseFS
// Provides distributed storage across multiple nodes with replication and erasure coding

#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================
// Enums
// ============================================================

/// Distributed filesystem backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistFSType {
    /// Object-based distributed FS (Ceph-inspired)
    CephLike,
    /// Scale-out NAS (GlusterFS-inspired)
    GlusterLike,
    /// Metadata-server based (MooseFS-inspired)
    MooseFSLike,
    /// Peer-to-peer block storage
    P2PBlock,
}

/// Replication strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationStrategy {
    /// Replicate to N nodes
    Replicated(u8),
    /// Erasure coding (k data, m parity)
    ErasureCoded { data_chunks: u8, parity_chunks: u8 },
    /// Single copy (no redundancy)
    None,
}

/// Node state in the cluster
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Active,
    Degraded,
    Down,
    Recovering,
    Maintenance,
}

/// Cluster health state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterHealth {
    Healthy,
    Degraded,
    Critical,
    Offline,
}

/// Volume state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeState {
    Created,
    Starting,
    Started,
    Stopping,
    Stopped,
    Degraded,
    Error,
}

/// Volume type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeType {
    /// All data on all nodes
    Distributed,
    /// Same data on all nodes (replicated)
    Replicated,
    /// Combination: striped across distributed sets, each replicated
    DistributedReplicated,
    /// Erasure coded across nodes
    ErasureCoded,
}

// ============================================================
// Structs
// ============================================================

/// A node in the distributed cluster
#[derive(Debug, Clone)]
pub struct DistNode {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub port: u16,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub state: NodeState,
    pub weight: u32, // Relative storage weight for data placement
}

impl DistNode {
    pub fn new(
        id: String,
        hostname: String,
        ip_address: String,
        port: u16,
        total_bytes: u64,
    ) -> Self {
        DistNode {
            id,
            hostname,
            ip_address,
            port,
            total_bytes,
            used_bytes: 0,
            state: NodeState::Active,
            weight: 100,
        }
    }

    pub fn free_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    pub fn usage_percent(&self) -> u8 {
        if self.total_bytes == 0 {
            return 0;
        }
        ((self.used_bytes * 100) / self.total_bytes) as u8
    }

    pub fn is_available(&self) -> bool {
        self.state == NodeState::Active || self.state == NodeState::Degraded
    }
}

/// A distributed volume (logical storage unit)
#[derive(Debug, Clone)]
pub struct DistVolume {
    pub name: String,
    pub volume_type: VolumeType,
    pub replication: ReplicationStrategy,
    pub brick_count: u32,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub state: VolumeState,
    pub mount_point: String,
    pub node_ids: Vec<String>, // Nodes hosting this volume
}

impl DistVolume {
    pub fn new(
        name: String,
        volume_type: VolumeType,
        replication: ReplicationStrategy,
        total_bytes: u64,
        mount_point: String,
    ) -> Self {
        DistVolume {
            name,
            volume_type,
            replication,
            brick_count: 0,
            total_bytes,
            used_bytes: 0,
            state: VolumeState::Created,
            mount_point,
            node_ids: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: String) {
        self.node_ids.push(node_id);
        self.brick_count += 1;
    }

    pub fn free_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    pub fn is_active(&self) -> bool {
        self.state == VolumeState::Started
    }

    pub fn start(&mut self) {
        self.state = VolumeState::Started;
    }

    pub fn stop(&mut self) {
        self.state = VolumeState::Stopped;
    }
}

/// Overall cluster statistics
#[derive(Debug, Clone, Default)]
pub struct ClusterStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub node_count: u32,
    pub active_nodes: u32,
    pub volume_count: u32,
    pub read_iops: u64,
    pub write_iops: u64,
    pub read_bandwidth_bps: u64,
    pub write_bandwidth_bps: u64,
}

impl ClusterStats {
    pub fn free_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    pub fn usage_percent(&self) -> u8 {
        if self.total_bytes == 0 {
            return 0;
        }
        ((self.used_bytes * 100) / self.total_bytes) as u8
    }
}

/// Distributed Filesystem Manager
#[derive(Debug)]
pub struct SigmaDistFS {
    nodes: Vec<DistNode>,
    volumes: Vec<DistVolume>,
    fs_type: DistFSType,
    cluster_name: String,
    health: ClusterHealth,
    running: bool,
}

impl SigmaDistFS {
    pub fn new(cluster_name: String, fs_type: DistFSType) -> Self {
        SigmaDistFS {
            nodes: Vec::new(),
            volumes: Vec::new(),
            fs_type,
            cluster_name,
            health: ClusterHealth::Offline,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.recalculate_health();
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.health = ClusterHealth::Offline;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn cluster_name(&self) -> &str {
        &self.cluster_name
    }

    pub fn health(&self) -> ClusterHealth {
        self.health
    }

    pub fn fs_type(&self) -> DistFSType {
        self.fs_type
    }

    // ---- Node Management ----

    pub fn add_node(&mut self, node: DistNode) -> bool {
        if self.nodes.iter().any(|n| n.id == node.id) {
            return false;
        }
        self.nodes.push(node);
        self.recalculate_health();
        true
    }

    pub fn remove_node(&mut self, node_id: &str) -> bool {
        let initial = self.nodes.len();
        self.nodes.retain(|n| n.id != node_id);
        let removed = self.nodes.len() < initial;
        if removed {
            self.recalculate_health();
        }
        removed
    }

    pub fn get_node(&self, node_id: &str) -> Option<&DistNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut DistNode> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn active_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_available()).count()
    }

    pub fn list_nodes(&self) -> &[DistNode] {
        &self.nodes
    }

    pub fn list_active_nodes(&self) -> Vec<&DistNode> {
        self.nodes.iter().filter(|n| n.is_available()).collect()
    }

    // ---- Volume Management ----

    pub fn create_volume(&mut self, volume: DistVolume) -> bool {
        if self.volumes.iter().any(|v| v.name == volume.name) {
            return false;
        }
        self.volumes.push(volume);
        true
    }

    pub fn delete_volume(&mut self, name: &str) -> bool {
        let initial = self.volumes.len();
        self.volumes.retain(|v| v.name != name);
        self.volumes.len() < initial
    }

    pub fn start_volume(&mut self, name: &str) -> bool {
        if let Some(v) = self.volumes.iter_mut().find(|v| v.name == name) {
            v.start();
            true
        } else {
            false
        }
    }

    pub fn stop_volume(&mut self, name: &str) -> bool {
        if let Some(v) = self.volumes.iter_mut().find(|v| v.name == name) {
            v.stop();
            true
        } else {
            false
        }
    }

    pub fn get_volume(&self, name: &str) -> Option<&DistVolume> {
        self.volumes.iter().find(|v| v.name == name)
    }

    pub fn volume_count(&self) -> usize {
        self.volumes.len()
    }

    pub fn list_volumes(&self) -> &[DistVolume] {
        &self.volumes
    }

    pub fn list_active_volumes(&self) -> Vec<&DistVolume> {
        self.volumes.iter().filter(|v| v.is_active()).collect()
    }

    // ---- Statistics ----

    pub fn cluster_stats(&self) -> ClusterStats {
        let total_bytes: u64 = self.nodes.iter().map(|n| n.total_bytes).sum();
        let used_bytes: u64 = self.nodes.iter().map(|n| n.used_bytes).sum();
        let active_nodes = self.active_node_count() as u32;

        ClusterStats {
            total_bytes,
            used_bytes,
            node_count: self.nodes.len() as u32,
            active_nodes,
            volume_count: self.volumes.len() as u32,
            read_iops: 0,
            write_iops: 0,
            read_bandwidth_bps: 0,
            write_bandwidth_bps: 0,
        }
    }

    // ---- Internal ----

    fn recalculate_health(&mut self) {
        let total = self.nodes.len();
        if total == 0 {
            self.health = ClusterHealth::Offline;
            return;
        }
        let active = self.active_node_count();
        let ratio = (active * 100) / total;
        self.health = match ratio {
            100 => ClusterHealth::Healthy,
            51..=99 => ClusterHealth::Degraded,
            1..=50 => ClusterHealth::Critical,
            _ => ClusterHealth::Offline,
        };
    }
}

// ============================================================
// Global Singleton
// ============================================================

static mut DISTFS_INSTANCE: Option<SigmaDistFS> = None;

pub fn init_sigmadistfs(cluster_name: &str, fs_type: DistFSType) {
    unsafe {
        DISTFS_INSTANCE = Some(SigmaDistFS::new(cluster_name.to_string(), fs_type));
        if let Some(ref mut mgr) = DISTFS_INSTANCE {
            mgr.start();
        }
    }
}

pub fn get_sigmadistfs() -> Option<&'static SigmaDistFS> {
    unsafe { DISTFS_INSTANCE.as_ref() }
}

pub fn get_sigmadistfs_mut() -> Option<&'static mut SigmaDistFS> {
    unsafe { DISTFS_INSTANCE.as_mut() }
}

// ============================================================
// Convenience API
// ============================================================

/// Add a storage node to the global cluster
pub fn add_distfs_node(node: DistNode) -> bool {
    if let Some(mgr) = get_sigmadistfs_mut() {
        mgr.add_node(node)
    } else {
        false
    }
}

/// Create a distributed volume in the global cluster
pub fn create_distfs_volume(volume: DistVolume) -> bool {
    if let Some(mgr) = get_sigmadistfs_mut() {
        mgr.create_volume(volume)
    } else {
        false
    }
}

/// Start a volume by name
pub fn start_distfs_volume(name: &str) -> bool {
    if let Some(mgr) = get_sigmadistfs_mut() {
        mgr.start_volume(name)
    } else {
        false
    }
}

/// Get cluster health
pub fn distfs_cluster_health() -> ClusterHealth {
    if let Some(mgr) = get_sigmadistfs() {
        mgr.health()
    } else {
        ClusterHealth::Offline
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, host: &str, ip: &str) -> DistNode {
        DistNode::new(
            id.to_string(),
            host.to_string(),
            ip.to_string(),
            24007,
            100 * 1024 * 1024 * 1024, // 100 GiB
        )
    }

    #[test]
    fn test_node_creation() {
        let node = make_node("n1", "storage1", "10.0.0.1");
        assert_eq!(node.id, "n1");
        assert_eq!(node.state, NodeState::Active);
        assert!(node.is_available());
        assert_eq!(node.usage_percent(), 0);
    }

    #[test]
    fn test_node_free_bytes() {
        let mut node = make_node("n1", "storage1", "10.0.0.1");
        node.used_bytes = 50 * 1024 * 1024 * 1024;
        assert_eq!(node.usage_percent(), 50);
        assert_eq!(node.free_bytes(), 50 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_volume_creation() {
        let mut vol = DistVolume::new(
            "data".to_string(),
            VolumeType::DistributedReplicated,
            ReplicationStrategy::Replicated(3),
            1024 * 1024 * 1024 * 1024, // 1 TiB
            "/mnt/distfs/data".to_string(),
        );
        assert_eq!(vol.state, VolumeState::Created);
        vol.add_node("n1".to_string());
        vol.add_node("n2".to_string());
        assert_eq!(vol.brick_count, 2);
        vol.start();
        assert!(vol.is_active());
        vol.stop();
        assert!(!vol.is_active());
    }

    #[test]
    fn test_manager_nodes() {
        let mut mgr = SigmaDistFS::new("test-cluster".to_string(), DistFSType::GlusterLike);
        mgr.start();
        assert!(mgr.is_running());

        let n1 = make_node("n1", "storage1", "10.0.0.1");
        let n2 = make_node("n2", "storage2", "10.0.0.2");

        assert!(mgr.add_node(n1));
        assert!(mgr.add_node(n2));
        // Duplicate
        let dup = make_node("n1", "storage1", "10.0.0.1");
        assert!(!mgr.add_node(dup));

        assert_eq!(mgr.node_count(), 2);
        assert_eq!(mgr.active_node_count(), 2);
        assert_eq!(mgr.health(), ClusterHealth::Healthy);

        assert!(mgr.remove_node("n1"));
        assert_eq!(mgr.node_count(), 1);
    }

    #[test]
    fn test_manager_volumes() {
        let mut mgr = SigmaDistFS::new("test-cluster".to_string(), DistFSType::CephLike);
        mgr.start();

        let vol = DistVolume::new(
            "backups".to_string(),
            VolumeType::Replicated,
            ReplicationStrategy::Replicated(2),
            500 * 1024 * 1024 * 1024,
            "/mnt/backups".to_string(),
        );
        assert!(mgr.create_volume(vol));
        assert_eq!(mgr.volume_count(), 1);

        assert!(mgr.start_volume("backups"));
        assert_eq!(mgr.list_active_volumes().len(), 1);

        assert!(mgr.stop_volume("backups"));
        assert_eq!(mgr.list_active_volumes().len(), 0);

        assert!(mgr.delete_volume("backups"));
        assert_eq!(mgr.volume_count(), 0);
    }

    #[test]
    fn test_cluster_stats() {
        let mut mgr = SigmaDistFS::new("stats-cluster".to_string(), DistFSType::GlusterLike);
        mgr.start();

        let mut n1 = make_node("n1", "s1", "10.0.0.1");
        n1.used_bytes = 25 * 1024 * 1024 * 1024;
        mgr.add_node(n1);

        let stats = mgr.cluster_stats();
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.active_nodes, 1);
        assert_eq!(stats.total_bytes, 100 * 1024 * 1024 * 1024);
        assert_eq!(stats.used_bytes, 25 * 1024 * 1024 * 1024);
        assert_eq!(stats.usage_percent(), 25);
    }

    #[test]
    fn test_global_instance() {
        init_sigmadistfs("sigma-cluster", DistFSType::GlusterLike);

        let node = make_node("g1", "gluster1", "192.168.1.10");
        assert!(add_distfs_node(node));

        assert_eq!(distfs_cluster_health(), ClusterHealth::Healthy);

        let mgr = get_sigmadistfs().unwrap();
        assert_eq!(mgr.node_count(), 1);
        assert_eq!(mgr.cluster_name(), "sigma-cluster");
    }
}
