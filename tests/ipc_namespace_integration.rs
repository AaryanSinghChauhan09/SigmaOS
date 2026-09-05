//! Integration tests for IPC namespace functionality
//!
//! This module tests the IPC namespace core implementation with all IPC object types
//! including message queues, semaphores, and shared memory isolation.

use sigmaos::ipc::{
    IpcNamespace, IpcObjectType, MessageQueueId, SemaphoreId, SharedMemoryId,
};
use sigmaos::kernel::namespaces::KernelNamespaceType;

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

#[test]
fn test_comprehensive_namespace_scenario() {
    // Create multiple namespaces
    let root_ns = IpcNamespace::new_root();
    let app1_ns = root_ns.create_child();
    let app2_ns = root_ns.create_child();

    // App 1 creates its own IPC objects
    let _mq1 = app1_ns
        .create_message_queue("app1_queue".to_string(), 256, 100)
        .unwrap();
    let _sem1 = app1_ns
        .create_semaphore("app1_lock".to_string(), 1, 1, 100)
        .unwrap();
    let _shm1 = app1_ns
        .create_shared_memory("app1_data".to_string(), 8192, 100)
        .unwrap();

    // App 2 creates its own IPC objects
    let _mq2 = app2_ns
        .create_message_queue("app2_queue".to_string(), 512, 200)
        .unwrap();
    let _sem2 = app2_ns
        .create_semaphore("app2_lock".to_string(), 1, 1, 200)
        .unwrap();
    let _shm2 = app2_ns
        .create_shared_memory("app2_data".to_string(), 16384, 200)
        .unwrap();

    // Root namespace has nothing
    let root_stats = root_ns.get_stats();
    assert_eq!(root_stats.total_objects, 0);

    // Each app has its own isolated objects
    let app1_stats = app1_ns.get_stats();
    assert_eq!(app1_stats.message_queue_count, 1);
    assert_eq!(app1_stats.semaphore_count, 1);
    assert_eq!(app1_stats.shared_memory_count, 1);
    assert_eq!(app1_stats.total_objects, 3);

    let app2_stats = app2_ns.get_stats();
    assert_eq!(app2_stats.message_queue_count, 1);
    assert_eq!(app2_stats.semaphore_count, 1);
    assert_eq!(app2_stats.shared_memory_count, 1);
    assert_eq!(app2_stats.total_objects, 3);

    // Verify isolation - app1 cannot see app2's objects
    assert!(app1_ns.get_message_queue_by_name("app2_queue").is_none());
    assert!(app2_ns.get_message_queue_by_name("app1_queue").is_none());
}
