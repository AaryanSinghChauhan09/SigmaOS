// Network Bonding for SigmaOS
// Network bonding per Wiki 06-Networking.md
// Provides network interface bonding for redundancy and load balancing

use std::string::{String, ToString};
use std::vec::Vec;

/// Bonding mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondingMode {
    BalanceRR,      // Round-robin
    ActiveBackup,
    BalanceXOR,
    Broadcast,
    Ieee8023ad,     // IEEE 802.3ad LACP
    BalanceTLB,     // Adaptive transmit load balancing
    BalanceALB,     // Adaptive load balancing
}

impl BondingMode {
    pub fn as_str(&self) -> &str {
        match self {
            BondingMode::BalanceRR => "balance-rr",
            BondingMode::ActiveBackup => "active-backup",
            BondingMode::BalanceXOR => "balance-xor",
            BondingMode::Broadcast => "broadcast",
            BondingMode::Ieee8023ad => "802.3ad",
            BondingMode::BalanceTLB => "balance-tlb",
            BondingMode::BalanceALB => "balance-alb",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "balance-rr" => Some(BondingMode::BalanceRR),
            "active-backup" => Some(BondingMode::ActiveBackup),
            "balance-xor" => Some(BondingMode::BalanceXOR),
            "broadcast" => Some(BondingMode::Broadcast),
            "802.3ad" => Some(BondingMode::Ieee8023ad),
            "balance-tlb" => Some(BondingMode::BalanceTLB),
            "balance-alb" => Some(BondingMode::BalanceALB),
            _ => None,
        }
    }
}

/// Bonding interface status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondStatus {
    Active,
    Inactive,
    Failed,
}

impl BondStatus {
    pub fn as_str(&self) -> &str {
        match self {
            BondStatus::Active => "active",
            BondStatus::Inactive => "inactive",
            BondStatus::Failed => "failed",
        }
    }
}

/// Slave interface
#[derive(Debug, Clone)]
pub struct SlaveInterface {
    pub name: String,
    pub status: BondStatus,
    pub link_speed_mbps: u32,
    pub is_primary: bool,
}

impl SlaveInterface {
    pub fn new(name: String) -> Self {
        SlaveInterface {
            name,
            status: BondStatus::Inactive,
            link_speed_mbps: 0,
            is_primary: false,
        }
    }

    pub fn set_status(&mut self, status: BondStatus) {
        self.status = status;
    }

    pub fn set_link_speed(&mut self, speed_mbps: u32) {
        self.link_speed_mbps = speed_mbps;
    }

    pub fn set_primary(&mut self, is_primary: bool) {
        self.is_primary = is_primary;
    }
}

/// Bond interface
#[derive(Debug, Clone)]
pub struct BondInterface {
    pub name: String,
    pub mode: BondingMode,
    pub slaves: Vec<SlaveInterface>,
    pub status: BondStatus,
    pub mtu: u32,
    pub active_slave: Option<String>,
}

impl BondInterface {
    pub fn new(name: String, mode: BondingMode) -> Self {
        BondInterface {
            name,
            mode,
            slaves: Vec::new(),
            status: BondStatus::Inactive,
            mtu: 1500,
            active_slave: None,
        }
    }

    pub fn add_slave(&mut self, slave: SlaveInterface) {
        self.slaves.push(slave);
    }

    pub fn remove_slave(&mut self, name: &str) -> bool {
        if let Some(pos) = self.slaves.iter().position(|s| s.name == name) {
            self.slaves.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_slave(&self, name: &str) -> Option<&SlaveInterface> {
        self.slaves.iter().find(|s| s.name == name)
    }

    pub fn get_active_slave(&self) -> Option<&SlaveInterface> {
        if let Some(ref active_name) = self.active_slave {
            self.slaves.iter().find(|s| s.name == *active_name)
        } else {
            None
        }
    }

    pub fn set_active_slave(&mut self, name: String) -> bool {
        if self.slaves.iter().any(|s| s.name == name) {
            self.active_slave = Some(name);
            true
        } else {
            false
        }
    }

    pub fn set_status(&mut self, status: BondStatus) {
        self.status = status;
    }

    pub fn set_mtu(&mut self, mtu: u32) {
        self.mtu = mtu;
    }

    pub fn get_total_bandwidth(&self) -> u32 {
        self.slaves.iter()
            .filter(|s| s.status == BondStatus::Active)
            .map(|s| s.link_speed_mbps)
            .sum()
    }

    pub fn get_active_slave_count(&self) -> usize {
        self.slaves.iter().filter(|s| s.status == BondStatus::Active).count()
    }
}

/// Network bonding manager
#[derive(Debug, Clone)]
pub struct NetworkBondingManager {
    pub bonds: Vec<BondInterface>,
}

impl Default for NetworkBondingManager {
    fn default() -> Self {
        NetworkBondingManager {
            bonds: Vec::new(),
        }
    }
}

impl NetworkBondingManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_bond(&mut self, name: String, mode: BondingMode) -> Result<(), String> {
        if self.bonds.iter().any(|b| b.name == name) {
            return Err(format!("Bond {} already exists", name));
        }

        let bond = BondInterface::new(name, mode);
        self.bonds.push(bond);
        Ok(())
    }

    pub fn delete_bond(&mut self, name: &str) -> bool {
        if let Some(pos) = self.bonds.iter().position(|b| b.name == name) {
            self.bonds.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_bond(&self, name: &str) -> Option<&BondInterface> {
        self.bonds.iter().find(|b| b.name == name)
    }

    pub fn get_bond_mut(&mut self, name: &str) -> Option<&mut BondInterface> {
        self.bonds.iter_mut().find(|b| b.name == name)
    }

    pub fn add_slave_to_bond(&mut self, bond_name: &str, slave_name: String) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        if bond.slaves.iter().any(|s| s.name == slave_name) {
            return Err(format!("Slave {} already in bond {}", slave_name, bond_name));
        }

        let slave = SlaveInterface::new(slave_name);
        bond.add_slave(slave);
        Ok(())
    }

    pub fn remove_slave_from_bond(&mut self, bond_name: &str, slave_name: &str) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        if bond.remove_slave(slave_name) {
            Ok(())
        } else {
            Err(format!("Slave {} not found in bond {}", slave_name, bond_name))
        }
    }

    pub fn set_bond_mode(&mut self, bond_name: &str, mode: BondingMode) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        bond.mode = mode;
        Ok(())
    }

    pub fn set_bond_mtu(&mut self, bond_name: &str, mtu: u32) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        bond.set_mtu(mtu);
        Ok(())
    }

    pub fn activate_bond(&mut self, bond_name: &str) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        if bond.slaves.is_empty() {
            return Err(format!("Bond {} has no slaves", bond_name));
        }

        bond.set_status(BondStatus::Active);

        // Activate primary slave or first slave
        if let Some(primary) = bond.slaves.iter().find(|s| s.is_primary) {
            bond.set_active_slave(primary.name.clone());
        } else if let Some(first) = bond.slaves.first() {
            bond.set_active_slave(first.name.clone());
        }

        Ok(())
    }

    pub fn deactivate_bond(&mut self, bond_name: &str) -> Result<(), String> {
        let bond = self.get_bond_mut(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        bond.set_status(BondStatus::Inactive);
        bond.active_slave = None;
        Ok(())
    }

    pub fn get_bond_status(&self, bond_name: &str) -> Result<String, String> {
        let bond = self.get_bond(bond_name)
            .ok_or_else(|| format!("Bond {} not found", bond_name))?;

        let mut status = format!("Bond: {}\n", bond.name);
        status.push_str(&format!("Mode: {}\n", bond.mode.as_str()));
        status.push_str(&format!("Status: {}\n", bond.status.as_str()));
        status.push_str(&format!("MTU: {}\n", bond.mtu));
        status.push_str(&format!("Active Slave: {}\n", bond.active_slave.as_ref().unwrap_or(&String::from("None"))));
        status.push_str(&format!("Total Bandwidth: {} Mbps\n", bond.get_total_bandwidth()));
        status.push_str(&format!("Active Slaves: {}/{}\n", bond.get_active_slave_count(), bond.slaves.len()));
        status.push_str("\nSlaves:\n");

        for slave in &bond.slaves {
            status.push_str(&format!(
                "  - {}: {} ({} Mbps){}\n",
                slave.name,
                slave.status.as_str(),
                slave.link_speed_mbps,
                if slave.is_primary { " [Primary]" } else { "" }
            ));
        }

        Ok(status)
    }

    pub fn list_bonds(&self) -> Vec<String> {
        self.bonds.iter()
            .map(|b| format!("{} ({}, {})", b.name, b.mode.as_str(), b.status.as_str()))
            .collect()
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Network Bonding Statistics:\n");
        stats.push_str(&format!("Total bonds: {}\n", self.bonds.len()));

        let total_slaves: usize = self.bonds.iter().map(|b| b.slaves.len()).sum();
        let active_slaves: usize = self.bonds.iter().map(|b| b.get_active_slave_count()).sum();
        let total_bandwidth: u32 = self.bonds.iter().map(|b| b.get_total_bandwidth()).sum();

        stats.push_str(&format!("Total slaves: {}\n", total_slaves));
        stats.push_str(&format!("Active slaves: {}\n", active_slaves));
        stats.push_str(&format!("Total bandwidth: {} Mbps\n", total_bandwidth));

        let active_bonds = self.bonds.iter().filter(|b| b.status == BondStatus::Active).count();
        stats.push_str(&format!("Active bonds: {}\n", active_bonds));

        stats
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bonding_mode_as_str() {
        assert_eq!(BondingMode::BalanceRR.as_str(), "balance-rr");
        assert_eq!(BondingMode::ActiveBackup.as_str(), "active-backup");
    }

    #[test]
    fn test_bonding_mode_from_str() {
        assert_eq!(BondingMode::from_str("balance-rr"), Some(BondingMode::BalanceRR));
        assert_eq!(BondingMode::from_str("invalid"), None);
    }

    #[test]
    fn test_slave_interface_creation() {
        let slave = SlaveInterface::new(String::from("eth0"));
        assert_eq!(slave.name, "eth0");
        assert_eq!(slave.status, BondStatus::Inactive);
    }

    #[test]
    fn test_slave_interface_set_status() {
        let mut slave = SlaveInterface::new(String::from("eth0"));
        slave.set_status(BondStatus::Active);
        assert_eq!(slave.status, BondStatus::Active);
    }

    #[test]
    fn test_bond_interface_creation() {
        let bond = BondInterface::new(String::from("bond0"), BondingMode::BalanceRR);
        assert_eq!(bond.name, "bond0");
        assert_eq!(bond.mode, BondingMode::BalanceRR);
    }

    #[test]
    fn test_bond_interface_add_slave() {
        let mut bond = BondInterface::new(String::from("bond0"), BondingMode::BalanceRR);
        bond.add_slave(SlaveInterface::new(String::from("eth0")));
        assert_eq!(bond.slaves.len(), 1);
    }

    #[test]
    fn test_bond_interface_remove_slave() {
        let mut bond = BondInterface::new(String::from("bond0"), BondingMode::BalanceRR);
        bond.add_slave(SlaveInterface::new(String::from("eth0")));
        assert!(bond.remove_slave("eth0"));
        assert_eq!(bond.slaves.len(), 0);
    }

    #[test]
    fn test_bond_interface_get_total_bandwidth() {
        let mut bond = BondInterface::new(String::from("bond0"), BondingMode::BalanceRR);
        let mut slave1 = SlaveInterface::new(String::from("eth0"));
        slave1.set_link_speed(1000);
        slave1.set_status(BondStatus::Active);
        let mut slave2 = SlaveInterface::new(String::from("eth1"));
        slave2.set_link_speed(1000);
        slave2.set_status(BondStatus::Active);
        bond.add_slave(slave1);
        bond.add_slave(slave2);
        assert_eq!(bond.get_total_bandwidth(), 2000);
    }

    #[test]
    fn test_network_bonding_manager_creation() {
        let manager = NetworkBondingManager::new();
        assert_eq!(manager.bonds.len(), 0);
    }

    #[test]
    fn test_network_bonding_manager_create_bond() {
        let mut manager = NetworkBondingManager::new();
        assert!(manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).is_ok());
        assert_eq!(manager.bonds.len(), 1);
    }

    #[test]
    fn test_network_bonding_manager_create_duplicate_bond() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        assert!(manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).is_err());
    }

    #[test]
    fn test_network_bonding_manager_add_slave_to_bond() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        assert!(manager.add_slave_to_bond("bond0", String::from("eth0")).is_ok());
    }

    #[test]
    fn test_network_bonding_manager_activate_bond() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        manager.add_slave_to_bond("bond0", String::from("eth0")).unwrap();
        assert!(manager.activate_bond("bond0").is_ok());
    }

    #[test]
    fn test_network_bonding_manager_activate_bond_no_slaves() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        assert!(manager.activate_bond("bond0").is_err());
    }

    #[test]
    fn test_network_bonding_manager_get_bond_status() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        manager.add_slave_to_bond("bond0", String::from("eth0")).unwrap();
        let status = manager.get_bond_status("bond0");
        assert!(status.is_ok());
        assert!(status.unwrap().contains("bond0"));
    }

    #[test]
    fn test_network_bonding_manager_list_bonds() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        let bonds = manager.list_bonds();
        assert_eq!(bonds.len(), 1);
    }

    #[test]
    fn test_network_bonding_manager_get_statistics() {
        let mut manager = NetworkBondingManager::new();
        manager.create_bond(String::from("bond0"), BondingMode::BalanceRR).unwrap();
        let stats = manager.get_statistics();
        assert!(stats.contains("Total bonds: 1"));
    }
}
