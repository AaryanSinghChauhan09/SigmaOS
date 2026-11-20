//! Integration tests — IPC channel send / receive semantics.

use sigmaos_system::ipc::{IpcChannel, IpcError, IpcMessage, IpcMessageKind};

// ── helpers ───────────────────────────────────────────────────────────────────

fn channel(cap: usize) -> IpcChannel {
    IpcChannel::new(1, cap)
}

fn data_msg(sender: u32, payload: &[u8]) -> IpcMessage {
    IpcMessage::data(sender, payload.to_vec())
}

// ── construction ──────────────────────────────────────────────────────────────

#[test]
fn new_channel_is_empty() {
    let ch = channel(8);
    assert!(ch.is_empty());
    assert_eq!(ch.len(), 0);
}

#[test]
fn new_channel_is_not_closed() {
    let ch = channel(8);
    assert!(!ch.is_closed());
}

#[test]
fn capacity_is_respected() {
    let ch = channel(4);
    assert_eq!(ch.capacity(), 4);
}

// ── send ──────────────────────────────────────────────────────────────────────

#[test]
fn send_increases_len() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"hello")).unwrap();
    assert_eq!(ch.len(), 1);
}

#[test]
fn send_to_full_channel_returns_error() {
    let mut ch = channel(2);
    ch.send(data_msg(1, b"a")).unwrap();
    ch.send(data_msg(1, b"b")).unwrap();
    let err = ch.send(data_msg(1, b"c")).unwrap_err();
    assert!(matches!(err, IpcError::ChannelFull(2)));
}

#[test]
fn send_to_closed_channel_returns_error() {
    let mut ch = channel(8);
    ch.close();
    let err = ch.send(data_msg(1, b"x")).unwrap_err();
    assert!(matches!(err, IpcError::ChannelClosed));
}

#[test]
fn send_oversized_message_returns_error() {
    let mut ch = IpcChannel::with_max_bytes(1, 8, 4);
    let err = ch.send(data_msg(1, b"hello world")).unwrap_err();
    assert!(matches!(err, IpcError::MessageTooLarge(4)));
}

#[test]
fn send_increments_statistics() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"abc")).unwrap();
    assert_eq!(ch.messages_sent, 1);
    assert_eq!(ch.bytes_sent, 3);
}

// ── recv ──────────────────────────────────────────────────────────────────────

#[test]
fn recv_from_empty_channel_returns_error() {
    let mut ch = channel(8);
    let err = ch.recv().unwrap_err();
    assert!(matches!(err, IpcError::ChannelEmpty));
}

#[test]
fn recv_returns_sent_message() {
    let mut ch = channel(8);
    ch.send(data_msg(42, b"ping")).unwrap();
    let msg = ch.recv().unwrap();
    assert_eq!(msg.sender, 42);
    assert_eq!(msg.payload, b"ping");
    assert_eq!(msg.kind, IpcMessageKind::Data);
}

#[test]
fn recv_fifo_ordering() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"first")).unwrap();
    ch.send(data_msg(1, b"second")).unwrap();
    ch.send(data_msg(1, b"third")).unwrap();

    assert_eq!(ch.recv().unwrap().payload, b"first");
    assert_eq!(ch.recv().unwrap().payload, b"second");
    assert_eq!(ch.recv().unwrap().payload, b"third");
}

#[test]
fn recv_increments_statistics() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"xy")).unwrap();
    ch.recv().unwrap();
    assert_eq!(ch.messages_received, 1);
    assert_eq!(ch.bytes_received, 2);
}

#[test]
fn recv_decrements_len() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"a")).unwrap();
    ch.send(data_msg(1, b"b")).unwrap();
    ch.recv().unwrap();
    assert_eq!(ch.len(), 1);
}

// ── sequence numbers ─────────────────────────────────────────────────────────

#[test]
fn sequence_numbers_are_monotonically_increasing() {
    let mut ch = channel(8);
    for i in 0..5u64 {
        ch.send(data_msg(1, b"x")).unwrap();
        let msg = ch.recv().unwrap();
        assert_eq!(msg.seq, i);
    }
}

// ── close / drain ─────────────────────────────────────────────────────────────

#[test]
fn pending_messages_drain_after_close() {
    let mut ch = channel(8);
    ch.send(data_msg(1, b"last")).unwrap();
    ch.close();

    // Can still recv the queued message
    let msg = ch.recv().unwrap();
    assert_eq!(msg.payload, b"last");
}

#[test]
fn recv_empty_closed_channel_returns_error() {
    let mut ch = channel(8);
    ch.close();
    let err = ch.recv().unwrap_err();
    assert!(matches!(err, IpcError::ChannelClosed));
}

// ── peek ──────────────────────────────────────────────────────────────────────

#[test]
fn peek_does_not_remove_message() {
    let mut ch = channel(8);
    ch.send(data_msg(7, b"peek-me")).unwrap();
    let _ = ch.peek().unwrap();
    assert_eq!(ch.len(), 1); // still there
}

#[test]
fn peek_on_empty_returns_none() {
    let ch = channel(8);
    assert!(ch.peek().is_none());
}

// ── control messages ─────────────────────────────────────────────────────────

#[test]
fn control_message_kind_preserved() {
    let mut ch = channel(8);
    let ctrl = IpcMessage::control(99, b"shutdown");
    ch.send(ctrl).unwrap();
    let msg = ch.recv().unwrap();
    assert_eq!(msg.kind, IpcMessageKind::Control);
    assert_eq!(msg.sender, 99);
    assert_eq!(msg.payload, b"shutdown");
}

// ── full / not-full ───────────────────────────────────────────────────────────

#[test]
fn is_full_when_capacity_reached() {
    let mut ch = channel(1);
    assert!(!ch.is_full());
    ch.send(data_msg(1, b"x")).unwrap();
    assert!(ch.is_full());
}

#[test]
fn not_full_after_recv() {
    let mut ch = channel(1);
    ch.send(data_msg(1, b"x")).unwrap();
    ch.recv().unwrap();
    assert!(!ch.is_full());
}
