//! Inter-process communication channel.
//!
//! `IpcChannel` is a bounded, synchronous, single-producer / single-consumer
//! message queue.  The design is:
//!
//! * **Race-free**: no global state; the channel is owned by whoever creates it.
//! * **Fail-fast**: sending to a full channel or a closed channel returns `Err`
//!   immediately rather than blocking indefinitely.
//! * **Zero-copy path**: `IpcMessage` carries a `Vec<u8>` payload; callers that
//!   want zero-copy semantics can use reference-counted slices at the call site.

use std::collections::VecDeque;
use thiserror::Error;

// ─── error ───────────────────────────────────────────────────────────────────

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum IpcError {
    #[error("channel is full (capacity {0})")]
    ChannelFull(usize),

    #[error("channel is empty")]
    ChannelEmpty,

    #[error("channel is closed")]
    ChannelClosed,

    #[error("message payload exceeds maximum size {0}")]
    MessageTooLarge(usize),

    #[error("permission denied: capability not held")]
    PermissionDenied,

    #[error("invalid channel identifier")]
    InvalidChannel,
}

// ─── message types ───────────────────────────────────────────────────────────

/// Discriminant for the kind of IPC message being sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcMessageKind {
    /// Plain data payload.
    Data,
    /// Control / signal to the remote end.
    Control,
    /// File-descriptor hand-off (represented as u32 in the payload).
    FileDescriptor,
    /// Capability delegation.
    Capability,
}

/// A single IPC message.
#[derive(Debug, Clone)]
pub struct IpcMessage {
    /// Sender's process identifier.
    pub sender: u32,
    /// Message type discriminant.
    pub kind: IpcMessageKind,
    /// Raw payload bytes.
    pub payload: Vec<u8>,
    /// Monotonically increasing sequence number assigned by the channel.
    pub seq: u64,
}

impl IpcMessage {
    /// Create a new data message.
    pub fn data(sender: u32, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            sender,
            kind: IpcMessageKind::Data,
            payload: payload.into(),
            seq: 0,
        }
    }

    /// Create a new control message.
    pub fn control(sender: u32, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            sender,
            kind: IpcMessageKind::Control,
            payload: payload.into(),
            seq: 0,
        }
    }
}

// ─── channel ─────────────────────────────────────────────────────────────────

/// A bounded IPC message channel.
///
/// Messages are ordered FIFO.  The channel is created with a fixed `capacity`;
/// attempts to send when full return `IpcError::ChannelFull` immediately.
pub struct IpcChannel {
    /// Unique identifier for this channel.
    pub id: u64,
    capacity: usize,
    max_message_bytes: usize,
    queue: VecDeque<IpcMessage>,
    next_seq: u64,
    closed: bool,
    // ── statistics ──────────────────────────────────────────────────────
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl IpcChannel {
    /// Default maximum payload size (64 KiB).
    pub const DEFAULT_MAX_BYTES: usize = 65_536;

    /// Create a new channel with the given capacity and default payload limit.
    pub fn new(id: u64, capacity: usize) -> Self {
        Self::with_max_bytes(id, capacity, Self::DEFAULT_MAX_BYTES)
    }

    /// Create a channel with a custom payload size limit.
    pub fn with_max_bytes(id: u64, capacity: usize, max_message_bytes: usize) -> Self {
        Self {
            id,
            capacity,
            max_message_bytes,
            queue: VecDeque::with_capacity(capacity),
            next_seq: 0,
            closed: false,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }

    /// Send a message.
    ///
    /// Returns `Err(IpcError::ChannelFull)` if the queue is at capacity.
    /// Returns `Err(IpcError::ChannelClosed)` if the channel has been closed.
    /// Returns `Err(IpcError::MessageTooLarge)` if the payload exceeds the limit.
    pub fn send(&mut self, mut msg: IpcMessage) -> Result<(), IpcError> {
        if self.closed {
            return Err(IpcError::ChannelClosed);
        }
        if self.queue.len() >= self.capacity {
            return Err(IpcError::ChannelFull(self.capacity));
        }
        if msg.payload.len() > self.max_message_bytes {
            return Err(IpcError::MessageTooLarge(self.max_message_bytes));
        }

        let payload_len = msg.payload.len() as u64;
        msg.seq = self.next_seq;
        self.next_seq += 1;
        self.queue.push_back(msg);
        self.messages_sent += 1;
        self.bytes_sent += payload_len;
        Ok(())
    }

    /// Receive the next message (FIFO order).
    ///
    /// Returns `Err(IpcError::ChannelEmpty)` if there are no pending messages.
    pub fn recv(&mut self) -> Result<IpcMessage, IpcError> {
        if self.queue.is_empty() {
            if self.closed {
                return Err(IpcError::ChannelClosed);
            }
            return Err(IpcError::ChannelEmpty);
        }
        let msg = self.queue.pop_front().unwrap();
        self.messages_received += 1;
        self.bytes_received += msg.payload.len() as u64;
        Ok(msg)
    }

    /// Peek at the next message without removing it.
    pub fn peek(&self) -> Option<&IpcMessage> {
        self.queue.front()
    }

    /// How many messages are currently queued.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// True when the queue has no pending messages.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// True when the queue has reached capacity.
    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.capacity
    }

    /// Maximum number of in-flight messages.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Mark the channel as closed.  Subsequent `send` calls will fail.
    /// Pending messages can still be drained with `recv`.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// True if the channel has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
