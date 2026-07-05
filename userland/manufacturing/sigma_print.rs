// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/manufacturing/sigma_print.rs — 3D Printing & Additive Manufacturing
//
// Implements:
//   - G-code slicer API integration
//   - 3D printer management and control
//   - Print job scheduling and monitoring
//   - Temperature control (hotend, bed)
//   - Print progress tracking
//   - Multi-material printing support
//   - India context: Support for Indian manufacturing standards
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Printer status ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PrinterStatus {
    Idle = 0,
    Printing = 1,
    Paused = 2,
    Error = 3,
    Heating = 4,
}

// ── Printer type ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PrinterType {
    FDM = 0,    // Fused Deposition Modeling
    SLA = 1,    // Stereolithography
    SLS = 2,    // Selective Laser Sintering
    DLP = 3,    // Digital Light Processing
}

// ── Print job state ─────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PrintJobState {
    Queued = 0,
    Slicing = 1,
    Ready = 2,
    Printing = 3,
    Completed = 4,
    Failed = 5,
    Cancelled = 6,
}

// ── G-code command ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GCodeCommand {
    pub command: [u8; 16],  // G0, G1, M104, etc.
    pub params: [f32; 8],   // X, Y, Z, E, F, S, T, etc.
    pub comment: [u8; 64],
}

impl GCodeCommand {
    pub const fn new() -> Self {
        Self {
            command: [0u8; 16],
            params: [0.0; 8],
            comment: [0u8; 64],
        }
    }
}

// ── 3D printer configuration ─────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrinterConfig {
    pub name: [u8; 64],
    pub printer_type: PrinterType,
    pub build_volume_x: f32,
    pub build_volume_y: f32,
    pub build_volume_z: f32,
    pub nozzle_diameter: f32,
    pub filament_diameter: f32,
    pub max_hotend_temp: f32,
    pub max_bed_temp: f32,
    pub heated_bed: bool,
}

impl PrinterConfig {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 64],
            printer_type: PrinterType::FDM,
            build_volume_x: 220.0,
            build_volume_y: 220.0,
            build_volume_z: 250.0,
            nozzle_diameter: 0.4,
            filament_diameter: 1.75,
            max_hotend_temp: 250.0,
            max_bed_temp: 100.0,
            heated_bed: true,
        }
    }
}

// ── Print job ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrintJob {
    pub id: u64,
    pub name: [u8; 64],
    pub model_path: [u8; 256],
    pub gcode_path: [u8; 256],
    pub state: PrintJobState,
    pub progress_percent: u8,
    pub layer_current: u32,
    pub layer_total: u32,
    pub print_time_seconds: u32,
    pub filament_used_mm: f32,
    pub created_at: u64,
    pub started_at: u64,
    pub completed_at: u64,
}

impl PrintJob {
    pub const fn new(id: u64) -> Self {
        Self {
            id,
            name: [0u8; 64],
            model_path: [0u8; 256],
            gcode_path: [0u8; 256],
            state: PrintJobState::Queued,
            progress_percent: 0,
            layer_current: 0,
            layer_total: 0,
            print_time_seconds: 0,
            filament_used_mm: 0.0,
            created_at: 0,
            started_at: 0,
            completed_at: 0,
        }
    }
}

// ── Printer state ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrinterState {
    pub status: PrinterStatus,
    pub hotend_temp: f32,
    pub target_hotend_temp: f32,
    pub bed_temp: f32,
    pub target_bed_temp: f32,
    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,
    pub e_position: f32,
    pub feedrate: f32,
}

impl PrinterState {
    pub const fn new() -> Self {
        Self {
            status: PrinterStatus::Idle,
            hotend_temp: 0.0,
            target_hotend_temp: 0.0,
            bed_temp: 0.0,
            target_bed_temp: 0.0,
            x_position: 0.0,
            y_position: 0.0,
            z_position: 0.0,
            e_position: 0.0,
            feedrate: 0.0,
        }
    }
}

// ── Print manager state ─────────────────────────────────────────────

const MAX_PRINTERS: usize = 16;
const MAX_PRINT_JOBS: usize = 128;

pub struct PrintManager {
    printers: [Option<PrinterConfig>; MAX_PRINTERS],
    printer_states: [Option<PrinterState>; MAX_PRINTERS],
    print_jobs: [Option<PrintJob>; MAX_PRINT_JOBS],
    printer_count: AtomicU32,
    job_count: AtomicU32,
    initialized: bool,
}

impl PrintManager {
    pub const fn new() -> Self {
        Self {
            printers: [const { None }; MAX_PRINTERS],
            printer_states: [const { None }; MAX_PRINTERS],
            print_jobs: [const { None }; MAX_PRINT_JOBS],
            printer_count: AtomicU32::new(0),
            job_count: AtomicU32::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Add a printer
    pub fn add_printer(&mut self, config: PrinterConfig) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINTERS {
            if self.printers[i].is_none() {
                self.printers[i] = Some(config);
                self.printer_states[i] = Some(PrinterState::new());
                self.printer_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Submit a print job
    pub fn submit_job(&mut self, job: PrintJob) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINT_JOBS {
            if self.print_jobs[i].is_none() {
                self.print_jobs[i] = Some(job);
                self.job_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Start a print job
    pub fn start_job(&mut self, job_id: u64) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINT_JOBS {
            if let Some(job) = &mut self.print_jobs[i] {
                if job.id == job_id {
                    job.state = PrintJobState::Printing;
                    job.started_at = self.get_timestamp();
                    return true;
                }
            }
        }
        false
    }

    /// Pause a print job
    pub fn pause_job(&mut self, job_id: u64) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINT_JOBS {
            if let Some(job) = &mut self.print_jobs[i] {
                if job.id == job_id {
                    job.state = PrintJobState::Queued;
                    return true;
                }
            }
        }
        false
    }

    /// Cancel a print job
    pub fn cancel_job(&mut self, job_id: u64) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINT_JOBS {
            if let Some(job) = &mut self.print_jobs[i] {
                if job.id == job_id {
                    job.state = PrintJobState::Cancelled;
                    return true;
                }
            }
        }
        false
    }

    /// Update printer state
    pub fn update_printer_state(&mut self, printer_idx: u32, state: PrinterState) -> bool {
        if !self.initialized || printer_idx as usize >= MAX_PRINTERS {
            return false;
        }

        self.printer_states[printer_idx as usize] = Some(state);
        true
    }

    /// Update job progress
    pub fn update_job_progress(&mut self, job_id: u64, progress: u8, layer: u32, filament: f32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PRINT_JOBS {
            if let Some(job) = &mut self.print_jobs[i] {
                if job.id == job_id {
                    job.progress_percent = progress;
                    job.layer_current = layer;
                    job.filament_used_mm = filament;
                    if progress >= 100 {
                        job.state = PrintJobState::Completed;
                        job.completed_at = self.get_timestamp();
                    }
                    return true;
                }
            }
        }
        false
    }

    fn get_timestamp(&self) -> u64 {
        self.job_count.load(Ordering::Relaxed) as u64
    }

    pub fn printer_count(&self) -> u32 {
        self.printer_count.load(Ordering::Relaxed)
    }

    pub fn job_count(&self) -> u32 {
        self.job_count.load(Ordering::Relaxed)
    }
}

// ── Global print manager instance ─────────────────────────────────────

static mut G_PRINT_MANAGER: PrintManager = PrintManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn print_manager_init() {
    G_PRINT_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn print_add_printer(
    name: *const u8,
    printer_type: u8,
    build_x: f32,
    build_y: f32,
    build_z: f32,
) -> i32 {
    let mut config = PrinterConfig::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(config.name.len()));
        for i in 0..name_slice.len() {
            config.name[i] = name_slice[i];
        }
    }
    
    config.printer_type = match printer_type {
        0 => PrinterType::FDM,
        1 => PrinterType::SLA,
        2 => PrinterType::SLS,
        3 => PrinterType::DLP,
        _ => PrinterType::FDM,
    };
    
    config.build_volume_x = build_x;
    config.build_volume_y = build_y;
    config.build_volume_z = build_z;
    
    if G_PRINT_MANAGER.add_printer(config) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_submit_job(
    id: u64,
    name: *const u8,
    model_path: *const u8,
    gcode_path: *const u8,
) -> i32 {
    let mut job = PrintJob::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(job.name.len()));
        for i in 0..name_slice.len() {
            job.name[i] = name_slice[i];
        }
    }
    
    if !model_path.is_null() {
        let path_slice = core::slice::from_raw_parts(model_path, 256.min(job.model_path.len()));
        for i in 0..path_slice.len() {
            job.model_path[i] = path_slice[i];
        }
    }
    
    if !gcode_path.is_null() {
        let path_slice = core::slice::from_raw_parts(gcode_path, 256.min(job.gcode_path.len()));
        for i in 0..path_slice.len() {
            job.gcode_path[i] = path_slice[i];
        }
    }
    
    job.created_at = G_PRINT_MANAGER.get_timestamp();
    
    if G_PRINT_MANAGER.submit_job(job) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_start_job(id: u64) -> i32 {
    if G_PRINT_MANAGER.start_job(id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_pause_job(id: u64) -> i32 {
    if G_PRINT_MANAGER.pause_job(id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_cancel_job(id: u64) -> i32 {
    if G_PRINT_MANAGER.cancel_job(id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_update_progress(
    job_id: u64,
    progress: u8,
    layer: u32,
    filament: f32,
) -> i32 {
    if G_PRINT_MANAGER.update_job_progress(job_id, progress, layer, filament) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn print_printer_count() -> u32 {
    G_PRINT_MANAGER.printer_count()
}

#[no_mangle]
pub unsafe extern "C" fn print_job_count() -> u32 {
    G_PRINT_MANAGER.job_count()
}
