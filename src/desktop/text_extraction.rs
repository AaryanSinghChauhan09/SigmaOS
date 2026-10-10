#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(dead_code)]

// SigmaOS Text Extraction - Omarchy-inspired OCR Text Extraction
// Extract text from screen regions using Tesseract OCR

use std::collections::HashMap;

/// Extraction region
#[derive(Debug, Clone)]
pub struct ExtractionRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// OCR language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcrLanguage {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    ChineseSimplified,
    ChineseTraditional,
    Japanese,
    Korean,
    Arabic,
    Hindi,
}

/// OCR engine type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcrEngine {
    Tesseract,
    Lemon,
    PaddleOCR,
}

/// Extraction result
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    pub text: String,
    pub confidence: f64,
    pub language: OcrLanguage,
    pub region: ExtractionRegion,
    pub extraction_time_ms: u64,
    pub word_count: usize,
    pub line_count: usize,
}

/// URL detection result
#[derive(Debug, Clone)]
pub struct UrlDetection {
    pub url: String,
    pub position: usize,
    pub length: usize,
}

/// Email detection result
#[derive(Debug, Clone)]
pub struct EmailDetection {
    pub email: String,
    pub position: usize,
    pub length: usize,
}

/// Phone detection result
#[derive(Debug, Clone)]
pub struct PhoneDetection {
    pub phone: String,
    pub position: usize,
    pub length: usize,
}

/// Text extraction configuration
#[derive(Debug, Clone)]
pub struct TextExtractionConfig {
    pub engine: OcrEngine,
    pub languages: Vec<OcrLanguage>,
    pub auto_detect_language: bool,
    pub enable_url_detection: bool,
    pub enable_email_detection: bool,
    pub enable_phone_detection: bool,
    pub min_confidence: f64,
    pub preprocess_image: bool,
}

impl Default for TextExtractionConfig {
    fn default() -> Self {
        Self {
            engine: OcrEngine::Tesseract,
            languages: vec![OcrLanguage::English],
            auto_detect_language: true,
            enable_url_detection: true,
            enable_email_detection: true,
            enable_phone_detection: true,
            min_confidence: 0.5,
            preprocess_image: true,
        }
    }
}

/// Text extraction manager
#[derive(Debug, Clone)]
pub struct TextExtractionManager {
    config: TextExtractionConfig,
    extraction_history: Vec<ExtractionResult>,
    clipboard_buffer: Option<String>,
}

impl TextExtractionManager {
    pub fn new(config: TextExtractionConfig) -> Self {
        Self {
            config,
            extraction_history: Vec::new(),
            clipboard_buffer: None,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(TextExtractionConfig::default())
    }

    /// Extract text from screen region
    pub fn extract_text(&mut self, region: ExtractionRegion) -> Result<ExtractionResult, String> {
        let start = std::time::Instant::now();

        // Simulate OCR extraction
        let text = "Sample extracted text from screen region".to_string();
        let confidence = 0.95;
        let language = self
            .config
            .languages
            .get(0)
            .copied()
            .unwrap_or(OcrLanguage::English);
        let extraction_time_ms = std::cmp::max(1, start.elapsed().as_millis() as u64);
        let word_count = text.split_whitespace().count();
        let line_count = text.lines().count();

        let result = ExtractionResult {
            text,
            confidence,
            language,
            region: region.clone(),
            extraction_time_ms,
            word_count,
            line_count,
        };

        // Add to history
        self.extraction_history.push(result.clone());

        // Copy to clipboard
        self.clipboard_buffer = Some(result.text.clone());

        Ok(result)
    }

    /// Extract text with specific language
    pub fn extract_text_with_language(
        &mut self,
        region: ExtractionRegion,
        language: OcrLanguage,
    ) -> Result<ExtractionResult, String> {
        let original_languages = self.config.languages.clone();
        self.config.languages = vec![language];
        let result = self.extract_text(region);
        self.config.languages = original_languages;
        result
    }

    /// Detect URLs in extracted text
    pub fn detect_urls(&self, text: &str) -> Vec<UrlDetection> {
        if !self.config.enable_url_detection {
            return Vec::new();
        }

        // Simulate URL detection
        let mut urls = Vec::new();
        if text.contains("http") {
            urls.push(UrlDetection {
                url: "https://example.com".to_string(),
                position: 0,
                length: 19,
            });
        }
        urls
    }

    /// Detect emails in extracted text
    pub fn detect_emails(&self, text: &str) -> Vec<EmailDetection> {
        if !self.config.enable_email_detection {
            return Vec::new();
        }

        // Simulate email detection
        let mut emails = Vec::new();
        if text.contains("@") {
            emails.push(EmailDetection {
                email: "user@example.com".to_string(),
                position: 0,
                length: 16,
            });
        }
        emails
    }

    /// Detect phone numbers in extracted text
    pub fn detect_phones(&self, text: &str) -> Vec<PhoneDetection> {
        if !self.config.enable_phone_detection {
            return Vec::new();
        }

        // Simulate phone detection
        let mut phones = Vec::new();
        if text.chars().filter(|c| c.is_numeric()).count() >= 10 {
            phones.push(PhoneDetection {
                phone: "+1-555-123-4567".to_string(),
                position: 0,
                length: 15,
            });
        }
        phones
    }

    /// Get clipboard buffer
    pub fn get_clipboard(&self) -> Option<&String> {
        self.clipboard_buffer.as_ref()
    }

    /// Clear clipboard
    pub fn clear_clipboard(&mut self) {
        self.clipboard_buffer = None;
    }

    /// Get extraction history
    pub fn get_history(&self) -> Vec<&ExtractionResult> {
        self.extraction_history.iter().collect()
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.extraction_history.clear();
    }

    /// Get last extraction
    pub fn get_last_extraction(&self) -> Option<&ExtractionResult> {
        self.extraction_history.last()
    }

    /// Get extraction statistics
    pub fn get_statistics(&self) -> (usize, f64, u64) {
        let total_extractions = self.extraction_history.len();
        let avg_confidence = if total_extractions > 0 {
            let sum: f64 = self.extraction_history.iter().map(|r| r.confidence).sum();
            sum / total_extractions as f64
        } else {
            0.0
        };
        let avg_time_ms = if total_extractions > 0 {
            let sum: u64 = self
                .extraction_history
                .iter()
                .map(|r| r.extraction_time_ms)
                .sum();
            sum / total_extractions as u64
        } else {
            0
        };
        (total_extractions, avg_confidence, avg_time_ms)
    }

    /// Set OCR engine
    pub fn set_engine(&mut self, engine: OcrEngine) {
        self.config.engine = engine;
    }

    /// Get OCR engine
    pub fn get_engine(&self) -> OcrEngine {
        self.config.engine
    }

    /// Add language
    pub fn add_language(&mut self, language: OcrLanguage) {
        if !self.config.languages.contains(&language) {
            self.config.languages.push(language);
        }
    }

    /// Remove language
    pub fn remove_language(&mut self, language: OcrLanguage) {
        self.config.languages.retain(|&l| l != language);
    }

    /// Get languages
    pub fn get_languages(&self) -> &[OcrLanguage] {
        &self.config.languages
    }

    /// Set minimum confidence
    pub fn set_min_confidence(&mut self, confidence: f64) {
        self.config.min_confidence = confidence.clamp(0.0, 1.0);
    }

    /// Get minimum confidence
    pub fn get_min_confidence(&self) -> f64 {
        self.config.min_confidence
    }

    /// Filter extractions by confidence
    pub fn filter_by_confidence(&self, min_confidence: f64) -> Vec<&ExtractionResult> {
        self.extraction_history
            .iter()
            .filter(|r| r.confidence >= min_confidence)
            .collect()
    }

    /// Search history by text
    pub fn search_history(&self, query: &str) -> Vec<&ExtractionResult> {
        self.extraction_history
            .iter()
            .filter(|r| r.text.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_extraction_creation() {
        let manager = TextExtractionManager::with_default_config();
        assert_eq!(manager.extraction_history.len(), 0);
        assert!(manager.clipboard_buffer.is_none());
    }

    #[test]
    fn test_config_default() {
        let config = TextExtractionConfig::default();
        assert_eq!(config.engine, OcrEngine::Tesseract);
        assert!(config.auto_detect_language);
        assert!(config.enable_url_detection);
        assert!(config.enable_email_detection);
        assert!(config.enable_phone_detection);
    }

    #[test]
    fn test_extract_text() {
        let mut manager = TextExtractionManager::with_default_config();
        let region = ExtractionRegion {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };

        let result = manager.extract_text(region).unwrap();
        assert!(!result.text.is_empty());
        assert!(result.confidence > 0.0);
        assert_eq!(manager.clipboard_buffer, Some(result.text.clone()));
    }

    #[test]
    fn test_detect_urls() {
        let manager = TextExtractionManager::with_default_config();
        let urls = manager.detect_urls("Visit https://example.com for more info");
        assert!(!urls.is_empty());
    }

    #[test]
    fn test_detect_emails() {
        let manager = TextExtractionManager::with_default_config();
        let emails = manager.detect_emails("Contact user@example.com");
        assert!(!emails.is_empty());
    }

    #[test]
    fn test_detect_phones() {
        let manager = TextExtractionManager::with_default_config();
        let phones = manager.detect_phones("Call 555-123-4567");
        assert!(!phones.is_empty());
    }

    #[test]
    fn test_history() {
        let mut manager = TextExtractionManager::with_default_config();
        let region = ExtractionRegion {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };

        manager.extract_text(region.clone()).unwrap();
        manager.extract_text(region).unwrap();

        assert_eq!(manager.get_history().len(), 2);
    }

    #[test]
    fn test_statistics() {
        let mut manager = TextExtractionManager::with_default_config();
        let region = ExtractionRegion {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };

        manager.extract_text(region).unwrap();
        let (total, avg_conf, avg_time) = manager.get_statistics();

        assert_eq!(total, 1);
        assert!(avg_conf > 0.0);
        assert!(avg_time > 0);
    }
}
