// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_firewall.rs — Stateful Packet Firewall (no_std)
// Language: Rust #![no_std]
// Pattern: OOP via Firewall struct + Rule / ConnTrack

#![no_std]

pub const MAX_RULES:    usize = 64;
pub const MAX_CONNTRACK: usize = 256;

// ── Rule ──────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RuleAction { Accept, Drop, Reject, Log }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction { In, Out, Both }

#[derive(Clone, Copy)]
pub struct Rule {
    pub action:    RuleAction,
    pub direction: Direction,
    pub src_ip:    Option<[u8; 4]>,
    pub src_mask:  [u8; 4],
    pub dst_ip:    Option<[u8; 4]>,
    pub dst_mask:  [u8; 4],
    pub src_port:  Option<u16>,
    pub dst_port:  Option<u16>,
    pub proto:     Option<u8>, // 0=any, 6=TCP, 17=UDP, 1=ICMP
    pub priority:  u8,
    pub enabled:   bool,
}

impl Rule {
    pub const fn default_accept() -> Self {
        Self {
            action: RuleAction::Accept, direction: Direction::Both,
            src_ip: None, src_mask: [255;4],
            dst_ip: None, dst_mask: [255;4],
            src_port: None, dst_port: None,
            proto: None, priority: 255, enabled: true,
        }
    }

    pub const fn block_port(port: u16, proto: u8) -> Self {
        Self {
            action: RuleAction::Drop, direction: Direction::In,
            src_ip: None, src_mask: [255;4],
            dst_ip: None, dst_mask: [255;4],
            src_port: None, dst_port: Some(port),
            proto: Some(proto), priority: 10, enabled: true,
        }
    }

    fn matches_ip(opt_ip: &Option<[u8; 4]>, mask: &[u8; 4], actual: &[u8; 4]) -> bool {
        match opt_ip {
            None => true,
            Some(rule_ip) => {
                for i in 0..4 {
                    if (rule_ip[i] & mask[i]) != (actual[i] & mask[i]) { return false; }
                }
                true
            }
        }
    }

    pub fn matches(&self, pkt: &Packet, dir: Direction) -> bool {
        if !self.enabled { return false; }
        if self.direction != Direction::Both && self.direction != dir { return false; }
        if !Self::matches_ip(&self.src_ip, &self.src_mask, &pkt.src_ip) { return false; }
        if !Self::matches_ip(&self.dst_ip, &self.dst_mask, &pkt.dst_ip) { return false; }
        if let Some(p) = self.proto { if p != pkt.proto { return false; } }
        if let Some(sp) = self.src_port { if sp != pkt.src_port { return false; } }
        if let Some(dp) = self.dst_port { if dp != pkt.dst_port { return false; } }
        true
    }
}

// ── Packet descriptor ─────────────────────────────────────────────────────────

pub struct Packet {
    pub src_ip:   [u8; 4],
    pub dst_ip:   [u8; 4],
    pub proto:    u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub flags:    u8, // TCP flags
    pub len:      usize,
}

// ── Connection Tracking ───────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConnState { New, Established, Related, Closing, Invalid }

#[derive(Clone, Copy)]
pub struct ConnEntry {
    pub src_ip:   [u8; 4],
    pub dst_ip:   [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub proto:    u8,
    pub state:    ConnState,
    pub pkts:     u64,
    pub bytes:    u64,
    pub age_ticks: u32,
}

pub struct ConnTrack {
    table:  [Option<ConnEntry>; MAX_CONNTRACK],
    count:  usize,
    ticks:  u32,
}

impl ConnTrack {
    pub const fn new() -> Self {
        Self { table: [const { None }; MAX_CONNTRACK], count: 0, ticks: 0 }
    }

    fn hash(pkt: &Packet) -> usize {
        let mut h: u32 = 0;
        for &b in &pkt.src_ip { h = h.wrapping_mul(31).wrapping_add(b as u32); }
        for &b in &pkt.dst_ip { h = h.wrapping_mul(31).wrapping_add(b as u32); }
        h = h.wrapping_mul(31).wrapping_add(pkt.src_port as u32);
        h = h.wrapping_mul(31).wrapping_add(pkt.dst_port as u32);
        (h as usize) % MAX_CONNTRACK
    }

    pub fn lookup(&self, pkt: &Packet) -> Option<&ConnEntry> {
        let slot = Self::hash(pkt);
        if let Some(ref e) = self.table[slot] {
            if e.src_ip == pkt.src_ip && e.dst_ip == pkt.dst_ip
               && e.src_port == pkt.src_port && e.dst_port == pkt.dst_port
               && e.proto == pkt.proto
            { return Some(e); }
        }
        None
    }

    pub fn insert(&mut self, pkt: &Packet, state: ConnState) {
        let slot = Self::hash(pkt);
        let entry = ConnEntry {
            src_ip: pkt.src_ip, dst_ip: pkt.dst_ip,
            src_port: pkt.src_port, dst_port: pkt.dst_port,
            proto: pkt.proto, state,
            pkts: 1, bytes: pkt.len as u64, age_ticks: 0,
        };
        if self.table[slot].is_none() { self.count += 1; }
        self.table[slot] = Some(entry);
    }

    pub fn update(&mut self, pkt: &Packet) {
        let slot = Self::hash(pkt);
        if let Some(ref mut e) = self.table[slot] {
            e.pkts  += 1;
            e.bytes += pkt.len as u64;
            e.age_ticks = 0;
            if pkt.proto == 6 && pkt.flags & 0x01 != 0 { // FIN
                e.state = ConnState::Closing;
            }
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        // Expire connections older than 300 ticks
        for slot in &mut self.table {
            if let Some(ref mut e) = slot {
                e.age_ticks += 1;
                if e.age_ticks > 300 { *slot = None; self.count -= 1; }
            }
        }
    }
}

// ── Firewall ──────────────────────────────────────────────────────────────────

pub struct Firewall {
    rules:    [Option<Rule>; MAX_RULES],
    n_rules:  usize,
    conntrack: ConnTrack,
    default_action: RuleAction,
}

impl Firewall {
    pub const fn new() -> Self {
        Self {
            rules: [const { None }; MAX_RULES],
            n_rules: 0,
            conntrack: ConnTrack::new(),
            default_action: RuleAction::Accept,
        }
    }

    pub fn add_rule(&mut self, rule: Rule) -> bool {
        if self.n_rules >= MAX_RULES { return false; }
        self.rules[self.n_rules] = Some(rule);
        self.n_rules += 1;
        // Sort by priority (lower = higher priority) — bubble sort (small N)
        let n = self.n_rules;
        for i in 0..n {
            for j in 0..n-i-1 {
                let pi = self.rules[j].map(|r| r.priority).unwrap_or(255);
                let pj = self.rules[j+1].map(|r| r.priority).unwrap_or(255);
                if pi > pj { self.rules.swap(j, j+1); }
            }
        }
        true
    }

    pub fn remove_rule(&mut self, idx: usize) {
        if idx < self.n_rules {
            self.rules[idx] = None;
            // Compact
            for i in idx..self.n_rules-1 { self.rules[i] = self.rules[i+1]; }
            self.n_rules -= 1;
        }
    }

    pub fn process(&mut self, pkt: &Packet, dir: Direction) -> RuleAction {
        // Conntrack: if known-established, fast-accept
        if let Some(conn) = self.conntrack.lookup(pkt) {
            if conn.state == ConnState::Established {
                self.conntrack.update(pkt);
                return RuleAction::Accept;
            }
        }

        // Match rules in priority order
        for slot in &self.rules[..self.n_rules] {
            if let Some(ref rule) = slot {
                if rule.matches(pkt, dir) {
                    match rule.action {
                        RuleAction::Accept => {
                            self.conntrack.insert(pkt, ConnState::Established);
                        }
                        _ => {}
                    }
                    return rule.action;
                }
            }
        }

        // Default
        if self.default_action == RuleAction::Accept {
            self.conntrack.insert(pkt, ConnState::New);
        }
        self.default_action
    }

    pub fn set_default(&mut self, action: RuleAction) { self.default_action = action; }
    pub fn tick(&mut self) { self.conntrack.tick(); }
    pub fn conn_count(&self) -> usize { self.conntrack.count }
}
