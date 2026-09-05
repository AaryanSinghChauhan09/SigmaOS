//! # IPC Namespace Implementation
//!
//! This module implements IPC namespace functionality for message queue, semaphore,
//! and shared memory isolation in SigmaOS. Each IPC namespace maintains its own
//! isolated IPC object spaces.
//!
//! ## Key Features
//!
//! - **Message Queue Isolation**: Message queues isolated per namespace
//! - **Semaphore Isolation**: Semaphores isolated per namespace
//! - **Shared Memory Isolation**: Shared memory isolated per namespace
//! - **Cross-namespace prevention**: Prevents access to IPC objects across namespaces
//! - **Thread-safe operations**: All operations use Arc<Mutex<>> for thread safety
//! - **Namespace-aware ID generation**: IPC IDs scoped to specific namespaces

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;

use crate::kernel::namespaces::{
    KernelNamespace, NamespaceId, KernelNamespaceType, NamespaceError, next_namespace_id,
};

/// IPC object identifier type
pub type IpcObjectId = u32;

/// Message queue identifier type
pub type MessageQueueId = u32;

/// Semaphore identifier type
pub type SemaphoreId = u32;

/// Shared memory identifier type
pub type SharedMemoryId = u32;

/// IPC object types that can exist within a namespace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpcObjectType {
    MessageQueue,
    Semaphore,
    SharedMemory,
}

/// Message queue object
#[derive(Debug, Clone)]
pub struct MessageQueueObject {
    pub id: MessageQueueId,
    pub name: String,
    pub capacity: usize,
    pub current_size: usize,
    pub creator_pid: u32,
}

impl MessageQueueObject {
    pub fn new(id: MessageQueueId, name: String, capacity: usize, creator_pid: u32) -> Self {
        MessageQueueObject {
            id,
            name,
            capacity,
            current_size: 0,
            creator_pid,
        }
    }
}

/// Semaphore object
#[derive(Debug, Clone)]
pub struct SemaphoreObject {
    pub id: SemaphoreId,
    pub name: String,
    pub value: u32,
    pub max_value: u32,
    pub creator_pid: u32,
    pub initial_value: u32,
}

impl SemaphoreObject {
    pub fn new(id: SemaphoreId, name: String, initial_value: u32, max_value: u32, creator_pid: u32) -> Self {
        SemaphoreObject {
            id,
            name,
            value: initial_value,
            max_value,
            creator_pid,
            initial_value,
        }
    }
}

/// Shared memory object
#[derive(Debug, Clone)]
pub struct SharedMemoryObject {
    pub id: SharedMemoryId,
    pub name: String,
    pub size: usize,
    pub creator_pid: u32,
    pub ref_count: u32,
}

impl SharedMemoryObject {
    pub fn new(id: SharedMemoryId, name: String, size: usize, creator_pid: u32) -> Self {
        SharedMemoryObject {
            id,
            name,
            size,
            creator_pid,
            ref_count: 1,
        }
    }
}

/// Registry for tracking IPC objects within a namespace
#[derive(Debug)]
pub struct IpcObjectRegistry {
    /// Message queues indexed by ID
    message_queues: BTreeMap<MessageQueueId, MessageQueueObject>,

    /// Semaphores indexed by ID
    semaphores: BTreeMap<SemaphoreId, SemaphoreObject>,

    /// Shared memory objects indexed by ID
    shared_memory: BTreeMap<SharedMemoryId, SharedMemoryObject>,

    /// Mapping from names to message queue IDs (for lookup)
    mq_names: BTreeMap<String, MessageQueueId>,

    /// Mapping from names to semaphore IDs (for lookup)
    sem_names: BTreeMap<String, SemaphoreId>,

    /// Mapping from names to shared memory IDs (for lookup)
    shm_names: BTreeMap<String, SharedMemoryId>,

    /// Next ID generator for message queues
    next_mq_id: u32,

    /// Next ID generator for semaphores
    next_sem_id: u32,

    /// Next ID generator for shared memory
    next_shm_id: u32,
}

impl IpcObjectRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        IpcObjectRegistry {
            message_queues: BTreeMap::new(),
            semaphores: BTreeMap::new(),
            shared_memory: BTreeMap::new(),
            mq_names: BTreeMap::new(),
            sem_names: BTreeMap::new(),
            shm_names: BTreeMap::new(),
            next_mq_id: 1,
            next_sem_id: 1,
            next_shm_id: 1,
        }
    }

    /// Create a new message queue in this registry
    pub fn create_message_queue(
        &mut self,
        name: String,
        capacity: usize,
        creator_pid: u32,
    ) -> Result<MessageQueueId, NamespaceError> {
        // Check if name already exists
        if self.mq_names.contains_key(&name) {
            return Err(NamespaceError::InvalidNamespaceId);
        }

        let id = self.next_mq_id;
        self.next_mq_id = self.next_mq_id.wrapping_add(1);

        let mq = MessageQueueObject::new(id, name.clone(), capacity, creator_pid);
        self.message_queues.insert(id, mq);
        self.mq_names.insert(name, id);

        Ok(id)
    }

    /// Look up a message queue by ID
    pub fn get_message_queue(&self, id: MessageQueueId) -> Option<&MessageQueueObject> {
        self.message_queues.get(&id)
    }

    /// Look up a message queue by name
    pub fn get_message_queue_by_name(&self, name: &str) -> Option<&MessageQueueObject> {
        self.mq_names
            .get(name)
            .and_then(|id| self.message_queues.get(id))
    }

    /// Get a mutable reference to a message queue by ID
    pub fn get_message_queue_mut(&mut self, id: MessageQueueId) -> Option<&mut MessageQueueObject> {
        self.message_queues.get_mut(&id)
    }

    /// Delete a message queue from this registry
    pub fn delete_message_queue(&mut self, id: MessageQueueId) -> Result<(), NamespaceError> {
        if let Some(mq) = self.message_queues.remove(&id) {
            self.mq_names.remove(&mq.name);
            Ok(())
        } else {
            Err(NamespaceError::ProcessNotInNamespace)
        }
    }

    /// Create a new semaphore in this registry
    pub fn create_semaphore(
        &mut self,
        name: String,
        initial_value: u32,
        max_value: u32,
        creator_pid: u32,
    ) -> Result<SemaphoreId, NamespaceError> {
        // Check if name already exists
        if self.sem_names.contains_key(&name) {
            return Err(NamespaceError::InvalidNamespaceId);
        }

        if initial_value > max_value {
            return Err(NamespaceError::NamespaceFull);
        }

        let id = self.next_sem_id;
        self.next_sem_id = self.next_sem_id.wrapping_add(1);

        let sem = SemaphoreObject::new(id, name.clone(), initial_value, max_value, creator_pid);
        self.semaphores.insert(id, sem);
        self.sem_names.insert(name, id);

        Ok(id)
    }

    /// Look up a semaphore by ID
    pub fn get_semaphore(&self, id: SemaphoreId) -> Option<&SemaphoreObject> {
        self.semaphores.get(&id)
    }

    /// Look up a semaphore by name
    pub fn get_semaphore_by_name(&self, name: &str) -> Option<&SemaphoreObject> {
        self.sem_names
            .get(name)
            .and_then(|id| self.semaphores.get(id))
    }

    /// Get a mutable reference to a semaphore by ID
    pub fn get_semaphore_mut(&mut self, id: SemaphoreId) -> Option<&mut SemaphoreObject> {
        self.semaphores.get_mut(&id)
    }

    /// Delete a semaphore from this registry
    pub fn delete_semaphore(&mut self, id: SemaphoreId) -> Result<(), NamespaceError> {
        if let Some(sem) = self.semaphores.remove(&id) {
            self.sem_names.remove(&sem.name);
            Ok(())
        } else {
            Err(NamespaceError::ProcessNotInNamespace)
        }
    }

    /// Create a new shared memory object in this registry
    pub fn create_shared_memory(
        &mut self,
        name: String,
        size: usize,
        creator_pid: u32,
    ) -> Result<SharedMemoryId, NamespaceError> {
        // Check if name already exists
        if self.shm_names.contains_key(&name) {
            return Err(NamespaceError::InvalidNamespaceId);
        }

        let id = self.next_shm_id;
        self.next_shm_id = self.next_shm_id.wrapping_add(1);

        let shm = SharedMemoryObject::new(id, name.clone(), size, creator_pid);
        self.shared_memory.insert(id, shm);
        self.shm_names.insert(name, id);

        Ok(id)
    }

    /// Look up a shared memory object by ID
    pub fn get_shared_memory(&self, id: SharedMemoryId) -> Option<&SharedMemoryObject> {
        self.shared_memory.get(&id)
    }

    /// Look up a shared memory object by name
    pub fn get_shared_memory_by_name(&self, name: &str) -> Option<&SharedMemoryObject> {
        self.shm_names
            .get(name)
            .and_then(|id| self.shared_memory.get(id))
    }

    /// Get a mutable reference to a shared memory object by ID
    pub fn get_shared_memory_mut(&mut self, id: SharedMemoryId) -> Option<&mut SharedMemoryObject> {
        self.shared_memory.get_mut(&id)
    }

    /// Delete a shared memory object from this registry
    pub fn delete_shared_memory(&mut self, id: SharedMemoryId) -> Result<(), NamespaceError> {
        if let Some(shm) = self.shared_memory.remove(&id) {
            self.shm_names.remove(&shm.name);
            Ok(())
        } else {
            Err(NamespaceError::ProcessNotInNamespace)
        }
    }

    /// Get the count of all IPC objects in this registry
    pub fn object_count(&self) -> usize {
        self.message_queues.len() + self.semaphores.len() + self.shared_memory.len()
    }

    /// Get the count of message queues
    pub fn message_queue_count(&self) -> usize {
        self.message_queues.len()
    }

    /// Get the count of semaphores
    pub fn semaphore_count(&self) -> usize {
        self.semaphores.len()
    }

    /// Get the count of shared memory objects
    pub fn shared_memory_count(&self) -> usize {
        self.shared_memory.len()
    }
}

impl Default for IpcObjectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about an IPC namespace
#[derive(Debug, Clone)]
pub struct IpcNamespaceStats {
    pub namespace_id: u64,
    pub ref_count: u32,
    pub message_queue_count: u32,
    pub semaphore_count: u32,
    pub shared_memory_count: u32,
    pub total_objects: u32,
    pub has_parent: bool,
}

/// Represents an IPC namespace that isolates IPC object spaces
pub struct IpcNamespace {
    /// Unique namespace ID
    namespace_id: NamespaceId,

    /// Reference count - how many processes use this namespace
    ref_count: AtomicU32,

    /// Parent namespace (if this is a child namespace)
    parent: Option<Arc<IpcNamespace>>,

    /// Registry of IPC objects in this namespace
    registry: Mutex<IpcObjectRegistry>,

    /// Namespace metadata
    metadata: String,
}

impl IpcNamespace {
    /// Create a new root IPC namespace
    pub fn new_root() -> Arc<Self> {
        let id = next_namespace_id();
        Arc::new(IpcNamespace {
            namespace_id: id,
            ref_count: AtomicU32::new(1),
            parent: None,
            registry: Mutex::new(IpcObjectRegistry::new()),
            metadata: format!("IPC Namespace Root (id: {})", id.raw()),
        })
    }

    /// Create a child IPC namespace that inherits from this one
    pub fn create_child(self: &Arc<Self>) -> Arc<Self> {
        let id = next_namespace_id();
        Arc::new(IpcNamespace {
            namespace_id: id,
            ref_count: AtomicU32::new(1),
            parent: Some(Arc::clone(self)),
            registry: Mutex::new(IpcObjectRegistry::new()),
            metadata: format!(
                "IPC Namespace Child (id: {}, parent: {})",
                id.raw(),
                self.namespace_id.raw()
            ),
        })
    }

    /// Create a new message queue within this namespace
    pub fn create_message_queue(
        &self,
        name: String,
        capacity: usize,
        creator_pid: u32,
    ) -> Result<MessageQueueId, NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.create_message_queue(name, capacity, creator_pid)
    }

    /// Get a message queue from this namespace by ID
    pub fn get_message_queue(&self, id: MessageQueueId) -> Option<MessageQueueObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_message_queue(id).cloned()
    }

    /// Get a message queue from this namespace by name
    pub fn get_message_queue_by_name(&self, name: &str) -> Option<MessageQueueObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_message_queue_by_name(name).cloned()
    }

    /// Delete a message queue from this namespace
    pub fn delete_message_queue(&self, id: MessageQueueId) -> Result<(), NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.delete_message_queue(id)
    }

    /// Create a new semaphore within this namespace
    pub fn create_semaphore(
        &self,
        name: String,
        initial_value: u32,
        max_value: u32,
        creator_pid: u32,
    ) -> Result<SemaphoreId, NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.create_semaphore(name, initial_value, max_value, creator_pid)
    }

    /// Get a semaphore from this namespace by ID
    pub fn get_semaphore(&self, id: SemaphoreId) -> Option<SemaphoreObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_semaphore(id).cloned()
    }

    /// Get a semaphore from this namespace by name
    pub fn get_semaphore_by_name(&self, name: &str) -> Option<SemaphoreObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_semaphore_by_name(name).cloned()
    }

    /// Delete a semaphore from this namespace
    pub fn delete_semaphore(&self, id: SemaphoreId) -> Result<(), NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.delete_semaphore(id)
    }

    /// Create a new shared memory object within this namespace
    pub fn create_shared_memory(
        &self,
        name: String,
        size: usize,
        creator_pid: u32,
    ) -> Result<SharedMemoryId, NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.create_shared_memory(name, size, creator_pid)
    }

    /// Get a shared memory object from this namespace by ID
    pub fn get_shared_memory(&self, id: SharedMemoryId) -> Option<SharedMemoryObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_shared_memory(id).cloned()
    }

    /// Get a shared memory object from this namespace by name
    pub fn get_shared_memory_by_name(&self, name: &str) -> Option<SharedMemoryObject> {
        let registry = self.registry.lock().unwrap();
        registry.get_shared_memory_by_name(name).cloned()
    }

    /// Delete a shared memory object from this namespace
    pub fn delete_shared_memory(&self, id: SharedMemoryId) -> Result<(), NamespaceError> {
        let mut registry = self.registry.lock().unwrap();
        registry.delete_shared_memory(id)
    }

    /// Attempt cross-namespace access (should always fail)
    pub fn can_access_from_namespace(&self, other_namespace_id: NamespaceId) -> bool {
        // IPC objects are isolated per namespace - cross-namespace access is prevented
        self.namespace_id == other_namespace_id
    }

    /// Get statistics about this namespace
    pub fn get_stats(&self) -> IpcNamespaceStats {
        let registry = self.registry.lock().unwrap();
        IpcNamespaceStats {
            namespace_id: self.namespace_id.raw(),
            ref_count: self.ref_count.load(Ordering::SeqCst),
            message_queue_count: registry.message_queue_count() as u32,
            semaphore_count: registry.semaphore_count() as u32,
            shared_memory_count: registry.shared_memory_count() as u32,
            total_objects: registry.object_count() as u32,
            has_parent: self.parent.is_some(),
        }
    }
}

impl KernelNamespace for IpcNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Ipc
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("{}, refs={}", self.metadata, self.ref_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_namespace_creation() {
        let ns = IpcNamespace::new_root();
        assert_eq!(ns.namespace_type(), KernelNamespaceType::Ipc);
        assert_eq!(ns.ref_count(), 1);
    }

    #[test]
    fn test_message_queue_creation() {
        let ns = IpcNamespace::new_root();
        let mq_id = ns
            .create_message_queue("test_mq".to_string(), 100, 1)
            .expect("Failed to create message queue");
        assert!(mq_id > 0);

        let mq = ns.get_message_queue(mq_id).expect("Failed to get message queue");
        assert_eq!(mq.name, "test_mq");
        assert_eq!(mq.capacity, 100);
        assert_eq!(mq.creator_pid, 1);
    }

    #[test]
    fn test_message_queue_by_name() {
        let ns = IpcNamespace::new_root();
        let _mq_id = ns
            .create_message_queue("named_mq".to_string(), 50, 2)
            .expect("Failed to create message queue");

        let mq = ns
            .get_message_queue_by_name("named_mq")
            .expect("Failed to get message queue by name");
        assert_eq!(mq.name, "named_mq");
        assert_eq!(mq.capacity, 50);
    }

    #[test]
    fn test_message_queue_isolation() {
        let ns1 = IpcNamespace::new_root();
        let ns2 = IpcNamespace::new_root();

        let mq_id_1 = ns1
            .create_message_queue("shared_name".to_string(), 100, 1)
            .expect("Failed to create MQ in ns1");
        let mq_id_2 = ns2
            .create_message_queue("shared_name".to_string(), 200, 2)
            .expect("Failed to create MQ in ns2");

        // IDs can be the same (reused) across namespaces
        assert_eq!(mq_id_1, mq_id_2);

        // But accessing from wrong namespace should fail
        assert!(ns1.get_message_queue(mq_id_1).is_some());
        assert!(ns2.get_message_queue(mq_id_2).is_some());

        // Verify the objects are different
        let mq1 = ns1.get_message_queue(mq_id_1).unwrap();
        let mq2 = ns2.get_message_queue(mq_id_2).unwrap();
        assert_eq!(mq1.capacity, 100);
        assert_eq!(mq2.capacity, 200);
        assert_eq!(mq1.creator_pid, 1);
        assert_eq!(mq2.creator_pid, 2);
    }

    #[test]
    fn test_semaphore_creation() {
        let ns = IpcNamespace::new_root();
        let sem_id = ns
            .create_semaphore("test_sem".to_string(), 5, 10, 1)
            .expect("Failed to create semaphore");
        assert!(sem_id > 0);

        let sem = ns.get_semaphore(sem_id).expect("Failed to get semaphore");
        assert_eq!(sem.name, "test_sem");
        assert_eq!(sem.value, 5);
        assert_eq!(sem.max_value, 10);
    }

    #[test]
    fn test_semaphore_isolation() {
        let ns1 = IpcNamespace::new_root();
        let ns2 = IpcNamespace::new_root();

        let sem_id_1 = ns1
            .create_semaphore("sem".to_string(), 3, 10, 1)
            .expect("Failed to create semaphore in ns1");
        let sem_id_2 = ns2
            .create_semaphore("sem".to_string(), 7, 15, 2)
            .expect("Failed to create semaphore in ns2");

        // Can reuse same ID across namespaces
        assert_eq!(sem_id_1, sem_id_2);

        // But values are independent
        let sem1 = ns1.get_semaphore(sem_id_1).unwrap();
        let sem2 = ns2.get_semaphore(sem_id_2).unwrap();
        assert_eq!(sem1.value, 3);
        assert_eq!(sem2.value, 7);
    }

    #[test]
    fn test_shared_memory_creation() {
        let ns = IpcNamespace::new_root();
        let shm_id = ns
            .create_shared_memory("test_shm".to_string(), 4096, 1)
            .expect("Failed to create shared memory");
        assert!(shm_id > 0);

        let shm = ns
            .get_shared_memory(shm_id)
            .expect("Failed to get shared memory");
        assert_eq!(shm.name, "test_shm");
        assert_eq!(shm.size, 4096);
    }

    #[test]
    fn test_shared_memory_isolation() {
        let ns1 = IpcNamespace::new_root();
        let ns2 = IpcNamespace::new_root();

        let shm_id_1 = ns1
            .create_shared_memory("shm".to_string(), 4096, 1)
            .expect("Failed to create SHM in ns1");
        let shm_id_2 = ns2
            .create_shared_memory("shm".to_string(), 8192, 2)
            .expect("Failed to create SHM in ns2");

        // Can reuse IDs
        assert_eq!(shm_id_1, shm_id_2);

        // But sizes are independent
        let shm1 = ns1.get_shared_memory(shm_id_1).unwrap();
        let shm2 = ns2.get_shared_memory(shm_id_2).unwrap();
        assert_eq!(shm1.size, 4096);
        assert_eq!(shm2.size, 8192);
    }

    #[test]
    fn test_ipc_object_deletion() {
        let ns = IpcNamespace::new_root();

        let mq_id = ns
            .create_message_queue("to_delete".to_string(), 100, 1)
            .expect("Failed to create message queue");
        assert!(ns.get_message_queue(mq_id).is_some());

        let result = ns.delete_message_queue(mq_id);
        assert!(result.is_ok());
        assert!(ns.get_message_queue(mq_id).is_none());
    }

    #[test]
    fn test_duplicate_object_names() {
        let ns = IpcNamespace::new_root();

        let _mq_id = ns
            .create_message_queue("duplicate".to_string(), 100, 1)
            .expect("Failed to create first message queue");

        let result = ns.create_message_queue("duplicate".to_string(), 200, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_namespace_stats() {
        let ns = IpcNamespace::new_root();

        let _mq_id = ns
            .create_message_queue("mq1".to_string(), 100, 1)
            .expect("Failed to create message queue");
        let _sem_id = ns
            .create_semaphore("sem1".to_string(), 5, 10, 1)
            .expect("Failed to create semaphore");
        let _shm_id = ns
            .create_shared_memory("shm1".to_string(), 4096, 1)
            .expect("Failed to create shared memory");

        let stats = ns.get_stats();
        assert_eq!(stats.message_queue_count, 1);
        assert_eq!(stats.semaphore_count, 1);
        assert_eq!(stats.shared_memory_count, 1);
        assert_eq!(stats.total_objects, 3);
        assert_eq!(stats.ref_count, 1);
        assert!(!stats.has_parent);
    }

    #[test]
    fn test_child_namespace() {
        let parent_ns = IpcNamespace::new_root();
        let child_ns = parent_ns.create_child();

        // Create object in parent
        let _mq_id = parent_ns
            .create_message_queue("parent_mq".to_string(), 100, 1)
            .expect("Failed to create message queue in parent");

        // Child should have its own registry
        let child_stats = child_ns.get_stats();
        assert_eq!(child_stats.message_queue_count, 0);

        // Parent stats unchanged
        let parent_stats = parent_ns.get_stats();
        assert_eq!(parent_stats.message_queue_count, 1);

        assert!(child_stats.has_parent);
    }

    #[test]
    fn test_cross_namespace_access_prevention() {
        let ns1 = IpcNamespace::new_root();
        let ns2 = IpcNamespace::new_root();

        let ns1_id = ns1.namespace_id();
        let ns2_id = ns2.namespace_id();

        // Cross-namespace access should be prevented
        assert!(!ns1.can_access_from_namespace(ns2_id));
        assert!(!ns2.can_access_from_namespace(ns1_id));
        assert!(ns1.can_access_from_namespace(ns1_id));
        assert!(ns2.can_access_from_namespace(ns2_id));
    }

    #[test]
    fn test_ipc_namespace_reference_counting() {
        let ns = IpcNamespace::new_root();
        assert_eq!(ns.ref_count(), 1);

        ns.increment_ref();
        assert_eq!(ns.ref_count(), 2);

        ns.decrement_ref();
        assert_eq!(ns.ref_count(), 1);
    }

    #[test]
    fn test_multiple_objects_per_namespace() {
        let ns = IpcNamespace::new_root();

        // Create multiple message queues
        for i in 0..5 {
            let _mq_id = ns
                .create_message_queue(format!("mq_{}", i), 100 + i * 10, 1)
                .expect("Failed to create message queue");
        }

        // Create multiple semaphores
        for i in 0..3 {
            let _sem_id = ns
                .create_semaphore(format!("sem_{}", i), 5, 10, 1)
                .expect("Failed to create semaphore");
        }

        // Create multiple shared memory objects
        for i in 0..2 {
            let _shm_id = ns
                .create_shared_memory(format!("shm_{}", i), 4096 * (i + 1), 1)
                .expect("Failed to create shared memory");
        }

        let stats = ns.get_stats();
        assert_eq!(stats.message_queue_count, 5);
        assert_eq!(stats.semaphore_count, 3);
        assert_eq!(stats.shared_memory_count, 2);
        assert_eq!(stats.total_objects, 10);
    }

    #[test]
    fn test_namespace_metadata() {
        let ns = IpcNamespace::new_root();
        let metadata = ns.metadata();
        assert!(metadata.contains("IPC Namespace"));
        assert!(metadata.contains("refs=1"));
    }
}
