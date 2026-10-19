# SigmaOS Text Extraction Components

## Overview
SigmaOS text extraction (OCR) system providing text capture from screen, images, clipboard, and selection, competitive with Omarchy text extraction (omancy-capture-text).

## Features
- **Screen text extraction** - OCR from screen
- **Image text extraction** - OCR from image files
- **Clipboard text extraction** - OCR from clipboard
- **Selection text extraction** - OCR from selected text
- **Multiple OCR engines**: Tesseract, OcrmyPdf, Custom
- **Language support** - Configurable language per extraction
- **Confidence scoring** - Per-extraction confidence
- **History tracking** - Up to 100 extraction results

## Structures

| Structure | Description |
|-----------|-------------|
| **TextExtractor** | Text extractor with result management |
| **TextExtractionResult** | Individual extraction result |
| **OcrEngine** | Tesseract, OcrmyPdf, Custom |
| **TextSource** | Screen, Image, Clipboard, Selection |

## Usage Examples

```rust
// Create text extractor
let mut extractor = TextExtractor::new();

// Extract from screen
let id = extractor.extract_from_screen(None, None).unwrap();

// Extract from image
let id = extractor.extract_from_image(
    "/path/to/image.png".to_string(),
    Some("eng".to_string()),
    None,
).unwrap();

// Set default engine
extractor.set_default_engine(OcrEngine::OcrmyPdf);
extractor.set_default_language("deu".to_string());

// List results
let results = extractor.list();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/text/extractor.rs` - Text extractor implementation
- `wiki/Text-Extraction.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
MEMORY_SAFETY, DETERMINISTIC_TESTING, DIRECT_DEVICE_CONTROL