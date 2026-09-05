//! # Namespace Integration Tests
//! 
//! Comprehensive end-to-end tests for namespace integration with process management
//! Tests namespace context propagation and lifecycle management

use std::sync::Arc;

// Import the necessary modules
use sigmaos::runtime::process::{ProcessDescriptor, ProcessNamespaceContext};

#[test]
fn test_namespace_context_creation() {
    let context = ProcessNamespaceContext::new_root();
    assert_eq!(context.metadata_summary(), "root_namespaces");
}

#[test]
fn test_namespace_context_cloning() {
    let context1 = ProcessNamespaceContext::new_root();
    let context2 = context1.clone_all();
    
    // Cloned contexts should have same metadata
    assert_eq!(
        context1.metadata_summary(),
        context2.metadata_summary()
    );
}

#[test]
fn test_namespace_context_child_creation() {
    let parent_context = ProcessNamespaceContext::new_root();
    let child_context = parent_context.create_child();
    
    // Child should have different metadata
    assert_eq!(child_context.metadata_summary(), "child_namespaces");
}

#[test]
fn test_process_descriptor_root_creation() {
    let desc = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root descriptor");
    
    assert_eq!(desc.kernel_pid, 1);
    assert_eq!(desc.namespace_pid, 1);
    assert_eq!(desc.parent_pid, 0);
    assert!(desc.is_isolated);
}

#[test]
fn test_process_descriptor_child_creation() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_child(2, 1, "child1".to_string())
        .expect("Failed to create child");
    
    assert_eq!(child.kernel_pid, 2);
    assert_eq!(child.parent_pid, 1);
    assert!(!child.is_isolated);
    assert_eq!(child.namespace_pid, 2); // Incremented from parent
}

#[test]
fn test_process_descriptor_isolated_child() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let isolated = parent.create_isolated_child(2, 1, "container".to_string())
        .expect("Failed to create isolated child");
    
    assert_eq!(isolated.kernel_pid, 2);
    assert!(isolated.is_isolated);
    assert_eq!(isolated.namespace_pid, 2);
}

#[test]
fn test_multi_level_process_hierarchy() {
    // Create hierarchy: root -> child1 -> grandchild1
    //                        -> child2
    
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let child1 = root.create_child(2, 1, "child1".to_string())
        .expect("Failed to create child1");
    
    let child2 = root.create_child(3, 1, "child2".to_string())
        .expect("Failed to create child2");
    
    let grandchild1 = child1.create_child(4, 2, "grandchild1".to_string())
        .expect("Failed to create grandchild1");
    
    // Non-isolated children should share context with root
    assert!(!child1.is_isolated);
    assert!(!child2.is_isolated);
    assert!(!grandchild1.is_isolated);
    
    // Verify PID allocation across hierarchy
    assert_eq!(child1.namespace_pid, 2);
    assert_eq!(child2.namespace_pid, 3);
    assert_eq!(grandchild1.namespace_pid, 4);
}

#[test]
fn test_namespace_reference_counting() {
    let context = ProcessNamespaceContext::new_root();
    
    context.increment_refs();
    context.decrement_refs();
    
    // Should not panic
}

#[test]
fn test_cross_namespace_access_prevention() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let isolated1 = parent.create_isolated_child(2, 1, "iso1".to_string())
        .expect("Failed to create isolated1");
    
    let isolated2 = parent.create_isolated_child(3, 1, "iso2".to_string())
        .expect("Failed to create isolated2");
    
    // Isolated namespaces cannot access each other
    assert!(!isolated1.can_access_process_namespaces(&isolated2));
    assert!(!isolated2.can_access_process_namespaces(&isolated1));
}

#[test]
fn test_sibling_access_non_isolated() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child1 = parent.create_child(2, 1, "child1".to_string())
        .expect("Failed to create child1");
    
    let child2 = parent.create_child(3, 1, "child2".to_string())
        .expect("Failed to create child2");
    
    // Non-isolated siblings can access each other
    assert!(child1.can_access_process_namespaces(&child2));
    assert!(child2.can_access_process_namespaces(&child1));
}

#[test]
fn test_cleanup_releases_resources() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_child(2, 1, "child".to_string())
        .expect("Failed to create child");
    
    // Cleanup should complete successfully
    child.cleanup().expect("Failed to cleanup");
}

#[test]
fn test_namespace_metadata_summary() {
    let context = ProcessNamespaceContext::new_root();
    let summary = context.metadata_summary();
    
    assert!(!summary.is_empty());
}

#[test]
fn test_process_descriptor_metadata() {
    let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor");
    
    let metadata = descriptor.metadata();
    
    assert!(metadata.contains("ProcessDescriptor"));
    assert!(metadata.contains("kernel_pid: 1"));
    assert!(metadata.contains("namespace_pid: 1"));
    assert!(metadata.contains("init"));
    assert!(metadata.contains("isolated: true"));
}

#[test]
fn test_large_process_hierarchy() {
    // Create a large process tree and verify all contexts work correctly
    
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let mut processes = vec![root];
    let mut kernel_pid = 2;
    
    // Create 20 child processes
    for i in 0..20 {
        let parent_idx = i / 3; // 3 children per parent
        let parent = &processes[parent_idx];
        
        let child = parent.create_child(kernel_pid, parent.kernel_pid, format!("proc_{}", i))
            .expect("Failed to create child");
        
        processes.push(child);
        kernel_pid += 1;
    }
    
    // Verify all processes are non-isolated (except root)
    for i in 1..processes.len() {
        assert!(!processes[i].is_isolated);
    }
    
    // Verify PIDs are sequential
    for (i, proc) in processes.iter().enumerate() {
        assert_eq!(proc.namespace_pid as usize, i + 1);
    }
}

#[test]
fn test_concurrent_descriptor_creation() {
    use std::thread;
    
    let desc = Arc::new(ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor"));
    
    let mut handles = vec![];
    
    // Spawn multiple threads to create process descriptors concurrently
    for i in 0..5 {
        let desc_clone = Arc::clone(&desc);
        let handle = thread::spawn(move || {
            let child = desc_clone.create_child(i + 2, 1, format!("thread_proc_{}", i));
            child.is_ok()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .collect();
    
    // All should succeed
    assert!(results.iter().all(|&r| r));
}

#[test]
fn test_namespace_context_refs() {
    let context = ProcessNamespaceContext::new_root();
    
    context.increment_refs();
    context.increment_refs();
    
    context.decrement_refs();
    context.decrement_refs();
    
    // Should not panic
}

#[test]
fn test_descriptor_isolation_flag() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    assert!(parent.is_isolated);
    
    let non_isolated_child = parent.create_child(2, 1, "child".to_string())
        .expect("Failed to create child");
    
    assert!(!non_isolated_child.is_isolated);
    
    let isolated_child = parent.create_isolated_child(3, 1, "isolated".to_string())
        .expect("Failed to create isolated");
    
    assert!(isolated_child.is_isolated);
}

#[test]
fn test_process_tree_relationships() {
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let parent = root.create_child(2, 1, "parent".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_child(3, 2, "child".to_string())
        .expect("Failed to create child");
    
    // Verify relationships
    assert_eq!(root.kernel_pid, 1);
    assert_eq!(root.parent_pid, 0);
    
    assert_eq!(parent.kernel_pid, 2);
    assert_eq!(parent.parent_pid, 1);
    
    assert_eq!(child.kernel_pid, 3);
    assert_eq!(child.parent_pid, 2);
}

#[test]
fn test_namespace_context_creation() {
    let context = ProcessNamespaceContext::new_root();
    
    assert_eq!(context.pid_namespace.namespace_type(), KernelNamespaceType::Pid);
    assert_eq!(context.ipc_namespace.namespace_type(), KernelNamespaceType::Ipc);
    assert_eq!(context.mount_namespace.namespace_type(), KernelNamespaceType::Mount);
}

#[test]
fn test_namespace_context_cloning() {
    let context1 = ProcessNamespaceContext::new_root();
    let context2 = context1.clone_all();
    
    // Should have same namespace IDs after cloning
    assert_eq!(
        context1.pid_namespace.namespace_id(),
        context2.pid_namespace.namespace_id()
    );
    assert_eq!(
        context1.ipc_namespace.namespace_id(),
        context2.ipc_namespace.namespace_id()
    );
    assert_eq!(
        context1.mount_namespace.namespace_id(),
        context2.mount_namespace.namespace_id()
    );
}

#[test]
fn test_namespace_context_child_creation() {
    let parent_context = ProcessNamespaceContext::new_root();
    let child_context = parent_context.create_child();
    
    // Child should have different namespace IDs
    assert_ne!(
        parent_context.pid_namespace.namespace_id(),
        child_context.pid_namespace.namespace_id()
    );
    assert_ne!(
        parent_context.ipc_namespace.namespace_id(),
        child_context.ipc_namespace.namespace_id()
    );
    assert_ne!(
        parent_context.mount_namespace.namespace_id(),
        child_context.mount_namespace.namespace_id()
    );
}

#[test]
fn test_process_descriptor_root_creation() {
    let desc = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root descriptor");
    
    assert_eq!(desc.kernel_pid, 1);
    assert_eq!(desc.namespace_pid, 1);
    assert_eq!(desc.parent_pid, 0);
    assert!(desc.is_isolated);
}

#[test]
fn test_process_descriptor_child_creation() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_child(2, 1, "child1".to_string())
        .expect("Failed to create child");
    
    assert_eq!(child.kernel_pid, 2);
    assert_eq!(child.parent_pid, 1);
    assert!(!child.is_isolated);
}

#[test]
fn test_process_descriptor_isolated_child() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let isolated = parent.create_isolated_child(2, 1, "container".to_string())
        .expect("Failed to create isolated child");
    
    assert_eq!(isolated.kernel_pid, 2);
    assert!(isolated.is_isolated);
    
    // Different namespaces
    assert_ne!(
        parent.namespace_context.pid_namespace.namespace_id(),
        isolated.namespace_context.pid_namespace.namespace_id()
    );
}

#[test]
fn test_pid_namespace_integration() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let pid_ns = parent.pid_namespace();
    
    // Should be able to allocate PIDs
    let pid1 = pid_ns.allocate_pid().expect("Failed to allocate PID 1");
    assert_eq!(pid1, 1);
    
    let pid2 = pid_ns.allocate_pid().expect("Failed to allocate PID 2");
    assert_eq!(pid2, 2);
    
    // PIDs should be in use
    assert!(pid_ns.is_pid_used(pid1));
    assert!(pid_ns.is_pid_used(pid2));
}

#[test]
fn test_ipc_namespace_integration() {
    let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor");
    
    let ipc_ns = descriptor.ipc_namespace();
    
    // Create message queue
    let mq_id = ipc_ns
        .create_message_queue("test_mq".to_string(), 100, 1)
        .expect("Failed to create message queue");
    
    // Retrieve and verify
    let mq = ipc_ns.get_message_queue(mq_id)
        .expect("Failed to get message queue");
    
    assert_eq!(mq.name, "test_mq");
    assert_eq!(mq.capacity, 100);
}

#[test]
fn test_mount_namespace_integration() {
    let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor");
    
    let mnt_ns = descriptor.mount_namespace();
    
    // Create mount
    let mount_id = mnt_ns
        .create_mount(
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountSource::Device(1),
            MountFlags::new(0),
            "defaults".to_string(),
            1,
        )
        .expect("Failed to create mount");
    
    // Verify mount exists
    assert!(mnt_ns.mount_exists(mount_id));
}

#[test]
fn test_namespace_isolation_between_processes() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child1 = parent.create_isolated_child(2, 1, "container1".to_string())
        .expect("Failed to create child1");
    
    let child2 = parent.create_isolated_child(3, 1, "container2".to_string())
        .expect("Failed to create child2");
    
    // Create IPC objects in each child
    let mq1_id = child1.ipc_namespace()
        .create_message_queue("shared_name".to_string(), 100, 2)
        .expect("Failed to create MQ in child1");
    
    let mq2_id = child2.ipc_namespace()
        .create_message_queue("shared_name".to_string(), 200, 3)
        .expect("Failed to create MQ in child2");
    
    // Both can have the same MQ name but with different properties (isolation)
    let mq1 = child1.ipc_namespace().get_message_queue(mq1_id).unwrap();
    let mq2 = child2.ipc_namespace().get_message_queue(mq2_id).unwrap();
    
    assert_eq!(mq1.capacity, 100);
    assert_eq!(mq2.capacity, 200);
}

#[test]
fn test_multi_level_namespace_hierarchy() {
    // Create hierarchy: root -> child1 -> grandchild1
    //                        -> child2
    
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let child1 = root.create_child(2, 1, "child1".to_string())
        .expect("Failed to create child1");
    
    let child2 = root.create_child(3, 1, "child2".to_string())
        .expect("Failed to create child2");
    
    let grandchild1 = child1.create_child(4, 2, "grandchild1".to_string())
        .expect("Failed to create grandchild1");
    
    // Non-isolated children should share PID namespace with root
    assert_eq!(
        root.pid_namespace().namespace_id(),
        child1.pid_namespace().namespace_id()
    );
    assert_eq!(
        root.pid_namespace().namespace_id(),
        grandchild1.pid_namespace().namespace_id()
    );
    assert_eq!(
        root.pid_namespace().namespace_id(),
        child2.pid_namespace().namespace_id()
    );
    
    // Verify PID allocation across hierarchy
    assert_eq!(child1.namespace_pid, 1);
    assert_eq!(child2.namespace_pid, 2);
    assert_eq!(grandchild1.namespace_pid, 3);
}

#[test]
fn test_namespace_reference_counting() {
    let context = ProcessNamespaceContext::new_root();
    
    let pid_ns = &context.pid_namespace;
    let ipc_ns = &context.ipc_namespace;
    let mnt_ns = &context.mount_namespace;
    
    assert_eq!(pid_ns.ref_count(), 1);
    assert_eq!(ipc_ns.ref_count(), 1);
    assert_eq!(mnt_ns.ref_count(), 1);
    
    context.increment_refs();
    
    assert_eq!(pid_ns.ref_count(), 2);
    assert_eq!(ipc_ns.ref_count(), 2);
    assert_eq!(mnt_ns.ref_count(), 2);
    
    context.decrement_refs();
    
    assert_eq!(pid_ns.ref_count(), 1);
    assert_eq!(ipc_ns.ref_count(), 1);
    assert_eq!(mnt_ns.ref_count(), 1);
}

#[test]
fn test_cross_namespace_access_prevention() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let isolated1 = parent.create_isolated_child(2, 1, "iso1".to_string())
        .expect("Failed to create isolated1");
    
    let isolated2 = parent.create_isolated_child(3, 1, "iso2".to_string())
        .expect("Failed to create isolated2");
    
    // Create mount in isolated1
    let mount_id = isolated1.mount_namespace()
        .create_mount(
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountSource::Device(1),
            MountFlags::new(0),
            "defaults".to_string(),
            1,
        )
        .expect("Failed to create mount");
    
    // Mount should exist in isolated1
    assert!(isolated1.mount_namespace().mount_exists(mount_id));
    
    // Mount should NOT exist in isolated2 (different namespace)
    assert!(!isolated2.mount_namespace().mount_exists(mount_id));
    
    // Cross-namespace access should be prevented
    let ns1_id = isolated1.mount_namespace().namespace_id();
    let ns2_id = isolated2.mount_namespace().namespace_id();
    
    assert!(!isolated1.mount_namespace()
        .can_access_mount_from_namespace(mount_id, ns2_id));
}

#[test]
fn test_cleanup_releases_pid() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_child(2, 1, "child".to_string())
        .expect("Failed to create child");
    
    let namespace_pid = child.namespace_pid;
    let pid_ns = child.pid_namespace();
    
    // PID should be in use
    assert!(pid_ns.is_pid_used(namespace_pid));
    
    // Cleanup should release PID
    child.cleanup().expect("Failed to cleanup");
    assert!(!pid_ns.is_pid_used(namespace_pid));
}

#[test]
fn test_multiple_ipc_objects_per_namespace() {
    let desc = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor");
    
    let ipc_ns = desc.ipc_namespace();
    
    // Create multiple message queues
    for i in 0..3 {
        let _mq_id = ipc_ns
            .create_message_queue(format!("mq_{}", i), 100 + i * 10, 1)
            .expect("Failed to create message queue");
    }
    
    // Create multiple semaphores
    for i in 0..2 {
        let _sem_id = ipc_ns
            .create_semaphore(format!("sem_{}", i), 5, 10, 1)
            .expect("Failed to create semaphore");
    }
    
    let stats = ipc_ns.get_stats();
    assert_eq!(stats.message_queue_count, 3);
    assert_eq!(stats.semaphore_count, 2);
    assert_eq!(stats.total_objects, 5);
}

#[test]
fn test_mount_namespace_hierarchy() {
    let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create parent");
    
    let child = parent.create_isolated_child(2, 1, "child".to_string())
        .expect("Failed to create child");
    
    // Parent creates mount
    let parent_mount = parent.mount_namespace()
        .create_mount(
            "/mnt/parent".to_string(),
            "ext4".to_string(),
            MountSource::Device(1),
            MountFlags::new(0),
            "defaults".to_string(),
            1,
        )
        .expect("Failed to create parent mount");
    
    // Child creates own mount
    let child_mount = child.mount_namespace()
        .create_mount(
            "/mnt/child".to_string(),
            "tmpfs".to_string(),
            MountSource::Tmpfs,
            MountFlags::new(0),
            "size=100M".to_string(),
            1,
        )
        .expect("Failed to create child mount");
    
    // Each should have their own mounts
    assert!(parent.mount_namespace().mount_exists(parent_mount));
    assert!(!parent.mount_namespace().mount_exists(child_mount));
    
    assert!(!child.mount_namespace().mount_exists(parent_mount));
    assert!(child.mount_namespace().mount_exists(child_mount));
}

#[test]
fn test_namespace_metadata_summary() {
    let context = ProcessNamespaceContext::new_root();
    let summary = context.metadata_summary();
    
    assert!(summary.contains("PID("));
    assert!(summary.contains("IPC("));
    assert!(summary.contains("Mount("));
}

#[test]
fn test_process_descriptor_access_control() {
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let child = root.create_child(2, 1, "child".to_string())
        .expect("Failed to create child");
    
    // Child should be able to access root's namespaces (shared)
    assert!(child.can_access_process_namespaces(&root));
    assert!(root.can_access_process_namespaces(&child));
    
    let isolated = root.create_isolated_child(3, 1, "isolated".to_string())
        .expect("Failed to create isolated");
    
    // Isolated cannot access root's namespaces
    assert!(!isolated.can_access_process_namespaces(&root));
    assert!(!root.can_access_process_namespaces(&isolated));
}

#[test]
fn test_concurrent_namespace_operations() {
    use std::thread;
    
    let desc = Arc::new(ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor"));
    
    let mut handles = vec![];
    
    // Spawn multiple threads to create IPC objects concurrently
    for i in 0..5 {
        let desc_clone = Arc::clone(&desc);
        let handle = thread::spawn(move || {
            let ipc_ns = desc_clone.ipc_namespace();
            let mq_id = ipc_ns
                .create_message_queue(format!("mq_thread_{}", i), 100, 1)
                .expect("Failed to create message queue");
            
            // Verify creation
            ipc_ns.get_message_queue(mq_id).is_some()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .collect();
    
    // All should succeed
    assert!(results.iter().all(|&r| r));
}

#[test]
fn test_namespace_metadata_detail() {
    let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create descriptor");
    
    let metadata = descriptor.metadata();
    
    assert!(metadata.contains("ProcessDescriptor"));
    assert!(metadata.contains("kernel_pid: 1"));
    assert!(metadata.contains("namespace_pid: 1"));
    assert!(metadata.contains("init"));
    assert!(metadata.contains("isolated: true"));
}

#[test]
fn test_large_process_hierarchy() {
    // Create a large process tree and verify all namespaces work correctly
    
    let root = ProcessDescriptor::new_root(1, 1, 0, "init".to_string())
        .expect("Failed to create root");
    
    let mut processes = vec![root];
    let mut kernel_pid = 2;
    
    // Create 20 child processes
    for i in 0..20 {
        let parent_idx = i / 3; // 3 children per parent
        let parent = &processes[parent_idx];
        
        let child = parent.create_child(kernel_pid, parent.kernel_pid, format!("proc_{}", i))
            .expect("Failed to create child");
        
        processes.push(child);
        kernel_pid += 1;
    }
    
    // Verify all processes share root's PID namespace
    let root_pid_ns_id = processes[0].pid_namespace().namespace_id();
    for proc in &processes {
        assert_eq!(proc.pid_namespace().namespace_id(), root_pid_ns_id);
    }
    
    // Verify no PIDs conflict
    let used_pids: Vec<_> = processes.iter().map(|p| p.namespace_pid).collect();
    let unique_pids: std::collections::HashSet<_> = used_pids.iter().copied().collect();
    assert_eq!(used_pids.len(), unique_pids.len());
}
