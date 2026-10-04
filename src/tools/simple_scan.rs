//! Sovereign Simple Scan (SANE & eSCL / AirScan Document Scanner) Engine
//! Inspired by Linux GNOME Simple Scan (`simple-scan`), SANE (Scanner Access Now Easy), and eSCL driverless AirScan

use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Scanner document input feed sources
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanSource {
    Flatbed,
    AdfFront,
    AdfDuplex,
}

/// Color modes for document scanning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanColorMode {
    Color,
    Grayscale,
    Lineart,
}

/// Image export formats for scanned pages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanExportFormat {
    Pdf,
    Jpeg,
    Png,
    Tiff,
}

/// Scanner configuration options (DPI, Geometry, Post-processing)
#[derive(Debug, Clone)]
pub struct SaneScanOptions {
    pub dpi_resolution: u32,
    pub source: ScanSource,
    pub color_mode: ScanColorMode,
    pub brightness: i32, // -100 to +100
    pub contrast: i32,   // -100 to +100
    pub auto_deskew: bool,
    pub auto_rotate: bool,
    pub width_mm: u32,
    pub height_mm: u32,
}

impl SaneScanOptions {
    pub fn default_a4() -> Self {
        SaneScanOptions {
            dpi_resolution: 300,
            source: ScanSource::Flatbed,
            color_mode: ScanColorMode::Color,
            brightness: 0,
            contrast: 0,
            auto_deskew: true,
            auto_rotate: true,
            width_mm: 210,  // A4 width
            height_mm: 297, // A4 height
        }
    }
}

/// Scanner hardware / network protocol representation
#[derive(Debug, Clone)]
pub struct SaneScannerDevice {
    pub device_name: String, // e.g. "genesys:libusb:001:002" or "airscan:escl:EPSON:http://192.168.1.50"
    pub vendor: String,
    pub model: String,
    pub is_network_escl: bool,
    pub supported_resolutions: Vec<u32>,
}

/// Individual scanned document page
#[derive(Debug, Clone)]
pub struct ScannedPage {
    pub page_number: u32,
    pub width_px: u32,
    pub height_px: u32,
    pub pixel_data: Vec<u8>,
    pub ocr_extracted_text: Option<String>,
}

/// Sovereign Simple Scan Engine
pub struct SovereignSimpleScanEngine {
    pub connected_scanners: Vec<SaneScannerDevice>,
    pub selected_scanner_idx: Option<usize>,
    pub current_options: SaneScanOptions,
    pub scanned_pages: Vec<ScannedPage>,
}

impl SovereignSimpleScanEngine {
    pub fn new() -> Self {
        let mut engine = SovereignSimpleScanEngine {
            connected_scanners: Vec::new(),
            selected_scanner_idx: None,
            current_options: SaneScanOptions::default_a4(),
            scanned_pages: Vec::new(),
        };

        // Pre-register default SANE & eSCL drivers
        engine.register_scanner_device(SaneScannerDevice {
            device_name: "airscan:escl:Fujitsu-ScanSnap:http://192.168.1.88".to_string(),
            vendor: "Fujitsu".to_string(),
            model: "ScanSnap iX1600 (AirScan)".to_string(),
            is_network_escl: true,
            supported_resolutions: vec![150, 300, 600, 1200],
        });

        engine.selected_scanner_idx = Some(0);
        engine
    }

    pub fn register_scanner_device(&mut self, dev: SaneScannerDevice) {
        self.connected_scanners.push(dev);
    }

    pub fn acquire_scan_page(&mut self) -> Result<u32, &'static str> {
        let _dev = self
            .selected_scanner_idx
            .and_then(|idx| self.connected_scanners.get(idx))
            .ok_or("No active scanner selected")?;

        let page_num = (self.scanned_pages.len() + 1) as u32;
        let width_px = (self.current_options.width_mm as f32
            * self.current_options.dpi_resolution as f32
            / 25.4) as u32;
        let height_px = (self.current_options.height_mm as f32
            * self.current_options.dpi_resolution as f32
            / 25.4) as u32;

        let total_pixels = (width_px * height_px) as usize;
        let dummy_pixels = vec![255; total_pixels * 3]; // RGB white canvas

        self.scanned_pages.push(ScannedPage {
            page_number: page_num,
            width_px,
            height_px,
            pixel_data: dummy_pixels,
            ocr_extracted_text: None,
        });

        Ok(page_num)
    }

    pub fn perform_ocr_text_extraction(
        &mut self,
        page_index: usize,
    ) -> Result<String, &'static str> {
        let page = self
            .scanned_pages
            .get_mut(page_index)
            .ok_or("Page index out of bounds")?;
        let simulated_ocr = format!(
            "Sovereign OCR Text Extracted from Scanned Page #{}",
            page.page_number
        );
        page.ocr_extracted_text = Some(simulated_ocr.clone());
        Ok(simulated_ocr)
    }

    pub fn reorder_pages(&mut self, from_idx: usize, to_idx: usize) -> Result<(), &'static str> {
        if from_idx >= self.scanned_pages.len() || to_idx >= self.scanned_pages.len() {
            return Err("Page index out of bounds");
        }
        let page = self.scanned_pages.remove(from_idx);
        self.scanned_pages.insert(to_idx, page);

        for (idx, p) in self.scanned_pages.iter_mut().enumerate() {
            p.page_number = (idx + 1) as u32;
        }
        Ok(())
    }

    pub fn export_document_multipage(
        &self,
        format: ScanExportFormat,
    ) -> Result<Vec<u8>, &'static str> {
        if self.scanned_pages.is_empty() {
            return Err("No scanned pages available to export");
        }

        let mut output = Vec::new();
        match format {
            ScanExportFormat::Pdf => {
                output.extend_from_slice(b"%PDF-1.7\n");
                output.extend_from_slice(
                    format!("%%Pages: {}\n", self.scanned_pages.len()).as_bytes(),
                );
                output.extend_from_slice(b"%%EOF\n");
            }
            ScanExportFormat::Jpeg => {
                output.extend_from_slice(&[0xFF, 0xD8, 0xFF, 0xE0]); // JPEG SOI & APP0
            }
            ScanExportFormat::Png => {
                output.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
                // PNG Signature
            }
            ScanExportFormat::Tiff => {
                output.extend_from_slice(b"II*\x00"); // TIFF Little-Endian Header
            }
        }
        Ok(output)
    }
}

impl Default for SovereignSimpleScanEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_scan_engine() {
        let mut scan = SovereignSimpleScanEngine::new();
        assert_eq!(scan.connected_scanners.len(), 1);

        let p1 = scan.acquire_scan_page().unwrap();
        assert_eq!(p1, 1);
        let p2 = scan.acquire_scan_page().unwrap();
        assert_eq!(p2, 2);

        assert!(scan.perform_ocr_text_extraction(0).is_ok());
        assert!(scan.scanned_pages[0].ocr_extracted_text.is_some());

        assert!(scan.reorder_pages(1, 0).is_ok());
        assert_eq!(scan.scanned_pages[0].page_number, 1);

        let pdf = scan
            .export_document_multipage(ScanExportFormat::Pdf)
            .unwrap();
        assert!(pdf.starts_with(b"%PDF-1.7"));
    }
}
