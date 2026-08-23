// nftables-Inspired Modern Firewall Framework
// Linux nftables provides a modern, flexible packet filtering framework with tables, chains, and rules
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

/// nftables-inspired table families
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftFamily {
    Ip,
    Ip6,
    Inet,
    Arp,
    Bridge,
    Netdev,
}

/// nftables-inspired hook types for chains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftHook {
    Prerouting,
    Input,
    Forward,
    Output,
    Postrouting,
    Ingress,
}

/// nftables-inspired chain priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NftPriority(i32);

impl NftPriority {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn raw(&self) -> i32 {
        self.0
    }
}

/// nftables-inspired chain types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftChainType {
    Filter,
    Nat,
    Route,
}

/// nftables-inspired chain policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftChainPolicy {
    Accept,
    Drop,
}

/// nftables-inspired verdict (action)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftVerdict {
    Accept,
    Drop,
    Queue,
    Continue,
    Return,
    Jump(u32), // chain number
    Goto(u32), // chain number
}

/// nftables-inspired payload protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftPayloadProtocol {
    Ether,
    Ip,
    Ip6,
    Tcp,
    Udp,
    Icmp,
    Icmpv6,
    Arp,
}

/// nftables-inspired register for storing intermediate values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftRegister {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
}

/// nftables-inspired expression types
#[derive(Debug, Clone)]
pub enum NftExpression {
    /// Payload expression: load data from packet
    Payload {
        protocol: NftPayloadProtocol,
        offset: u32,
        len: u32,
        dest_reg: NftRegister,
    },
    /// Meta expression: load packet metadata
    Meta {
        key: NftMetaKey,
        dest_reg: NftRegister,
    },
    /// Comparison expression
    Cmp {
        op: NftCmpOp,
        left_reg: NftRegister,
        right_reg: NftRegister,
    },
    /// Immediate value
    Immediate {
        data: Vec<u8>,
        dest_reg: NftRegister,
    },
    /// Counter expression
    Counter { packets: u64, bytes: u64 },
    /// Verdict expression
    Verdict(NftVerdict),
    /// Match expression
    Match {
        left_reg: NftRegister,
        right_reg: NftRegister,
        op: NftCmpOp,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftMetaKey {
    Iif,
    Oif,
    Iifname,
    Oifname,
    Iiftype,
    Oiftype,
    Skuid,
    Skgid,
    Nftrace,
    Rtclassid,
    Secmark,
    Nfproto,
    L4proto,
    BriVproto,
    BriVbrvlan,
    Dscp,
    Hdrlen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftCmpOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

/// nftables-inspired rule
#[derive(Debug, Clone)]
pub struct NftRule {
    pub handle: u64,
    pub expressions: Vec<NftExpression>,
    pub comment: String,
}

impl NftRule {
    pub fn new(handle: u64) -> Self {
        Self {
            handle,
            expressions: Vec::new(),
            comment: String::new(),
        }
    }

    pub fn with_expression(mut self, expr: NftExpression) -> Self {
        self.expressions.push(expr);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }
}

/// nftables-inspired chain
#[derive(Debug, Clone)]
pub struct NftChain {
    pub name: String,
    pub family: NftFamily,
    pub table: String,
    pub chain_type: NftChainType,
    pub hook: Option<NftHook>,
    pub priority: Option<NftPriority>,
    pub policy: NftChainPolicy,
    pub rules: Vec<NftRule>,
    pub next_handle: u64,
}

impl NftChain {
    pub fn new(name: String, family: NftFamily, table: String) -> Self {
        Self {
            name,
            family,
            table,
            chain_type: NftChainType::Filter,
            hook: None,
            priority: None,
            policy: NftChainPolicy::Accept,
            rules: Vec::new(),
            next_handle: 1,
        }
    }

    pub fn with_hook(mut self, hook: NftHook, priority: NftPriority) -> Self {
        self.hook = Some(hook);
        self.priority = Some(priority);
        self
    }

    pub fn with_policy(mut self, policy: NftChainPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn add_rule(&mut self, rule: NftRule) -> u64 {
        let handle = rule.handle;
        self.rules.push(rule);
        handle
    }

    pub fn create_rule(&mut self) -> NftRule {
        let handle = self.next_handle;
        self.next_handle += 1;
        NftRule::new(handle)
    }

    pub fn remove_rule(&mut self, handle: u64) -> Result<(), &'static str> {
        let pos = self.rules.iter().position(|r| r.handle == handle);
        if let Some(pos) = pos {
            self.rules.remove(pos);
            Ok(())
        } else {
            Err("Rule not found")
        }
    }

    pub fn get_rule(&self, handle: u64) -> Option<&NftRule> {
        self.rules.iter().find(|r| r.handle == handle)
    }
}

/// nftables-inspired table
#[derive(Debug, Clone)]
pub struct NftTable {
    pub name: String,
    pub family: NftFamily,
    pub chains: BTreeMap<String, NftChain>,
}

impl NftTable {
    pub fn new(name: String, family: NftFamily) -> Self {
        Self {
            name,
            family,
            chains: BTreeMap::new(),
        }
    }

    pub fn add_chain(&mut self, chain: NftChain) -> Result<(), &'static str> {
        if self.chains.contains_key(&chain.name) {
            return Err("Chain already exists");
        }
        self.chains.insert(chain.name.clone(), chain);
        Ok(())
    }

    pub fn get_chain(&self, name: &str) -> Option<&NftChain> {
        self.chains.get(name)
    }

    pub fn get_chain_mut(&mut self, name: &str) -> Option<&mut NftChain> {
        self.chains.get_mut(name)
    }

    pub fn remove_chain(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.chains.contains_key(name) {
            return Err("Chain does not exist");
        }
        self.chains.remove(name);
        Ok(())
    }

    pub fn list_chains(&self) -> Vec<&NftChain> {
        self.chains.values().collect()
    }
}

/// nftables-inspired set for address matching
#[derive(Debug, Clone)]
pub struct NftSet {
    pub name: String,
    pub table: String,
    pub family: NftFamily,
    pub key_type: NftDataType,
    pub data: Vec<Vec<u8>>,
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftDataType {
    Ipv4Addr,
    Ipv6Addr,
    EtherAddr,
    Ipv4AddrMask,
    Ipv6AddrMask,
    Port,
    Mark,
}

impl NftSet {
    pub fn new(name: String, table: String, family: NftFamily, key_type: NftDataType) -> Self {
        Self {
            name,
            table,
            family,
            key_type,
            data: Vec::new(),
            is_anonymous: false,
        }
    }

    pub fn anonymous(
        name: String,
        table: String,
        family: NftFamily,
        key_type: NftDataType,
    ) -> Self {
        Self {
            name,
            table,
            family,
            key_type,
            data: Vec::new(),
            is_anonymous: true,
        }
    }

    pub fn add_element(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        if !self.data.contains(&data) {
            self.data.push(data);
        }
        Ok(())
    }

    pub fn remove_element(&mut self, data: &[u8]) {
        self.data.retain(|d| d != data);
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        self.data.iter().any(|d| d == data)
    }
}

/// nftables-inspired map for key-value pairs
#[derive(Debug, Clone)]
pub struct NftMap {
    pub name: String,
    pub table: String,
    pub family: NftFamily,
    pub key_type: NftDataType,
    pub data_type: NftDataType,
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl NftMap {
    pub fn new(
        name: String,
        table: String,
        family: NftFamily,
        key_type: NftDataType,
        data_type: NftDataType,
    ) -> Self {
        Self {
            name,
            table,
            family,
            key_type,
            data_type,
            data: BTreeMap::new(),
        }
    }

    pub fn add_element(&mut self, key: Vec<u8>, data: Vec<u8>) {
        self.data.insert(key, data);
    }

    pub fn get_element(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    pub fn remove_element(&mut self, key: &[u8]) {
        self.data.remove(key);
    }
}

/// nftables-inspired counter for statistics
#[derive(Debug, Clone)]
pub struct NftCounter {
    pub packets: u64,
    pub bytes: u64,
}

impl NftCounter {
    pub fn new() -> Self {
        Self {
            packets: 0,
            bytes: 0,
        }
    }

    pub fn increment(&mut self, bytes: u64) {
        self.packets += 1;
        self.bytes += bytes;
    }

    pub fn reset(&mut self) {
        self.packets = 0;
        self.bytes = 0;
    }
}

/// nftables-inspired quota for rate limiting
#[derive(Debug, Clone)]
pub struct NftQuota {
    pub bytes: u64,
    pub used: u64,
    pub over: bool,
}

impl NftQuota {
    pub fn new(bytes: u64) -> Self {
        Self {
            bytes,
            used: 0,
            over: false,
        }
    }

    pub fn consume(&mut self, amount: u64) -> bool {
        if self.used + amount > self.bytes {
            self.over = true;
            false
        } else {
            self.used += amount;
            true
        }
    }

    pub fn reset(&mut self) {
        self.used = 0;
        self.over = false;
    }
}

/// nftables-inspired conntrack (connection tracking) state
#[derive(Debug, Clone)]
pub struct NftConntrack {
    pub connections: BTreeMap<u64, NftConnection>,
    pub next_id: u64,
}

#[derive(Debug, Clone)]
pub struct NftConnection {
    pub id: u64,
    pub src_addr: String,
    pub src_port: u16,
    pub dst_addr: String,
    pub dst_port: u16,
    pub protocol: NftPayloadProtocol,
    pub state: NftConnState,
    pub created: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NftConnState {
    New,
    Established,
    Related,
    Invalid,
}

impl NftConntrack {
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn add_connection(
        &mut self,
        src_addr: String,
        src_port: u16,
        dst_addr: String,
        dst_port: u16,
        protocol: NftPayloadProtocol,
        timeout: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let conn = NftConnection {
            id,
            src_addr,
            src_port,
            dst_addr,
            dst_port,
            protocol,
            state: NftConnState::New,
            created: 0,
            timeout,
        };

        let key = self.calculate_conn_key(&conn);
        self.connections.insert(key, conn);
        id
    }

    pub fn get_connection(
        &self,
        src_addr: &str,
        src_port: u16,
        dst_addr: &str,
        dst_port: u16,
        protocol: NftPayloadProtocol,
    ) -> Option<&NftConnection> {
        let key = self.calculate_key(src_addr, src_port, dst_addr, dst_port, protocol);
        self.connections.get(&key)
    }

    pub fn update_connection_state(
        &mut self,
        src_addr: &str,
        src_port: u16,
        dst_addr: &str,
        dst_port: u16,
        protocol: NftPayloadProtocol,
        new_state: NftConnState,
    ) {
        let key = self.calculate_key(src_addr, src_port, dst_addr, dst_port, protocol);
        if let Some(conn) = self.connections.get_mut(&key) {
            conn.state = new_state;
        }
    }

    pub fn cleanup_expired(&mut self, current_time: u64) -> usize {
        let mut expired = Vec::new();

        for (&key, conn) in &self.connections {
            if current_time - conn.created > conn.timeout {
                expired.push(key);
            }
        }

        let count = expired.len();

        for key in expired {
            self.connections.remove(&key);
        }

        count
    }

    fn calculate_conn_key(&self, conn: &NftConnection) -> u64 {
        self.calculate_key(
            &conn.src_addr,
            conn.src_port,
            &conn.dst_addr,
            conn.dst_port,
            conn.protocol,
        )
    }

    fn calculate_key(
        &self,
        src_addr: &str,
        src_port: u16,
        dst_addr: &str,
        dst_port: u16,
        protocol: NftPayloadProtocol,
    ) -> u64 {
        let mut hash: u64 = 5381;
        for byte in src_addr.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash = hash.wrapping_mul(33).wrapping_add(src_port as u64);
        for byte in dst_addr.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash = hash.wrapping_mul(33).wrapping_add(dst_port as u64);
        hash = hash.wrapping_mul(33).wrapping_add(protocol as u64);
        hash
    }
}

/// nftables-inspired firewall manager
pub struct NftablesManager {
    tables: BTreeMap<String, NftTable>,
    sets: BTreeMap<String, NftSet>,
    maps: BTreeMap<String, NftMap>,
    counters: BTreeMap<String, NftCounter>,
    quotas: BTreeMap<String, NftQuota>,
    conntrack: NftConntrack,
}

// ================= Stateful Iptables-to-Nftables Translation Engine =================

pub struct IptablesRuleLegacy {
    pub chain: String,
    pub protocol: String,
    pub dport: u16,
    pub action: String,
}

pub struct IptablesToNftablesTranslator;

impl IptablesToNftablesTranslator {
    /// Translates legacy iptables chains and rules into modern nftables expressions
    pub fn translate_rule(legacy_rule: &IptablesRuleLegacy, rule_handle: u64) -> NftRule {
        let mut expressions = Vec::new();

        // 1. Add Protocol expression (e.g. -p tcp)
        let proto = match legacy_rule.protocol.as_str() {
            "tcp" => NftPayloadProtocol::Tcp,
            "udp" => NftPayloadProtocol::Udp,
            _ => NftPayloadProtocol::Ip,
        };
        expressions.push(NftExpression::Payload {
            protocol: proto,
            offset: 9, // Protocol field in IPv4 header
            len: 1,
            dest_reg: NftRegister::Reg1,
        });

        // 2. Add Destination Port expression (e.g. --dport 80)
        expressions.push(NftExpression::Payload {
            protocol: proto,
            offset: 2, // Destination port field offset in TCP/UDP header
            len: 2,
            dest_reg: NftRegister::Reg2,
        });

        // 3. Add Verdict expression (e.g. -j ACCEPT)
        let verdict = match legacy_rule.action.as_str() {
            "ACCEPT" => NftVerdict::Accept,
            "DROP" => NftVerdict::Drop,
            _ => NftVerdict::Continue,
        };
        expressions.push(NftExpression::Verdict(verdict));

        NftRule {
            handle: rule_handle,
            expressions,
            comment: alloc::format!(
                "Translated from iptables -A {} -p {} --dport {} -j {}",
                legacy_rule.chain, legacy_rule.protocol, legacy_rule.dport, legacy_rule.action
            ),
        }
    }
}

// ================= Kubernetes-style Service Load Balancer & Proxy =================

#[derive(Clone)]
pub struct ServiceBackend {
    pub pod_ip: String,
    pub port: u16,
}

pub struct KubeProxyLoadBalancer {
    pub service_vip: String,
    pub service_port: u16,
    pub backends: Vec<ServiceBackend>,
    pub next_backend_idx: usize,
}

impl KubeProxyLoadBalancer {
    pub fn new(vip: &str, port: u16) -> Self {
        Self {
            service_vip: vip.to_string(),
            service_port: port,
            backends: Vec::new(),
            next_backend_idx: 0,
        }
    }

    pub fn register_pod_backend(&mut self, ip: &str, port: u16) {
        self.backends.push(ServiceBackend {
            pod_ip: ip.to_string(),
            port,
        });
    }

    /// Selects a backend Pod destination using round-robin (kube-proxy IPTABLES/IPVS-parity)
    pub fn route_connection(&mut self) -> Result<ServiceBackend, &'static str> {
        if self.backends.is_empty() {
            return Err("KubeProxy: No backend pods registered to receive cluster VIP traffic");
        }
        let backend = self.backends[self.next_backend_idx].clone();
        self.next_backend_idx = (self.next_backend_idx + 1) % self.backends.len();
        Ok(backend)
    }
}

impl NftablesManager {
    pub fn new() -> Self {
        Self {
            tables: BTreeMap::new(),
            sets: BTreeMap::new(),
            maps: BTreeMap::new(),
            counters: BTreeMap::new(),
            quotas: BTreeMap::new(),
            conntrack: NftConntrack::new(),
        }
    }

    pub fn add_table(&mut self, table: NftTable) -> Result<(), &'static str> {
        if self.tables.contains_key(&table.name) {
            return Err("Table already exists");
        }
        self.tables.insert(table.name.clone(), table);
        Ok(())
    }

    pub fn get_table(&self, name: &str) -> Option<&NftTable> {
        self.tables.get(name)
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut NftTable> {
        self.tables.get_mut(name)
    }

    pub fn remove_table(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.tables.contains_key(name) {
            return Err("Table does not exist");
        }
        self.tables.remove(name);
        Ok(())
    }

    pub fn add_set(&mut self, set: NftSet) -> Result<(), &'static str> {
        if self.sets.contains_key(&set.name) {
            return Err("Set already exists");
        }
        self.sets.insert(set.name.clone(), set);
        Ok(())
    }

    pub fn get_set(&self, name: &str) -> Option<&NftSet> {
        self.sets.get(name)
    }

    pub fn get_set_mut(&mut self, name: &str) -> Option<&mut NftSet> {
        self.sets.get_mut(name)
    }

    pub fn add_map(&mut self, map: NftMap) -> Result<(), &'static str> {
        if self.maps.contains_key(&map.name) {
            return Err("Map already exists");
        }
        self.maps.insert(map.name.clone(), map);
        Ok(())
    }

    pub fn get_map(&self, name: &str) -> Option<&NftMap> {
        self.maps.get(name)
    }

    pub fn get_map_mut(&mut self, name: &str) -> Option<&mut NftMap> {
        self.maps.get_mut(name)
    }

    pub fn add_counter(&mut self, name: String) -> Result<(), &'static str> {
        if self.counters.contains_key(&name) {
            return Err("Counter already exists");
        }
        self.counters.insert(name, NftCounter::new());
        Ok(())
    }

    pub fn get_counter(&self, name: &str) -> Option<&NftCounter> {
        self.counters.get(name)
    }

    pub fn get_counter_mut(&mut self, name: &str) -> Option<&mut NftCounter> {
        self.counters.get_mut(name)
    }

    pub fn add_quota(&mut self, name: String, bytes: u64) -> Result<(), &'static str> {
        if self.quotas.contains_key(&name) {
            return Err("Quota already exists");
        }
        self.quotas.insert(name, NftQuota::new(bytes));
        Ok(())
    }

    pub fn get_quota(&self, name: &str) -> Option<&NftQuota> {
        self.quotas.get(name)
    }

    pub fn get_quota_mut(&mut self, name: &str) -> Option<&mut NftQuota> {
        self.quotas.get_mut(name)
    }

    pub fn conntrack(&mut self) -> &mut NftConntrack {
        &mut self.conntrack
    }

    /// Process a packet through the nftables rules
    pub fn process_packet(
        &mut self,
        src_addr: String,
        src_port: u16,
        dst_addr: String,
        dst_port: u16,
        protocol: NftPayloadProtocol,
        hook: NftHook,
        family: NftFamily,
    ) -> NftVerdict {
        // Find the appropriate table and chain for this hook
        for table in self.tables.values() {
            if table.family != family {
                continue;
            }

            for chain in table.chains.values() {
                if chain.hook != Some(hook) {
                    continue;
                }

                // Process rules in this chain
                for rule in &chain.rules {
                    // Evaluate rule expressions
                    if self.evaluate_rule(rule, &src_addr, src_port, &dst_addr, dst_port, protocol)
                    {
                        // Extract verdict from rule
                        for expr in &rule.expressions {
                            if let NftExpression::Verdict(verdict) = expr {
                                return *verdict;
                            }
                        }
                    }
                }

                // Apply chain policy if no rule matched
                return match chain.policy {
                    NftChainPolicy::Accept => NftVerdict::Accept,
                    NftChainPolicy::Drop => NftVerdict::Drop,
                };
            }
        }

        // Default policy if no table/chain found
        NftVerdict::Accept
    }

    fn evaluate_rule(
        &self,
        rule: &NftRule,
        _src_addr: &str,
        _src_port: u16,
        _dst_addr: &str,
        _dst_port: u16,
        _protocol: NftPayloadProtocol,
    ) -> bool {
        // Simplified rule evaluation
        // In a real implementation, this would evaluate all expressions

        for expr in &rule.expressions {
            match expr {
                NftExpression::Verdict(_) => continue,
                NftExpression::Counter { .. } => continue,
                _ => {
                    // For simplicity, assume all other expressions match
                    // Real implementation would have complex expression evaluation
                }
            }
        }

        true
    }

    /// Get comprehensive statistics
    pub fn get_stats(&self) -> NftablesStats {
        let total_tables = self.tables.len();
        let total_chains: usize = self.tables.values().map(|t| t.chains.len()).sum();
        let total_rules: usize = self
            .tables
            .values()
            .flat_map(|t| t.chains.values())
            .map(|c| c.rules.len())
            .sum();
        let total_sets = self.sets.len();
        let total_maps = self.maps.len();
        let total_counters = self.counters.len();
        let total_quotas = self.quotas.len();
        let total_connections = self.conntrack.connections.len();

        NftablesStats {
            total_tables,
            total_chains,
            total_rules,
            total_sets,
            total_maps,
            total_counters,
            total_quotas,
            total_connections,
        }
    }

    /// Flush all rules from all chains
    pub fn flush_rules(&mut self) {
        for table in self.tables.values_mut() {
            for chain in table.chains.values_mut() {
                chain.rules.clear();
                chain.next_handle = 1;
            }
        }
    }

    /// Export configuration to ruleset format
    pub fn export_ruleset(&self) -> String {
        let mut output = String::new();

        for table in self.tables.values() {
            output.push_str(&format!(
                "table {} {} {{\n",
                format_family(table.family),
                table.name
            ));

            for chain in table.chains.values() {
                output.push_str(&format!("  chain {} {{\n", chain.name));

                if let Some(hook) = chain.hook {
                    output.push_str(&format!(
                        "    type {} hook {} priority {};\n",
                        format_chain_type(chain.chain_type),
                        format_hook(hook),
                        chain.priority.map_or(0, |p| p.raw())
                    ));
                }

                output.push_str(&format!("    policy {};\n", format_policy(chain.policy)));

                for rule in &chain.rules {
                    output.push_str(&format!("    # rule handle {}\n", rule.handle));
                    if !rule.comment.is_empty() {
                        output.push_str(&format!("    # {}\n", rule.comment));
                    }
                }

                output.push_str("  }\n");
            }

            output.push_str("}\n");
        }

        output
    }
}

fn format_family(family: NftFamily) -> &'static str {
    match family {
        NftFamily::Ip => "ip",
        NftFamily::Ip6 => "ip6",
        NftFamily::Inet => "inet",
        NftFamily::Arp => "arp",
        NftFamily::Bridge => "bridge",
        NftFamily::Netdev => "netdev",
    }
}

fn format_hook(hook: NftHook) -> &'static str {
    match hook {
        NftHook::Prerouting => "prerouting",
        NftHook::Input => "input",
        NftHook::Forward => "forward",
        NftHook::Output => "output",
        NftHook::Postrouting => "postrouting",
        NftHook::Ingress => "ingress",
    }
}

fn format_chain_type(chain_type: NftChainType) -> &'static str {
    match chain_type {
        NftChainType::Filter => "filter",
        NftChainType::Nat => "nat",
        NftChainType::Route => "route",
    }
}

fn format_policy(policy: NftChainPolicy) -> &'static str {
    match policy {
        NftChainPolicy::Accept => "accept",
        NftChainPolicy::Drop => "drop",
    }
}

/// nftables statistics
#[derive(Debug)]
pub struct NftablesStats {
    pub total_tables: usize,
    pub total_chains: usize,
    pub total_rules: usize,
    pub total_sets: usize,
    pub total_maps: usize,
    pub total_counters: usize,
    pub total_quotas: usize,
    pub total_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = NftTable::new("filter".to_string(), NftFamily::Ip);
        assert_eq!(table.family, NftFamily::Ip);
        assert_eq!(table.name, "filter");
    }

    #[test]
    fn test_chain_creation() {
        let chain = NftChain::new("input".to_string(), NftFamily::Ip, "filter".to_string())
            .with_hook(NftHook::Input, NftPriority::new(0))
            .with_policy(NftChainPolicy::Drop);

        assert_eq!(chain.hook, Some(NftHook::Input));
        assert_eq!(chain.policy, NftChainPolicy::Drop);
    }

    #[test]
    fn test_manager_operations() {
        let mut manager = NftablesManager::new();
        let table = NftTable::new("filter".to_string(), NftFamily::Ip);
        manager.add_table(table).unwrap();

        assert!(manager.get_table("filter").is_some());
    }

    #[test]
    fn test_set_operations() {
        let set = NftSet::new(
            "blocked".to_string(),
            "filter".to_string(),
            NftFamily::Ip,
            NftDataType::Ipv4Addr,
        );
        let mut manager = NftablesManager::new();
        manager.add_set(set).unwrap();

        let set = manager.get_set_mut("blocked").unwrap();
        set.add_element(vec![192, 168, 1, 1]).unwrap();

        assert!(set.contains(&vec![192, 168, 1, 1]));
    }

    #[test]
    fn test_iptables_to_nftables_translation() {
        let legacy_rule = IptablesRuleLegacy {
            chain: "INPUT".to_string(),
            protocol: "tcp".to_string(),
            dport: 80,
            action: "ACCEPT".to_string(),
        };

        let nft_rule = IptablesToNftablesTranslator::translate_rule(&legacy_rule, 101);
        assert_eq!(nft_rule.handle, 101);
        assert!(nft_rule.comment.contains("Translated from iptables"));
        assert_eq!(nft_rule.expressions.len(), 3);

        if let NftExpression::Verdict(verdict) = &nft_rule.expressions[2] {
            assert_eq!(*verdict, NftVerdict::Accept);
        } else {
            panic!("Expected verdict expression as third element");
        }
    }

    #[test]
    fn test_kubeproxy_load_balancer() {
        let mut lb = KubeProxyLoadBalancer::new("10.96.0.1", 80);
        lb.register_pod_backend("10.244.1.5", 8080);
        lb.register_pod_backend("10.244.2.8", 8080);

        let dest1 = lb.route_connection().unwrap();
        assert_eq!(dest1.pod_ip, "10.244.1.5");

        let dest2 = lb.route_connection().unwrap();
        assert_eq!(dest2.pod_ip, "10.244.2.8");

        let dest3 = lb.route_connection().unwrap();
        assert_eq!(dest3.pod_ip, "10.244.1.5"); // Loops back via round-robin
    }
}
