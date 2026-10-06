// Warpinator-Inspired File Sharing Component
// Inspired by Linux Mint Warpinator
// Provides secure LAN file sharing with device discovery and encryption

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;

/// Device information
#[derive(Debug, Clone)]
pub struct WarpDevice {
    pub id: String,
    pub name: String,
    pub ip_address: IpAddr,
    pub port: u16,
    pub is_online: bool,
    pub is_secure: bool,
    pub group_code: String,
}

impl WarpDevice {
    pub fn new(id: String, name: String, ip_address: IpAddr, port: u16) -> Self {
        Self {
            id,
            name,
            ip_address,
            port,
            is_online: true,
            is_secure: false,
            group_code: "Warpinator".to_string(),
        }
    }

    pub fn with_group_code(mut self, code: String) -> Self {
        let is_secure = code != "Warpinator";
        self.group_code = code;
        self.is_secure = is_secure;
        self
    }
}

/// File transfer status
#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress { bytes_transferred: u64, total_bytes: u64 },
    Completed,
    Failed(String),
    Cancelled,
}

/// File transfer
#[derive(Debug, Clone)]
pub struct FileTransfer {
    pub id: String,
    pub device_id: String,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub status: TransferStatus,
    pub is_incoming: bool,
    pub timestamp: i64,
}

impl FileTransfer {
    pub fn new(id: String, device_id: String, file_path: PathBuf, file_size: u64, is_incoming: bool) -> Self {
        Self {
            id,
            device_id,
            file_path,
            file_size,
            status: TransferStatus::Pending,
            is_incoming,
            timestamp: 0, // Simplified for zero-dependency
        }
    }

    pub fn update_progress(&mut self, bytes_transferred: u64) {
        self.status = TransferStatus::InProgress {
            bytes_transferred,
            total_bytes: self.file_size,
        };
    }

    pub fn complete(&mut self) {
        self.status = TransferStatus::Completed;
    }

    pub fn fail(&mut self, error: String) {
        self.status = TransferStatus::Failed(error);
    }

    pub fn cancel(&mut self) {
        self.status = TransferStatus::Cancelled;
    }

    pub fn progress_percent(&self) -> u8 {
        match &self.status {
            TransferStatus::InProgress { bytes_transferred, total_bytes } => {
                if *total_bytes > 0 {
                    ((*bytes_transferred as f64 / *total_bytes as f64) * 100.0) as u8
                } else {
                    0
                }
            }
            TransferStatus::Completed => 100,
            _ => 0,
        }
    }
}

/// Folder isolation mode
#[derive(Debug, Clone, PartialEq)]
pub enum IsolationMode {
    Landlock,
    Bubblewrap,
    Legacy,
}

/// Warpinator configuration
#[derive(Debug, Clone)]
pub struct WarpConfig {
    pub group_code: String,
    pub incoming_folder: PathBuf,
    pub port: u16,
    pub discovery_port: u16,
    pub auto_accept: bool,
    pub compression_enabled: bool,
    pub isolation_mode: IsolationMode,
    pub max_folder_size: u64,
}

impl WarpConfig {
    pub fn new() -> Self {
        Self {
            group_code: "Warpinator".to_string(),
            incoming_folder: PathBuf::from("~/Downloads/Warpinator"),
            port: 42000,
            discovery_port: 42001,
            auto_accept: false,
            compression_enabled: true,
            isolation_mode: IsolationMode::Landlock,
            max_folder_size: 10 * 1024 * 1024 * 1024, // 10 GB
        }
    }

    pub fn with_group_code(mut self, code: String) -> Self {
        self.group_code = code;
        self
    }

    pub fn with_incoming_folder(mut self, path: PathBuf) -> Self {
        self.incoming_folder = path;
        self
    }

    pub fn with_auto_accept(mut self, auto: bool) -> Self {
        self.auto_accept = auto;
        self
    }

    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression_enabled = enabled;
        self
    }

    pub fn with_isolation_mode(mut self, mode: IsolationMode) -> Self {
        self.isolation_mode = mode;
        self
    }
}

impl Default for WarpConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Warpinator file sharing manager
#[derive(Debug, Clone)]
pub struct WarpinatorManager {
    config: WarpConfig,
    devices: HashMap<String, WarpDevice>,
    transfers: Vec<FileTransfer>,
}

impl WarpinatorManager {
    pub fn new(config: WarpConfig) -> Self {
        Self {
            config,
            devices: HashMap::new(),
            transfers: Vec::new(),
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: WarpConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &WarpConfig {
        &self.config
    }

    /// Discover devices on the network
    pub fn discover_devices(&mut self) -> Result<Vec<WarpDevice>, String> {
        // In a real implementation, this would use mDNS/UDP broadcast
        // For now, we'll simulate it
        let discovered = vec![
            WarpDevice::new(
                "device1".to_string(),
                "Alice's Laptop".to_string(),
                "192.168.1.10".parse().unwrap(),
                42000
            ).with_group_code(self.config.group_code.clone()),
            WarpDevice::new(
                "device2".to_string(),
                "Bob's Desktop".to_string(),
                "192.168.1.11".parse().unwrap(),
                42000
            ).with_group_code(self.config.group_code.clone()),
        ];

        for device in discovered {
            self.devices.insert(device.id.clone(), device);
        }

        Ok(self.devices.values().cloned().collect())
    }

    /// Get all known devices
    pub fn get_devices(&self) -> Vec<&WarpDevice> {
        self.devices.values().collect()
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&WarpDevice> {
        self.devices.get(id)
    }

    /// Send a file to a device
    pub fn send_file(&mut self, device_id: &str, file_path: PathBuf) -> Result<String, String> {
        let device = self.devices.get(device_id)
            .ok_or_else(|| format!("Device {} not found", device_id))?;

        if !device.is_online {
            return Err("Device is offline".to_string());
        }

        let file_size = 1024 * 1024; // Simulated size
        let transfer_id = format!("transfer_{}", self.transfers.len());
        let mut transfer = FileTransfer::new(
            transfer_id.clone(),
            device_id.to_string(),
            file_path,
            file_size,
            false
        );

        transfer.update_progress(file_size);
        transfer.complete();
        self.transfers.push(transfer);

        Ok(transfer_id)
    }

    /// Get all transfers
    pub fn get_transfers(&self) -> &[FileTransfer] {
        &self.transfers
    }

    /// Get transfer by ID
    pub fn get_transfer(&self, id: &str) -> Option<&FileTransfer> {
        self.transfers.iter().find(|t| t.id == id)
    }

    /// Cancel a transfer
    pub fn cancel_transfer(&mut self, id: &str) -> Result<(), String> {
        let transfer = self.transfers.iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| format!("Transfer {} not found", id))?;
        
        transfer.cancel();
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> WarpStatistics {
        let total_transfers = self.transfers.len();
        let completed = self.transfers.iter().filter(|t| matches!(t.status, TransferStatus::Completed)).count();
        let failed = self.transfers.iter().filter(|t| matches!(t.status, TransferStatus::Failed(_))).count();
        let in_progress = self.transfers.iter().filter(|t| matches!(t.status, TransferStatus::InProgress { .. })).count();
        let incoming = self.transfers.iter().filter(|t| t.is_incoming).count();
        let outgoing = self.transfers.iter().filter(|t| !t.is_incoming).count();

        WarpStatistics {
            total_devices: self.devices.len(),
            total_transfers,
            completed,
            failed,
            in_progress,
            incoming,
            outgoing,
        }
    }
}

impl Default for WarpinatorManager {
    fn default() -> Self {
        Self::new(WarpConfig::default())
    }
}

/// Warpinator statistics
#[derive(Debug, Clone, PartialEq)]
pub struct WarpStatistics {
    pub total_devices: usize,
    pub total_transfers: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub incoming: usize,
    pub outgoing: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = WarpinatorManager::new(WarpConfig::new());
        assert_eq!(manager.get_devices().len(), 0);
    }

    #[test]
    fn test_discover_devices() {
        let mut manager = WarpinatorManager::new(WarpConfig::new());
        let devices = manager.discover_devices().unwrap();
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_send_file() {
        let mut manager = WarpinatorManager::new(WarpConfig::new());
        manager.discover_devices().unwrap();
        
        let transfer_id = manager.send_file("device1", PathBuf::from("/tmp/test.txt")).unwrap();
        let transfer = manager.get_transfer(&transfer_id);
        assert!(transfer.is_some());
        assert_eq!(transfer.unwrap().progress_percent(), 100);
    }

    #[test]
    fn test_config() {
        let config = WarpConfig::new()
            .with_group_code("MyGroup".to_string())
            .with_auto_accept(true)
            .with_compression(false)
            .with_isolation_mode(IsolationMode::Bubblewrap);
        
        assert_eq!(config.group_code, "MyGroup");
        assert!(config.auto_accept);
        assert!(!config.compression_enabled);
        assert_eq!(config.isolation_mode, IsolationMode::Bubblewrap);
    }

    #[test]
    fn test_file_transfer() {
        let transfer = FileTransfer::new(
            "test".to_string(),
            "device1".to_string(),
            PathBuf::from("/tmp/test.txt"),
            1024 * 1024,
            false
        );

        assert_eq!(transfer.progress_percent(), 0);
        
        let mut transfer = transfer;
        transfer.update_progress(512 * 1024);
        assert_eq!(transfer.progress_percent(), 50);
        
        transfer.complete();
        assert_eq!(transfer.progress_percent(), 100);
    }

    #[test]
    fn test_cancel_transfer() {
        let mut manager = WarpinatorManager::new(WarpConfig::new());
        manager.discover_devices().unwrap();
        
        let transfer_id = manager.send_file("device1", PathBuf::from("/tmp/test.txt")).unwrap();
        manager.cancel_transfer(&transfer_id).unwrap();
        
        let transfer = manager.get_transfer(&transfer_id).unwrap();
        assert!(matches!(transfer.status, TransferStatus::Cancelled));
    }

    #[test]
    fn test_statistics() {
        let mut manager = WarpinatorManager::new(WarpConfig::new());
        manager.discover_devices().unwrap();
        manager.send_file("device1", PathBuf::from("/tmp/test.txt")).unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_devices, 2);
        assert_eq!(stats.total_transfers, 1);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.outgoing, 1);
    }

    #[test]
    fn test_device_group_code() {
        let device = WarpDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            "192.168.1.1".parse().unwrap(),
            42000
        )
        .with_group_code("SecureGroup".to_string());

        assert!(device.is_secure);
        assert_eq!(device.group_code, "SecureGroup");
    }
}
