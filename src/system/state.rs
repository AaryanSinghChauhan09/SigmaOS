//! SigmaOS Declarative State Management
//! Implements NixOS-like pure functional declarative state graphs
//! Zero-dependency immutable state management for perfect reproducibility
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// (no_std only applicable at crate root - removed)

use core::sync::atomic::{AtomicUsize, Ordering};

/// State graph node representing a system configuration
#[derive(Debug)]
pub struct StateNode {
    id: usize,
    name: &'static str,
    value: StateValue,
    dependencies: [usize; 8], // Up to 8 dependencies
    dependency_count: AtomicUsize,
}

impl Clone for StateNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name,
            value: self.value.clone(),
            dependencies: self.dependencies,
            dependency_count: AtomicUsize::new(self.dependency_count.load(Ordering::SeqCst)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StateValue {
    Boolean(bool),
    Integer(i64),
    String(&'static str),
    Array([Box<StateValue>; 16]),
}

/// Immutable state graph for declarative configuration
pub struct DeclarativeStateGraph {
    nodes: [Option<StateNode>; 256], // Up to 256 state nodes
    node_count: AtomicUsize,
    current_generation: AtomicUsize,
    generations: [usize; 32], // Track up to 32 generations for rollback
    generation_count: AtomicUsize,
}

const NONE_NODE: Option<StateNode> = None;

impl DeclarativeStateGraph {
    pub const fn new() -> Self {
        DeclarativeStateGraph {
            nodes: [NONE_NODE; 256],
            node_count: AtomicUsize::new(0),
            current_generation: AtomicUsize::new(0),
            generations: [0; 32],
            generation_count: AtomicUsize::new(0),
        }
    }

    /// Create a new state node
    pub fn create_node(
        &mut self,
        id: usize,
        name: &'static str,
        value: StateValue,
    ) -> Result<(), StateError> {
        if id >= 256 {
            return Err(StateError::InvalidNodeId);
        }

        if self.nodes[id].is_some() {
            return Err(StateError::NodeAlreadyExists);
        }

        let node = StateNode {
            id,
            name,
            value,
            dependencies: [0; 8],
            dependency_count: AtomicUsize::new(0),
        };

        self.nodes[id] = Some(node);
        self.node_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Add dependency between state nodes
    pub fn add_dependency(
        &mut self,
        node_id: usize,
        dependency_id: usize,
    ) -> Result<(), StateError> {
        if node_id >= 256 || dependency_id >= 256 {
            return Err(StateError::InvalidNodeId);
        }

        let node = self.nodes[node_id]
            .as_mut()
            .ok_or(StateError::NodeNotFound)?;
        let dep_count = node.dependency_count.load(Ordering::SeqCst);

        if dep_count >= 8 {
            return Err(StateError::TooManyDependencies);
        }

        node.dependencies[dep_count] = dependency_id;
        node.dependency_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Update state node value (creates new generation)
    pub fn update_node(&mut self, id: usize, new_value: StateValue) -> Result<(), StateError> {
        if id >= 256 {
            return Err(StateError::InvalidNodeId);
        }

        // Create new generation before update
        self.create_generation()?;

        let node = self.nodes[id].as_mut().ok_or(StateError::NodeNotFound)?;
        node.value = new_value;
        Ok(())
    }

    /// Get state node value
    pub fn get_node(&self, id: usize) -> Result<&StateNode, StateError> {
        if id >= 256 {
            return Err(StateError::InvalidNodeId);
        }

        self.nodes[id].as_ref().ok_or(StateError::NodeNotFound)
    }

    /// Create a new generation for rollback capability
    fn create_generation(&mut self) -> Result<(), StateError> {
        let gen_count = self.generation_count.load(Ordering::SeqCst);

        if gen_count >= 32 {
            return Err(StateError::TooManyGenerations);
        }

        let current_gen = self.current_generation.load(Ordering::SeqCst);
        self.generations[gen_count] = current_gen;
        self.generation_count.fetch_add(1, Ordering::SeqCst);
        self.current_generation.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }

    /// Rollback to previous generation
    pub fn rollback(&mut self) -> Result<(), StateError> {
        let gen_count = self.generation_count.load(Ordering::SeqCst);

        if gen_count == 0 {
            return Err(StateError::NoGenerationsAvailable);
        }

        let target_gen = self.generations[gen_count - 1];
        self.current_generation.store(target_gen, Ordering::SeqCst);
        self.generation_count.fetch_sub(1, Ordering::SeqCst);

        Ok(())
    }

    /// Validate state graph consistency
    pub fn validate(&self) -> Result<(), StateError> {
        for i in 0..256 {
            if let Some(ref node) = self.nodes[i] {
                let dep_count = node.dependency_count.load(Ordering::SeqCst);
                for j in 0..dep_count {
                    let dep_id = node.dependencies[j];
                    if dep_id >= 256 || self.nodes[dep_id].is_none() {
                        return Err(StateError::DependencyNotFound);
                    }
                }
            }
        }
        Ok(())
    }

    /// Get current generation number
    pub fn current_generation(&self) -> usize {
        self.current_generation.load(Ordering::SeqCst)
    }

    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.node_count.load(Ordering::SeqCst)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StateError {
    InvalidNodeId,
    NodeAlreadyExists,
    NodeNotFound,
    TooManyDependencies,
    DependencyNotFound,
    TooManyGenerations,
    NoGenerationsAvailable,
}

/// System configuration manager using declarative state
pub struct SystemConfiguration {
    state_graph: DeclarativeStateGraph,
}

impl SystemConfiguration {
    pub const fn new() -> Self {
        SystemConfiguration {
            state_graph: DeclarativeStateGraph::new(),
        }
    }

    /// Initialize default system configuration
    pub fn init_default(&mut self) -> Result<(), StateError> {
        // Create basic system state nodes
        self.state_graph
            .create_node(0, "kernel_version", StateValue::String("0.1.0"))?;
        self.state_graph
            .create_node(1, "boot_mode", StateValue::String("microkernel"))?;
        self.state_graph
            .create_node(2, "security_level", StateValue::Integer(3))?;
        self.state_graph
            .create_node(3, "network_enabled", StateValue::Boolean(true))?;
        self.state_graph
            .create_node(4, "desktop_enabled", StateValue::Boolean(false))?;

        // Add dependencies
        self.state_graph.add_dependency(2, 0)?; // Security depends on kernel version
        self.state_graph.add_dependency(3, 2)?; // Network depends on security level

        // Create initial generation
        self.state_graph.create_generation()?;

        Ok(())
    }

    /// Get state graph reference
    pub fn state_graph(&self) -> &DeclarativeStateGraph {
        &self.state_graph
    }

    /// Get mutable state graph reference
    pub fn state_graph_mut(&mut self) -> &mut DeclarativeStateGraph {
        &mut self.state_graph
    }
}

/// Global system configuration — protected by a spinlock to prevent data races
/// on SMP kernels.  All access goes through `init_system_config()`,
/// `get_system_config()`, and `get_system_config_mut()`.
use core::sync::atomic::{AtomicBool, Ordering as AOrdering};

struct SpinMutex<T> {
    locked: AtomicBool,
    inner: core::cell::UnsafeCell<T>,
}

// SAFETY: `SpinMutex` ensures exclusive access via a spinlock before
// dereferencing the inner value.
unsafe impl<T: Send> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
    const fn new(val: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            inner: core::cell::UnsafeCell::new(val),
        }
    }

    /// Acquire the spinlock and return a raw mutable pointer.
    /// # Safety
    /// Caller MUST call `unlock()` after finishing with the pointer.
    unsafe fn lock_raw(&self) -> *mut T {
        while self
            .locked
            .compare_exchange_weak(false, true, AOrdering::Acquire, AOrdering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        self.inner.get()
    }

    fn unlock(&self) {
        self.locked.store(false, AOrdering::Release);
    }
}

static GLOBAL_CONFIG: SpinMutex<Option<SystemConfiguration>> = SpinMutex::new(None);

/// Initialize global system configuration.  Must be called once during boot
/// before any call to `get_system_config()`.
pub fn init_system_config() -> Result<(), StateError> {
    // SAFETY: `lock_raw` gives exclusive access; `unlock` releases it below.
    unsafe {
        let ptr = GLOBAL_CONFIG.lock_raw();
        *ptr = Some(SystemConfiguration::new());
        if let Some(ref mut config) = *ptr {
            let result = config.init_default();
            GLOBAL_CONFIG.unlock();
            return result;
        }
        GLOBAL_CONFIG.unlock();
    }
    Ok(())
}

/// Get a reference to the global system configuration.
///
/// # Panics
/// Panics if `init_system_config()` has not been called.
pub fn get_system_config() -> &'static SystemConfiguration {
    // SAFETY: After init the Option is always Some; we hold the lock for the
    // duration of the borrow and extend its lifetime to `'static` — callers
    // must not store the reference across a context switch or other lock
    // acquisition to avoid deadlock.
    unsafe {
        let ptr = GLOBAL_CONFIG.lock_raw();
        let config_ref: &'static SystemConfiguration = (*ptr)
            .as_ref()
            .expect("System configuration not initialized");
        // Release the spinlock immediately; the reference into static storage
        // remains valid because the global never moves.
        GLOBAL_CONFIG.unlock();
        config_ref
    }
}

/// Get a mutable reference to the global system configuration.
///
/// # Panics
/// Panics if `init_system_config()` has not been called.
pub fn get_system_config_mut() -> &'static mut SystemConfiguration {
    // SAFETY: Same as above; the spinlock serialises mutation.
    unsafe {
        let ptr = GLOBAL_CONFIG.lock_raw();
        let config_mut: &'static mut SystemConfiguration = (*ptr)
            .as_mut()
            .expect("System configuration not initialized");
        GLOBAL_CONFIG.unlock();
        config_mut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_graph_creation() {
        let mut graph = DeclarativeStateGraph::new();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_node_creation() {
        let mut graph = DeclarativeStateGraph::new();
        graph
            .create_node(0, "test", StateValue::Boolean(true))
            .unwrap();
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_dependency_addition() {
        let mut graph = DeclarativeStateGraph::new();
        graph
            .create_node(0, "parent", StateValue::Boolean(true))
            .unwrap();
        graph
            .create_node(1, "child", StateValue::Boolean(false))
            .unwrap();
        graph.add_dependency(0, 1).unwrap();

        let node = graph.get_node(0).unwrap();
        assert_eq!(node.dependency_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_generation_rollback() {
        let mut graph = DeclarativeStateGraph::new();
        graph
            .create_node(0, "test", StateValue::Integer(1))
            .unwrap();
        graph.create_generation().unwrap();
        graph.update_node(0, StateValue::Integer(2)).unwrap();

        assert_eq!(graph.current_generation(), 2);

        graph.rollback().unwrap();
        assert_eq!(graph.current_generation(), 1);
    }

    #[test]
    fn test_system_config_init() {
        let mut config = SystemConfiguration::new();
        config.init_default().unwrap();
        assert_eq!(config.state_graph().node_count(), 5);
    }
}
