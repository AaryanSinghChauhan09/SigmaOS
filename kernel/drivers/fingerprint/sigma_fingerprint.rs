// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/drivers/fingerprint/sigma_fingerprint.rs — Fingerprint Sensor Driver
// Implements: Fingerprint sensor support, secure enrollment, and authentication.
// Inspired by Linux libfprint, fprintd, and Windows Hello biometric framework.
//
// Reference: Linux drivers/input/misc/uinput.c, libfprint (LGPL-2.1)

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Fingerprint Constants ─────────────────────────────────────────────────────
const MAX_FINGERPRINTS: usize = 10;
const FINGERPRINT_TEMPLATE_SIZE: usize = 512;
const MAX_ENROLLMENT_ATTEMPTS: u8 = 5;

// ── Fingerprint Sensor Types ───────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FingerprintSensorType {
    Swipe = 0,       // Swipe-type sensor (older)
    Touch = 1,        // Touch-type sensor (modern)
    Optical = 2,      // Optical sensor
    Capacitive = 3,   // Capacitive sensor
    Ultrasonic = 4,   // Ultrasonic sensor (Qualcomm Sense ID)
}

// ── Fingerprint Match Quality ─────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MatchQuality {
    Excellent = 0,    // Perfect match
    Good = 1,         // Good match
    Fair = 2,         // Fair match
    Poor = 3,         // Poor match
    Failed = 4,       // No match
}

// ── Fingerprint Enrollment State ─────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EnrollmentState {
    Idle = 0,
    Enrolling = 1,
    Complete = 2,
    Failed = 3,
}

// ── Fingerprint Template ─────────────────────────────────────────────────────
#[repr(C)]
pub struct FingerprintTemplate {
    pub finger_id: u8,
    pub quality: MatchQuality,
    pub template_data: [u8; FINGERPRINT_TEMPLATE_SIZE],
    pub template_size: usize,
    pub enrolled: bool,
}

impl FingerprintTemplate {
    pub const fn new(finger_id: u8) -> Self {
        Self {
            finger_id,
            quality: MatchQuality::Failed,
            template_data: [0u8; FINGERPRINT_TEMPLATE_SIZE],
            template_size: 0,
            enrolled: false,
        }
    }
}

// ── Fingerprint Sensor ───────────────────────────────────────────────────────
#[repr(C)]
pub struct FingerprintSensor {
    pub sensor_type: FingerprintSensorType,
    pub vendor_id: u16,
    pub product_id: u16,
    pub width: u16,        // Sensor width in pixels
    pub height: u16,       // Sensor height in pixels
    pub dpi: u16,          // Resolution in DPI
    pub initialized: bool,
    pub secure_enrollment: bool, // TPM-backed enrollment
}

impl FingerprintSensor {
    pub const fn new(sensor_type: FingerprintSensorType) -> Self {
        Self {
            sensor_type,
            vendor_id: 0,
            product_id: 0,
            width: 0,
            height: 0,
            dpi: 0,
            initialized: false,
            secure_enrollment: true,
        }
    }
}

// ── Fingerprint Manager ─────────────────────────────────────────────────────
pub struct FingerprintManager {
    pub sensor: FingerprintSensor,
    pub templates: [FingerprintTemplate; MAX_FINGERPRINTS],
    pub template_count: usize,
    pub enrollment_state: EnrollmentState,
    pub current_enrollment_finger: u8,
    pub enrollment_attempts: u8,
    pub device_locked: bool,
}

impl FingerprintManager {
    pub const fn new(sensor_type: FingerprintSensorType) -> Self {
        Self {
            sensor: FingerprintSensor::new(sensor_type),
            templates: [
                FingerprintTemplate::new(0),
                FingerprintTemplate::new(1),
                FingerprintTemplate::new(2),
                FingerprintTemplate::new(3),
                FingerprintTemplate::new(4),
                FingerprintTemplate::new(5),
                FingerprintTemplate::new(6),
                FingerprintTemplate::new(7),
                FingerprintTemplate::new(8),
                FingerprintTemplate::new(9),
            ],
            template_count: 0,
            enrollment_state: EnrollmentState::Idle,
            current_enrollment_finger: 0,
            enrollment_attempts: 0,
            device_locked: false,
        }
    }

    /// Initialize fingerprint sensor (inspired by Linux fprint_init)
    pub unsafe fn init(&mut self, vendor_id: u16, product_id: u16) -> i32 {
        self.sensor.vendor_id = vendor_id;
        self.sensor.product_id = product_id;

        // Set default sensor parameters based on type
        match self.sensor.sensor_type {
            FingerprintSensorType::Touch | FingerprintSensorType::Ultrasonic => {
                self.sensor.width = 160;
                self.sensor.height = 160;
                self.sensor.dpi = 500;
            }
            FingerprintSensorType::Swipe => {
                self.sensor.width = 144;
                self.sensor.height = 512;
                self.sensor.dpi = 500;
            }
            _ => {
                self.sensor.width = 128;
                self.sensor.height = 128;
                self.sensor.dpi = 350;
            }
        }

        self.sensor.initialized = true;
        0
    }

    /// Start enrollment process (inspired by fprintd enroll)
    pub unsafe fn start_enrollment(&mut self, finger_id: u8) -> i32 {
        if !self.sensor.initialized {
            return -1;
        }

        if finger_id >= MAX_FINGERPRINTS as u8 {
            return -2;
        }

        if self.enrollment_state != EnrollmentState::Idle {
            return -3;
        }

        // Check if finger already enrolled
        if self.templates[finger_id as usize].enrolled {
            return -4;
        }

        self.enrollment_state = EnrollmentState::Enrolling;
        self.current_enrollment_finger = finger_id;
        self.enrollment_attempts = 0;

        0
    }

    /// Cancel enrollment process
    pub unsafe fn cancel_enrollment(&mut self) -> i32 {
        if self.enrollment_state != EnrollmentState::Enrolling {
            return -1;
        }

        self.enrollment_state = EnrollmentState::Idle;
        self.current_enrollment_finger = 0;
        self.enrollment_attempts = 0;

        0
    }

    /// Capture fingerprint during enrollment (inspired by libfprint capture)
    pub unsafe fn capture_enrollment(&mut self, scan_data: *const u8, data_len: usize) -> i32 {
        if self.enrollment_state != EnrollmentState::Enrolling {
            return -1;
        }

        if scan_data.is_null() || data_len == 0 {
            return -2;
        }

        if self.enrollment_attempts >= MAX_ENROLLMENT_ATTEMPTS {
            self.enrollment_state = EnrollmentState::Failed;
            return -3;
        }

        // Simulate template extraction (in production: use actual fingerprint processing)
        let template = &mut self.templates[self.current_enrollment_finger as usize];
        let src = core::slice::from_raw_parts(scan_data, data_len.min(FINGERPRINT_TEMPLATE_SIZE));

        let mut i = 0;
        while i < data_len && i < FINGERPRINT_TEMPLATE_SIZE {
            template.template_data[i] = src[i];
            i += 1;
        }

        template.template_size = data_len;
        template.finger_id = self.current_enrollment_finger;
        template.quality = MatchQuality::Good;
        template.enrolled = true;

        self.enrollment_attempts += 1;
        self.template_count += 1;
        self.enrollment_state = EnrollmentState::Complete;

        0
    }

    /// Verify fingerprint against enrolled templates (inspired by fprintd verify)
    pub unsafe fn verify(&mut self, scan_data: *const u8, data_len: usize) -> (MatchQuality, u8) {
        if !self.sensor.initialized {
            return (MatchQuality::Failed, 255);
        }

        if scan_data.is_null() || data_len == 0 {
            return (MatchQuality::Failed, 255);
        }

        if self.template_count == 0 {
            return (MatchQuality::Failed, 255);
        }

        let src = core::slice::from_raw_parts(scan_data, data_len.min(FINGERPRINT_TEMPLATE_SIZE));

        // Check against all enrolled templates
        for i in 0..self.template_count {
            let template = &self.templates[i];
            if !template.enrolled {
                continue;
            }

            // Simplified matching (in production: use actual fingerprint matching algorithm)
            let mut match_count = 0;
            let compare_len = data_len.min(template.template_size).min(FINGERPRINT_TEMPLATE_SIZE);

            let mut j = 0;
            while j < compare_len {
                if src[j] == template.template_data[j] {
                    match_count += 1;
                }
                j += 1;
            }

            let match_ratio = (match_count * 100) / compare_len;

            let quality = if match_ratio > 90 {
                MatchQuality::Excellent
            } else if match_ratio > 75 {
                MatchQuality::Good
            } else if match_ratio > 50 {
                MatchQuality::Fair
            } else if match_ratio > 25 {
                MatchQuality::Poor
            } else {
                MatchQuality::Failed
            };

            if quality != MatchQuality::Failed {
                return (quality, template.finger_id);
            }
        }

        (MatchQuality::Failed, 255)
    }

    /// Delete enrolled fingerprint
    pub unsafe fn delete_fingerprint(&mut self, finger_id: u8) -> i32 {
        if finger_id >= MAX_FINGERPRINTS as u8 {
            return -1;
        }

        if !self.templates[finger_id as usize].enrolled {
            return -2;
        }

        // Clear template data
        let template = &mut self.templates[finger_id as usize];
        template.enrolled = false;
        template.template_size = 0;
        template.quality = MatchQuality::Failed;

        let mut i = 0;
        while i < FINGERPRINT_TEMPLATE_SIZE {
            template.template_data[i] = 0;
            i += 1;
        }

        self.template_count -= 1;
        0
    }

    /// Delete all enrolled fingerprints
    pub unsafe fn delete_all(&mut self) -> i32 {
        for i in 0..MAX_FINGERPRINTS {
            if self.templates[i].enrolled {
                self.delete_fingerprint(i as u8);
            }
        }
        0
    }

    /// Get enrollment state
    pub unsafe fn get_enrollment_state(&self) -> EnrollmentState {
        self.enrollment_state
    }

    /// Get template count
    pub unsafe fn get_template_count(&self) -> usize {
        self.template_count
    }

    /// Check if device is locked
    pub unsafe fn is_locked(&self) -> bool {
        self.device_locked
    }

    /// Lock device (require fingerprint to unlock)
    pub unsafe fn lock_device(&mut self) -> i32 {
        if self.template_count == 0 {
            return -1; // Cannot lock without enrolled fingerprints
        }

        self.device_locked = true;
        0
    }

    /// Unlock device with fingerprint
    pub unsafe fn unlock_device(&mut self, scan_data: *const u8, data_len: usize) -> i32 {
        if !self.device_locked {
            return -1; // Already unlocked
        }

        let (quality, _finger_id) = self.verify(scan_data, data_len);

        if quality != MatchQuality::Failed {
            self.device_locked = false;
            0
        } else {
            -2
        }
    }

    /// Set secure enrollment mode (TPM-backed)
    pub unsafe fn set_secure_enrollment(&mut self, secure: bool) {
        self.sensor.secure_enrollment = secure;
    }

    /// Get sensor info
    pub unsafe fn get_sensor_info(&self) -> (FingerprintSensorType, u16, u16, u16, u16) {
        (
            self.sensor.sensor_type,
            self.sensor.width,
            self.sensor.height,
            self.sensor.dpi,
            self.sensor.vendor_id,
        )
    }
}

static mut FPRINT_MANAGER: FingerprintManager = FingerprintManager::new(FingerprintSensorType::Touch);

// ── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_init(vendor_id: u16, product_id: u16) -> i32 {
    FPRINT_MANAGER.init(vendor_id, product_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_start_enrollment(finger_id: u8) -> i32 {
    FPRINT_MANAGER.start_enrollment(finger_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_cancel_enrollment() -> i32 {
    FPRINT_MANAGER.cancel_enrollment()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_capture_enrollment(scan_data: *const u8, data_len: usize) -> i32 {
    FPRINT_MANAGER.capture_enrollment(scan_data, data_len)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_verify(scan_data: *const u8, data_len: usize, quality: *mut u8, finger_id: *mut u8) -> i32 {
    if quality.is_null() || finger_id.is_null() {
        return -1;
    }

    let (match_quality, matched_finger) = FPRINT_MANAGER.verify(scan_data, data_len);
    *quality = match_quality as u8;
    *finger_id = matched_finger;

    if match_quality != MatchQuality::Failed { 0 } else { -2 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_delete(finger_id: u8) -> i32 {
    FPRINT_MANAGER.delete_fingerprint(finger_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_delete_all() -> i32 {
    FPRINT_MANAGER.delete_all()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_get_template_count() -> usize {
    FPRINT_MANAGER.get_template_count()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_get_enrollment_state() -> u8 {
    FPRINT_MANAGER.get_enrollment_state() as u8
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_lock_device() -> i32 {
    FPRINT_MANAGER.lock_device()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_unlock_device(scan_data: *const u8, data_len: usize) -> i32 {
    FPRINT_MANAGER.unlock_device(scan_data, data_len)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_is_locked() -> i32 {
    if FPRINT_MANAGER.is_locked() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_set_secure_enrollment(secure: i32) {
    FPRINT_MANAGER.set_secure_enrollment(secure != 0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fingerprint_get_sensor_info(sensor_type: *mut u8, width: *mut u16, height: *mut u16, dpi: *mut u16) -> i32 {
    if sensor_type.is_null() || width.is_null() || height.is_null() || dpi.is_null() {
        return -1;
    }

    let (s_type, w, h, d, _vendor) = FPRINT_MANAGER.get_sensor_info();
    *sensor_type = s_type as u8;
    *width = w;
    *height = h;
    *dpi = d;

    0
}
