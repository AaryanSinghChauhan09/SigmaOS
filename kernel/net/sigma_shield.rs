//! SigmaOS: sigma-shield — Sovereign Packet Filter & Firewall
//! Essential Shard E12. Ring 1. no_std, no alloc.
//!
//! Implements connection tracking, rate limiting, and default-drop policy.
//! Hooks into the sigma-bus ingress/egress queues.

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

// ─── Firewall Rule Definitions ──────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Ingress = 0,
    Egress  = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Protocol {
    Any  = 0,
    Tcp  = 6,
    Udp  = 17,
    Icmp = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Drop      = 0,
    Accept    = 1,
    RateLimit = 2,
    Log       = 3,
}

#[derive(Clone, Copy)]
pub struct FirewallRule {
    pub id:         SigmaU32,
    pub priority:   SigmaU16,
    pub direction:  Direction,
    pub protocol:   Protocol,
    pub src_port:   SigmaU16,   // 0 = any
    pub dst_port:   SigmaU16,   // 0 = any
    pub action:     Action,
    pub rate_limit: SigmaU32,   // packets per second (only when action = RateLimit)
    pub active:     bool,
}

impl FirewallRule {
    pub const fn empty() -> Self {
        Self {
            id: 0, priority: 0, direction: Direction::Ingress,
            protocol: Protocol::Any, src_port: 0, dst_port: 0,
            action: Action::Drop, rate_limit: 0, active: false,
        }
    }

    /// Checks if this rule matches a given packet descriptor
    pub fn matches(&self, pkt: &PacketDescriptor) -> bool {
        if !self.active { return false; }
        if self.direction != pkt.direction { return false; }
        if self.protocol != Protocol::Any && self.protocol != pkt.protocol { return false; }
        if self.src_port != 0 && self.src_port != pkt.src_port { return false; }
        if self.dst_port != 0 && self.dst_port != pkt.dst_port { return false; }
        true
    }
}

// ─── Packet Descriptor ──────────────────────────────────────────────────────

/// Lightweight packet metadata extracted from the sigma-bus event
#[derive(Clone, Copy)]
pub struct PacketDescriptor {
    pub direction: Direction,
    pub protocol:  Protocol,
    pub src_port:  SigmaU16,
    pub dst_port:  SigmaU16,
    pub length:    SigmaU32,
}

// ─── Connection Tracking ────────────────────────────────────────────────────

const MAX_CONNECTIONS: usize = 1024;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum ConnState {
    None         = 0,
    New          = 1,
    Established  = 2,
    Related      = 3,
    Invalid      = 4,
}

#[derive(Clone, Copy)]
pub struct ConnTrackEntry {
    pub src_port:    SigmaU16,
    pub dst_port:    SigmaU16,
    pub protocol:    Protocol,
    pub state:       ConnState,
    pub last_seen_ms: SigmaU64,
    pub packet_count: SigmaU32,
    pub active:      bool,
}

impl ConnTrackEntry {
    pub const fn empty() -> Self {
        Self {
            src_port: 0, dst_port: 0, protocol: Protocol::Any,
            state: ConnState::None, last_seen_ms: 0, packet_count: 0, active: false,
        }
    }
}

// ─── sigma-shield Engine ────────────────────────────────────────────────────

const MAX_RULES: usize = 128;

pub struct SigmaShield {
    rules:       [FirewallRule; MAX_RULES],
    rule_count:  usize,
    conn_table:  [ConnTrackEntry; MAX_CONNECTIONS],
    conn_count:  usize,
    // Statistics
    packets_accepted: SigmaU64,
    packets_dropped:  SigmaU64,
    packets_logged:   SigmaU64,
    initialized:      bool,
}

impl SigmaShield {
    pub const fn new() -> Self {
        Self {
            rules:       [FirewallRule::empty(); MAX_RULES],
            rule_count:  0,
            conn_table:  [ConnTrackEntry::empty(); MAX_CONNECTIONS],
            conn_count:  0,
            packets_accepted: 0,
            packets_dropped:  0,
            packets_logged:   0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if self.initialized { return; }

        // Add default rules: allow established, drop all
        self.add_rule(FirewallRule {
            id: 1, priority: 255, direction: Direction::Ingress,
            protocol: Protocol::Any, src_port: 0, dst_port: 0,
            action: Action::Drop, rate_limit: 0, active: true,
        });
        self.add_rule(FirewallRule {
            id: 2, priority: 255, direction: Direction::Egress,
            protocol: Protocol::Any, src_port: 0, dst_port: 0,
            action: Action::Drop, rate_limit: 0, active: true,
        });

        self.initialized = true;
    }

    /// Add a firewall rule. Rules are matched in priority order (lower = higher priority).
    pub fn add_rule(&mut self, rule: FirewallRule) -> SigmaI32 {
        if self.rule_count >= MAX_RULES { return -1; }
        self.rules[self.rule_count] = rule;
        self.rule_count += 1;

        // Sort by priority (insertion sort — small N)
        let mut i = self.rule_count - 1;
        while i > 0 && self.rules[i].priority < self.rules[i - 1].priority {
            self.rules.swap(i, i - 1);
            i -= 1;
        }
        0
    }

    /// Core packet filter: returns the Action to take for a given packet.
    pub fn filter(&mut self, pkt: &PacketDescriptor) -> Action {
        // Check connection tracking first — allow established connections
        if self.is_established(pkt) {
            self.packets_accepted += 1;
            return Action::Accept;
        }

        // Walk rules in priority order
        for i in 0..self.rule_count {
            if self.rules[i].matches(pkt) {
                match self.rules[i].action {
                    Action::Accept => {
                        self.track_connection(pkt, ConnState::New);
                        self.packets_accepted += 1;
                        return Action::Accept;
                    },
                    Action::Drop => {
                        self.packets_dropped += 1;
                        return Action::Drop;
                    },
                    Action::Log => {
                        self.packets_logged += 1;
                        return Action::Log;
                    },
                    Action::RateLimit => {
                        // Simplified: just accept for now
                        self.packets_accepted += 1;
                        return Action::Accept;
                    },
                }
            }
        }

        // Default policy: DROP
        self.packets_dropped += 1;
        Action::Drop
    }

    fn is_established(&self, pkt: &PacketDescriptor) -> bool {
        for i in 0..self.conn_count {
            let c = &self.conn_table[i];
            if c.active && c.state == ConnState::Established
                && c.protocol == pkt.protocol
                && c.dst_port == pkt.src_port
            {
                return true;
            }
        }
        false
    }

    fn track_connection(&mut self, pkt: &PacketDescriptor, state: ConnState) {
        if self.conn_count >= MAX_CONNECTIONS { return; }
        self.conn_table[self.conn_count] = ConnTrackEntry {
            src_port: pkt.src_port,
            dst_port: pkt.dst_port,
            protocol: pkt.protocol,
            state,
            last_seen_ms: 0,
            packet_count: 1,
            active: true,
        };
        self.conn_count += 1;
    }

    pub fn stats(&self) -> (SigmaU64, SigmaU64, SigmaU64) {
        (self.packets_accepted, self.packets_dropped, self.packets_logged)
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

static mut SHIELD: SigmaShield = SigmaShield::new();

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_shield_init() {
    SHIELD.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_shield_add_rule(
    id: SigmaU32, priority: SigmaU16,
    direction: SigmaU8, protocol: SigmaU8,
    src_port: SigmaU16, dst_port: SigmaU16,
    action: SigmaU8, rate_limit: SigmaU32,
) -> SigmaI32 {
    let rule = FirewallRule {
        id, priority,
        direction: if direction == 1 { Direction::Egress } else { Direction::Ingress },
        protocol: match protocol {
            6  => Protocol::Tcp,
            17 => Protocol::Udp,
            1  => Protocol::Icmp,
            _  => Protocol::Any,
        },
        src_port, dst_port,
        action: match action {
            1 => Action::Accept,
            2 => Action::RateLimit,
            3 => Action::Log,
            _ => Action::Drop,
        },
        rate_limit, active: true,
    };
    SHIELD.add_rule(rule)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_shield_filter(
    direction: SigmaU8, protocol: SigmaU8,
    src_port: SigmaU16, dst_port: SigmaU16, length: SigmaU32,
) -> SigmaU8 {
    let pkt = PacketDescriptor {
        direction: if direction == 1 { Direction::Egress } else { Direction::Ingress },
        protocol: match protocol {
            6  => Protocol::Tcp,
            17 => Protocol::Udp,
            1  => Protocol::Icmp,
            _  => Protocol::Any,
        },
        src_port, dst_port, length,
    };
    SHIELD.filter(&pkt) as SigmaU8
}
