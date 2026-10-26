//! Verification test suite for SigmaOS IPC Handler Plan & Canonical IPC Objects
//!
//! Validates:
//! 1. Formal architecture specification file existence (`docs/ipc-handler.md` and `wiki/ipc-handler.md`).
//! 2. Pull Request template updates in `.github/PULL_REQUEST_TEMPLATE.md`.
//! 3. Canonical IPC Objects (`IpcChannel`, `IpcEndpoint`, `MessageBuffer`, `SharedMemoryRegion`, `WaitQueue`, `FileDescriptorHandle`, `IpcPermissions`, `IpcStatistics`) in `src/ipc/ipc.rs`.

#[path = "../src/ipc/ipc.rs"]
mod ipc;

use ipc::{
    FileDescriptorHandle, IPCError, IPCType, IpcChannel, IpcPermissions, IpcStatistics,
    SharedMemoryRegion,
};
use std::fs;
use std::path::Path;

#[test]
fn test_ipc_handler_documentation_exists() {
    let docs_path = Path::new("docs/ipc-handler.md");
    let wiki_path = Path::new("wiki/ipc-handler.md");

    assert!(
        docs_path.exists(),
        "docs/ipc-handler.md specification should exist"
    );
    assert!(
        wiki_path.exists(),
        "wiki/ipc-handler.md specification mirror should exist"
    );

    let docs_content = fs::read_to_string(docs_path).expect("Failed to read docs/ipc-handler.md");
    let wiki_content = fs::read_to_string(wiki_path).expect("Failed to read wiki/ipc-handler.md");

    assert!(docs_content.contains("# SigmaOS IPC Handler Master Architecture & Specification"));
    assert!(docs_content.contains("## Overview & Boundary Definition"));
    assert!(docs_content.contains("## Periodic Task Cadence"));
    assert!(docs_content.contains("## Development Phases & Deliverables"));
    assert!(docs_content.contains("## Missing Components Matrix"));
    assert!(docs_content.contains("## Industry-Standard Rules & Guidelines"));
    assert!(docs_content.contains("## Initial 10-PR Roadmap"));
    assert!(docs_content.contains("## Governance & Merge Policy"));

    assert_eq!(
        docs_content, wiki_content,
        "docs/ipc-handler.md and wiki/ipc-handler.md must match"
    );
}

#[test]
fn test_pull_request_template_ipc_sections() {
    let pr_template_path = Path::new(".github/PULL_REQUEST_TEMPLATE.md");
    assert!(
        pr_template_path.exists(),
        ".github/PULL_REQUEST_TEMPLATE.md must exist"
    );

    let content =
        fs::read_to_string(pr_template_path).expect("Failed to read PULL_REQUEST_TEMPLATE.md");
    assert!(content.contains("- [ ] IPC handler"));
    assert!(content.contains("- [ ] Pipes / FIFOs"));
    assert!(content.contains("- [ ] UNIX-domain sockets"));
    assert!(content.contains("- [ ] Shared memory"));
    assert!(content.contains("- [ ] Message queues"));
    assert!(content.contains("## IPC semantics"));
    assert!(content.contains("## Security review"));
}

#[test]
fn test_canonical_ipc_channel_and_objects() {
    let perms = IpcPermissions::new(1001, 1001, 0o600);
    let mut channel = IpcChannel::new(100, IPCType::Pipe, 256, perms);

    assert_eq!(channel.channel_id, 100);
    assert_eq!(channel.channel_type, IPCType::Pipe);
    assert!(!channel.closed);
    assert_eq!(channel.read_wait_queue.waiter_count(), 0);
    assert_eq!(channel.write_wait_queue.waiter_count(), 0);

    // Test write and read lifecycle
    let test_data = b"SigmaOS IPC Handshake Payload";
    let written = channel
        .write(test_data)
        .expect("Writing to open channel should succeed");
    assert_eq!(written, test_data.len());
    assert_eq!(channel.stats.bytes_written, test_data.len() as u64);
    assert_eq!(channel.stats.messages_sent, 1);

    let mut read_buf = [0u8; 64];
    let read_bytes = channel
        .read(&mut read_buf)
        .expect("Reading from populated channel should succeed");
    assert_eq!(read_bytes, test_data.len());
    assert_eq!(&read_buf[..read_bytes], test_data);
    assert_eq!(channel.stats.bytes_read, test_data.len() as u64);
    assert_eq!(channel.stats.messages_received, 1);

    // Test buffer empty return
    let read_empty = channel.read(&mut read_buf);
    assert_eq!(read_empty, Err(IPCError::BufferEmpty));

    // Test channel closure and EOF behavior
    channel.close();
    assert!(channel.closed);

    // Reading from closed empty channel returns 0 (EOF)
    let eof_read = channel
        .read(&mut read_buf)
        .expect("Reading from closed empty channel returns EOF 0");
    assert_eq!(eof_read, 0);

    // Writing to closed channel returns EndpointClosed error
    let write_closed = channel.write(test_data);
    assert_eq!(write_closed, Err(IPCError::EndpointClosed));
}

#[test]
fn test_ipc_descriptor_handle_and_shared_memory() {
    let fd_handle = FileDescriptorHandle::new(7, 100, true);
    assert_eq!(fd_handle.fd, 7);
    assert_eq!(fd_handle.channel_id, 100);
    assert!(fd_handle.is_read_end);
    assert!(!fd_handle.is_closed);

    let perms = IpcPermissions::new(1001, 1001, 0o600);
    let shm_region = SharedMemoryRegion::new(501, 4096, perms);
    assert_eq!(shm_region.region_id, 501);
    assert_eq!(shm_region.size, 4096);
    assert_eq!(shm_region.permissions.owner_uid, 1001);

    let stats = IpcStatistics::default();
    assert_eq!(stats.bytes_read, 0);
    assert_eq!(stats.bytes_written, 0);
    assert_eq!(stats.messages_sent, 0);
    assert_eq!(stats.messages_received, 0);
}
