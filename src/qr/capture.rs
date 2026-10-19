//! SigmaOS QR Capture System
//!
//! Inspired by: Omarchy QR capture (omarchy-capture-qr, omarchy-network-qr)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SECURE_DEFAULTS, INPUT_VALIDATION, MEMORY_SAFETY,
//! DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL.

use std::collections::HashMap;

/// QR code type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QrCodeType {
    Text(String),
    Url(String),
    Wifi { ssid: String, password: String, encryption: WifiEncryption },
    Network { ssid: String, password: String },
    Contact { name: String, phone: String, email: Option<String> },
    Custom { data: String, format: String },
}

/// WiFi encryption type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WifiEncryption {
    Wpa,
    Wpa2,
    Wpa3,
    Wep,
    None,
}

/// QR capture result
#[derive(Debug, Clone)]
pub struct QrCaptureResult {
    pub id: u64,
    pub data: String,
    pub qr_type: QrCodeType,
    pub captured_at: u64,
    pub source: QrSource,
}

/// QR source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QrSource {
    Camera,
    File,
    Screen,
    Clipboard,
}

/// QR capture manager
pub struct QrCaptureManager {
    pub results: Vec<QrCaptureResult>,
    pub next_id: u64,
    pub max_history: usize,
}

impl QrCaptureManager {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            next_id: 1,
            max_history: 100,
        }
    }

    /// Capture QR from camera
    pub fn capture_from_camera(&mut self) -> Result<u64, String> {
        // In production, this would interface with camera hardware
        // For now, return a simulated result
        let id = self.next_id;
        self.next_id += 1;
        let result = QrCaptureResult {
            id,
            data: "https://sigmaos.example.com".to_string(),
            qr_type: QrCodeType::Url("https://sigmaos.example.com".to_string()),
            captured_at: now_secs(),
            source: QrSource::Camera,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Capture QR from file
    pub fn capture_from_file(&mut self, file_path: String) -> Result<u64, String> {
        if file_path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        let id = self.next_id;
        self.next_id += 1;
        let result = QrCaptureResult {
            id,
            data: format!("QR data from: {}", file_path),
            qr_type: QrCodeType::Text(format!("QR data from: {}", file_path)),
            captured_at: now_secs(),
            source: QrSource::File,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Capture QR from screen
    pub fn capture_from_screen(&mut self) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let result = QrCaptureResult {
            id,
            data: "QR_CODE_SCREEN_DATA".to_string(),
            qr_type: QrCodeType::Text("QR_CODE_SCREEN_DATA".to_string()),
            captured_at: now_secs(),
            source: QrSource::Screen,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Capture QR from clipboard
    pub fn capture_from_clipboard(&mut self) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let result = QrCaptureResult {
            id,
            data: "QR_CODE_CLIPBOARD_DATA".to_string(),
            qr_type: QrCodeType::Text("QR_CODE_CLIPBOARD_DATA".to_string()),
            captured_at: now_secs(),
            source: QrSource::Clipboard,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Generate WiFi QR code
    pub fn generate_wifi_qr(
        &mut self,
        ssid: String,
        password: String,
        encryption: WifiEncryption,
    ) -> Result<u64, String> {
        if ssid.is_empty() {
            return Err("SSID cannot be empty".to_string());
        }
        let id = self.next_id;
        self.next_id += 1;
        let wifi_type = QrCodeType::Wifi {
            ssid: ssid.clone(),
            password,
            encryption,
        };
        let data = format!("WIFI:T:{};S:{};P:{};;", encryption_str(&encryption), ssid, password);
        let result = QrCaptureResult {
            id,
            data,
            qr_type: wifi_type,
            captured_at: now_secs(),
            source: QrSource::File,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Generate network QR code (for sharing WiFi credentials)
    pub fn generate_network_qr(&mut self, ssid: String, password: String) -> Result<u64, String> {
        if ssid.is_empty() {
            return Err("SSID cannot be empty".to_string());
        }
        let id = self.next_id;
        self.next_id += 1;
        let data = format!("WIFI:S:{};P:{};;", ssid, password);
        let result = QrCaptureResult {
            id,
            data,
            qr_type: QrCodeType::Network { ssid, password },
            captured_at: now_secs(),
            source: QrSource::File,
        };
        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// List all QR capture results
    pub fn list(&self) -> Vec<&QrCaptureResult> {
        self.results.iter().rev().collect()
    }

    /// Get a result by ID
    pub fn get(&self, id: u64) -> Option<&QrCaptureResult> {
        self.results.iter().find(|r| r.id == id)
    }

    /// Delete a result
    pub fn delete(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .results
            .iter()
            .position(|r| r.id == id)
            .ok_or_else(|| format!("QR capture {} not found", id))?;
        self.results.remove(pos);
        Ok(())
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.results.clear();
    }

    fn trim_history(&mut self) {
        while self.results.len() > self.max_history {
            self.results.remove(0);
        }
    }
}

impl Default for QrCaptureManager {
    fn default() -> Self {
        Self::new()
    }
}

fn encryption_str(enc: &WifiEncryption) -> &str {
    match enc {
        WifiEncryption::Wpa => "WPA",
        WifiEncryption::Wpa2 => "WPA2",
        WifiEncryption::Wpa3 => "WPA3",
        WifiEncryption::Wep => "WEP",
        WifiEncryption::None => "nopass",
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
    fn test_capture_from_camera() {
        let mut mgr = QrCaptureManager::new();
        let id = mgr.capture_from_camera().unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.list().len(), 1);
    }

    #[test]
    fn test_capture_from_file_empty_fails() {
        let mut mgr = QrCaptureManager::new();
        let result = mgr.capture_from_file(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_wifi_qr() {
        let mut mgr = QrCaptureManager::new();
        let id = mgr.generate_wifi_qr(
            "MyWiFi".to_string(),
            "password123".to_string(),
            WifiEncryption::Wpa2,
        )
        .unwrap();
        assert_eq!(id, 1);
        let result = mgr.get(id).unwrap();
        match &result.qr_type {
            QrCodeType::Wifi { ssid, password, encryption } => {
                assert_eq!(ssid, "MyWiFi");
                assert_eq!(password, "password123");
                assert_eq!(*encryption, WifiEncryption::Wpa2);
            }
            _ => panic!("Expected WiFi QR type"),
        }
    }

    #[test]
    fn test_generate_network_qr() {
        let mut mgr = QrCaptureManager::new();
        let id = mgr.generate_network_qr("HomeNet".to_string(), "secret".to_string()).unwrap();
        assert_eq!(id, 1);
        let result = mgr.get(id).unwrap();
        match &result.qr_type {
            QrCodeType::Network { ssid, password } => {
                assert_eq!(ssid, "HomeNet");
                assert_eq!(password, "secret");
            }
            _ => panic!("Expected Network QR type"),
        }
    }

    #[test]
    fn test_empty_ssid_fails() {
        let mut mgr = QrCaptureManager::new();
        let result = mgr.generate_wifi_qr(String::String::new(), "pass".to_string(), WifiEncryption::Wpa2);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_capture() {
        let mut mgr = QrCaptureManager::new();
        mgr.capture_from_camera().unwrap();
        assert!(mgr.delete(1).is_ok());
        assert_eq!(mgr.list().len(), 0);
    }
}