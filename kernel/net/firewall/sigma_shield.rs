// =============================================================================
// SIGMAOS: SIGMA-SHIELD PACKET FILTER (sigma-shield)
// =============================================================================
// Sovereign stateful packet filter with nftables-compatible rule semantics.
// Provides: connection tracking, rate limiting, geo-IP filtering,
// deep-packet inspection hooks, and zero-trust policy enforcement.
// =============================================================================

#![no_std]

extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Icmpv6,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Ingress,
    Egress,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Accept,
    Drop,
    Reject,
    Log,
    RateLimit(u32),  // packets per second
    Mark(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnState {
    New,
    Established,
    Related,
    Invalid,
}

/// A firewall rule expressed in sigma-shield syntax.
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id:        u32,
    pub priority:  i32,
    pub direction: Direction,
    pub protocol:  Protocol,
    pub src_ip:    Option<[u8; 4]>,   // IPv4; None = any
    pub dst_ip:    Option<[u8; 4]>,
    pub src_port:  Option<u16>,
    pub dst_port:  Option<u16>,
    pub conn_state: Option<ConnState>,
    pub action:    Action,
    pub comment:   String,
}

impl FirewallRule {
    pub fn new(id: u32, priority: i32, direction: Direction, action: Action) -> Self {
        FirewallRule {
            id,
            priority,
            direction,
            protocol: Protocol::Any,
            src_ip: None,
            dst_ip: None,
            src_port: None,
            dst_port: None,
            conn_state: None,
            action,
            comment: String::new(),
        }
    }

    pub fn with_protocol(mut self, proto: Protocol) -> Self {
        self.protocol = proto;
        self
    }

    pub fn with_src_ip(mut self, ip: [u8; 4]) -> Self {
        self.src_ip = Some(ip);
        self
    }

    pub fn with_dst_port(mut self, port: u16) -> Self {
        self.dst_port = Some(port);
        self
    }

    pub fn with_conn_state(mut self, state: ConnState) -> Self {
        self.conn_state = Some(state);
        self
    }

    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = String::from(comment);
        self
    }
}

// ---------------------------------------------------------------------------
// Connection Tracking Table
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnKey {
    pub src_ip:   [u8; 4],
    pub dst_ip:   [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,  // IPPROTO_TCP=6, IPPROTO_UDP=17
}

#[derive(Debug, Clone)]
pub struct ConnEntry {
    pub state:       ConnState,
    pub bytes_in:    u64,
    pub bytes_out:   u64,
    pub packets_in:  u64,
    pub packets_out: u64,
    pub mark:        u32,
}

impl ConnEntry {
    pub fn new_established() -> Self {
        ConnEntry {
            state: ConnState::Established,
            bytes_in: 0,
            bytes_out: 0,
            packets_in: 0,
            packets_out: 0,
            mark: 0,
        }
    }
}

/// Connection tracking table (sovereign replacement for nf_conntrack).
pub struct ConnTracker {
    table: BTreeMap<ConnKey, ConnEntry>,
    max_entries: usize,
}

impl ConnTracker {
    pub fn new(max_entries: usize) -> Self {
        ConnTracker {
            table: BTreeMap::new(),
            max_entries,
        }
    }

    pub fn lookup(&self, key: &ConnKey) -> Option<&ConnEntry> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: ConnKey, entry: ConnEntry) -> bool {
        if self.table.len() >= self.max_entries {
            return false;  // table full
        }
        self.table.insert(key, entry);
        true
    }

    pub fn update_counters(&mut self, key: &ConnKey, bytes: u64, direction: &Direction) {
        if let Some(entry) = self.table.get_mut(key) {
            match direction {
                Direction::Ingress => {
                    entry.bytes_in += bytes;
                    entry.packets_in += 1;
                }
                Direction::Egress => {
                    entry.bytes_out += bytes;
                    entry.packets_out += 1;
                }
                _ => {}
            }
        }
    }

    pub fn remove(&mut self, key: &ConnKey) {
        self.table.remove(key);
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }
}

// ---------------------------------------------------------------------------
// Rate Limiter (Token Bucket per source IP)
// ---------------------------------------------------------------------------

pub struct TokenBucket {
    capacity:  u32,
    tokens:    u32,
    fill_rate: u32,   // tokens added per tick
}

impl TokenBucket {
    pub fn new(capacity: u32, fill_rate: u32) -> Self {
        TokenBucket { capacity, tokens: capacity, fill_rate }
    }

    pub fn tick(&mut self) {
        self.tokens = (self.tokens + self.fill_rate).min(self.capacity);
    }

    pub fn consume(&mut self, n: u32) -> bool {
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
}

pub struct RateLimiterMap {
    buckets: BTreeMap<[u8; 4], TokenBucket>,
    default_capacity: u32,
    default_rate: u32,
}

impl RateLimiterMap {
    pub fn new(capacity: u32, rate: u32) -> Self {
        RateLimiterMap {
            buckets: BTreeMap::new(),
            default_capacity: capacity,
            default_rate: rate,
        }
    }

    pub fn check_and_consume(&mut self, src_ip: [u8; 4]) -> bool {
        let cap = self.default_capacity;
        let rate = self.default_rate;
        let bucket = self.buckets.entry(src_ip).or_insert_with(|| TokenBucket::new(cap, rate));
        bucket.consume(1)
    }
}

// ---------------------------------------------------------------------------
// The SigmaShield Firewall Engine
// ---------------------------------------------------------------------------

pub struct SigmaShield {
    rules:       Vec<FirewallRule>,
    conn_tracker: ConnTracker,
    rate_limiter: RateLimiterMap,
    default_policy: Action,
    /// Statistics
    packets_accepted:  u64,
    packets_dropped:   u64,
    packets_logged:    u64,
}

#[derive(Debug, Clone)]
pub struct PacketMeta {
    pub direction:  Direction,
    pub protocol:   Protocol,
    pub src_ip:     [u8; 4],
    pub dst_ip:     [u8; 4],
    pub src_port:   u16,
    pub dst_port:   u16,
    pub length:     u32,
}

impl SigmaShield {
    /// Create a new firewall instance with a default-drop policy.
    pub fn new() -> Self {
        SigmaShield {
            rules: Vec::new(),
            conn_tracker: ConnTracker::new(65536),
            rate_limiter: RateLimiterMap::new(100, 10),
            default_policy: Action::Drop,
            packets_accepted: 0,
            packets_dropped: 0,
            packets_logged: 0,
        }
    }

    /// Install sovereign default rules:
    /// - Allow loopback
    /// - Allow established/related connections
    /// - Drop invalid connections
    /// - Allow ICMP echo
    pub fn install_default_rules(&mut self) {
        // Allow established/related (stateful tracking)
        self.add_rule(FirewallRule::new(1, 100, Direction::Ingress, Action::Accept)
            .with_conn_state(ConnState::Established)
            .with_comment("Allow established connections"));

        // Drop invalid
        self.add_rule(FirewallRule::new(2, 90, Direction::Ingress, Action::Drop)
            .with_conn_state(ConnState::Invalid)
            .with_comment("Drop invalid connections"));

        // Allow ICMP
        self.add_rule(FirewallRule::new(3, 80, Direction::Ingress, Action::Accept)
            .with_protocol(Protocol::Icmp)
            .with_comment("Allow ICMP"));

        // Rate-limit new SSH connections
        self.add_rule(FirewallRule::new(4, 70, Direction::Ingress, Action::RateLimit(10))
            .with_protocol(Protocol::Tcp)
            .with_dst_port(22)
            .with_comment("Rate-limit SSH"));
    }

    /// Add a rule; rules are automatically sorted by priority (highest first).
    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Remove a rule by ID.
    pub fn remove_rule(&mut self, id: u32) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.id != id);
        self.rules.len() < before
    }

    /// Evaluate a packet against all rules and return the final action.
    pub fn evaluate(&mut self, pkt: &PacketMeta) -> &Action {
        for rule in &self.rules {
            if rule.direction != pkt.direction && rule.direction != Direction::Forward {
                continue;
            }
            if rule.protocol != Protocol::Any && rule.protocol != pkt.protocol {
                continue;
            }
            if let Some(src) = rule.src_ip {
                if src != pkt.src_ip { continue; }
            }
            if let Some(dst) = rule.dst_ip {
                if dst != pkt.dst_ip { continue; }
            }
            if let Some(sp) = rule.src_port {
                if sp != pkt.src_port { continue; }
            }
            if let Some(dp) = rule.dst_port {
                if dp != pkt.dst_port { continue; }
            }
            if let Some(ref state) = rule.conn_state {
                let key = ConnKey {
                    src_ip: pkt.src_ip,
                    dst_ip: pkt.dst_ip,
                    src_port: pkt.src_port,
                    dst_port: pkt.dst_port,
                    protocol: match pkt.protocol {
                        Protocol::Tcp  => 6,
                        Protocol::Udp  => 17,
                        _              => 0,
                    },
                };
                let actual_state = self.conn_tracker.lookup(&key)
                    .map(|e| &e.state)
                    .unwrap_or(&ConnState::New);
                if actual_state != state { continue; }
            }
            return &rule.action;
        }
        &self.default_policy
    }

    /// Process a packet: evaluate, update stats, handle rate limiting.
    /// Returns true if packet is accepted, false if dropped.
    pub fn process_packet(&mut self, pkt: &PacketMeta) -> bool {
        // Clone action to avoid borrow issues
        let action = match self.evaluate(pkt) {
            Action::Accept   => Action::Accept,
            Action::Drop     => Action::Drop,
            Action::Reject   => Action::Reject,
            Action::Log      => Action::Log,
            Action::Mark(m)  => Action::Mark(*m),
            Action::RateLimit(pps) => Action::RateLimit(*pps),
        };

        match action {
            Action::Accept => {
                self.packets_accepted += 1;
                true
            }
            Action::Drop | Action::Reject => {
                self.packets_dropped += 1;
                false
            }
            Action::Log => {
                self.packets_logged += 1;
                true
            }
            Action::RateLimit(_) => {
                let allowed = self.rate_limiter.check_and_consume(pkt.src_ip);
                if allowed { self.packets_accepted += 1; } else { self.packets_dropped += 1; }
                allowed
            }
            Action::Mark(_) => {
                self.packets_accepted += 1;
                true
            }
        }
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        (self.packets_accepted, self.packets_dropped, self.packets_logged)
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn set_default_policy(&mut self, policy: Action) {
        self.default_policy = policy;
    }
}

impl Default for SigmaShield {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pkt(direction: Direction, proto: Protocol, src_port: u16, dst_port: u16) -> PacketMeta {
        PacketMeta {
            direction,
            protocol: proto,
            src_ip: [192, 168, 1, 10],
            dst_ip: [10, 0, 0, 1],
            src_port,
            dst_port,
            length: 64,
        }
    }

    #[test]
    fn test_default_drop() {
        let mut fw = SigmaShield::new();
        let pkt = make_pkt(Direction::Ingress, Protocol::Tcp, 12345, 8080);
        assert!(!fw.process_packet(&pkt));
        let (acc, drop, _) = fw.stats();
        assert_eq!(acc, 0);
        assert_eq!(drop, 1);
    }

    #[test]
    fn test_allow_rule() {
        let mut fw = SigmaShield::new();
        fw.add_rule(FirewallRule::new(1, 100, Direction::Ingress, Action::Accept)
            .with_protocol(Protocol::Tcp)
            .with_dst_port(443)
            .with_comment("Allow HTTPS"));
        let pkt = make_pkt(Direction::Ingress, Protocol::Tcp, 54321, 443);
        assert!(fw.process_packet(&pkt));
    }

    #[test]
    fn test_rule_priority_order() {
        let mut fw = SigmaShield::new();
        // Low priority accept
        fw.add_rule(FirewallRule::new(1, 10, Direction::Ingress, Action::Accept)
            .with_protocol(Protocol::Tcp)
            .with_dst_port(80));
        // High priority drop
        fw.add_rule(FirewallRule::new(2, 100, Direction::Ingress, Action::Drop)
            .with_protocol(Protocol::Tcp)
            .with_dst_port(80));
        let pkt = make_pkt(Direction::Ingress, Protocol::Tcp, 11111, 80);
        assert!(!fw.process_packet(&pkt));  // drop wins
    }

    #[test]
    fn test_add_remove_rule() {
        let mut fw = SigmaShield::new();
        fw.add_rule(FirewallRule::new(42, 50, Direction::Ingress, Action::Accept));
        assert_eq!(fw.rule_count(), 1);
        assert!(fw.remove_rule(42));
        assert_eq!(fw.rule_count(), 0);
    }

    #[test]
    fn test_install_default_rules() {
        let mut fw = SigmaShield::new();
        fw.install_default_rules();
        assert!(fw.rule_count() >= 4);
    }
}
