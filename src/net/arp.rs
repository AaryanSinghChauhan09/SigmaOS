/// SigmaOS ARP Implementation (Phase 2 Networking)
/// Inspired by Linux's net/ipv4/arp.c

use std::collections::HashMap;

pub const ARP_REQUEST: u16 = 1;
pub const ARP_REPLY: u16 = 2;

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: [u8; 4],
    pub mac: [u8; 6],
    pub timestamp: u64,
}

pub struct ArpTable {
    entries: HashMap<[u8; 4], ArpEntry>,
}

impl ArpTable {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn insert(&mut self, ip: [u8; 4], mac: [u8; 6], timestamp: u64) {
        self.entries.insert(ip, ArpEntry { ip, mac, timestamp });
    }

    pub fn lookup(&self, ip: &[u8; 4]) -> Option<&ArpEntry> {
        self.entries.get(ip)
    }

    pub fn remove_stale(&mut self, current_time: u64, max_age: u64) {
        self.entries.retain(|_, e| current_time - e.timestamp < max_age);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arp_table() {
        let mut table = ArpTable::new();
        table.insert([192, 168, 1, 1], [0xAA; 6], 1000);
        assert!(table.lookup(&[192, 168, 1, 1]).is_some());
        assert!(table.lookup(&[192, 168, 1, 2]).is_none());
    }

    #[test]
    fn test_arp_stale_removal() {
        let mut table = ArpTable::new();
        table.insert([10, 0, 0, 1], [0xBB; 6], 100);
        table.remove_stale(500, 300);
        assert!(table.lookup(&[10, 0, 0, 1]).is_none());
    }
}
