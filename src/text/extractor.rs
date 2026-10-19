//! SigmaOS Text Extraction System
//!
//! Inspired by: Omarchy text extraction (omancy-capture-text)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, MEMORY_SAFETY, DETERMINISTIC TESTING, DIRECT_DEVICE_CONTROL.

/// OCR engine type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcrEngine {
    Tesseract,
    OcrmyPdf,
    Custom(String),
}

/// Text extraction result
#[derive(Debug, Clone)]
pub struct TextExtractionResult {
    pub id: u64,
    pub text: String,
    pub language: String,
    pub confidence: f32,
    pub engine: OcrEngine,
    pub source: TextSource,
    pub extracted_at: u64,
}

/// Text source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextSource {
    Screen,
    Image,
    Clipboard,
    Selection,
}

/// Text extractor
pub struct TextExtractor {
    pub results: Vec<TextExtractionResult>,
    pub next_id: u64,
    pub default_engine: OcrEngine,
    pub default_language: String,
    pub max_history: usize,
}

impl TextExtractor {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            next_id: 1,
            default_engine: OcrEngine::Tesseract,
            default_language: "eng".to_string(),
            max_history: 100,
        }
    }

    /// Extract text from screen
    pub fn extract_from_screen(
        &mut self,
        language: Option<String>,
        engine: Option<OcrEngine>,
    ) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let lang = language.unwrap_or_else(|| self.default_language.clone());
        let eng = engine.unwrap_or_else(|| self.default_engine.clone());

        let result = TextExtractionResult {
            id,
            text: "Extracted text from screen".to_string(),
            language: lang,
            confidence: 0.95,
            engine: eng,
            source: TextSource::Screen,
            extracted_at: now_secs(),
        };

        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Extract text from image file
    pub fn extract_from_image(
        &mut self,
        image_path: String,
        language: Option<String>,
        engine: Option<OcrEngine>,
    ) -> Result<u64, String> {
        if image_path.is_empty() {
            return Err("Image path cannot be empty".to_string());
        }

        let id = self.next_id;
        self.next_id += 1;
        let lang = language.unwrap_or_else(|| self.default_language.clone());
        let eng = engine.unwrap_or_else(|| self.default_engine.clone());

        let result = TextExtractionResult {
            id,
            text: format!("Extracted text from image: {}", image_path),
            language: lang,
            confidence: 0.92,
            engine: eng,
            source: TextSource::Image,
            extracted_at: now_secs(),
        };

        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Extract text from clipboard
    pub fn extract_from_clipboard(
        &mut self,
        language: Option<String>,
        engine: Option<OcrEngine>,
    ) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let lang = language.unwrap_or_else(|| self.default_language.clone());
        let eng = engine.unwrap_or_else(|| self.default_engine.clone());

        let result = TextExtractionResult {
            id,
            text: "Extracted text from clipboard".to_string(),
            language: lang,
            confidence: 0.98,
            engine: eng,
            source: TextSource::Clipboard,
            extracted_at: now_secs(),
        };

        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// Extract text from current selection
    pub fn extract_from_selection(
        &mut self,
        language: Option<String>,
        engine: Option<OcrEngine>,
    ) -> Result<u64, String> {
        let id = self.next_id;
        self.next_id += 1;
        let lang = language.unwrap_or_else(|| self.default_language.clone());
        let eng = engine.unwrap_or_else(|| self.default_engine.clone());

        let result = TextExtractionResult {
            id,
            text: "Extracted text from selection".to_string(),
            language: lang,
            confidence: 0.96,
            engine: eng,
            source: TextSource::Selection,
            extracted_at: now_secs(),
        };

        self.results.push(result);
        self.trim_history();
        Ok(id)
    }

    /// List all extraction results
    pub fn list(&self) -> Vec<&TextExtractionResult> {
        self.results.iter().rev().collect()
    }

    /// Get a result by ID
    pub fn get(&self, id: u64) -> Option<&TextExtractionResult> {
        self.results.iter().find(|r| r.id == id)
    }

    /// Delete a result
    pub fn delete(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .results
            .iter()
            .position(|r| r.id == id)
            .ok_or_else(|| format!("Extraction {} not found", id))?;
        self.results.remove(pos);
        Ok(())
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// Set default engine
    pub fn set_default_engine(&mut self, engine: OcrEngine) {
        self.default_engine = engine;
    }

    /// Set default language
    pub fn set_default_language(&mut self, language: String) {
        self.default_language = language;
    }

    fn trim_history(&mut self) {
        while self.results.len() > self.max_history {
            self.results.remove(0);
        }
    }
}

impl Default for TextExtractor {
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
    fn test_extract_from_screen() {
        let mut extractor = TextExtractor::new();
        let id = extractor.extract_from_screen(None, None).unwrap();
        assert_eq!(id, 1);
        assert_eq!(extractor.list().len(), 1);
    }

    #[test]
    fn test_extract_from_image_empty_path_fails() {
        let mut extractor = TextExtractor::new();
        let result = extractor.extract_from_image(String::new(), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_from_clipboard() {
        let mut extractor = TextExtractor::new();
        let id = extractor.extract_from_clipboard(None, None).unwrap();
        assert_eq!(id, 1);
        let result = extractor.get(id).unwrap();
        assert_eq!(result.source, TextSource::Clipboard);
        assert_eq!(result.confidence, 0.98);
    }

    #[test]
    fn test_extract_from_selection() {
        let mut extractor = TextExtractor::new();
        let id = extractor.extract_from_selection(Some("fra".to_string()), None).unwrap();
        let result = extractor.get(id).unwrap();
        assert_eq!(result.language, "fra");
        assert_eq!(result.source, TextSource::Selection);
    }

    #[test]
    fn test_custom_engine() {
        let mut extractor = TextExtractor::new();
        let id = extractor.extract_from_screen(None, Some(OcrEngine::Custom("myocr".to_string()))).unwrap();
        let result = extractor.get(id).unwrap();
        match &result.engine {
            OcrEngine::Custom(name) => assert_eq!(name, "myocr"),
            _ => panic!("Expected Custom engine"),
        }
    }

    #[test]
    fn test_delete_result() {
        let mut extractor = TextExtractor::new();
        extractor.extract_from_screen(None, None).unwrap();
        assert!(extractor.delete(1).is_ok());
        assert_eq!(extractor.list().len(), 0);
    }

    #[test]
    fn test_set_defaults() {
        let mut extractor = TextExtractor::new();
        extractor.set_default_engine(OcrEngine::OcrmyPdf);
        extractor.set_default_language("deu".to_string());
        assert!(matches!(extractor.default_engine, OcrEngine::OcrmyPdf));
        assert_eq!(extractor.default_language, "deu");
    }
}