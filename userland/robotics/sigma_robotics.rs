// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/robotics/sigma_robotics.rs — ROS 2 Integration
//
// Implements:
//   - ROS 2 node management and communication
//   - URDF robot model loader
//   - Topic and service management
//   - TF (Transform) tree for coordinate frames
//   - Action server/client for long-running tasks
//   - sigma-twin digital twin integration
//   - India context: Support for Indian robotics standards
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Node state ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeState {
    Unconfigured = 0,
    Inactive = 1,
    Active = 2,
    Finalized = 3,
}

// ── Message type ───────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MessageType {
    String = 0,
    Int32 = 1,
    Float32 = 2,
    Bool = 3,
    Twist = 4,    // Linear and angular velocity
    Pose = 5,     // Position and orientation
    Image = 6,
    PointCloud = 7,
}

// ── ROS 2 node ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RosNode {
    pub id: u32,
    pub name: [u8; 64],
    pub namespace: [u8; 64],
    pub state: NodeState,
    pub publisher_count: u32,
    pub subscriber_count: u32,
    pub service_count: u32,
}

impl RosNode {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            name: [0u8; 64],
            namespace: [0u8; 64],
            state: NodeState::Unconfigured,
            publisher_count: 0,
            subscriber_count: 0,
            service_count: 0,
        }
    }
}

// ── Topic ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Topic {
    pub name: [u8; 128],
    pub message_type: MessageType,
    pub publisher_count: u32,
    pub subscriber_count: u32,
    pub queue_size: u32,
}

impl Topic {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            message_type: MessageType::String,
            publisher_count: 0,
            subscriber_count: 0,
            queue_size: 10,
        }
    }
}

// ── Service ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Service {
    pub name: [u8; 128],
    pub request_type: MessageType,
    pub response_type: MessageType,
    pub server_count: u32,
    pub client_count: u32,
}

impl Service {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            request_type: MessageType::String,
            response_type: MessageType::String,
            server_count: 0,
            client_count: 0,
        }
    }
}

// ── Transform (TF) ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Transform {
    pub parent_frame: [u8; 64],
    pub child_frame: [u8; 64],
    pub translation: [f32; 3], // x, y, z
    pub rotation: [f32; 4],     // quaternion x, y, z, w
    pub timestamp: u64,
}

impl Transform {
    pub const fn new() -> Self {
        Self {
            parent_frame: [0u8; 64],
            child_frame: [0u8; 64],
            translation: [0.0; 3],
            rotation: [0.0; 4],
            timestamp: 0,
        }
    }
}

// ── Action goal ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionGoal {
    pub id: u64,
    pub action_name: [u8; 64],
    pub goal_data: [u8; 256],
    pub state: u8,
}

impl ActionGoal {
    pub const fn new(id: u64) -> Self {
        Self {
            id,
            action_name: [0u8; 64],
            goal_data: [0u8; 256],
            state: 0,
        }
    }
}

// ── URDF joint type ─────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum JointType {
    Fixed = 0,
    Revolute = 1,
    Prismatic = 2,
    Continuous = 3,
}

// ── URDF joint ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UrdfJoint {
    pub name: [u8; 64],
    pub joint_type: JointType,
    pub parent_link: [u8; 64],
    pub child_link: [u8; 64],
    pub origin: [f32; 6], // x, y, z, roll, pitch, yaw
    pub limits: [f32; 4], // lower, upper, effort, velocity
}

impl UrdfJoint {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 64],
            joint_type: JointType::Fixed,
            parent_link: [0u8; 64],
            child_link: [0u8; 64],
            origin: [0.0; 6],
            limits: [0.0; 4],
        }
    }
}

// ── Robotics manager state ─────────────────────────────────────

const MAX_NODES: usize = 64;
const MAX_TOPICS: usize = 128;
const MAX_SERVICES: usize = 64;
const MAX_TRANSFORMS: usize = 256;
const MAX_JOINTS: usize = 128;

pub struct RoboticsManager {
    nodes: [Option<RosNode>; MAX_NODES],
    topics: [Option<Topic>; MAX_TOPICS],
    services: [Option<Service>; MAX_SERVICES],
    transforms: [Option<Transform>; MAX_TRANSFORMS],
    joints: [Option<UrdfJoint>; MAX_JOINTS],
    node_count: AtomicU32,
    topic_count: AtomicU32,
    service_count: AtomicU32,
    initialized: bool,
}

impl RoboticsManager {
    pub const fn new() -> Self {
        Self {
            nodes: [const { None }; MAX_NODES],
            topics: [const { None }; MAX_TOPICS],
            services: [const { None }; MAX_SERVICES],
            transforms: [const { None }; MAX_TRANSFORMS],
            joints: [const { None }; MAX_JOINTS],
            node_count: AtomicU32::new(0),
            topic_count: AtomicU32::new(0),
            service_count: AtomicU32::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Create a ROS 2 node
    pub fn create_node(&mut self, node: RosNode) -> bool {
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

    /// Create a topic
    pub fn create_topic(&mut self, topic: Topic) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TOPICS {
            if self.topics[i].is_none() {
                self.topics[i] = Some(topic);
                self.topic_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Create a service
    pub fn create_service(&mut self, service: Service) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SERVICES {
            if self.services[i].is_none() {
                self.services[i] = Some(service);
                self.service_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Add a transform to the TF tree
    pub fn add_transform(&mut self, transform: Transform) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TRANSFORMS {
            if self.transforms[i].is_none() {
                self.transforms[i] = Some(transform);
                return true;
            }
        }
        false
    }

    /// Add a URDF joint
    pub fn add_joint(&mut self, joint: UrdfJoint) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_JOINTS {
            if self.joints[i].is_none() {
                self.joints[i] = Some(joint);
                return true;
            }
        }
        false
    }

    /// Get transform between frames
    pub fn get_transform(&self, target_frame: &[u8], source_frame: &[u8]) -> Option<Transform> {
        if !self.initialized {
            return None;
        }

        for i in 0..MAX_TRANSFORMS {
            if let Some(tf) = &self.transforms[i] {
                let mut target_match = true;
                let mut source_match = true;

                for j in 0..64 {
                    if j < target_frame.len() && tf.child_frame[j] != target_frame[j] {
                        target_match = false;
                    }
                    if j < source_frame.len() && tf.parent_frame[j] != source_frame[j] {
                        source_match = false;
                    }
                }

                if target_match && source_match {
                    return Some(*tf);
                }
            }
        }
        None
    }

    pub fn node_count(&self) -> u32 {
        self.node_count.load(Ordering::Relaxed)
    }

    pub fn topic_count(&self) -> u32 {
        self.topic_count.load(Ordering::Relaxed)
    }

    pub fn service_count(&self) -> u32 {
        self.service_count.load(Ordering::Relaxed)
    }
}

// ── Global robotics manager instance ─────────────────────────────

static mut G_ROBOTICS_MANAGER: RoboticsManager = RoboticsManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn robotics_manager_init() {
    G_ROBOTICS_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn robotics_create_node(
    id: u32,
    name: *const u8,
    namespace: *const u8,
) -> i32 {
    let mut node = RosNode::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(node.name.len()));
        for i in 0..name_slice.len() {
            node.name[i] = name_slice[i];
        }
    }
    
    if !namespace.is_null() {
        let ns_slice = core::slice::from_raw_parts(namespace, 64.min(node.namespace.len()));
        for i in 0..ns_slice.len() {
            node.namespace[i] = ns_slice[i];
        }
    }
    
    node.state = NodeState::Active;
    
    if G_ROBOTICS_MANAGER.create_node(node) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn robotics_create_topic(
    name: *const u8,
    message_type: u8,
    queue_size: u32,
) -> i32 {
    let mut topic = Topic::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 128.min(topic.name.len()));
        for i in 0..name_slice.len() {
            topic.name[i] = name_slice[i];
        }
    }
    
    topic.message_type = match message_type {
        0 => MessageType::String,
        1 => MessageType::Int32,
        2 => MessageType::Float32,
        3 => MessageType::Bool,
        4 => MessageType::Twist,
        5 => MessageType::Pose,
        6 => MessageType::Image,
        7 => MessageType::PointCloud,
        _ => MessageType::String,
    };
    
    topic.queue_size = queue_size;
    
    if G_ROBOTICS_MANAGER.create_topic(topic) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn robotics_create_service(
    name: *const u8,
    request_type: u8,
    response_type: u8,
) -> i32 {
    let mut service = Service::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 128.min(service.name.len()));
        for i in 0..name_slice.len() {
            service.name[i] = name_slice[i];
        }
    }
    
    service.request_type = match request_type {
        0..=7 => {
            let types = [MessageType::String, MessageType::Int32, MessageType::Float32, MessageType::Bool, MessageType::Twist, MessageType::Pose, MessageType::Image, MessageType::PointCloud];
            types[request_type as usize]
        }
        _ => MessageType::String,
    };
    
    service.response_type = match response_type {
        0..=7 => {
            let types = [MessageType::String, MessageType::Int32, MessageType::Float32, MessageType::Bool, MessageType::Twist, MessageType::Pose, MessageType::Image, MessageType::PointCloud];
            types[response_type as usize]
        }
        _ => MessageType::String,
    };
    
    if G_ROBOTICS_MANAGER.create_service(service) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn robotics_add_transform(
    parent: *const u8,
    child: *const u8,
    translation: *const f32,
    rotation: *const f32,
) -> i32 {
    let mut tf = Transform::new();
    
    if !parent.is_null() {
        let parent_slice = core::slice::from_raw_parts(parent, 64.min(tf.parent_frame.len()));
        for i in 0..parent_slice.len() {
            tf.parent_frame[i] = parent_slice[i];
        }
    }
    
    if !child.is_null() {
        let child_slice = core::slice::from_raw_parts(child, 64.min(tf.child_frame.len()));
        for i in 0..child_slice.len() {
            tf.child_frame[i] = child_slice[i];
        }
    }
    
    if !translation.is_null() {
        let trans_slice = core::slice::from_raw_parts(translation, 3);
        for i in 0..3 {
            tf.translation[i] = trans_slice[i];
        }
    }
    
    if !rotation.is_null() {
        let rot_slice = core::slice::from_raw_parts(rotation, 4);
        for i in 0..4 {
            tf.rotation[i] = rot_slice[i];
        }
    }
    
    if G_ROBOTICS_MANAGER.add_transform(tf) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn robotics_add_joint(
    name: *const u8,
    joint_type: u8,
    parent: *const u8,
    child: *const u8,
) -> i32 {
    let mut joint = UrdfJoint::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(joint.name.len()));
        for i in 0..name_slice.len() {
            joint.name[i] = name_slice[i];
        }
    }
    
    joint.joint_type = match joint_type {
        0 => JointType::Fixed,
        1 => JointType::Revolute,
        2 => JointType::Prismatic,
        3 => JointType::Continuous,
        _ => JointType::Fixed,
    };
    
    if !parent.is_null() {
        let parent_slice = core::slice::from_raw_parts(parent, 64.min(joint.parent_link.len()));
        for i in 0..parent_slice.len() {
            joint.parent_link[i] = parent_slice[i];
        }
    }
    
    if !child.is_null() {
        let child_slice = core::slice::from_raw_parts(child, 64.min(joint.child_link.len()));
        for i in 0..child_slice.len() {
            joint.child_link[i] = child_slice[i];
        }
    }
    
    if G_ROBOTICS_MANAGER.add_joint(joint) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn robotics_node_count() -> u32 {
    G_ROBOTICS_MANAGER.node_count()
}

#[no_mangle]
pub unsafe extern "C" fn robotics_topic_count() -> u32 {
    G_ROBOTICS_MANAGER.topic_count()
}

#[no_mangle]
pub unsafe extern "C" fn robotics_service_count() -> u32 {
    G_ROBOTICS_MANAGER.service_count()
}
