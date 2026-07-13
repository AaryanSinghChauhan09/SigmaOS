// sigma_ipc_agent.rs — sigma-agent: IPC Message-Passing Bus
// Language: Rust (#![no_std], no external crates)
// OOP: Agent trait (abstract), ConcreteAgent (impl), AgentBus (composition)
// Specification: wiki_repo/sigma-agent.md
#![no_std]
#![allow(dead_code)]

// ═══════════════════════════════════════════════════════════════
//  § 1. Core types (all first-principles, no std/alloc)
// ═══════════════════════════════════════════════════════════════

/// Fixed-size byte ring buffer used for lock-free message queues.
pub struct RingBuffer<const CAP: usize> {
    data: [u8; CAP],
    head: usize,
    tail: usize,
    len:  usize,
}

impl<const CAP: usize> RingBuffer<CAP> {
    pub const fn new() -> Self {
        Self {
            data: [0u8; CAP],
            head: 0,
            tail: 0,
            len:  0,
        }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.len >= CAP { return false; }
        self.data[self.tail] = byte;
        self.tail = (self.tail + 1) % CAP;
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 { return None; }
        let b = self.data[self.head];
        self.head = (self.head + 1) % CAP;
        self.len -= 1;
        Some(b)
    }

    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn is_full(&self)  -> bool { self.len >= CAP }
    pub fn len(&self)      -> usize { self.len }
}

// ═══════════════════════════════════════════════════════════════
//  § 2. IPC Message format
// ═══════════════════════════════════════════════════════════════

pub const MSG_MAGIC: u16 = 0xA551;
pub const MAX_PAYLOAD: usize = 128;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum MsgKind {
    Ping       = 0x00,
    Pong       = 0x01,
    Register   = 0x10,
    Unregister = 0x11,
    Command    = 0x20,
    Response   = 0x21,
    Event      = 0x30,
    Error      = 0xFF,
}

#[repr(C, packed)]
pub struct IpcMessage {
    pub magic:       u16,
    pub kind:        MsgKind,
    pub src_agent:   u8,
    pub dst_agent:   u8,
    pub seq:         u16,
    pub payload_len: u8,
    pub payload:     [u8; MAX_PAYLOAD],
    pub checksum:    u16,
}

impl IpcMessage {
    pub const fn new(kind: MsgKind, src: u8, dst: u8, seq: u16) -> Self {
        Self {
            magic:       MSG_MAGIC,
            kind,
            src_agent:   src,
            dst_agent:   dst,
            seq,
            payload_len: 0,
            payload:     [0u8; MAX_PAYLOAD],
            checksum:    0,
        }
    }

    /// Write payload bytes (first `n` bytes of `data`).
    pub fn set_payload(&mut self, data: &[u8]) {
        let n = if data.len() > MAX_PAYLOAD { MAX_PAYLOAD } else { data.len() };
        let mut i = 0;
        while i < n {
            self.payload[i] = data[i];
            i += 1;
        }
        self.payload_len = n as u8;
        self.checksum = self.compute_checksum();
    }

    /// 16-bit Fletcher checksum over header + payload.
    fn compute_checksum(&self) -> u16 {
        let mut s1: u16 = 0;
        let mut s2: u16 = 0;
        let bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>() - 2, // Exclude checksum field
            )
        };
        let mut i = 0;
        while i < bytes.len() {
            s1 = s1.wrapping_add(bytes[i] as u16);
            s2 = s2.wrapping_add(s1);
            i += 1;
        }
        (s2 << 8) | (s1 & 0xFF)
    }

    pub fn validate(&self) -> bool {
        self.magic == MSG_MAGIC && self.checksum == self.compute_checksum()
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 3. Agent trait (abstract interface — OOP polymorphism)
// ═══════════════════════════════════════════════════════════════

pub trait Agent {
    fn agent_id(&self) -> u8;
    fn name(&self) -> &[u8];           // UTF-8 fixed slice (no alloc)
    fn on_message(&mut self, msg: &IpcMessage) -> Option<IpcMessage>;
    fn on_register(&mut self) {}
    fn on_unregister(&mut self) {}
}

// ═══════════════════════════════════════════════════════════════
//  § 4. ConcreteAgent — generic system agent (implements Agent)
// ═══════════════════════════════════════════════════════════════

pub struct ConcreteAgent {
    pub id:       u8,
    pub name_buf: [u8; 32],
    pub name_len: usize,
    pub seq_out:  u16,
    pub rx_count: u32,
}

impl ConcreteAgent {
    pub const fn new(id: u8, name: &[u8]) -> Self {
        let mut buf = [0u8; 32];
        let n = if name.len() > 32 { 32 } else { name.len() };
        let mut i = 0;
        while i < n {
            buf[i] = name[i];
            i += 1;
        }
        Self {
            id,
            name_buf: buf,
            name_len: n,
            seq_out:  0,
            rx_count: 0,
        }
    }
}

impl Agent for ConcreteAgent {
    fn agent_id(&self) -> u8 { self.id }

    fn name(&self) -> &[u8] { &self.name_buf[..self.name_len] }

    fn on_message(&mut self, msg: &IpcMessage) -> Option<IpcMessage> {
        self.rx_count += 1;
        match msg.kind {
            MsgKind::Ping => {
                self.seq_out = self.seq_out.wrapping_add(1);
                let mut resp = IpcMessage::new(MsgKind::Pong, self.id, msg.src_agent, self.seq_out);
                resp.set_payload(b"pong");
                Some(resp)
            }
            MsgKind::Command => {
                self.seq_out = self.seq_out.wrapping_add(1);
                let mut resp = IpcMessage::new(MsgKind::Response, self.id, msg.src_agent, self.seq_out);
                resp.set_payload(b"ok");
                Some(resp)
            }
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 5. AgentBus — composition hub routing messages between agents
// ═══════════════════════════════════════════════════════════════

const MAX_AGENTS: usize = 16;

pub struct AgentBus {
    agents: [Option<ConcreteAgent>; MAX_AGENTS],
    count:  usize,
    global_seq: u16,
}

impl AgentBus {
    pub const fn new() -> Self {
        const NONE_AGENT: Option<ConcreteAgent> = None;
        Self {
            agents:     [NONE_AGENT; MAX_AGENTS],
            count:      0,
            global_seq: 0,
        }
    }

    /// Register an agent on the bus.
    pub fn register(&mut self, agent: ConcreteAgent) -> bool {
        if self.count >= MAX_AGENTS { return false; }
        let idx = self.count;
        self.agents[idx] = Some(agent);
        if let Some(a) = &mut self.agents[idx] {
            a.on_register();
        }
        self.count += 1;
        true
    }

    /// Route a message to its destination agent; return reply if any.
    pub fn route(&mut self, msg: IpcMessage) -> Option<IpcMessage> {
        if !msg.validate() { return None; }
        let dst = msg.dst_agent as usize;
        let mut i = 0;
        while i < self.count {
            if let Some(agent) = &mut self.agents[i] {
                if agent.agent_id() == dst as u8 {
                    return agent.on_message(&msg);
                }
            }
            i += 1;
        }
        None
    }

    /// Broadcast an event message to all registered agents.
    pub fn broadcast(&mut self, kind: MsgKind) {
        self.global_seq = self.global_seq.wrapping_add(1);
        let seq = self.global_seq;
        let mut i = 0;
        while i < self.count {
            if let Some(agent) = &mut self.agents[i] {
                let msg = IpcMessage::new(kind, 0xFF, agent.agent_id(), seq);
                let _ = agent.on_message(&msg);
            }
            i += 1;
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 6. Tests (no_std compatible — no std::test harness)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut rb: RingBuffer<8> = RingBuffer::new();
        assert!(rb.push(42));
        assert!(rb.push(99));
        assert_eq!(rb.pop(), Some(42));
        assert_eq!(rb.pop(), Some(99));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_ipc_message_checksum() {
        let mut msg = IpcMessage::new(MsgKind::Ping, 1, 2, 0);
        msg.set_payload(b"hello");
        assert!(msg.validate());
    }

    #[test]
    fn test_agent_bus_ping_pong() {
        let mut bus = AgentBus::new();
        let a1 = ConcreteAgent::new(1, b"kernel-agent");
        let a2 = ConcreteAgent::new(2, b"ui-agent");
        assert!(bus.register(a1));
        assert!(bus.register(a2));
        let mut ping = IpcMessage::new(MsgKind::Ping, 1, 2, 0);
        ping.set_payload(b"ping");
        let resp = bus.route(ping);
        assert!(resp.is_some());
        let r = resp.unwrap();
        assert_eq!(r.kind, MsgKind::Pong);
        assert_eq!(r.src_agent, 2);
    }

    #[test]
    fn test_concrete_agent_counts() {
        let mut a = ConcreteAgent::new(10, b"test");
        let mut msg = IpcMessage::new(MsgKind::Command, 0, 10, 1);
        msg.set_payload(b"run");
        let _ = a.on_message(&msg);
        assert_eq!(a.rx_count, 1);
    }
}
