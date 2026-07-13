// Sigma Storage Driver - Storage Controller Prototype
// Implements NVMe and AHCI storage controller support
// No external dependencies - implementing from first principles

use std::fmt;

/// Storage controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageController {
    NVMe,
    AHCI,
    SATA,
    Unknown,
}

impl StorageController {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageController::NVMe => "NVMe",
            StorageController::AHCI => "AHCI",
            StorageController::SATA => "SATA",
            StorageController::Unknown => "Unknown",
        }
    }
}

/// Block device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDeviceType {
    SSD,
    HDD,
    NVMe,
    Unknown,
}

impl BlockDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockDeviceType::SSD => "SSD",
            BlockDeviceType::HDD => "HDD",
            BlockDeviceType::NVMe => "NVMe",
            BlockDeviceType::Unknown => "Unknown",
        }
    }
}

/// Block size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockSize {
    Bytes512,
    Bytes4096,
}

impl BlockSize {
    pub fn as_u64(&self) -> u64 {
        match self {
            BlockSize::Bytes512 => 512,
            BlockSize::Bytes4096 => 4096,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockSize::Bytes512 => "512B",
            BlockSize::Bytes4096 => "4KB",
        }
    }
}

/// Block device
#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub device_id: [u8; 32],
    pub name: String,
    pub device_type: BlockDeviceType,
    pub controller: StorageController,
    pub size: u64,
    pub block_size: BlockSize,
    pub block_count: u64,
    pub model: String,
    pub serial: String,
    pub initialized: bool,
}

impl BlockDevice {
    pub fn new(
        name: String,
        device_type: BlockDeviceType,
        controller: StorageController,
        size: u64,
        block_size: BlockSize,
        model: String,
        serial: String,
    ) -> Self {
        let device_id = Self::generate_device_id(&name, &serial);
        let block_count = size / block_size.as_u64();
        
        BlockDevice {
            device_id,
            name,
            device_type,
            controller,
            size,
            block_size,
            block_count,
            model,
            serial,
            initialized: false,
        }
    }
    
    fn generate_device_id(name: &str, serial: &str) -> [u8; 32] {
        // Placeholder for actual hardware ID
        let mut hash = [0u8; 32];
        let name_bytes = name.as_bytes();
        for (i, &byte) in name_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let serial_bytes = serial.as_bytes();
        for (i, &byte) in serial_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    pub fn get_device_id(&self) -> String {
        self.device_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("Device already initialized".to_string());
        }
        
        self.initialized = true;
        Ok(())
    }
    
    pub fn read_block(&self, block_number: u64) -> Result<Vec<u8>, String> {
        if !self.initialized {
            return Err("Device not initialized".to_string());
        }
        
        if block_number >= self.block_count {
            return Err("Block number out of range".to_string());
        }
        
        // Simulate read operation
        let block_size = self.block_size.as_u64() as usize;
        let mut buffer = vec![0u8; block_size];
        
        // Fill with simulated data
        for i in 0..block_size {
            buffer[i] = (block_number as u8).wrapping_add(i as u8);
        }
        
        Ok(buffer)
    }
    
    pub fn write_block(&self, block_number: u64, data: &[u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("Device not initialized".to_string());
        }
        
        if block_number >= self.block_count {
            return Err("Block number out of range".to_string());
        }
        
        if data.len() != self.block_size.as_u64() as usize {
            return Err("Data size mismatch".to_string());
        }
        
        // Simulate write operation
        Ok(())
    }
    
    pub fn get_info(&self) -> BlockDeviceInfo {
        BlockDeviceInfo {
            device_id: self.get_device_id(),
            name: self.name.clone(),
            device_type: self.device_type,
            controller: self.controller,
            size: self.size,
            block_size: self.block_size,
            block_count: self.block_count,
            model: self.model.clone(),
            serial: self.serial.clone(),
            initialized: self.initialized,
        }
    }
}

/// Block device information
#[derive(Debug, Clone)]
pub struct BlockDeviceInfo {
    pub device_id: String,
    pub name: String,
    pub device_type: BlockDeviceType,
    pub controller: StorageController,
    pub size: u64,
    pub block_size: BlockSize,
    pub block_count: u64,
    pub model: String,
    pub serial: String,
    pub initialized: bool,
}

impl fmt::Display for BlockDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block Device Information\n\
             Device ID: {}\n\
             Name: {}\n\
             Type: {}\n\
             Controller: {}\n\
             Size: {} GB\n\
             Block Size: {}\n\
             Block Count: {}\n\
             Model: {}\n\
             Serial: {}\n\
             Initialized: {}",
            self.device_id,
            self.name,
            self.device_type.as_str(),
            self.controller.as_str(),
            self.size / (1024 * 1024 * 1024),
            self.block_size.as_str(),
            self.block_count,
            self.model,
            self.serial,
            self.initialized
        )
    }
}

/// I/O request
#[derive(Debug, Clone)]
pub struct IORequest {
    pub request_id: [u8; 32],
    pub device_id: String,
    pub block_number: u64,
    pub block_count: u64,
    pub is_write: bool,
    pub completed: bool,
}

impl IORequest {
    pub fn new(device_id: String, block_number: u64, block_count: u64, is_write: bool) -> Self {
        let request_id = Self::generate_request_id(&device_id, block_number);
        
        IORequest {
            request_id,
            device_id,
            block_number,
            block_count,
            is_write,
            completed: false,
        }
    }
    
    fn generate_request_id(device_id: &str, block_number: u64) -> [u8; 32] {
        // Placeholder for actual request ID
        let mut hash = [0u8; 32];
        let device_bytes = device_id.as_bytes();
        for (i, &byte) in device_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let block_bytes = block_number.to_be_bytes();
        for (i, &byte) in block_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    pub fn get_request_id(&self) -> String {
        self.request_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

/// Storage driver
pub struct StorageDriver {
    devices: Vec<BlockDevice>,
    io_queue: Vec<IORequest>,
}

impl StorageDriver {
    pub fn new() -> Self {
        StorageDriver {
            devices: Vec::new(),
            io_queue: Vec::new(),
        }
    }
    
    /// Detect storage devices
    pub fn detect_devices(&mut self) {
        // Simulate device detection
        let nvme1 = BlockDevice::new(
            "nvme0n1".to_string(),
            BlockDeviceType::NVMe,
            StorageController::NVMe,
            512 * 1024 * 1024 * 1024,
            BlockSize::Bytes4096,
            "Samsung 980 PRO".to_string(),
            "S5Z2NX0M123456".to_string(),
        );
        
        let sata1 = BlockDevice::new(
            "sda".to_string(),
            BlockDeviceType::SSD,
            StorageController::AHCI,
            1024 * 1024 * 1024 * 1024,
            BlockSize::Bytes512,
            "Samsung 870 EVO".to_string(),
            "S3Z2NX0M654321".to_string(),
        );
        
        self.devices.push(nvme1);
        self.devices.push(sata1);
    }
    
    /// Get device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&BlockDevice> {
        self.devices
            .iter()
            .find(|d| d.get_device_id() == device_id)
    }
    
    /// Get device by ID (mutable)
    pub fn get_device_mut(&mut self, device_id: &str) -> Option<&mut BlockDevice> {
        self.devices
            .iter_mut()
            .find(|d| d.get_device_id() == device_id)
    }
    
    /// Initialize device
    pub fn initialize_device(&mut self, device_id: &str) -> Result<(), String> {
        let device = self.get_device_mut(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        device.initialize()
    }
    
    /// Read blocks
    pub fn read_blocks(&mut self, device_id: &str, block_number: u64, block_count: u64) -> Result<Vec<Vec<u8>>, String> {
        let device = self.get_device(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        if !device.initialized {
            return Err("Device not initialized".to_string());
        }
        
        let mut blocks = Vec::new();
        for i in 0..block_count {
            let block = device.read_block(block_number + i)?;
            blocks.push(block);
        }
        
        Ok(blocks)
    }
    
    /// Write blocks
    pub fn write_blocks(&mut self, device_id: &str, block_number: u64, data: &[u8]) -> Result<(), String> {
        let device = self.get_device(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        if !device.initialized {
            return Err("Device not initialized".to_string());
        }
        
        let block_size = device.block_size.as_u64() as usize;
        let block_count = (data.len() + block_size - 1) / block_size;
        
        for i in 0..block_count {
            let start = i * block_size;
            let end = std::cmp::min(start + block_size, data.len());
            let block_data = &data[start..end];
            
            device.write_block(block_number + i as u64, block_data)?;
        }
        
        Ok(())
    }
    
    /// Queue I/O request
    pub fn queue_io(&mut self, request: IORequest) {
        self.io_queue.push(request);
    }
    
    /// Process I/O queue
    pub fn process_io_queue(&mut self) -> Result<(), String> {
        for request in &mut self.io_queue {
            if !request.completed {
                // Simulate I/O processing
                request.complete();
            }
        }
        Ok(())
    }
    
    /// List all devices
    pub fn list_devices(&self) -> Vec<&BlockDevice> {
        self.devices.iter().collect()
    }
    
    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for StorageDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_device_creation() {
        let device = BlockDevice::new(
            "nvme0n1".to_string(),
            BlockDeviceType::NVMe,
            StorageController::NVMe,
            512 * 1024 * 1024 * 1024,
            BlockSize::Bytes4096,
            "Test Model".to_string(),
            "TEST123".to_string(),
        );
        
        assert_eq!(device.name, "nvme0n1");
        assert_eq!(device.device_type, BlockDeviceType::NVMe);
        assert!(!device.initialized);
    }
    
    #[test]
    fn test_device_initialization() {
        let mut device = BlockDevice::new(
            "sda".to_string(),
            BlockDeviceType::SSD,
            StorageController::AHCI,
            1024 * 1024 * 1024 * 1024,
            BlockSize::Bytes512,
            "Test Model".to_string(),
            "TEST123".to_string(),
        );
        
        assert!(device.initialize().is_ok());
        assert!(device.initialized);
    }
    
    #[test]
    fn test_block_read() {
        let mut device = BlockDevice::new(
            "nvme0n1".to_string(),
            BlockDeviceType::NVMe,
            StorageController::NVMe,
            512 * 1024 * 1024 * 1024,
            BlockSize::Bytes4096,
            "Test Model".to_string(),
            "TEST123".to_string(),
        );
        
        device.initialize().unwrap();
        
        let block = device.read_block(0);
        assert!(block.is_ok());
        assert_eq!(block.unwrap().len(), 4096);
    }
    
    #[test]
    fn test_block_write() {
        let device = BlockDevice::new(
            "sda".to_string(),
            BlockDeviceType::SSD,
            StorageController::AHCI,
            1024 * 1024 * 1024 * 1024,
            BlockSize::Bytes512,
            "Test Model".to_string(),
            "TEST123".to_string(),
        );
        
        device.initialize().unwrap();
        
        let data = vec![0u8; 512];
        assert!(device.write_block(0, &data).is_ok());
    }
    
    #[test]
    fn test_storage_driver() {
        let mut driver = StorageDriver::new();
        driver.detect_devices();
        
        assert_eq!(driver.device_count(), 2);
        
        let devices = driver.list_devices();
        assert_eq!(devices.len(), 2);
    }
}
