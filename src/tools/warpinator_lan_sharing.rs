extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Warpinator-inspired LAN file sharing GUI
/// Provides text-based interface for secure LAN file sharing
/// with device discovery, group codes, and encryption

#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub ip_address: String,
    pub is_online: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone)]
pub struct TransferItem {
    pub transfer_id: String,
    pub from_device: String,
    pub to_device: String,
    pub file_path: String,
    pub file_size: u64,
    pub bytes_transferred: u64,
    pub status: TransferStatus,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct GroupCode {
    pub code: String,
    pub is_valid: bool,
    pub creation_time: u64,
}

#[derive(Debug, Clone)]
pub struct TransferResult {
    pub success: bool,
    pub transfer_id: String,
    pub bytes_transferred: u64,
    pub duration_seconds: u64,
    pub message: String,
}

/// Warpinator-inspired LAN file sharing manager
pub struct WarpinatorLanSharing {
    pub group_code: GroupCode,
    pub devices: Vec<DeviceInfo>,
    pub transfers: Vec<TransferItem>,
    pub incoming_directory: String,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

impl WarpinatorLanSharing {
    pub fn new() -> Self {
        Self {
            group_code: GroupCode {
                code: String::from("default"),
                is_valid: true,
                creation_time: 0,
            },
            devices: Vec::new(),
            transfers: Vec::new(),
            incoming_directory: String::from("/home/user/Downloads/Warpinator"),
            compression_enabled: false,
            encryption_enabled: true,
        }
    }

    /// Generate a new group code
    pub fn generate_group_code(&mut self) -> String {
        let code = format!("group_{}", self.get_current_timestamp());
        self.group_code = GroupCode {
            code: code.clone(),
            is_valid: true,
            creation_time: self.get_current_timestamp(),
        };
        code
    }

    /// Set group code
    pub fn set_group_code(&mut self, code: &str) {
        self.group_code = GroupCode {
            code: code.to_string(),
            is_valid: true,
            creation_time: self.get_current_timestamp(),
        };
    }

    /// Add a device
    pub fn add_device(&mut self, device_id: &str, device_name: &str, ip_address: &str) {
        let device = DeviceInfo {
            device_id: device_id.to_string(),
            device_name: device_name.to_string(),
            ip_address: ip_address.to_string(),
            is_online: true,
            last_seen: self.get_current_timestamp(),
        };
        
        if !self.devices.iter().any(|d| d.device_id == device_id) {
            self.devices.push(device);
        }
    }

    /// Remove a device
    pub fn remove_device(&mut self, device_id: &str) -> Result<(), String> {
        match self.find_device_index(device_id) {
            Some(index) => {
                self.devices.remove(index);
                Ok(())
            }
            None => Err(format!("Device {} not found", device_id)),
        }
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&DeviceInfo> {
        self.devices.iter().collect()
    }

    /// Get device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&DeviceInfo> {
        self.find_device(device_id)
    }

    /// Start a file transfer
    pub fn start_transfer(&mut self, from_device: &str, to_device: &str, file_path: &str, file_size: u64) -> TransferResult {
        let transfer_id = format!("transfer_{}", self.transfers.len());
        
        let transfer = TransferItem {
            transfer_id: transfer_id.clone(),
            from_device: from_device.to_string(),
            to_device: to_device.to_string(),
            file_path: file_path.to_string(),
            file_size,
            bytes_transferred: 0,
            status: TransferStatus::Pending,
            timestamp: self.get_current_timestamp(),
        };
        
        self.transfers.push(transfer);
        
        TransferResult {
            success: true,
            transfer_id,
            bytes_transferred: 0,
            duration_seconds: 0,
            message: format!("Transfer started from {} to {}", from_device, to_device),
        }
    }

    /// Update transfer progress
    pub fn update_transfer_progress(&mut self, transfer_id: &str, bytes_transferred: u64) {
        if let Some(transfer) = self.transfers.iter_mut().find(|t| t.transfer_id == transfer_id) {
            transfer.bytes_transferred = bytes_transferred;
            if bytes_transferred >= transfer.file_size {
                transfer.status = TransferStatus::Completed;
            } else {
                transfer.status = TransferStatus::InProgress;
            }
        }
    }

    /// Cancel a transfer
    pub fn cancel_transfer(&mut self, transfer_id: &str) -> Result<(), String> {
        match self.find_transfer_index(transfer_id) {
            Some(index) => {
                self.transfers[index].status = TransferStatus::Cancelled;
                Ok(())
            }
            None => Err(format!("Transfer {} not found", transfer_id)),
        }
    }

    /// List all transfers
    pub fn list_transfers(&self) -> Vec<&TransferItem> {
        self.transfers.iter().collect()
    }

    /// Get transfer by ID
    pub fn get_transfer(&self, transfer_id: &str) -> Option<&TransferItem> {
        self.find_transfer(transfer_id)
    }

    /// Set incoming directory
    pub fn set_incoming_directory(&mut self, directory: &str) {
        self.incoming_directory = directory.to_string();
    }

    /// Enable compression
    pub fn enable_compression(&mut self) {
        self.compression_enabled = true;
    }

    /// Disable compression
    pub fn disable_compression(&mut self) {
        self.compression_enabled = false;
    }

    /// Enable encryption
    pub fn enable_encryption(&mut self) {
        self.encryption_enabled = true;
    }

    /// Disable encryption
    pub fn disable_encryption(&mut self) {
        self.encryption_enabled = false;
    }

    /// Get transfer statistics
    pub fn get_transfer_statistics(&self) -> TransferStatistics {
        let total_transfers = self.transfers.len();
        let completed_transfers = self.transfers.iter().filter(|t| t.status == TransferStatus::Completed).count();
        let in_progress_transfers = self.transfers.iter().filter(|t| t.status == TransferStatus::InProgress).count();
        let failed_transfers = self.transfers.iter().filter(|t| t.status == TransferStatus::Failed).count();
        let total_bytes_transferred: u64 = self.transfers.iter().map(|t| t.bytes_transferred).sum();
        
        TransferStatistics {
            total_transfers,
            completed_transfers,
            in_progress_transfers,
            failed_transfers,
            total_bytes_transferred,
        }
    }

    /// Display device list in text-based GUI format
    pub fn display_device_list(&self) -> String {
        let mut output = String::from("=== Warpinator Devices ===\n\n");
        
        if self.devices.is_empty() {
            output.push_str("No devices available.\n");
            return output;
        }
        
        for (i, device) in self.devices.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, device.device_name));
            output.push_str(&format!("   ID: {}\n", device.device_id));
            output.push_str(&format!("   IP: {}\n", device.ip_address));
            output.push_str(&format!("   Status: {}\n", if device.is_online { "Online" } else { "Offline" }));
            output.push_str(&format!("   Last Seen: {}\n", self.format_timestamp(device.last_seen)));
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} devices\n", self.devices.len()));
        output
    }

    /// Display transfer list in text-based GUI format
    pub fn display_transfer_list(&self) -> String {
        let mut output = String::from("=== Warpinator Transfers ===\n\n");
        
        if self.transfers.is_empty() {
            output.push_str("No transfers available.\n");
            return output;
        }
        
        for (i, transfer) in self.transfers.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, transfer.transfer_id));
            output.push_str(&format!("   From: {}\n", transfer.from_device));
            output.push_str(&format!("   To: {}\n", transfer.to_device));
            output.push_str(&format!("   File: {}\n", transfer.file_path));
            output.push_str(&format!("   Size: {} bytes\n", transfer.file_size));
            output.push_str(&format!("   Transferred: {} bytes\n", transfer.bytes_transferred));
            output.push_str(&format!("   Status: {}\n", self.status_to_string(&transfer.status)));
            output.push_str(&format!("   Time: {}\n", self.format_timestamp(transfer.timestamp)));
            output.push_str("\n");
        }
        
        output.push_str(&format!("Total: {} transfers\n", self.transfers.len()));
        output
    }

    /// Display configuration in text-based GUI format
    pub fn display_config(&self) -> String {
        let mut output = String::from("=== Warpinator Configuration ===\n\n");
        output.push_str(&format!("Group Code: {}\n", self.group_code.code));
        output.push_str(&format!("Incoming Directory: {}\n", self.incoming_directory));
        output.push_str(&format!("Compression: {}\n", if self.compression_enabled { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("Encryption: {}\n", if self.encryption_enabled { "Enabled" } else { "Disabled" }));
        output
    }

    /// Display statistics in text-based GUI format
    pub fn display_statistics(&self) -> String {
        let stats = self.get_transfer_statistics();
        let mut output = String::from("=== Transfer Statistics ===\n\n");
        output.push_str(&format!("Total Transfers: {}\n", stats.total_transfers));
        output.push_str(&format!("Completed: {}\n", stats.completed_transfers));
        output.push_str(&format!("In Progress: {}\n", stats.in_progress_transfers));
        output.push_str(&format!("Failed: {}\n", stats.failed_transfers));
        output.push_str(&format!("Total Bytes Transferred: {}\n", stats.total_bytes_transferred));
        output
    }

    // Helper methods
    fn find_device(&self, device_id: &str) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.device_id == device_id)
    }

    fn find_device_index(&self, device_id: &str) -> Option<usize> {
        self.devices.iter().position(|d| d.device_id == device_id)
    }

    fn find_transfer(&self, transfer_id: &str) -> Option<&TransferItem> {
        self.transfers.iter().find(|t| t.transfer_id == transfer_id)
    }

    fn find_transfer_index(&self, transfer_id: &str) -> Option<usize> {
        self.transfers.iter().position(|t| t.transfer_id == transfer_id)
    }

    fn get_current_timestamp(&self) -> u64 {
        0
    }

    fn format_timestamp(&self, timestamp: u64) -> String {
        format!("Timestamp: {}", timestamp)
    }

    fn status_to_string(&self, status: &TransferStatus) -> String {
        match status {
            TransferStatus::Pending => String::from("Pending"),
            TransferStatus::InProgress => String::from("In Progress"),
            TransferStatus::Completed => String::from("Completed"),
            TransferStatus::Failed => String::from("Failed"),
            TransferStatus::Cancelled => String::from("Cancelled"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransferStatistics {
    pub total_transfers: usize,
    pub completed_transfers: usize,
    pub in_progress_transfers: usize,
    pub failed_transfers: usize,
    pub total_bytes_transferred: u64,
}

impl Default for WarpinatorLanSharing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_code_generation() {
        let mut warpinator = WarpinatorLanSharing::new();
        let code = warpinator.generate_group_code();
        
        assert!(!code.is_empty());
        assert_eq!(warpinator.group_code.code, code);
        assert!(warpinator.group_code.is_valid);
    }

    #[test]
    fn test_device_management() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        warpinator.add_device("device1", "Laptop", "192.168.1.100");
        warpinator.add_device("device2", "Desktop", "192.168.1.101");
        
        assert_eq!(warpinator.devices.len(), 2);
        
        let result = warpinator.remove_device("device1");
        assert!(result.is_ok());
        assert_eq!(warpinator.devices.len(), 1);
    }

    #[test]
    fn test_transfer_creation() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        let result = warpinator.start_transfer("device1", "device2", "/home/user/file.txt", 1024);
        
        assert!(result.success);
        assert_eq!(warpinator.transfers.len(), 1);
        assert_eq!(warpinator.transfers[0].status, TransferStatus::Pending);
    }

    #[test]
    fn test_transfer_progress() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        let result = warpinator.start_transfer("device1", "device2", "/home/user/file.txt", 1024);
        let transfer_id = result.transfer_id;
        
        warpinator.update_transfer_progress(&transfer_id, 512);
        assert_eq!(warpinator.transfers[0].bytes_transferred, 512);
        assert_eq!(warpinator.transfers[0].status, TransferStatus::InProgress);
        
        warpinator.update_transfer_progress(&transfer_id, 1024);
        assert_eq!(warpinator.transfers[0].status, TransferStatus::Completed);
    }

    #[test]
    fn test_transfer_cancellation() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        let result = warpinator.start_transfer("device1", "device2", "/home/user/file.txt", 1024);
        let transfer_id = result.transfer_id;
        
        let cancel_result = warpinator.cancel_transfer(&transfer_id);
        assert!(cancel_result.is_ok());
        assert_eq!(warpinator.transfers[0].status, TransferStatus::Cancelled);
    }

    #[test]
    fn test_compression_toggle() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        assert!(!warpinator.compression_enabled);
        warpinator.enable_compression();
        assert!(warpinator.compression_enabled);
        warpinator.disable_compression();
        assert!(!warpinator.compression_enabled);
    }

    #[test]
    fn test_encryption_toggle() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        assert!(warpinator.encryption_enabled);
        warpinator.disable_encryption();
        assert!(!warpinator.encryption_enabled);
        warpinator.enable_encryption();
        assert!(warpinator.encryption_enabled);
    }

    #[test]
    fn test_statistics() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        warpinator.start_transfer("device1", "device2", "/home/user/file1.txt", 1024);
        warpinator.start_transfer("device1", "device2", "/home/user/file2.txt", 2048);
        
        let stats = warpinator.get_transfer_statistics();
        assert_eq!(stats.total_transfers, 2);
        assert_eq!(stats.completed_transfers, 0);
        assert_eq!(stats.in_progress_transfers, 0);
    }

    #[test]
    fn test_display_output() {
        let mut warpinator = WarpinatorLanSharing::new();
        
        warpinator.add_device("device1", "Laptop", "192.168.1.100");
        warpinator.start_transfer("device1", "device2", "/home/user/file.txt", 1024);
        
        let device_output = warpinator.display_device_list();
        assert!(device_output.contains("Laptop"));
        assert!(device_output.contains("192.168.1.100"));
        
        let transfer_output = warpinator.display_transfer_list();
        assert!(transfer_output.contains("/home/user/file.txt"));
        
        let config_output = warpinator.display_config();
        assert!(config_output.contains("Group Code"));
        assert!(config_output.contains("Encryption"));
        
        let stats_output = warpinator.display_statistics();
        assert!(stats_output.contains("Total Transfers: 1"));
    }
}
