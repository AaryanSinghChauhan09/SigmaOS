# PDFCraft: PDF Viewer, Annotation & Form Engine

PDFCraft renders complex vector PDF documents with text extraction, form filling, and digital signature verification.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Adobe Acrobat Pro, macOS Preview, Foxit
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct PdfAnnotation { pub page_index: u32, pub rect: [f32; 4], pub contents: String }
pub struct FormField { pub field_name: String, pub field_value: String, pub is_read_only: bool }
```

---

## ✨ Features & Capabilities
- **Compliant PDF 2.0 and PDF/A archival vector rendering**
- **Interactive AcroForms filling and flattened export**
- **Cryptographic digital signatures (X.509 PKCS#7 / PAdES)**
- **Searchable OCR text layer insertion and extraction**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
