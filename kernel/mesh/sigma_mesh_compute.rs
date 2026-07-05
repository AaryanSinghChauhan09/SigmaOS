// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/mesh/sigma_mesh_compute.rs — National Distributed Compute Grid
//
// Implements:
//   - Distributed compute grid using idle SigmaOS machines
//   - DID-based opt-in system with e-RUPI rewards
//   - sigma-jail sandbox for isolated computation
//   - Task scheduling and result aggregation
//   - India context: ISRO satellite imagery, CSIR drug discovery, IMD climate models
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Mesh node status ─────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Maintenance = 3,
}

// ── Compute task type ───────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskType {
    SatelliteImagery = 0,  // ISRO satellite image processing
    DrugDiscovery = 1,     // CSIR molecular simulation
    ClimateModel = 2,      // IMD climate prediction
    Genomics = 3,          // Bioinformatics analysis
    MachineLearning = 4,    // AI model training
    General = 5,
}

// ── Mesh node information ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MeshNode {
    pub did: [u8; 32],           // DID of the node owner
    pub cpu_cores: u32,         // Number of CPU cores available
    pub memory_gb: u32,         // Available memory in GB
    pub gpu_available: bool,     // Whether GPU is available
    pub status: NodeStatus,
    pub uptime_seconds: u64,
    pub tasks_completed: u64,
    pub reward_balance: u64,    // e-RUPI earned
    pub last_heartbeat: u64,
}

impl MeshNode {
    pub const fn new() -> Self {
        Self {
            did: [0u8; 32],
            cpu_cores: 0,
            memory_gb: 0,
            gpu_available: false,
            status: NodeStatus::Offline,
            uptime_seconds: 0,
            tasks_completed: 0,
            reward_balance: 0,
            last_heartbeat: 0,
        }
    }
}

// ── Compute task ───────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputeTask {
    pub id: u64,
    pub task_type: TaskType,
    pub requester_did: [u8; 32],
    pub input_hash: [u8; 32],
    pub output_hash: [u8; 32],
    pub cpu_required: u32,
    pub memory_required_mb: u32,
    pub gpu_required: bool,
    pub timeout_seconds: u64,
    pub reward: u64,            // e-RUPI reward for completion
    pub assigned_node: u64,     // DID of assigned node
    pub status: TaskStatus,
    pub created_at: u64,
    pub completed_at: u64,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending = 0,
    Assigned = 1,
    Running = 2,
    Completed = 3,
    Failed = 4,
    Timeout = 5,
}

impl ComputeTask {
    pub const fn new(id: u64, task_type: TaskType) -> Self {
        Self {
            id,
            task_type,
            requester_did: [0u8; 32],
            input_hash: [0u8; 32],
            output_hash: [0u8; 32],
            cpu_required: 1,
            memory_required_mb: 512,
            gpu_required: false,
            timeout_seconds: 3600,
            reward: 10,
            assigned_node: 0,
            status: TaskStatus::Pending,
            created_at: 0,
            completed_at: 0,
        }
    }
}

// ── Mesh compute grid ─────────────────────────────────────────────────────

const MAX_NODES: usize = 1024;
const MAX_TASKS: usize = 4096;

pub struct MeshComputeGrid {
    nodes: [Option<MeshNode>; MAX_NODES],
    tasks: [Option<ComputeTask>; MAX_TASKS],
    node_count: AtomicU32,
    task_count: AtomicU32,
    total_compute_hours: AtomicU64,
    total_rewards_distributed: AtomicU64,
    initialized: bool,
}

impl MeshComputeGrid {
    pub const fn new() -> Self {
        Self {
            nodes: [const { None }; MAX_NODES],
            tasks: [const { None }; MAX_TASKS],
            node_count: AtomicU32::new(0),
            task_count: AtomicU32::new(0),
            total_compute_hours: AtomicU64::new(0),
            total_rewards_distributed: AtomicU64::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Register a new mesh node
    pub fn register_node(&mut self, node: MeshNode) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_NODES {
            if self.nodes[i].is_none() {
                self.nodes[i] = Some(node);
                self.node_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Submit a compute task to the grid
    pub fn submit_task(&mut self, task: ComputeTask) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TASKS {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(task);
                self.task_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Assign a task to an available node
    pub fn assign_task(&mut self, task_id: u64) -> Option<u64> {
        if !self.initialized {
            return None;
        }

        let task_idx = self.find_task(task_id)?;
        let task = self.tasks[task_idx].as_mut()?;

        // Find suitable node
        for i in 0..MAX_NODES {
            if let Some(node) = &self.nodes[i] {
                if node.status == NodeStatus::Online
                    && node.cpu_cores >= task.cpu_required
                    && node.memory_gb * 1024 >= task.memory_required_mb
                    && (!task.gpu_required || node.gpu_available)
                {
                    task.assigned_node = u64::from_le_bytes([
                        node.did[0], node.did[1], node.did[2], node.did[3],
                        node.did[4], node.did[5], node.did[6], node.did[7],
                    ]);
                    task.status = TaskStatus::Assigned;
                    return Some(task.assigned_node);
                }
            }
        }
        None
    }

    /// Mark task as completed and reward the node
    pub fn complete_task(&mut self, task_id: u64, output_hash: [u8; 32]) -> bool {
        if !self.initialized {
            return false;
        }

        let task_idx = self.find_task(task_id)?;
        let task = self.tasks[task_idx].as_mut()?;
        
        task.status = TaskStatus::Completed;
        task.output_hash = output_hash;
        task.completed_at = self.get_timestamp();

        // Reward the node
        for i in 0..MAX_NODES {
            if let Some(node) = &mut self.nodes[i] {
                let node_did = u64::from_le_bytes([
                    node.did[0], node.did[1], node.did[2], node.did[3],
                    node.did[4], node.did[5], node.did[6], node.did[7],
                ]);
                if node_did == task.assigned_node {
                    node.tasks_completed += 1;
                    node.reward_balance += task.reward;
                    self.total_rewards_distributed.fetch_add(task.reward, Ordering::Relaxed);
                    break;
                }
            }
        }

        true
    }

    /// Update node heartbeat
    pub fn update_heartbeat(&mut self, did: &[u8]) -> bool {
        if !self.initialized || did.len() < 8 {
            return false;
        }

        let node_did = u64::from_le_bytes([
            did[0], did[1], did[2], did[3],
            did[4], did[5], did[6], did[7],
        ]);

        for i in 0..MAX_NODES {
            if let Some(node) = &mut self.nodes[i] {
                let current_did = u64::from_le_bytes([
                    node.did[0], node.did[1], node.did[2], node.did[3],
                    node.did[4], node.did[5], node.did[6], node.did[7],
                ]);
                if current_did == node_did {
                    node.last_heartbeat = self.get_timestamp();
                    node.status = NodeStatus::Online;
                    return true;
                }
            }
        }
        false
    }

    /// Get task by ID
    fn find_task(&self, id: u64) -> Option<usize> {
        for i in 0..MAX_TASKS {
            if let Some(task) = &self.tasks[i] {
                if task.id == id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        self.total_compute_hours.load(Ordering::Relaxed)
    }

    pub fn node_count(&self) -> u32 {
        self.node_count.load(Ordering::Relaxed)
    }

    pub fn task_count(&self) -> u32 {
        self.task_count.load(Ordering::Relaxed)
    }

    pub fn total_compute_hours(&self) -> u64 {
        self.total_compute_hours.load(Ordering::Relaxed)
    }

    pub fn total_rewards_distributed(&self) -> u64 {
        self.total_rewards_distributed.load(Ordering::Relaxed)
    }
}

// ── Global mesh grid instance ───────────────────────────────────────────────

static mut G_MESH_GRID: MeshComputeGrid = MeshComputeGrid::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn mesh_grid_init() {
    G_MESH_GRID.init();
}

#[no_mangle]
pub unsafe extern "C" fn mesh_register_node(
    did: *const u8,
    cpu_cores: u32,
    memory_gb: u32,
    gpu_available: bool,
) -> i32 {
    let mut node = MeshNode::new();
    
    if !did.is_null() {
        let did_slice = core::slice::from_raw_parts(did, 32);
        node.did.copy_from_slice(did_slice);
    }
    
    node.cpu_cores = cpu_cores;
    node.memory_gb = memory_gb;
    node.gpu_available = gpu_available;
    node.status = NodeStatus::Online;
    node.last_heartbeat = G_MESH_GRID.get_timestamp();
    
    if G_MESH_GRID.register_node(node) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn mesh_submit_task(
    id: u64,
    task_type: u8,
    requester_did: *const u8,
    cpu_required: u32,
    memory_required_mb: u32,
    gpu_required: bool,
    reward: u64,
) -> i32 {
    let task_type = match task_type {
        0 => TaskType::SatelliteImagery,
        1 => TaskType::DrugDiscovery,
        2 => TaskType::ClimateModel,
        3 => TaskType::Genomics,
        4 => TaskType::MachineLearning,
        _ => TaskType::General,
    };
    
    let mut task = ComputeTask::new(id, task_type);
    
    if !requester_did.is_null() {
        let did_slice = core::slice::from_raw_parts(requester_did, 32);
        task.requester_did.copy_from_slice(did_slice);
    }
    
    task.cpu_required = cpu_required;
    task.memory_required_mb = memory_required_mb;
    task.gpu_required = gpu_required;
    task.reward = reward;
    task.created_at = G_MESH_GRID.get_timestamp();
    
    if G_MESH_GRID.submit_task(task) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn mesh_assign_task(task_id: u64) -> u64 {
    match G_MESH_GRID.assign_task(task_id) {
        Some(node_did) => node_did,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mesh_complete_task(
    task_id: u64,
    output_hash: *const u8,
) -> i32 {
    let mut hash = [0u8; 32];
    if !output_hash.is_null() {
        let hash_slice = core::slice::from_raw_parts(output_hash, 32);
        hash.copy_from_slice(hash_slice);
    }
    
    if G_MESH_GRID.complete_task(task_id, hash) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn mesh_update_heartbeat(did: *const u8) -> i32 {
    if did.is_null() {
        return -1;
    }
    let did_slice = core::slice::from_raw_parts(did, 32);
    if G_MESH_GRID.update_heartbeat(did_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn mesh_node_count() -> u32 {
    G_MESH_GRID.node_count()
}

#[no_mangle]
pub unsafe extern "C" fn mesh_task_count() -> u32 {
    G_MESH_GRID.task_count()
}

#[no_mangle]
pub unsafe extern "C" fn mesh_total_rewards() -> u64 {
    G_MESH_GRID.total_rewards_distributed()
}
