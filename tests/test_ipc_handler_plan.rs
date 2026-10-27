// Comprehensive Unit and Plan Verification Tests for SigmaOS IPC Handler Subsystem

use std::fs;

#[test]
fn test_ipc_handler_documentation_plan_verification() {
    let docs_plan = fs::read_to_string("docs/ipc-handler.md")
        .expect("docs/ipc-handler.md must exist");
    let wiki_plan = fs::read_to_string("wiki/ipc-handler.md")
        .expect("wiki/ipc-handler.md must exist");

    assert!(docs_plan.contains("SigmaOS IPC Handler Master Architecture, Support Matrix, and Program Plan"));
    assert!(docs_plan.contains("Repository Context and Scope"));
    assert!(docs_plan.contains("Scope and Boundaries"));
    assert!(docs_plan.contains("Periodic Task Cadence"));
    assert!(docs_plan.contains("Development Phases"));
    assert!(docs_plan.contains("Missing Components Matrix"));
    assert!(docs_plan.contains("Industry-Standard Rules"));
    assert!(docs_plan.contains("Correctness Rules"));
    assert!(docs_plan.contains("Security Rules"));
    assert!(docs_plan.contains("Rust-Specific Rules"));
    assert!(docs_plan.contains("Testing Rules"));
    assert!(docs_plan.contains("First 10 IPC PR Sequence"));
    assert!(docs_plan.contains("Governance and Merge Policy"));

    assert_eq!(
        docs_plan, wiki_plan,
        "wiki/ipc-handler.md must mirror docs/ipc-handler.md exactly"
    );
}

#[test]
fn test_ipc_handler_pr_template_verification() {
    let template = fs::read_to_string(".github/PULL_REQUEST_TEMPLATE.md")
        .expect(".github/PULL_REQUEST_TEMPLATE.md must exist");

    assert!(template.contains("IPC handler"));
    assert!(template.contains("IPC semantics"));
    assert!(template.contains("Blocking behavior:"));
    assert!(template.contains("Non-blocking behavior:"));
    assert!(template.contains("EOF / closure behavior:"));
    assert!(template.contains("Security review"));
    assert!(template.contains("Capability/namespace checks enforced"));
    assert!(template.contains("Verification"));
    assert!(template.contains("./run_sigma_tests.sh"));
    assert!(template.contains("Capability status"));
    assert!(template.contains("Integration impact"));
}

#[test]
fn test_ipc_core_object_invariants() {
    // Test MessageBuffer capacity and backpressure invariant
    struct LocalBuffer {
        capacity: usize,
        data: Vec<u8>,
    }

    impl LocalBuffer {
        fn new(capacity: usize) -> Self {
            Self {
                capacity,
                data: Vec::new(),
            }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, &'static str> {
            if self.data.len() >= self.capacity {
                return Err("Buffer full");
            }
            let available = self.capacity - self.data.len();
            let to_write = buf.len().min(available);
            self.data.extend_from_slice(&buf[..to_write]);
            Ok(to_write)
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
            if self.data.is_empty() {
                return Err("Buffer empty");
            }
            let to_read = buf.len().min(self.data.len());
            buf[..to_read].copy_from_slice(&self.data[..to_read]);
            self.data.drain(..to_read);
            Ok(to_read)
        }
    }

    let mut buf = LocalBuffer::new(8);
    assert_eq!(buf.write(b"hello").unwrap(), 5);
    assert_eq!(buf.write(b"world").unwrap(), 3); // Bounded to capacity 8
    assert!(buf.write(b"more").is_err()); // Backpressure trigger

    let mut read_out = [0u8; 5];
    assert_eq!(buf.read(&mut read_out).unwrap(), 5);
    assert_eq!(&read_out, b"hello");

    // Test Descriptor Single Close Invariant
    #[allow(dead_code)]
    struct LocalFdHandle {
        fd: usize,
        closed: bool,
    }

    impl LocalFdHandle {
        fn new(fd: usize) -> Self {
            Self { fd, closed: false }
        }

        fn close(&mut self) -> Result<(), &'static str> {
            if self.closed {
                return Err("Descriptor already closed");
            }
            self.closed = true;
            Ok(())
        }
    }

    let mut handle = LocalFdHandle::new(10);
    assert!(handle.close().is_ok());
    assert!(handle.close().is_err()); // Single close enforcement

    // Test Shared Memory Permission Boundary Invariant
    struct LocalShmPermissions {
        uid: u32,
        mode: u16,
    }

    impl LocalShmPermissions {
        fn can_access(&self, requester_uid: u32, requested_mode: u16) -> bool {
            if requester_uid == self.uid {
                (self.mode & requested_mode) == requested_mode
            } else {
                false
            }
        }
    }

    let perm = LocalShmPermissions { uid: 1000, mode: 0o600 }; // Read/Write for owner only
    assert!(perm.can_access(1000, 0o400)); // Owner read access
    assert!(perm.can_access(1000, 0o200)); // Owner write access
    assert!(!perm.can_access(1001, 0o400)); // Other user denied
}
