//! SigmaOS Network Tools
//!
//! Inspired by: Omarchy network (omancy-network-band, omancy-network-password, omancy-network-qr, omancy-network-speedtest, omancy-network-status)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, NETWORK_ISOLATION, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL.

/// Network band
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkBand {
    TwoGhz,
    FiveGhz,
    SixGhz,
    Unknown,
}

/// Network status
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub interface: String,
    pub ip_address: String,
    pub mac_address: String,
    pub ssid: Option<String>,
    pub signal_strength: u32,
    pub band: NetworkBand,
    pub tx_speed: f32,
    pub rx_speed: f32,
}

/// Network password info
#[derive(Debug, Clone)]
pub struct NetworkPassword {
    pub ssid: String,
    pub password: String,
    pub encryption: String,
}

/// Network QR data
#[derive(Debug, Clone)]
pub struct NetworkQrData {
    pub ssid: String,
    pub password: String,
    pub encryption: String,
    pub qr_data: String,
}

/// Speedtest result
#[derive(Debug, Clone)]
pub struct SpeedtestResult {
    pub download_mbps: f32,
    pub upload_mbps: f32,
    pub ping_ms: u32,
    pub tested_at: u64,
}

/// Network tools manager
pub struct NetworkToolsManager {
    pub status: Option<NetworkStatus>,
    pub saved_passwords: Vec<NetworkPassword>,
    pub last_speedtest: Option<SpeedtestResult>,
}

impl NetworkToolsManager {
    pub fn new() -> Self {
        Self {
            status: None,
            saved_passwords: Vec::new(),
            last_speedtest: None,
        }
    }

    /// Set network status
    pub fn set_status(&mut self, status: NetworkStatus) {
        self.status = Some(status);
    }

    /// Get network status
    pub fn get_status(&self) -> Option<&NetworkStatus> {
        self.status.as_ref()
    }

    /// Save network password
    pub fn save_password(&mut self, password: NetworkPassword) {
        self.saved_passwords.push(password);
    }

    /// Get saved passwords
    pub fn list_passwords(&self) -> Vec<&NetworkPassword> {
        self.saved_passwords.iter().collect()
    }

    /// Generate network QR data
    pub fn generate_network_qr(&self, ssid: &str, password: &str, encryption: &str) -> NetworkQrData {
        let qr_data = format!("WIFI:S:{};P:{};T:{};;", ssid, password, encryption);
        NetworkQrData {
            ssid: ssid.to_string(),
            password: password.to_string(),
            encryption: encryption.to_string(),
            qr_data,
        }
    }

    /// Run speedtest (simulated)
    pub fn run_speedtest(&mut self) -> SpeedtestResult {
        let result = SpeedtestResult {
            download_mbps: 150.5,
            upload_mbps: 45.2,
            ping_ms: 12,
            tested_at: now_secs(),
        };
        self.last_speedtest = Some(result.clone());
        result
    }

    /// Get last speedtest
    pub fn get_last_speedtest(&self) -> Option<&SpeedtestResult> {
        self.last_speedtest.as_ref()
    }

    /// Clear saved passwords
    pub fn clear_passwords(&mut self) {
        self.saved_passwords.clear();
    }
}

impl Default for NetworkToolsManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_status() {
        let mut mgr = NetworkToolsManager::new();
        let status = NetworkStatus {
            interface: "wlan0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            mac_address: "aa:bb:cc:dd:ee:ff".to_string(),
            ssid: Some("MyWiFi".to_string()),
            signal_strength: 85,
            band: NetworkBand::FiveGhz,
            tx_speed: 100.0,
            rx_speed: 200.0,
        };
        mgr.set_status(status);
        assert!(mgr.get_status().is_some());
        assert_eq!(mgr.get_status().unwrap().ssid, Some("MyWiFi".to_string()));
    }

    #[test]
    fn test_save_password() {
        let mut mgr = NetworkToolsManager::new();
        let pwd = NetworkPassword {
            ssid: "HomeWiFi".to_string(),
            password: "secret123".to_string(),
            encryption: "WPA2".to_string(),
        };
        mgr.save_password(pwd);
        assert_eq!(mgr.list_passwords().len(), 1);
    }

    #[test]
    fn test_generate_network_qr() {
        let mgr = NetworkToolsManager::new();
        let qr = mgr.generate_network_qr("MyWiFi", "password123", "WPA2");
        assert_eq!(qr.ssid, "MyWiFi");
        assert!(qr.qr_data.contains("WIFI:S:MyWiFi"));
    }

    #[test]
    fn test_run_speedtest() {
        let mut mgr = NetworkToolsManager::new();
        let result = mgr.run_speedtest();
        assert!(result.download_mbps > 0.0);
        assert!(result.upload_mbps > 0.0);
        assert!(mgr.get_last_speedtest().is_some());
    }

    #[test]
    fn test_clear_passwords() {
        let mut mgr = NetworkToolsManager::new();
        mgr.save_password(NetworkPassword {
            ssid: "test".to_string(),
            password: "test".to_string(),
            encryption: "WPA2".to_string(),
        });
        assert_eq!(mgr.list_passwords().len(), 1);
        mgr.clear_passwords();
        assert_eq!(mgr.list_passwords().len(), 0);
    }
}