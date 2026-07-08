// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/printer/printer_device_base.rs — Base Device Trait for Printer Drivers
//
// Defines the OOP base class for all printer devices using Rust traits.
// This provides a common interface for printer operations with CUPS compatibility.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
 type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Printer Error Codes ─────────────────────────────────────────────

pub const PRINTER_OK: I32 = 0;
pub const PRINTER_ERR_NO_DEVICE: I32 = -1;
pub const PRINTER_ERR_INIT_FAILED: I32 = -2;
pub const PRINTER_ERR_OUT_OF_MEM: I32 = -3;
pub const PRINTER_ERR_NOT_SUPPORTED: I32 = -4;
pub const PRINTER_ERR_IO: I32 = -5;
pub const PRINTER_ERR_TIMEOUT: I32 = -6;
pub const PRINTER_ERR_PAPER_JAM: I32 = -7;
pub const PRINTER_ERR_OUT_OF_PAPER: I32 = -8;
pub const PRINTER_ERR_TONER_LOW: I32 = -9;
pub const PRINTER_ERR_COVER_OPEN: I32 = -10;

// ─── Printer Type ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrinterType {
    USB,
    Parallel,
    Network,
    Serial,
    Unknown,
}

// ─── Printer Status ─────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrinterStatus {
    Idle,
    Printing,
    Busy,
    Offline,
    Error,
}

// ─── Printer Media Type ─────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Plain,
    Photo,
    Transparency,
    Envelope,
    Cardstock,
    Label,
    Unknown,
}

// ─── Print Quality ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintQuality {
    Draft,
    Normal,
    High,
    Photo,
}

// ─── Color Mode ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    Monochrome,
    Grayscale,
    Color,
}

// ─── Printer Capability ─────────────────────────

#[repr(C)]
pub struct PrinterCapability {
    pub max_width: U32,
    pub max_height: U32,
    pub min_width: U32,
    pub min_height: U32,
    pub supports_color: bool,
    pub supports_duplex: bool,
    pub supports_auto_duplex: bool,
    pub max_resolution_dpi: U32,
    pub supported_media_types: [MediaType; 16],
    pub media_type_count: U8,
}

impl PrinterCapability {
    pub const fn new() -> Self {
        PrinterCapability {
            max_width: 827, // A4 width in points
            max_height: 1169, // A4 height in points
            min_width: 72,
            min_height: 72,
            supports_color: true,
            supports_duplex: true,
            supports_auto_duplex: false,
            max_resolution_dpi: 1200,
            supported_media_types: [MediaType::Unknown; 16],
            media_type_count: 0,
        }
    }
}

// ─── Printer Info ─────────────────────────────

#[repr(C)]
pub struct PrinterInfo {
    pub manufacturer: [U8; 64],
    pub model: [U8; 64],
    pub serial_number: [U8; 32],
    pub firmware_version: [U8; 32],
    pub uri: [U8; 128],
}

impl PrinterInfo {
    pub const fn new() -> Self {
        PrinterInfo {
            manufacturer: [0; 64],
            model: [0; 64],
            serial_number: [0; 32],
            firmware_version: [0; 32],
            uri: [0; 128],
        }
    }
}

// ─── Print Job ─────────────────────────────────

#[repr(C)]
pub struct PrintJob {
    pub job_id: U32,
    pub title: [U8; 128],
    pub user: [U8; 64],
    pub copies: U32,
    pub priority: U32,
    pub pages: U32,
    pub completed_pages: U32,
    pub status: PrinterStatus,
    pub color_mode: ColorMode,
    pub print_quality: PrintQuality,
    pub media_type: MediaType,
    pub duplex: bool,
}

impl PrintJob {
    pub const fn new() -> Self {
        PrintJob {
            job_id: 0,
            title: [0; 128],
            user: [0; 64],
            copies: 1,
            priority: 50,
            pages: 0,
            completed_pages: 0,
            status: PrinterStatus::Idle,
            color_mode: ColorMode::Color,
            print_quality: PrintQuality::Normal,
            media_type: MediaType::Plain,
            duplex: false,
        }
    }
}

// ─── Printer Device Trait ─────────────────────────

/// Trait for printer device operations
pub trait PrinterDevice {
    /// Initialize the printer device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Get printer type
    fn get_printer_type(&self) -> PrinterType;
    
    /// Get printer info
    fn get_printer_info(&self, info: *mut PrinterInfo) -> I32;
    
    /// Get capabilities
    fn get_capabilities(&self, caps: *mut PrinterCapability) -> I32;
    
    /// Get printer status
    fn get_status(&self) -> PrinterStatus;
    
    /// Submit print job
    fn submit_job(&mut self, job: *mut PrintJob, data: *const U8, length: U32) -> I32;
    
    /// Cancel job
    fn cancel_job(&mut self, job_id: U32) -> I32;
    
    /// Get job status
    fn get_job_status(&self, job_id: U32, job: *mut PrintJob) -> I32;
    
    /// List jobs
    fn list_jobs(&self, jobs: *mut PrintJob, count: *mut U32) -> I32;
    
    /// Set print quality
    fn set_print_quality(&mut self, quality: PrintQuality) -> I32;
    
    /// Set color mode
    fn set_color_mode(&mut self, mode: ColorMode) -> I32;
    
    /// Set media type
    fn set_media_type(&mut self, media: MediaType) -> I32;
    
    /// Set duplex mode
    fn set_duplex(&mut self, duplex: bool) -> I32;
    
    /// Eject page
    fn eject_page(&mut self) -> I32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Printer Statistics ─────────────────────────

#[repr(C)]
pub struct PrinterStats {
    pub total_jobs: U32,
    pub total_pages: U32,
    pub total_bytes: U64,
    pub failed_jobs: U32,
    pub paper_jams: U32,
    pub uptime_seconds: U64,
}

impl PrinterStats {
    pub const fn new() -> Self {
        PrinterStats {
            total_jobs: 0,
            total_pages: 0,
            total_bytes: 0,
            failed_jobs: 0,
            paper_jams: 0,
            uptime_seconds: 0,
        }
    }
}
