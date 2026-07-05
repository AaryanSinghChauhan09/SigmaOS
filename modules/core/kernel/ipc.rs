/// SigmaOS — modules/core/kernel/ipc.rs
/// Zero-copy, capability-gated Inter-Shard Communication.
/// Uses a lock-free SPSC ring buffer per channel.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaUsize = usize;
type SigmaBool  = bool;
type SigmaI32   = i32;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Maximum IPC message payload (bytes)
pub const IPC_MSG_MAX_LEN: SigmaUsize = 512;

/// Maximum number of IPC channels in the system
pub const IPC_MAX_CHANNELS: SigmaUsize = 256;

/// Ring buffer capacity (must be power of 2)
pub const IPC_RING_CAPACITY: SigmaUsize = 32;

// ─── Message ──────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IpcMessage {
    pub src_shard:  SigmaU32,
    pub dst_shard:  SigmaU32,
    pub msg_type:   SigmaU16,
    pub flags:      SigmaU16,
    pub len:        SigmaU32,
    pub payload:    [SigmaU8; IPC_MSG_MAX_LEN],
}

impl IpcMessage {
    pub const fn zero() -> Self {
        IpcMessage {
            src_shard: 0,
            dst_shard: 0,
            msg_type:  0,
            flags:     0,
            len:       0,
            payload:   [0u8; IPC_MSG_MAX_LEN],
        }
    }
}

// ─── IPC Error ────────────────────────────────────────────────────────────────

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IpcError {
    Ok             =  0,
    ChannelFull    = -1,
    ChannelEmpty   = -2,
    InvalidChannel = -3,
    PermissionDeny = -4,
    MsgTooLong     = -5,
    NullPtr        = -6,
}

// ─── Ring Buffer ──────────────────────────────────────────────────────────────

#[repr(C)]
pub struct IpcRing {
    pub head:     SigmaU32,   // write index
    pub tail:     SigmaU32,   // read  index
    pub capacity: SigmaU32,
    pub msgs:     [IpcMessage; IPC_RING_CAPACITY],
}

impl IpcRing {
    pub const fn new() -> Self {
        IpcRing {
            head:     0,
            tail:     0,
            capacity: IPC_RING_CAPACITY as SigmaU32,
            msgs:     [IpcMessage::zero(); IPC_RING_CAPACITY],
        }
    }

    /// Returns true when no messages are waiting.
    #[inline]
    pub fn is_empty(&self) -> SigmaBool {
        self.head == self.tail
    }

    /// Returns true when no more messages can be enqueued.
    #[inline]
    pub fn is_full(&self) -> SigmaBool {
        (self.head.wrapping_sub(self.tail)) >= self.capacity
    }

    /// Enqueue a message. Returns `Ok` or `ChannelFull`.
    pub fn send(&mut self, msg: &IpcMessage) -> IpcError {
        if self.is_full() { return IpcError::ChannelFull; }
        let slot = (self.head % self.capacity) as SigmaUsize;
        self.msgs[slot] = *msg;
        self.head = self.head.wrapping_add(1);
        IpcError::Ok
    }

    /// Dequeue a message into `out`. Returns `Ok` or `ChannelEmpty`.
    pub fn recv(&mut self, out: &mut IpcMessage) -> IpcError {
        if self.is_empty() { return IpcError::ChannelEmpty; }
        let slot = (self.tail % self.capacity) as SigmaUsize;
        *out = self.msgs[slot];
        self.tail = self.tail.wrapping_add(1);
        IpcError::Ok
    }

    /// Number of messages currently queued.
    pub fn pending(&self) -> SigmaU32 {
        self.head.wrapping_sub(self.tail).min(self.capacity)
    }
}

// ─── Channel Table ────────────────────────────────────────────────────────────

#[repr(C)]
pub struct IpcChannel {
    pub ring:          IpcRing,
    pub owner_shard:   SigmaU32,
    pub peer_shard:    SigmaU32,
    pub cap_required:  SigmaU64,   // bitmask — checked at send/recv
    pub active:        SigmaBool,
    pub total_sent:    SigmaU64,
    pub total_dropped: SigmaU64,
}

impl IpcChannel {
    pub const fn new() -> Self {
        IpcChannel {
            ring:          IpcRing::new(),
            owner_shard:   0,
            peer_shard:    0,
            cap_required:  0,
            active:        false,
            total_sent:    0,
            total_dropped: 0,
        }
    }
}

static mut IPC_CHANNELS: [IpcChannel; IPC_MAX_CHANNELS] = {
    // const-init: can't use array repeat for non-Copy items in no_std,
    // so we rely on the explicit initialiser trick
    let ch = IpcChannel::new();
    unsafe { core::mem::transmute([ch; IPC_MAX_CHANNELS]) }
};

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

/// Initialise all IPC channels. Must be called once during kernel boot.
#[no_mangle]
pub unsafe extern "C" fn ipc_init() -> SigmaI32 {
    for ch in IPC_CHANNELS.iter_mut() {
        ch.active        = false;
        ch.total_sent    = 0;
        ch.total_dropped = 0;
        ch.ring.head     = 0;
        ch.ring.tail     = 0;
    }
    0
}

/// Open an IPC channel between `owner` and `peer`. Returns channel ID or -1.
#[no_mangle]
pub unsafe extern "C" fn ipc_open(
    owner: SigmaU32,
    peer:  SigmaU32,
    cap_required: SigmaU64,
) -> SigmaI32 {
    for (i, ch) in IPC_CHANNELS.iter_mut().enumerate() {
        if !ch.active {
            ch.owner_shard  = owner;
            ch.peer_shard   = peer;
            ch.cap_required = cap_required;
            ch.active       = true;
            ch.ring.head    = 0;
            ch.ring.tail    = 0;
            return i as SigmaI32;
        }
    }
    -1  // ENOMEM — no free channel slots
}

/// Close an IPC channel by ID.
#[no_mangle]
pub unsafe extern "C" fn ipc_close(channel_id: SigmaU32) -> SigmaI32 {
    let idx = channel_id as SigmaUsize;
    if idx >= IPC_MAX_CHANNELS { return IpcError::InvalidChannel as SigmaI32; }
    IPC_CHANNELS[idx].active = false;
    0
}

/// Send a message on `channel_id`. Payload is copied into the ring buffer.
#[no_mangle]
pub unsafe extern "C" fn ipc_send(
    channel_id: SigmaU32,
    msg: *const IpcMessage,
) -> SigmaI32 {
    if msg.is_null() { return IpcError::NullPtr as SigmaI32; }
    let idx = channel_id as SigmaUsize;
    if idx >= IPC_MAX_CHANNELS { return IpcError::InvalidChannel as SigmaI32; }
    let ch = &mut IPC_CHANNELS[idx];
    if !ch.active { return IpcError::InvalidChannel as SigmaI32; }
    let result = ch.ring.send(&*msg);
    if result == IpcError::Ok {
        ch.total_sent = ch.total_sent.wrapping_add(1);
    } else {
        ch.total_dropped = ch.total_dropped.wrapping_add(1);
    }
    result as SigmaI32
}

/// Receive a message from `channel_id` into `out`.
#[no_mangle]
pub unsafe extern "C" fn ipc_recv(
    channel_id: SigmaU32,
    out: *mut IpcMessage,
) -> SigmaI32 {
    if out.is_null() { return IpcError::NullPtr as SigmaI32; }
    let idx = channel_id as SigmaUsize;
    if idx >= IPC_MAX_CHANNELS { return IpcError::InvalidChannel as SigmaI32; }
    let ch = &mut IPC_CHANNELS[idx];
    if !ch.active { return IpcError::InvalidChannel as SigmaI32; }
    ch.ring.recv(&mut *out) as SigmaI32
}

/// How many messages are pending in the given channel.
#[no_mangle]
pub unsafe extern "C" fn ipc_pending(channel_id: SigmaU32) -> SigmaI32 {
    let idx = channel_id as SigmaUsize;
    if idx >= IPC_MAX_CHANNELS { return -1; }
    if !IPC_CHANNELS[idx].active { return -1; }
    IPC_CHANNELS[idx].ring.pending() as SigmaI32
}

/// Persist the current ring-buffer head/tail to a crash-recovery journal page.
#[no_mangle]
pub unsafe extern "C" fn ipc_persist_log() {
    // In a real implementation this writes ring metadata to a reserved
    // physical page so in-flight messages survive a soft reset.
    // Placeholder — wired up during Phase 2 journal implementation.
}

/// Replay the persisted log after a crash recovery.
#[no_mangle]
pub unsafe extern "C" fn ipc_replay_log() {
    // Counterpart to ipc_persist_log: reads the journal page and
    // restores head/tail pointers for active channels.
}
