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
