// #![no_std]
// #![no_main]

/// OOP-based Firewall for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 761
/// Implements packet filtering and network security

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RuleID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction { Accept = 0, Drop = 1, Reject = 2, Log = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol { TCP = 6, UDP = 17, ICMP = 1, Any = 255 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallError { Success = 0, InvalidRule = 1, NotFound = 2 }

pub trait FirewallRule {
    fn id(&self) -> RuleID;
    fn action(&self) -> RuleAction;
    fn protocol(&self) -> Protocol;
    fn source_ip(&self) -> &[u8];
    fn destination_ip(&self) -> &[u8];
    fn source_port(&self) -> u16;
    fn destination_port(&self) -> u16;
}

#[repr(C)]
pub struct SimpleFirewallRule {
    pub id: RuleID,
    pub action: AtomicUsize,
    pub protocol: AtomicUsize,
    pub source_ip: [u8; 4],
    pub destination_ip: [u8; 4],
    pub source_port: AtomicUsize,
    pub destination_port: AtomicUsize,
}

impl SimpleFirewallRule {
    pub fn new(id: RuleID, action: RuleAction, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> Self {
        let mut src_ip = [0u8; 4];
        let mut dst_ip = [0u8; 4];
        let src_len = source_ip.len().min(4);
        let dst_len = destination_ip.len().min(4);
        for i in 0..src_len { src_ip[i] = source_ip[i]; }
        for i in 0..dst_len { dst_ip[i] = destination_ip[i]; }
        SimpleFirewallRule {
            id,
            action: AtomicUsize::new(action as usize),
            protocol: AtomicUsize::new(protocol as usize),
            source_ip: src_ip,
            destination_ip: dst_ip,
            source_port: AtomicUsize::new(source_port as usize),
            destination_port: AtomicUsize::new(destination_port as usize),
        }
    }
}

impl FirewallRule for SimpleFirewallRule {
    fn id(&self) -> RuleID { self.id }
    fn action(&self) -> RuleAction {
        match self.action.load(Ordering::SeqCst) {
            0 => RuleAction::Accept,
            1 => RuleAction::Drop,
            2 => RuleAction::Reject,
            3 => RuleAction::Log,
            _ => RuleAction::Drop,
        }
    }
    fn protocol(&self) -> Protocol {
        match self.protocol.load(Ordering::SeqCst) {
            6 => Protocol::TCP,
            17 => Protocol::UDP,
            1 => Protocol::ICMP,
            _ => Protocol::Any,
        }
    }
    fn source_ip(&self) -> &[u8] { &self.source_ip }
    fn destination_ip(&self) -> &[u8] { &self.destination_ip }
    fn source_port(&self) -> u16 { self.source_port.load(Ordering::SeqCst) as u16 }
    fn destination_port(&self) -> u16 { self.destination_port.load(Ordering::SeqCst) as u16 }
}

pub trait Firewall {
    fn add_rule(&mut self, rule: Box<dyn FirewallRule>) -> Result<RuleID, FirewallError>;
    fn remove_rule(&mut self, id: RuleID) -> Result<(), FirewallError>;
    fn get_rule(&self, id: RuleID) -> Option<&dyn FirewallRule>;
    fn filter_packet(&self, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> RuleAction;
}

#[repr(C)]
pub struct SimpleFirewall {
    pub rules: Vec<Option<Box<dyn FirewallRule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFirewall {
    pub fn new() -> Self {
        SimpleFirewall {
            rules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Firewall for SimpleFirewall {
    fn add_rule(&mut self, rule: Box<dyn FirewallRule>) -> Result<RuleID, FirewallError> {
        let id = rule.id();
        self.rules.push(Some(rule));
        Ok(id)
    }

    fn remove_rule(&mut self, id: RuleID) -> Result<(), FirewallError> {
        for i in 0..self.rules.len {
            let rule_option = unsafe { &mut *self.rules.data.add(i) };
            if let Some(ref rule) = *rule_option {
                if rule.id() == id {
                    *rule_option = None;
                    return Ok(());
                }
            }
        }
        Err(FirewallError::NotFound)
    }

    fn get_rule(&self, id: RuleID) -> Option<&dyn FirewallRule> {
        for i in 0..self.rules.len {
            let rule_option = unsafe { &*self.rules.data.add(i) };
            if let Some(ref rule) = *rule_option {
                if rule.id() == id { return Some(rule.as_ref()); }
            }
        }
        None
    }

    fn filter_packet(&self, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> RuleAction {
        for i in 0..self.rules.len {
            let rule_option = unsafe { &*self.rules.data.add(i) };
            if let Some(ref rule) = *rule_option {
                if rule.protocol() == Protocol::Any || rule.protocol() == protocol {
                    if rule.source_ip() == source_ip || rule.source_ip() == &[0, 0, 0, 0] {
                        if rule.destination_ip() == destination_ip || rule.destination_ip() == &[0, 0, 0, 0] {
                            if rule.source_port() == source_port || rule.source_port() == 0 {
                                if rule.destination_port() == destination_port || rule.destination_port() == 0 {
                                    return rule.action();
                                }
                            }
                        }
                    }
                }
            }
        }
        RuleAction::Accept
    }
}

pub trait NAT {
    fn add_mapping(&mut self, internal_ip: &[u8], internal_port: u16, external_port: u16) -> Result<(), FirewallError>;
    fn remove_mapping(&mut self, internal_port: u16) -> Result<(), FirewallError>;
    fn translate(&self, internal_ip: &[u8], internal_port: u16) -> Option<(u16, [u8; 4])>;
}

#[repr(C)]
pub struct SimpleNAT {
    pub mappings: Vec<([u8; 4], u16, u16)>,
}

impl SimpleNAT {
    pub fn new() -> Self {
        SimpleNAT {
            mappings: Vec::new(),
        }
    }
}

impl NAT for SimpleNAT {
    fn add_mapping(&mut self, internal_ip: &[u8], internal_port: u16, external_port: u16) -> Result<(), FirewallError> {
        let mut ip_array = [0u8; 4];
        let ip_len = internal_ip.len().min(4);
        for i in 0..ip_len { ip_array[i] = internal_ip[i]; }
        self.mappings.push((ip_array, internal_port, external_port));
        Ok(())
    }

    fn remove_mapping(&mut self, internal_port: u16) -> Result<(), FirewallError> {
        for i in 0..self.mappings.len {
            let mapping = unsafe { &*self.mappings.data.add(i) };
            if mapping.1 == internal_port {
                self.mappings.remove(i);
                return Ok(());
            }
        }
        Err(FirewallError::NotFound)
    }

    fn translate(&self, internal_ip: &[u8], internal_port: u16) -> Option<(u16, [u8; 4])> {
        for i in 0..self.mappings.len {
            let &(ref ip, int_port, ext_port) = unsafe { &*self.mappings.data.add(i) };
            if ip == internal_ip && int_port == internal_port {
                return Some((ext_port, *ip));
            }
        }
        None
    }
}

// =========================================================================
// Linux-inspired Uncomplicated Firewall (UFW) Engine
// =========================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UfwLoggingLevel {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Full = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UfwAppProfile {
    pub name: [u8; 16],
    pub port: u16,
    pub protocol: Protocol,
}

impl UfwAppProfile {
    pub fn new(name_str: &str, port: u16, protocol: Protocol) -> Self {
        let mut name = [0u8; 16];
        let bytes = name_str.as_bytes();
        let len = bytes.len().min(16);
        for i in 0..len {
            name[i] = bytes[i];
        }
        UfwAppProfile { name, port, protocol }
    }

    pub fn matches(&self, query: &str) -> bool {
        let q_bytes = query.as_bytes();
        let mut name_len = 0;
        while name_len < 16 && self.name[name_len] != 0 {
            name_len += 1;
        }
        if name_len != q_bytes.len() {
            return false;
        }
        for i in 0..name_len {
            let c1 = self.name[i].to_ascii_lowercase();
            let c2 = q_bytes[i].to_ascii_lowercase();
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UfwRule {
    pub id: RuleID,
    pub action: RuleAction,
    pub protocol: Protocol,
    pub port: u16,
    pub is_limit: bool,
    pub app_name: [u8; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RateLimitRecord {
    pub source_ip: [u8; 4],
    pub port: u16,
    pub count: usize,
    pub blocked_until: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UfwLogEntry {
    pub protocol: Protocol,
    pub source_ip: [u8; 4],
    pub destination_ip: [u8; 4],
    pub source_port: u16,
    pub destination_port: u16,
    pub action: RuleAction,
}

#[repr(C)]
pub struct UfwEngine {
    pub is_enabled: bool,
    pub logging_level: UfwLoggingLevel,
    pub rules: Vec<UfwRule>,
    pub app_profiles: Vec<UfwAppProfile>,
    pub rate_records: Vec<RateLimitRecord>,
    pub logs: Vec<UfwLogEntry>,
    pub next_rule_id: usize,
}

impl UfwEngine {
    pub fn new() -> Self {
        let mut engine = UfwEngine {
            is_enabled: false,
            logging_level: UfwLoggingLevel::Low,
            rules: Vec::new(),
            app_profiles: Vec::new(),
            rate_records: Vec::new(),
            logs: Vec::new(),
            next_rule_id: 1,
        };
        // Prepopulate with Linux-standard Application Profiles
        engine.app_profiles.push(UfwAppProfile::new("Ssh", 22, Protocol::TCP));
        engine.app_profiles.push(UfwAppProfile::new("Http", 80, Protocol::TCP));
        engine.app_profiles.push(UfwAppProfile::new("Https", 443, Protocol::TCP));
        engine.app_profiles.push(UfwAppProfile::new("Ftp", 21, Protocol::TCP));
        engine.app_profiles.push(UfwAppProfile::new("Nginx", 80, Protocol::TCP));
        engine.app_profiles.push(UfwAppProfile::new("Samba", 445, Protocol::TCP));
        engine
    }

    pub fn filter_packet(&mut self, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16, current_tick: u64) -> RuleAction {
        if !self.is_enabled {
            return RuleAction::Accept;
        }

        // UFW default policy: drop incoming traffic unless explicitly allowed
        let mut final_action = RuleAction::Drop;
        let mut matched_rule: Option<UfwRule> = None;

        for i in 0..self.rules.len {
            let rule = unsafe { &*self.rules.data.add(i) };
            if rule.protocol == Protocol::Any || rule.protocol == protocol {
                if rule.port == destination_port || rule.port == 0 {
                    matched_rule = Some(*rule);
                    break;
                }
            }
        }

        if let Some(rule) = matched_rule {
            if rule.is_limit {
                let mut record_idx = None;
                for i in 0..self.rate_records.len {
                    let rec = unsafe { &*self.rate_records.data.add(i) };
                    if rec.source_ip == source_ip && rec.port == destination_port {
                        record_idx = Some(i);
                        break;
                    }
                }

                if let Some(idx) = record_idx {
                    let rec = unsafe { &mut *self.rate_records.data.add(idx) };
                    if current_tick > rec.blocked_until {
                        rec.count = 1;
                        rec.blocked_until = current_tick + 100;
                        final_action = rule.action;
                    } else {
                        rec.count += 1;
                        if rec.count > 6 {
                            final_action = RuleAction::Drop; // Rate limit exceeded!
                        } else {
                            final_action = rule.action;
                        }
                    }
                } else {
                    let mut ip_arr = [0u8; 4];
                    let len = source_ip.len().min(4);
                    for i in 0..len { ip_arr[i] = source_ip[i]; }
                    self.rate_records.push(RateLimitRecord {
                        source_ip: ip_arr,
                        port: destination_port,
                        count: 1,
                        blocked_until: current_tick + 100,
                    });
                    final_action = rule.action;
                }
            } else {
                final_action = rule.action;
            }
        }

        let should_log = match self.logging_level {
            UfwLoggingLevel::Off => false,
            UfwLoggingLevel::Low => final_action == RuleAction::Drop || final_action == RuleAction::Reject,
            UfwLoggingLevel::Medium => true,
            UfwLoggingLevel::High => true,
            UfwLoggingLevel::Full => true,
        };

        if should_log {
            let mut src_ip_arr = [0u8; 4];
            let mut dst_ip_arr = [0u8; 4];
            for i in 0..source_ip.len().min(4) { src_ip_arr[i] = source_ip[i]; }
            for i in 0..destination_ip.len().min(4) { dst_ip_arr[i] = destination_ip[i]; }
            self.logs.push(UfwLogEntry {
                protocol,
                source_ip: src_ip_arr,
                destination_ip: dst_ip_arr,
                source_port,
                destination_port,
                action: final_action,
            });
        }

        final_action
    }

    pub fn execute_ufw_command(&mut self, cmd: &str) -> Result<UfwCommandResponse, FirewallError> {
        let trimmed = cmd.trim();
        // Parse "ufw " base
        if trimmed.len() < 4 {
            return Err(FirewallError::InvalidRule);
        }
        let first_four = &trimmed[..4];
        let mut is_ufw = true;
        let ufw_lower = "ufw ";
        for (c1, c2) in first_four.bytes().zip(ufw_lower.bytes()) {
            if c1.to_ascii_lowercase() != c2 {
                is_ufw = false;
                break;
            }
        }
        if !is_ufw {
            return Err(FirewallError::InvalidRule);
        }

        let cmd_body = trimmed[4..].trim();

        // Manual case-insensitive commands matching
        if cmd_body.len() == 6 && cmd_body.eq_ignore_ascii_case("enable") {
            self.is_enabled = true;
            return Ok(UfwCommandResponse::new("Firewall is active and enabled on system startup", true));
        } else if cmd_body.len() == 7 && cmd_body.eq_ignore_ascii_case("disable") {
            self.is_enabled = false;
            return Ok(UfwCommandResponse::new("Firewall stopped and disabled on system startup", true));
        } else if cmd_body.len() >= 8 && cmd_body[..8].eq_ignore_ascii_case("logging ") {
            let level_str = cmd_body[8..].trim();
            if level_str.eq_ignore_ascii_case("off") {
                self.logging_level = UfwLoggingLevel::Off;
            } else if level_str.eq_ignore_ascii_case("low") {
                self.logging_level = UfwLoggingLevel::Low;
            } else if level_str.eq_ignore_ascii_case("medium") {
                self.logging_level = UfwLoggingLevel::Medium;
            } else if level_str.eq_ignore_ascii_case("high") {
                self.logging_level = UfwLoggingLevel::High;
            } else if level_str.eq_ignore_ascii_case("full") {
                self.logging_level = UfwLoggingLevel::Full;
            } else {
                return Err(FirewallError::InvalidRule);
            }
            return Ok(UfwCommandResponse::new("Logging enabled", true));
        } else if cmd_body.eq_ignore_ascii_case("status") || cmd_body.eq_ignore_ascii_case("status verbose") {
            let mut buf = UfwBuffer::new();
            buf.write_str("Status: ");
            if self.is_enabled {
                buf.write_str("active\n");
            } else {
                buf.write_str("inactive\n");
                return Ok(UfwCommandResponse {
                    success: true,
                    message: buf.data,
                    message_len: buf.len,
                });
            }

            buf.write_str("Logging: on (");
            let lvl_name = match self.logging_level {
                UfwLoggingLevel::Off => "off",
                UfwLoggingLevel::Low => "low",
                UfwLoggingLevel::Medium => "medium",
                UfwLoggingLevel::High => "high",
                UfwLoggingLevel::Full => "full",
            };
            buf.write_str(lvl_name);
            buf.write_str(")\n");
            buf.write_str("Default: deny (incoming), allow (outgoing), disabled (routed)\n\n");
            buf.write_str("To                         Action      From\n");
            buf.write_str("--                         ------      ----\n");

            for i in 0..self.rules.len {
                let rule = unsafe { &*self.rules.data.add(i) };
                buf.write_num(rule.port as usize);
                let proto_str = match rule.protocol {
                    Protocol::TCP => "/tcp",
                    Protocol::UDP => "/udp",
                    Protocol::ICMP => "/icmp",
                    Protocol::Any => "",
                };
                buf.write_str(proto_str);
                buf.write_str("                     ");
                let act_str = match rule.action {
                    RuleAction::Accept => {
                        if rule.is_limit { "LIMIT" } else { "ALLOW" }
                    }
                    RuleAction::Drop => "DROP",
                    RuleAction::Reject => "REJECT",
                    RuleAction::Log => "LOG",
                };
                buf.write_str(act_str);
                buf.write_str(" IN    Anywhere\n");
            }

            return Ok(UfwCommandResponse {
                success: true,
                message: buf.data,
                message_len: buf.len,
            });
        } else if cmd_body.len() > 6 && (cmd_body[..6].eq_ignore_ascii_case("allow ") || cmd_body[..6].eq_ignore_ascii_case("limit ")) {
            let is_limit = cmd_body[..6].eq_ignore_ascii_case("limit ");
            let arg = cmd_body[6..].trim();

            let mut matched_app = None;
            for i in 0..self.app_profiles.len {
                let app = unsafe { &*self.app_profiles.data.add(i) };
                if app.matches(arg) {
                    matched_app = Some(*app);
                    break;
                }
            }

            if let Some(app) = matched_app {
                let id = self.next_rule_id;
                self.next_rule_id += 1;
                self.rules.push(UfwRule {
                    id,
                    action: RuleAction::Accept,
                    protocol: app.protocol,
                    port: app.port,
                    is_limit,
                    app_name: app.name,
                });
                let mut msg = UfwBuffer::new();
                msg.write_str("Rule added (App profile: ");
                msg.write_str(arg);
                msg.write_str(")");
                return Ok(UfwCommandResponse {
                    success: true,
                    message: msg.data,
                    message_len: msg.len,
                });
            }

            // Port / protocol parse
            let mut port_str = arg;
            let mut proto = Protocol::Any;
            if let Some(slash_idx) = arg.find('/') {
                port_str = &arg[..slash_idx];
                let proto_str = &arg[slash_idx + 1..];
                if proto_str.eq_ignore_ascii_case("tcp") {
                    proto = Protocol::TCP;
                } else if proto_str.eq_ignore_ascii_case("udp") {
                    proto = Protocol::UDP;
                }
            }

            let mut port = 0u16;
            for b in port_str.bytes() {
                if b >= b'0' && b <= b'9' {
                    port = port * 10 + (b - b'0') as u16;
                } else {
                    return Err(FirewallError::InvalidRule);
                }
            }

            let id = self.next_rule_id;
            self.next_rule_id += 1;
            let mut app_name = [0u8; 16];
            for (i, b) in arg.bytes().take(16).enumerate() {
                app_name[i] = b;
            }

            self.rules.push(UfwRule {
                id,
                action: RuleAction::Accept,
                protocol: proto,
                port,
                is_limit,
                app_name,
            });

            if is_limit {
                return Ok(UfwCommandResponse::new("Rule added (rate limiting)", true));
            } else {
                return Ok(UfwCommandResponse::new("Rule added", true));
            }
        } else if cmd_body.len() > 5 && cmd_body[..5].eq_ignore_ascii_case("deny ") {
            let arg = cmd_body[5..].trim();

            let mut matched_app = None;
            for i in 0..self.app_profiles.len {
                let app = unsafe { &*self.app_profiles.data.add(i) };
                if app.matches(arg) {
                    matched_app = Some(*app);
                    break;
                }
            }

            if let Some(app) = matched_app {
                let id = self.next_rule_id;
                self.next_rule_id += 1;
                self.rules.push(UfwRule {
                    id,
                    action: RuleAction::Drop,
                    protocol: app.protocol,
                    port: app.port,
                    is_limit: false,
                    app_name: app.name,
                });
                let mut msg = UfwBuffer::new();
                msg.write_str("Rule added (App profile: ");
                msg.write_str(arg);
                msg.write_str(")");
                return Ok(UfwCommandResponse {
                    success: true,
                    message: msg.data,
                    message_len: msg.len,
                });
            }

            let mut port_str = arg;
            let mut proto = Protocol::Any;
            if let Some(slash_idx) = arg.find('/') {
                port_str = &arg[..slash_idx];
                let proto_str = &arg[slash_idx + 1..];
                if proto_str.eq_ignore_ascii_case("tcp") {
                    proto = Protocol::TCP;
                } else if proto_str.eq_ignore_ascii_case("udp") {
                    proto = Protocol::UDP;
                }
            }

            let mut port = 0u16;
            for b in port_str.bytes() {
                if b >= b'0' && b <= b'9' {
                    port = port * 10 + (b - b'0') as u16;
                } else {
                    return Err(FirewallError::InvalidRule);
                }
            }

            let id = self.next_rule_id;
            self.next_rule_id += 1;
            let mut app_name = [0u8; 16];
            for (i, b) in arg.bytes().take(16).enumerate() {
                app_name[i] = b;
            }

            self.rules.push(UfwRule {
                id,
                action: RuleAction::Drop,
                protocol: proto,
                port,
                is_limit: false,
                app_name,
            });

            return Ok(UfwCommandResponse::new("Rule added", true));
        }

        Err(FirewallError::InvalidRule)
    }
}

// =========================================================================
// Helpers for no_std UFW
// =========================================================================

struct UfwBuffer {
    data: [u8; 2048],
    len: usize,
}

impl UfwBuffer {
    fn new() -> Self {
        UfwBuffer { data: [0u8; 2048], len: 0 }
    }

    fn write_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let limit = (self.len + bytes.len()).min(2048);
        for i in self.len..limit {
            self.data[i] = bytes[i - self.len];
        }
        self.len = limit;
    }

    fn write_num(&mut self, mut num: usize) {
        if num == 0 {
            self.write_str("0");
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 20;
        while num > 0 {
            i -= 1;
            buf[i] = b'0' + (num % 10) as u8;
            num /= 10;
        }
        unsafe {
            let s = core::str::from_utf8_unchecked(&buf[i..]);
            self.write_str(s);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UfwCommandResponse {
    pub success: bool,
    pub message: [u8; 2048],
    pub message_len: usize,
}

impl UfwCommandResponse {
    pub fn new(msg: &str, success: bool) -> Self {
        let mut message = [0u8; 2048];
        let bytes = msg.as_bytes();
        let len = bytes.len().min(2048);
        for i in 0..len {
            message[i] = bytes[i];
        }
        UfwCommandResponse {
            success,
            message,
            message_len: len,
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.message[..self.message_len]) }
    }
}

// =========================================================================
// OOP heap allocation-free/custom-heap Vec implementation
// =========================================================================

pub struct Vec<T> { pub data: *mut T, pub len: usize, pub capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    if let Ok(layout) = Layout::from_size_align(size, 8) {
        std_alloc(layout)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ufw_command_parsing() {
        let mut ufw = UfwEngine::new();
        assert!(!ufw.is_enabled);

        // test ufw enable
        let res = ufw.execute_ufw_command("ufw enable").unwrap();
        assert!(res.success);
        assert!(ufw.is_enabled);
        assert_eq!(res.as_str(), "Firewall is active and enabled on system startup");

        // test ufw disable
        let res = ufw.execute_ufw_command("ufw disable").unwrap();
        assert!(res.success);
        assert!(!ufw.is_enabled);
        assert_eq!(res.as_str(), "Firewall stopped and disabled on system startup");

        // test ufw logging level
        ufw.execute_ufw_command("ufw logging medium").unwrap();
        assert_eq!(ufw.logging_level, UfwLoggingLevel::Medium);

        // test ufw allow command with custom port
        let res = ufw.execute_ufw_command("ufw allow 8080/tcp").unwrap();
        assert!(res.success);
        assert_eq!(res.as_str(), "Rule added");

        // test ufw deny command with custom port
        let res = ufw.execute_ufw_command("ufw deny 9000/udp").unwrap();
        assert!(res.success);

        // test ufw limit command with custom port
        let res = ufw.execute_ufw_command("ufw limit 2222/tcp").unwrap();
        assert!(res.success);
        assert_eq!(res.as_str(), "Rule added (rate limiting)");

        // test status output formatting
        ufw.is_enabled = true;
        let status_res = ufw.execute_ufw_command("ufw status").unwrap();
        assert!(status_res.success);
        let status_str = status_res.as_str();
        assert!(status_str.contains("Status: active"));
        assert!(status_str.contains("Logging: on (medium)"));
        assert!(status_str.contains("8080/tcp"));
        assert!(status_str.contains("9000/udp"));
        assert!(status_str.contains("2222/tcp"));
    }

    #[test]
    fn test_ufw_rate_limiting() {
        let mut ufw = UfwEngine::new();
        ufw.execute_ufw_command("ufw enable").unwrap();
        ufw.execute_ufw_command("ufw limit 22/tcp").unwrap();

        let source_ip = [192, 168, 1, 50];
        let destination_ip = [10, 0, 0, 1];

        // Simulate 6 allowed requests within the 100-tick window
        for tick in 1..=6 {
            let action = ufw.filter_packet(Protocol::TCP, &source_ip, &destination_ip, 54321, 22, tick);
            assert_eq!(action, RuleAction::Accept);
        }

        // The 7th request should be rate-limited and dropped
        let block_action = ufw.filter_packet(Protocol::TCP, &source_ip, &destination_ip, 54321, 22, 7);
        assert_eq!(block_action, RuleAction::Drop);

        // Ensure rate records tracked the attempts
        assert_eq!(ufw.rate_records.len, 1);
        let rec = unsafe { &*ufw.rate_records.data.add(0) };
        assert_eq!(rec.count, 7);

        // Simulating window expiry at tick 150 (tick 7 + 100 = 107 blocked_until, so 150 resets)
        let reset_action = ufw.filter_packet(Protocol::TCP, &source_ip, &destination_ip, 54321, 22, 150);
        assert_eq!(reset_action, RuleAction::Accept);
    }

    #[test]
    fn test_ufw_application_profiles() {
        let mut ufw = UfwEngine::new();
        ufw.execute_ufw_command("ufw enable").unwrap();

        // Allow application profile
        let res = ufw.execute_ufw_command("ufw allow Nginx").unwrap();
        assert!(res.success);
        assert_eq!(res.as_str(), "Rule added (App profile: Nginx)");

        // Filter packet destined to HTTP port 80
        let allowed_action = ufw.filter_packet(Protocol::TCP, &[192, 168, 1, 100], &[10, 0, 0, 1], 12345, 80, 1);
        assert_eq!(allowed_action, RuleAction::Accept);

        // Filter packet destined to HTTPS port 443 (not allowed yet, so should default drop)
        let blocked_action = ufw.filter_packet(Protocol::TCP, &[192, 168, 1, 100], &[10, 0, 0, 1], 12345, 443, 1);
        assert_eq!(blocked_action, RuleAction::Drop);

        // Allow HTTPS app profile
        ufw.execute_ufw_command("ufw allow Https").unwrap();
        let allowed_https_action = ufw.filter_packet(Protocol::TCP, &[192, 168, 1, 100], &[10, 0, 0, 1], 12345, 443, 1);
        assert_eq!(allowed_https_action, RuleAction::Accept);
    }
}
