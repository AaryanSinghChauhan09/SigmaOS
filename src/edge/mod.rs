//! Edge Computing Platform (Edge Computing Inspiration)
//! Lightweight edge runtime, distributed computing, and offline support

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Edge node state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeNodeState {
    Online,
    Offline,
    Updating,
    Degraded,
}

/// Edge application state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeAppState {
    Running,
    Stopped,
    Updating,
    Error,
}

/// Workload Assignment Policy inspired by Kubernetes / FreeBSD VNET edge topology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentPolicy {
    WorkloadBalance,
    LatencyMinimization,
    GeoAffinity,
    FailoverRedundancy,
}

/// Edge node
#[derive(Debug, Clone)]
pub struct EdgeNode {
    pub id: String,
    pub name: String,
    pub location: String,
    pub state: EdgeNodeState,
    pub cpu_capacity: u32,
    pub memory_capacity: u64,
    pub storage_capacity: u64,
    pub network_bandwidth: u32,
    pub cpu_allocated: u32,
    pub memory_allocated: u64,
    pub applications: Vec<EdgeApplication>,
}

impl EdgeNode {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            id: format!("edge_{}", name),
            name: name.to_string(),
            location: location.to_string(),
            state: EdgeNodeState::Online,
            cpu_capacity: 8,
            memory_capacity: 16384,
            storage_capacity: 102400,
            network_bandwidth: 1000,
            cpu_allocated: 0,
            memory_allocated: 0,
            applications: Vec::new(),
        }
    }

    pub fn available_cpu(&self) -> u32 {
        self.cpu_capacity.saturating_sub(self.cpu_allocated)
    }

    pub fn available_memory(&self) -> u64 {
        self.memory_capacity.saturating_sub(self.memory_allocated)
    }

    pub fn can_host(&self, req: &ResourceRequirements) -> bool {
        self.state == EdgeNodeState::Online
            && self.available_cpu() >= req.cpu
            && self.available_memory() >= req.memory
    }

    pub fn add_application(&mut self, mut app: EdgeApplication) -> bool {
        if self.can_host(&app.resource_requirements) {
            self.cpu_allocated += app.resource_requirements.cpu;
            self.memory_allocated += app.resource_requirements.memory;
            app.assigned_node_id = Some(self.id.clone());
            self.applications.push(app);
            true
        } else {
            false
        }
    }

    pub fn remove_application(&mut self, app_id: &str) -> Option<EdgeApplication> {
        if let Some(pos) = self.applications.iter().position(|a| a.id == app_id || a.name == app_id) {
            let app = self.applications.remove(pos);
            self.cpu_allocated = self.cpu_allocated.saturating_sub(app.resource_requirements.cpu);
            self.memory_allocated = self.memory_allocated.saturating_sub(app.resource_requirements.memory);
            Some(app)
        } else {
            None
        }
    }

    pub fn set_offline(&mut self) {
        self.state = EdgeNodeState::Offline;
    }

    pub fn set_online(&mut self) {
        self.state = EdgeNodeState::Online;
    }
}

/// Edge application
#[derive(Debug, Clone)]
pub struct EdgeApplication {
    pub id: String,
    pub name: String,
    pub version: String,
    pub state: EdgeAppState,
    pub resource_requirements: ResourceRequirements,
    pub sync_policy: SyncPolicy,
    pub assignment_policy: AssignmentPolicy,
    pub assigned_node_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub cpu: u32,
    pub memory: u64,
    pub storage: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPolicy {
    RealTime,
    Periodic,
    Manual,
    None,
}

impl EdgeApplication {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            id: format!("app_{}", name),
            name: name.to_string(),
            version: version.to_string(),
            state: EdgeAppState::Stopped,
            resource_requirements: ResourceRequirements {
                cpu: 1,
                memory: 1024,
                storage: 5120,
            },
            sync_policy: SyncPolicy::Periodic,
            assignment_policy: AssignmentPolicy::WorkloadBalance,
            assigned_node_id: None,
        }
    }

    pub fn start(&mut self) -> Result<(), EdgeError> {
        self.state = EdgeAppState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), EdgeError> {
        self.state = EdgeAppState::Stopped;
        Ok(())
    }

    pub fn update(&mut self, version: &str) -> Result<(), EdgeError> {
        self.state = EdgeAppState::Updating;
        self.version = version.to_string();
        self.state = EdgeAppState::Running;
        Ok(())
    }
}

/// Linux epoll / FreeBSD kqueue inspired Edge Trigger Event
#[derive(Debug, Clone)]
pub struct EdgeTriggerEvent {
    pub event_id: u64,
    pub source_node_id: String,
    pub event_type: String, // "node_down", "app_error", "high_load", "failover_triggered"
    pub payload: String,
    pub timestamp_ms: u64,
}

/// Sovereign Edge Assignment Scheduler
pub struct SovereignEdgeAssignmentScheduler {
    pub pending_events: Vec<EdgeTriggerEvent>,
    pub next_event_id: u64,
}

impl SovereignEdgeAssignmentScheduler {
    pub fn new() -> Self {
        Self {
            pending_events: Vec::new(),
            next_event_id: 1,
        }
    }

    pub fn push_event(&mut self, source_node: &str, event_type: &str, payload: &str, timestamp: u64) -> u64 {
        let id = self.next_event_id;
        self.next_event_id += 1;
        self.pending_events.push(EdgeTriggerEvent {
            event_id: id,
            source_node_id: source_node.to_string(),
            event_type: event_type.to_string(),
            payload: payload.to_string(),
            timestamp_ms: timestamp,
        });
        id
    }

    pub fn select_best_node_idx(
        &self,
        nodes: &[EdgeNode],
        app: &EdgeApplication,
        preferred_location: Option<&str>,
    ) -> Option<usize> {
        match app.assignment_policy {
            AssignmentPolicy::GeoAffinity => {
                if let Some(loc) = preferred_location {
                    if let Some(pos) = nodes.iter().position(|n| n.location == loc && n.can_host(&app.resource_requirements)) {
                        return Some(pos);
                    }
                }
                nodes.iter().enumerate().filter(|(_, n)| n.can_host(&app.resource_requirements)).max_by_key(|(_, n)| n.available_memory()).map(|(idx, _)| idx)
            }
            AssignmentPolicy::WorkloadBalance => {
                nodes.iter().enumerate().filter(|(_, n)| n.can_host(&app.resource_requirements)).max_by_key(|(_, n)| n.available_cpu()).map(|(idx, _)| idx)
            }
            AssignmentPolicy::LatencyMinimization | AssignmentPolicy::FailoverRedundancy => {
                nodes.iter().enumerate().filter(|(_, n)| n.can_host(&app.resource_requirements)).max_by_key(|(_, n)| n.available_memory()).map(|(idx, _)| idx)
            }
        }
    }
}

/// Data pipeline
#[derive(Debug, Clone)]
pub struct DataPipeline {
    pub id: String,
    pub name: String,
    pub source: String,
    pub destination: String,
    pub transformation: String,
    pub batch_size: u32,
}

impl DataPipeline {
    pub fn new(name: &str, source: &str, destination: &str) -> Self {
        Self {
            id: format!("pipeline_{}", name),
            name: name.to_string(),
            source: source.to_string(),
            destination: destination.to_string(),
            transformation: "identity".to_string(),
            batch_size: 100,
        }
    }

    pub fn set_transformation(&mut self, transformation: &str) {
        self.transformation = transformation.to_string();
    }

    pub fn set_batch_size(&mut self, batch_size: u32) {
        self.batch_size = batch_size;
    }

    pub fn process(&self) -> Result<(), EdgeError> {
        Ok(())
    }
}

/// Sync policy
#[derive(Debug, Clone)]
pub struct SyncPolicyConfig {
    pub policy: SyncPolicy,
    pub interval: u64,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    LocalWins,
    RemoteWins,
    Manual,
    Merge,
}

impl SyncPolicyConfig {
    pub fn new(policy: SyncPolicy, interval: u64) -> Self {
        Self {
            policy,
            interval,
            conflict_resolution: ConflictResolution::LocalWins,
        }
    }

    pub fn set_conflict_resolution(&mut self, resolution: ConflictResolution) {
        self.conflict_resolution = resolution;
    }
}

/// SigmaEdge - Edge Computing Platform
pub struct SigmaEdge {
    pub nodes: Vec<EdgeNode>,
    pub gateways: Vec<EdgeGateway>,
    pub data_pipelines: Vec<DataPipeline>,
    pub sync_policies: Vec<SyncPolicyConfig>,
    pub scheduler: SovereignEdgeAssignmentScheduler,
}

/// Edge gateway
#[derive(Debug, Clone)]
pub struct EdgeGateway {
    pub id: String,
    pub name: String,
    pub location: String,
    pub connected_nodes: Vec<String>,
    pub bandwidth: u32,
}

impl EdgeGateway {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            id: format!("gateway_{}", name),
            name: name.to_string(),
            location: location.to_string(),
            connected_nodes: Vec::new(),
            bandwidth: 10000,
        }
    }

    pub fn connect_node(&mut self, node_id: &str) {
        self.connected_nodes.push(node_id.to_string());
    }

    pub fn disconnect_node(&mut self, node_id: &str) {
        self.connected_nodes.retain(|id| id != node_id);
    }
}

impl SigmaEdge {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            gateways: Vec::new(),
            data_pipelines: Vec::new(),
            sync_policies: Vec::new(),
            scheduler: SovereignEdgeAssignmentScheduler::new(),
        }
    }

    pub fn add_node(&mut self, node: EdgeNode) {
        self.nodes.push(node);
    }

    pub fn get_node(&mut self, id: &str) -> Option<&mut EdgeNode> {
        self.nodes.iter_mut().find(|n| n.id == id || n.name == id)
    }

    pub fn add_gateway(&mut self, gateway: EdgeGateway) {
        self.gateways.push(gateway);
    }

    pub fn get_gateway(&mut self, id: &str) -> Option<&mut EdgeGateway> {
        self.gateways.iter_mut().find(|g| g.id == id || g.name == id)
    }

    pub fn add_data_pipeline(&mut self, pipeline: DataPipeline) {
        self.data_pipelines.push(pipeline);
    }

    pub fn add_sync_policy(&mut self, policy: SyncPolicyConfig) {
        self.sync_policies.push(policy);
    }

    pub fn deploy_application(&mut self, app: EdgeApplication, preferred_location: Option<&str>) -> Result<String, EdgeError> {
        if let Some(idx) = self.scheduler.select_best_node_idx(&self.nodes, &app, preferred_location) {
            let node = &mut self.nodes[idx];
            let node_id = node.id.clone();
            if node.add_application(app) {
                Ok(node_id)
            } else {
                Err(EdgeError::DeploymentFailed)
            }
        } else {
            Err(EdgeError::NodeNotFound)
        }
    }

    pub fn handle_node_failure(&mut self, failed_node_id: &str) -> usize {
        let mut reallocated = 0;
        let mut apps_to_reallocate = Vec::new();

        if let Some(node) = self.get_node(failed_node_id) {
            node.set_offline();
            apps_to_reallocate = node.applications.drain(..).collect();
        }

        for mut app in apps_to_reallocate {
            app.assigned_node_id = None;
            if self.deploy_application(app, None).is_ok() {
                reallocated += 1;
            }
        }

        reallocated
    }

    pub fn get_edge_stats(&self) -> EdgeStats {
        EdgeStats {
            total_nodes: self.nodes.len(),
            online_nodes: self.nodes.iter().filter(|n| n.state == EdgeNodeState::Online).count(),
            total_gateways: self.gateways.len(),
            total_applications: self.nodes.iter().map(|n| n.applications.len()).sum(),
            running_applications: self.nodes.iter()
                .flat_map(|n| n.applications.iter())
                .filter(|a| a.state == EdgeAppState::Running)
                .count(),
            total_pipelines: self.data_pipelines.len(),
        }
    }

    pub fn list_nodes(&self) -> Vec<&EdgeNode> {
        self.nodes.iter().collect()
    }

    pub fn list_gateways(&self) -> Vec<&EdgeGateway> {
        self.gateways.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct EdgeStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub total_gateways: usize,
    pub total_applications: usize,
    pub running_applications: usize,
    pub total_pipelines: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeError {
    NodeNotFound,
    GatewayNotFound,
    ApplicationNotFound,
    DeploymentFailed,
    UpdateFailed,
    SyncFailed,
}

impl Default for SigmaEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_node_creation_and_capacity() {
        let mut node = EdgeNode::new("edge-1", "datacenter-1");
        assert_eq!(node.name, "edge-1");
        assert_eq!(node.available_cpu(), 8);

        let app = EdgeApplication::new("worker", "1.0");
        assert!(node.add_application(app));
        assert_eq!(node.available_cpu(), 7);
    }

    #[test]
    fn test_auto_assignment_policy_and_failover() {
        let mut edge = SigmaEdge::new();
        let mut n1 = EdgeNode::new("node-a", "us-east");
        let mut n2 = EdgeNode::new("node-b", "us-west");

        n1.cpu_capacity = 4;
        n2.cpu_capacity = 16;

        edge.add_node(n1);
        edge.add_node(n2);

        let mut app = EdgeApplication::new("analytic-engine", "2.0");
        app.assignment_policy = AssignmentPolicy::WorkloadBalance;

        let deployed_node_id = edge.deploy_application(app, None).unwrap();
        assert_eq!(deployed_node_id, "edge_node-b"); // Picked node-b with 16 CPUs

        // Failover testing
        let reallocated_count = edge.handle_node_failure("edge_node-b");
        assert_eq!(reallocated_count, 1);
        assert_eq!(edge.get_node("edge_node-a").unwrap().applications.len(), 1);
    }

    #[test]
    fn test_edge_trigger_events() {
        let mut scheduler = SovereignEdgeAssignmentScheduler::new();
        let ev_id = scheduler.push_event("edge_1", "app_error", "OOM killed", 1000);
        assert_eq!(ev_id, 1);
        assert_eq!(scheduler.pending_events.len(), 1);
    }
}
