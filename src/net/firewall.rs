#[cfg(not(target_os = "none"))]
#[cfg(not(target_os = "none"))]
use std_std::boxed::Box;


/// OOP-based Firewall & AI Intrusion Detection System (IDS) for SigmaOS
/// Implements standard packet filtering, Snort-style signature checking,
/// and CrowdStrike Falcon-inspired AI anomaly rate monitoring.


use std::vec::Vec;
use std::boxed::Box;
use std::string::String;
use std::string::ToString;
use core::sync::atomic::{AtomicU32, Ordering};

pub type RuleID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction { Accept = 0, Drop = 1, Reject = 2, Log = 3 }

/// Action taken on matching packet
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol { TCP = 6, UDP = 17, ICMP = 1, Any = 255 }

/// Supported packet protocols
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
