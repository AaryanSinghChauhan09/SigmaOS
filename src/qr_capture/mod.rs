// QR Code Capture Component
// Inspired by Omarchy's QR code capture feature
// Provides screen region selection and QR code decoding

use std::path::PathBuf;

/// Capture region on screen
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CaptureRegion {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

/// QR code decode result
#[derive(Debug, Clone)]
pub struct QrDecodeResult {
    pub content: String,
    pub format: QrFormat,
    pub error_correction: QrErrorCorrection,
    pub is_sensitive: bool,
}

impl QrDecodeResult {
    pub fn new(content: String) -> Self {
        // Mark as sensitive if it looks like it contains secrets
        let is_sensitive = content.contains("otpauth://") 
            || content.contains("password")
            || content.contains("token")
            || content.contains("secret");
        
        Self {
            content,
            format: QrFormat::QrCode,
            error_correction: QrErrorCorrection::Medium,
            is_sensitive,
        }
    }

    pub fn with_format(mut self, format: QrFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_error_correction(mut self, ec: QrErrorCorrection) -> Self {
        self.error_correction = ec;
        self
    }

    /// Check if this is a 2FA setup code
    pub fn is_2fa_setup(&self) -> bool {
        self.content.starts_with("otpauth://")
    }

    /// Extract TOTP parameters if this is a 2FA code
    pub fn extract_totp_params(&self) -> Option<TotpParams> {
        if !self.is_2fa_setup() {
            return None;
        }

        // Parse otpauth://totp/label?secret=SECRET&issuer=ISSUER
        // Simple manual parsing
        let without_prefix = self.content.strip_prefix("otpauth://totp/")?;
        let parts: Vec<&str> = without_prefix.split('?').collect();
        if parts.is_empty() {
            return None;
        }

        let label = parts[0].to_string();
        let mut secret = String::new();
        let mut issuer = String::new();

        if parts.len() > 1 {
            let query = parts[1];
            for param in query.split('&') {
                let kv: Vec<&str> = param.split('=').collect();
                if kv.len() == 2 {
                    match kv[0] {
                        "secret" => secret = kv[1].to_string(),
                        "issuer" => issuer = kv[1].to_string(),
                        _ => {}
                    }
                }
            }
        }

        Some(TotpParams {
            label,
            secret,
            issuer,
        })
    }
}

/// QR/barcode format
#[derive(Debug, Clone, PartialEq)]
pub enum QrFormat {
    QrCode,
    DataMatrix,
    Aztec,
    Pdf417,
}

/// Error correction level
#[derive(Debug, Clone, PartialEq)]
pub enum QrErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

/// TOTP parameters extracted from 2FA QR code
#[derive(Debug, Clone)]
pub struct TotpParams {
    pub label: String,
    pub secret: String,
    pub issuer: String,
}

/// Capture mode
#[derive(Debug, Clone, PartialEq)]
pub enum CaptureMode {
    Region,
    Fullscreen,
    Window,
}

/// QR capture configuration
#[derive(Debug, Clone)]
pub struct QrCaptureConfig {
    pub mode: CaptureMode,
    pub sensitive_clipboard: bool,
    pub auto_copy: bool,
    pub notify_on_decode: bool,
}

impl QrCaptureConfig {
    pub fn new() -> Self {
        Self {
            mode: CaptureMode::Region,
            sensitive_clipboard: true,
            auto_copy: true,
            notify_on_decode: false,
        }
    }

    pub fn with_mode(mut self, mode: CaptureMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_sensitive_clipboard(mut self, sensitive: bool) -> Self {
        self.sensitive_clipboard = sensitive;
        self
    }

    pub fn with_auto_copy(mut self, auto: bool) -> Self {
        self.auto_copy = auto;
        self
    }

    pub fn with_notify(mut self, notify: bool) -> Self {
        self.notify_on_decode = notify;
        self
    }
}

impl Default for QrCaptureConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// QR code capture manager
#[derive(Debug, Clone)]
pub struct QrCaptureManager {
    config: QrCaptureConfig,
    history: Vec<QrDecodeResult>,
}

impl QrCaptureManager {
    pub fn new(config: QrCaptureConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: QrCaptureConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &QrCaptureConfig {
        &self.config
    }

    /// Capture and decode QR code from screen region
    pub fn capture_qr(&mut self, region: CaptureRegion) -> Result<QrDecodeResult, String> {
        if !region.is_valid() {
            return Err("Invalid capture region".to_string());
        }

        // In a real implementation, this would:
        // 1. Capture the screen region
        // 2. Convert to image format
        // 3. Run QR code detection and decoding
        // For now, we'll simulate it
        let content = "otpauth://totp/Example:alice@google.com?secret=JBSWY3DPEHPK3PXP&issuer=Example".to_string();
        let result = QrDecodeResult::new(content);

        // Add to history if not sensitive
        if !result.is_sensitive {
            self.history.push(result.clone());
        }

        Ok(result)
    }

    /// Capture QR code from fullscreen
    pub fn capture_qr_fullscreen(&mut self) -> Result<QrDecodeResult, String> {
        // In a real implementation, this would capture the entire screen
        // For now, we'll simulate with a large region
        let region = CaptureRegion::new(0, 0, 1920, 1080);
        self.capture_qr(region)
    }

    /// Get decode history (non-sensitive only)
    pub fn get_history(&self) -> &[QrDecodeResult] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> QrCaptureStatistics {
        let total = self.history.len();
        let sensitive_count = self.history.iter().filter(|r| r.is_sensitive).count();
        let totp_count = self.history.iter().filter(|r| r.is_2fa_setup()).count();

        QrCaptureStatistics {
            total_decodes: total,
            sensitive_decodes: sensitive_count,
            totp_decodes: totp_count,
        }
    }
}

impl Default for QrCaptureManager {
    fn default() -> Self {
        Self::new(QrCaptureConfig::default())
    }
}

/// QR capture statistics
#[derive(Debug, Clone, PartialEq)]
pub struct QrCaptureStatistics {
    pub total_decodes: usize,
    pub sensitive_decodes: usize,
    pub totp_decodes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = QrCaptureManager::new(QrCaptureConfig::new());
        assert_eq!(manager.get_history().len(), 0);
    }

    #[test]
    fn test_capture_region() {
        let region = CaptureRegion::new(100, 100, 200, 200);
        assert!(region.is_valid());
        assert_eq!(region.area(), 40000);
    }

    #[test]
    fn test_invalid_region() {
        let region = CaptureRegion::new(0, 0, 0, 0);
        assert!(!region.is_valid());
    }

    #[test]
    fn test_capture_qr() {
        let mut manager = QrCaptureManager::new(QrCaptureConfig::new());
        let region = CaptureRegion::new(100, 100, 200, 200);
        let result = manager.capture_qr(region).unwrap();
        
        assert!(result.is_2fa_setup());
        assert!(result.is_sensitive);
    }

    #[test]
    fn test_qr_decode_result() {
        let result = QrDecodeResult::new("test content".to_string());
        assert!(!result.is_sensitive);
        assert!(!result.is_2fa_setup());
    }

    #[test]
    fn test_sensitive_detection() {
        let result = QrDecodeResult::new("otpauth://totp/test".to_string());
        assert!(result.is_sensitive);
        assert!(result.is_2fa_setup());
    }

    #[test]
    fn test_totp_extraction() {
        let result = QrDecodeResult::new(
            "otpauth://totp/Example:alice@google.com?secret=JBSWY3DPEHPK3PXP&issuer=Example".to_string()
        );
        
        let params = result.extract_totp_params();
        assert!(params.is_some());
        
        let p = params.unwrap();
        assert_eq!(p.label, "Example:alice@google.com");
        assert_eq!(p.secret, "JBSWY3DPEHPK3PXP");
        assert_eq!(p.issuer, "Example");
    }

    #[test]
    fn test_config() {
        let config = QrCaptureConfig::new()
            .with_mode(CaptureMode::Fullscreen)
            .with_sensitive_clipboard(false)
            .with_auto_copy(true)
            .with_notify(true);
        
        assert_eq!(config.mode, CaptureMode::Fullscreen);
        assert!(!config.sensitive_clipboard);
        assert!(config.auto_copy);
        assert!(config.notify_on_decode);
    }

    #[test]
    fn test_statistics() {
        let mut manager = QrCaptureManager::new(QrCaptureConfig::new());
        let region = CaptureRegion::new(100, 100, 200, 200);
        
        // Capture a sensitive code (won't be added to history)
        manager.capture_qr(region).unwrap();
        
        // Create a non-sensitive result and add it
        let non_sensitive = QrDecodeResult::new("https://example.com".to_string());
        manager.history.push(non_sensitive);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_decodes, 1);
        assert_eq!(stats.sensitive_decodes, 0);
        assert_eq!(stats.totp_decodes, 0);
    }

    #[test]
    fn test_clear_history() {
        let mut manager = QrCaptureManager::new(QrCaptureConfig::new());
        let non_sensitive = QrDecodeResult::new("test".to_string());
        manager.history.push(non_sensitive);
        
        assert_eq!(manager.get_history().len(), 1);
        manager.clear_history();
        assert_eq!(manager.get_history().len(), 0);
    }
}
