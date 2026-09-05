//! # Process Descriptor
//! 
//! This module provides ProcessDescriptor which tracks the complete lifecycle
//! of a process including its namespace context and resource associations.

use crate::runtime::process::process::{ProcessNamespaceContext, ProcessID};

/// Process Descriptor tracks complete process state including namespace context
#[derive(Clone)]
pub struct ProcessDescriptor {
    /// Process ID in the kernel
    pub kernel_pid: ProcessID,
    /// Process ID in the namespace (may differ from kernel_pid)
    pub namespace_pid: u32,
    /// Parent kernel PID
    pub parent_pid: ProcessID,
    /// Process name
    pub name: String,
    /// Associated namespace context
    pub namespace_context: ProcessNamespaceContext,
    /// Creation timestamp
    pub created_at: u64,
    /// Flag indicating if process is isolated (has unique namespaces)
    pub is_isolated: bool,
}

impl ProcessDescriptor {
    /// Create a new process descriptor with root namespaces
    pub fn new_root(
        kernel_pid: ProcessID,
        namespace_pid: u32,
        parent_pid: ProcessID,
        name: String,
    ) -> Result<Self, String> {
        let namespace_context = ProcessNamespaceContext::new_root();

        Ok(ProcessDescriptor {
            kernel_pid,
            namespace_pid,
            parent_pid,
            name,
            namespace_context,
            created_at: 0,
            is_isolated: true,
        })
    }

    /// Create a child process descriptor inheriting namespaces
    pub fn create_child(
        &self,
        kernel_pid: ProcessID,
        parent_kernel_pid: ProcessID,
        name: String,
    ) -> Result<Self, String> {
        // Allocate next PID in sequence
        let namespace_pid = self.namespace_pid + 1;

        // Child inherits parent's namespaces (not isolated)
        let namespace_context = self.namespace_context.clone_all();

        Ok(ProcessDescriptor {
            kernel_pid,
            namespace_pid,
            parent_pid: parent_kernel_pid,
            name,
            namespace_context,
            created_at: 0,
            is_isolated: false,
        })
    }

    /// Create a child with isolated namespaces (like containers)
    pub fn create_isolated_child(
        &self,
        kernel_pid: ProcessID,
        parent_kernel_pid: ProcessID,
        name: String,
    ) -> Result<Self, String> {
        // Allocate next PID in sequence
        let namespace_pid = self.namespace_pid + 1;

        // Child gets NEW namespaces (isolated)
        let namespace_context = self.namespace_context.create_child();

        Ok(ProcessDescriptor {
            kernel_pid,
            namespace_pid,
            parent_pid: parent_kernel_pid,
            name,
            namespace_context,
            created_at: 0,
            is_isolated: true,
        })
    }

    /// Release namespace resources (cleanup on process exit)
    pub fn cleanup(&self) -> Result<(), String> {
        // Decrement reference counts
        self.namespace_context.decrement_refs();

        Ok(())
    }

    /// Get metadata about this process descriptor
    pub fn metadata(&self) -> String {
        format!(
            "ProcessDescriptor {{ kernel_pid: {}, namespace_pid: {}, name: '{}', namespaces: [{}], isolated: {} }}",
            self.kernel_pid,
            self.namespace_pid,
            self.name,
            self.namespace_context.metadata_summary(),
            self.is_isolated
        )
    }

    /// Check if this process can access another process's namespace
    pub fn can_access_process_namespaces(&self, other: &ProcessDescriptor) -> bool {
        // Same namespace context = can access
        // For now, non-isolated siblings in same parent can access
        self.is_isolated == other.is_isolated && self.parent_pid == other.parent_pid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_process_descriptor_creation() {
        let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string());
        assert!(descriptor.is_ok());

        let desc = descriptor.unwrap();
        assert_eq!(desc.kernel_pid, 1);
        assert_eq!(desc.namespace_pid, 1);
        assert_eq!(desc.parent_pid, 0);
        assert_eq!(desc.name, "init");
        assert!(desc.is_isolated);
    }

    #[test]
    fn test_child_process_descriptor() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let child = parent
            .create_child(2, 1, "child".to_string())
            .expect("Failed to create child");

        assert_eq!(child.kernel_pid, 2);
        assert_eq!(child.parent_pid, 1);
        assert_eq!(child.name, "child");
        assert!(!child.is_isolated);
        assert_eq!(child.namespace_pid, 2); // Incremented from parent
    }

    #[test]
    fn test_isolated_child_process() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let isolated_child = parent
            .create_isolated_child(2, 1, "container".to_string())
            .expect("Failed to create isolated child");

        assert_eq!(isolated_child.kernel_pid, 2);
        assert!(isolated_child.is_isolated);
        
        // Namespaces should be different metadata from parent
        assert_ne!(
            parent.namespace_context.metadata_summary(),
            isolated_child.namespace_context.metadata_summary()
        );
    }

    #[test]
    fn test_namespace_inheritance() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let child1 = parent
            .create_child(2, 1, "child1".to_string())
            .expect("Failed to create child1");
        let child2 = parent
            .create_child(3, 1, "child2".to_string())
            .expect("Failed to create child2");

        // Non-isolated children should share namespace context with parent
        assert_eq!(
            parent.namespace_context.metadata_summary(),
            child1.namespace_context.metadata_summary()
        );
        assert_eq!(
            parent.namespace_context.metadata_summary(),
            child2.namespace_context.metadata_summary()
        );
    }

    #[test]
    fn test_can_access_process_namespaces() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let child = parent
            .create_child(2, 1, "child".to_string())
            .expect("Failed to create child");

        // Child can access parent's namespace (same parent, not isolated)
        assert!(parent.can_access_process_namespaces(&child));
        assert!(child.can_access_process_namespaces(&parent));

        let isolated = parent
            .create_isolated_child(3, 1, "isolated".to_string())
            .expect("Failed to create isolated child");

        // Cannot access across isolated namespaces
        assert!(!parent.can_access_process_namespaces(&isolated));
        assert!(!isolated.can_access_process_namespaces(&parent));
    }

    #[test]
    fn test_descriptor_metadata() {
        let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let metadata = descriptor.metadata();
        
        assert!(metadata.contains("ProcessDescriptor"));
        assert!(metadata.contains("init"));
        assert!(metadata.contains("isolated: true"));
    }

    #[test]
    fn test_multiple_children_pid_allocation() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();

        let child1 = parent
            .create_child(2, 1, "child1".to_string())
            .expect("Failed to create child1");
        let child2 = parent
            .create_child(3, 1, "child2".to_string())
            .expect("Failed to create child2");
        let child3 = parent
            .create_child(4, 1, "child3".to_string())
            .expect("Failed to create child3");

        // Each should get sequential PID in namespace
        assert_eq!(child1.namespace_pid, 2);
        assert_eq!(child2.namespace_pid, 3);
        assert_eq!(child3.namespace_pid, 4);
    }

    #[test]
    fn test_cleanup() {
        let parent = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();
        let child = parent
            .create_child(2, 1, "child".to_string())
            .expect("Failed to create child");

        // Cleanup should complete successfully
        let result = child.cleanup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_namespace_access() {
        let descriptor = ProcessDescriptor::new_root(1, 1, 0, "init".to_string()).unwrap();

        // Should be able to access namespace context
        let context = &descriptor.namespace_context;
        assert!(!context.metadata_summary().is_empty());
    }
}
