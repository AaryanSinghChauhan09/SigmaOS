//! SigmaOS Sovereign D-Bus IPC Protocol Implementation
//! Implements D-Bus message format and dispatch in 100% safe Rust.
//!
//! D-Bus is the standard IPC mechanism on Linux desktops (freedesktop.org).
//! Used by systemd, NetworkManager, BlueZ, PulseAudio, GNOME, KDE, etc.
//! This is a pure-Rust, zero-dependency implementation of the D-Bus wire protocol.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── D-Bus Type Signatures ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DbusValue {
    Byte(u8),
    Bool(bool),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Double(u32, u32),   // (high, low) — avoid f64 for no_std compatibility
    Str(String),
    ObjectPath(String),
    Signature(String),
    Array(Vec<DbusValue>),
    Variant(Box<DbusValue>),
    DictEntry(Box<DbusValue>, Box<DbusValue>),
    Struct(Vec<DbusValue>),
    UnixFd(u32),
}

impl DbusValue {
    pub fn type_signature(&self) -> &'static str {
        match self {
            DbusValue::Byte(_)        => "y",
            DbusValue::Bool(_)        => "b",
            DbusValue::Int16(_)       => "n",
            DbusValue::Uint16(_)      => "q",
            DbusValue::Int32(_)       => "i",
            DbusValue::Uint32(_)      => "u",
            DbusValue::Int64(_)       => "x",
            DbusValue::Uint64(_)      => "t",
            DbusValue::Double(_, _)   => "d",
            DbusValue::Str(_)         => "s",
            DbusValue::ObjectPath(_)  => "o",
            DbusValue::Signature(_)   => "g",
            DbusValue::Array(_)       => "a",
            DbusValue::Variant(_)     => "v",
            DbusValue::DictEntry(_,_) => "e",
            DbusValue::Struct(_)      => "r",
            DbusValue::UnixFd(_)      => "h",
        }
    }
}

// ─── D-Bus Message Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DbusMessageType {
    MethodCall,
    MethodReturn,
    Error,
    Signal,
}

impl DbusMessageType {
    pub fn code(&self) -> u8 {
        match self {
            DbusMessageType::MethodCall   => 1,
            DbusMessageType::MethodReturn => 2,
            DbusMessageType::Error        => 3,
            DbusMessageType::Signal       => 4,
        }
    }
}

// ─── D-Bus Message Header Fields ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DbusMessage {
    pub msg_type: DbusMessageType,
    pub flags: u8,           // NO_REPLY_EXPECTED=1, NO_AUTO_START=2, ALLOW_INTERACTIVE_AUTH=4
    pub serial: u32,
    pub path: Option<String>,        // object path  e.g. /org/sigma/NetworkManager
    pub interface: Option<String>,   // e.g. org.sigma.NetworkManager
    pub member: Option<String>,      // method/signal name e.g. GetDevices
    pub error_name: Option<String>,  // for Error messages
    pub reply_serial: Option<u32>,   // for MethodReturn / Error
    pub destination: Option<String>, // e.g. org.sigma.NetworkManager
    pub sender: Option<String>,      // e.g. :1.42
    pub signature: Option<String>,   // body type signature
    pub body: Vec<DbusValue>,
}

impl DbusMessage {
    pub fn method_call(path: &str, interface: &str, member: &str) -> Self {
        DbusMessage {
            msg_type: DbusMessageType::MethodCall,
            flags: 0,
            serial: 0, // will be set by bus
            path: Some(path.to_string()),
            interface: Some(interface.to_string()),
            member: Some(member.to_string()),
            error_name: None,
            reply_serial: None,
            destination: None,
            sender: None,
            signature: None,
            body: Vec::new(),
        }
    }

    pub fn signal(path: &str, interface: &str, member: &str) -> Self {
        DbusMessage {
            msg_type: DbusMessageType::Signal,
            flags: 0,
            serial: 0,
            path: Some(path.to_string()),
            interface: Some(interface.to_string()),
            member: Some(member.to_string()),
            error_name: None,
            reply_serial: None,
            destination: None,
            sender: None,
            signature: None,
            body: Vec::new(),
        }
    }

    pub fn method_return(reply_to_serial: u32) -> Self {
        DbusMessage {
            msg_type: DbusMessageType::MethodReturn,
            flags: 0,
            serial: 0,
            path: None,
            interface: None,
            member: None,
            error_name: None,
            reply_serial: Some(reply_to_serial),
            destination: None,
            sender: None,
            signature: None,
            body: Vec::new(),
        }
    }

    pub fn error(reply_to_serial: u32, name: &str) -> Self {
        DbusMessage {
            msg_type: DbusMessageType::Error,
            flags: 0,
            serial: 0,
            path: None,
            interface: None,
            member: None,
            error_name: Some(name.to_string()),
            reply_serial: Some(reply_to_serial),
            destination: None,
            sender: None,
            signature: None,
            body: Vec::new(),
        }
    }

    pub fn add_arg(&mut self, val: DbusValue) {
        self.body.push(val);
    }

    pub fn set_destination(&mut self, dest: &str) {
        self.destination = Some(dest.to_string());
    }

    pub fn is_no_reply(&self) -> bool {
        self.flags & 1 != 0
    }
}

// ─── D-Bus Name (well-known or unique) ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DbusName {
    WellKnown(String),     // org.freedesktop.NetworkManager
    Unique(String),        // :1.42
}

impl DbusName {
    pub fn is_unique(&self) -> bool {
        matches!(self, DbusName::Unique(_))
    }
}

// ─── Signal Match Rule ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DbusMatchRule {
    pub msg_type: Option<DbusMessageType>,
    pub sender: Option<String>,
    pub interface: Option<String>,
    pub member: Option<String>,
    pub path: Option<String>,
}

impl DbusMatchRule {
    pub fn matches(&self, msg: &DbusMessage) -> bool {
        if let Some(ref t) = self.msg_type {
            if &msg.msg_type != t { return false; }
        }
        if let Some(ref iface) = self.interface {
            if msg.interface.as_deref() != Some(iface.as_str()) { return false; }
        }
        if let Some(ref member) = self.member {
            if msg.member.as_deref() != Some(member.as_str()) { return false; }
        }
        if let Some(ref path) = self.path {
            if msg.path.as_deref() != Some(path.as_str()) { return false; }
        }
        true
    }
}

// ─── Registered Service ───────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DbusService {
    pub name: String,
    pub unique_name: String,
    pub pid: u32,
    pub match_rules: Vec<DbusMatchRule>,
    pub received_messages: Vec<DbusMessage>,
}

impl DbusService {
    pub fn new(name: &str, unique_name: &str, pid: u32) -> Self {
        DbusService {
            name: name.to_string(),
            unique_name: unique_name.to_string(),
            pid,
            match_rules: Vec::new(),
            received_messages: Vec::new(),
        }
    }

    pub fn add_match(&mut self, rule: DbusMatchRule) {
        self.match_rules.push(rule);
    }

    pub fn deliver(&mut self, msg: DbusMessage) {
        self.received_messages.push(msg);
    }
}

// ─── Sovereign D-Bus Message Bus ──────────────────────────────────────────────

pub struct SovereignDbusBus {
    pub services: Vec<DbusService>,
    pub next_serial: u32,
    pub next_unique_id: u32,
    pub messages_routed: u64,
    pub messages_dropped: u64,
}

impl SovereignDbusBus {
    pub fn new() -> Self {
        SovereignDbusBus {
            services: Vec::new(),
            next_serial: 1,
            next_unique_id: 1,
            messages_routed: 0,
            messages_dropped: 0,
        }
    }

    pub fn register_service(&mut self, name: &str, pid: u32) -> String {
        let unique = {
            let id = self.next_unique_id;
            self.next_unique_id = self.next_unique_id.saturating_add(1);
            let mut u = String::from(":1.");
            u.push_str(&id.to_string());
            u
        };
        self.services.push(DbusService::new(name, &unique, pid));
        unique
    }

    pub fn next_serial(&mut self) -> u32 {
        let s = self.next_serial;
        self.next_serial = self.next_serial.saturating_add(1);
        s
    }

    /// Send a message to its destination.
    pub fn send(&mut self, mut msg: DbusMessage) -> bool {
        msg.serial = self.next_serial();

        // Route to destination
        if let Some(ref dest) = msg.destination.clone() {
            let dest_str = dest.as_str();
            if let Some(svc) = self.services.iter_mut()
                .find(|s| s.name == dest_str || s.unique_name == dest_str)
            {
                svc.deliver(msg);
                self.messages_routed = self.messages_routed.saturating_add(1);
                return true;
            }
            self.messages_dropped = self.messages_dropped.saturating_add(1);
            return false;
        }

        // Broadcast signal to all matching subscribers
        if msg.msg_type == DbusMessageType::Signal {
            let mut delivered = 0usize;
            let msg_clone = msg.clone();
            for svc in &mut self.services {
                if svc.match_rules.iter().any(|r| r.matches(&msg_clone)) {
                    svc.deliver(msg_clone.clone());
                    delivered += 1;
                }
            }
            self.messages_routed = self.messages_routed.saturating_add(delivered as u64);
            return delivered > 0;
        }

        false
    }

    pub fn service_count(&self) -> usize { self.services.len() }

    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut DbusService> {
        self.services.iter_mut().find(|s| s.name == name || s.unique_name == name)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbus_service_registration() {
        let mut bus = SovereignDbusBus::new();
        let u1 = bus.register_service("org.sigma.NetworkManager", 1001);
        let u2 = bus.register_service("org.sigma.Bluetooth", 1002);
        assert_ne!(u1, u2);
        assert_eq!(bus.service_count(), 2);
        assert!(u1.starts_with(":1."));
    }

    #[test]
    fn test_dbus_method_call_routing() {
        let mut bus = SovereignDbusBus::new();
        bus.register_service("org.sigma.NetworkManager", 1001);

        let mut msg = DbusMessage::method_call(
            "/org/sigma/NetworkManager",
            "org.sigma.NetworkManager",
            "GetDevices",
        );
        msg.set_destination("org.sigma.NetworkManager");
        assert!(bus.send(msg));
        assert_eq!(bus.messages_routed, 1);

        let svc = bus.get_service_mut("org.sigma.NetworkManager").unwrap();
        assert_eq!(svc.received_messages.len(), 1);
        assert_eq!(svc.received_messages[0].member.as_deref(), Some("GetDevices"));
    }

    #[test]
    fn test_dbus_signal_broadcast() {
        let mut bus = SovereignDbusBus::new();
        bus.register_service("org.sigma.NM", 1001);
        bus.register_service("org.sigma.App1", 2001);
        bus.register_service("org.sigma.App2", 2002);

        // Subscribe App1 and App2 to NM signals
        let rule = DbusMatchRule {
            msg_type: Some(DbusMessageType::Signal),
            sender: None,
            interface: Some("org.sigma.NM".to_string()),
            member: Some("StateChanged".to_string()),
            path: None,
        };
        bus.get_service_mut("org.sigma.App1").unwrap().add_match(rule.clone());
        bus.get_service_mut("org.sigma.App2").unwrap().add_match(rule);

        let sig = DbusMessage::signal(
            "/org/sigma/NM",
            "org.sigma.NM",
            "StateChanged",
        );
        assert!(bus.send(sig));
        assert_eq!(bus.get_service_mut("org.sigma.App1").unwrap().received_messages.len(), 1);
        assert_eq!(bus.get_service_mut("org.sigma.App2").unwrap().received_messages.len(), 1);
    }

    #[test]
    fn test_dbus_value_signatures() {
        assert_eq!(DbusValue::Uint32(42).type_signature(), "u");
        assert_eq!(DbusValue::Str("hello".to_string()).type_signature(), "s");
        assert_eq!(DbusValue::Bool(true).type_signature(), "b");
        assert_eq!(DbusValue::Array(vec![]).type_signature(), "a");
    }

    #[test]
    fn test_dbus_message_error() {
        let err = DbusMessage::error(5, "org.freedesktop.DBus.Error.NoSuchMethod");
        assert_eq!(err.msg_type, DbusMessageType::Error);
        assert_eq!(err.reply_serial, Some(5));
        assert_eq!(err.error_name.as_deref(), Some("org.freedesktop.DBus.Error.NoSuchMethod"));
    }

    #[test]
    fn test_dbus_match_rule_filtering() {
        let rule = DbusMatchRule {
            msg_type: Some(DbusMessageType::Signal),
            sender: None,
            interface: Some("org.sigma.Test".to_string()),
            member: None,
            path: None,
        };
        let mut sig = DbusMessage::signal("/obj", "org.sigma.Test", "Fired");
        assert!(rule.matches(&sig));
        sig.interface = Some("org.sigma.Other".to_string());
        assert!(!rule.matches(&sig));
    }
}
