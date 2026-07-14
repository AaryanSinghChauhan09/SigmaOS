/// SigmaOS: ============================================================================
/// Shard Traits - OOP Abstractions for Microkernel Components
/// ============================================================================
/// Defines common interfaces for all kernel shards using Rust traits.
/// This enables polymorphism, composition, and reduced coupling between components.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ============================================================================
// CORE SHARD TRAITS
// ============================================================================

/// Base trait for all kernel shards
pub trait Shard {
    /// Initialize the shard
    fn init(&self) -> Result<(), ShardError>;
    
    /// Check if shard is operational
    fn is_operational(&self) -> bool;
    
    /// Get shard identifier
    fn shard_id(&self) -> u64;
    
    /// Gracefully shutdown the shard
    fn shutdown(&self) -> Result<(), ShardError>;
}

/// Trait for schedulable entities
pub trait Schedulable: Shard {
    /// Get current priority
    fn priority(&self) -> u8;
    
    /// Set priority
    fn set_priority(&self, priority: u8);
    
    /// Get CPU affinity mask
    fn cpu_affinity(&self) -> u64;
    
    /// Set CPU affinity mask
    fn set_cpu_affinity(&self, mask: u64);
}

/// Trait for IPC-capable shards
pub trait IpcCapable: Shard {
    /// Send message to another shard
    fn send(&self, target: u64, message: &[u8]) -> Result<(), IpcError>;
    
    /// Receive message from another shard
    fn receive(&self) -> Result<Vec<u8>, IpcError>;
    
    /// Get message queue depth
    fn queue_depth(&self) -> usize;
}

/// Trait for memory-managing shards
pub trait MemoryManager: Shard {
    /// Allocate memory region
    fn allocate(&self, size: usize) -> Result<u64, MemoryError>;
    
    /// Free memory region
    fn free(&self, addr: u64) -> Result<(), MemoryError>;
    
    /// Get memory usage statistics
    fn memory_stats(&self) -> MemoryStats;
}

/// Trait for security-enforcing shards
pub trait SecurityEnforcer: Shard {
    /// Check capability for operation
    fn check_capability(&self, cap: u64) -> bool;
    
    /// Grant capability
    fn grant_capability(&self, cap: u64) -> Result<(), SecurityError>;
    
    /// Revoke capability
    fn revoke_capability(&self, cap: u64) -> Result<(), SecurityError>;
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub enum ShardError {
    InitializationFailed,
    AlreadyInitialized,
    NotOperational,
    InvalidState,
    ResourceExhausted,
}

#[derive(Debug, Clone, Copy)]
pub enum IpcError {
    QueueFull,
    QueueEmpty,
    InvalidTarget,
    PermissionDenied,
    MessageTooLarge,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryError {
    OutOfMemory,
    InvalidAddress,
    AlignmentError,
    PermissionDenied,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityError {
    AccessDenied,
    InvalidCapability,
    PermissionRevoked,
}

// ============================================================================
// STATISTICS STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub cached: u64,
}

// ============================================================================
// BASE SHARD IMPLEMENTATION
// ============================================================================

pub struct BaseShard {
    shard_id: AtomicU64,
    operational: AtomicBool,
}

impl BaseShard {
    pub const fn new(id: u64) -> Self {
        Self {
            shard_id: AtomicU64::new(id),
            operational: AtomicBool::new(false),
        }
    }
}

impl Shard for BaseShard {
    fn init(&self) -> Result<(), ShardError> {
        if self.operational.load(Ordering::SeqCst) {
            return Err(ShardError::AlreadyInitialized);
        }
        self.operational.store(true, Ordering::SeqCst);
        Ok(())
    }
    
    fn is_operational(&self) -> bool {
        self.operational.load(Ordering::SeqCst)
    }
    
    fn shard_id(&self) -> u64 {
        self.shard_id.load(Ordering::SeqCst)
    }
    
    fn shutdown(&self) -> Result<(), ShardError> {
        if !self.operational.load(Ordering::SeqCst) {
            return Err(ShardError::NotOperational);
        }
        self.operational.store(false, Ordering::SeqCst);
        Ok(())
    }
}
