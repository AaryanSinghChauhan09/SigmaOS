// SPDX-License-Identifier: MIT OR GPL-2.0
//! Sovereign Publication-Inspired Permission Subsystem for SigmaOS
//!
//! Inspired by leading tech and security publications (ItsFOSS, HowToGeek, XDA,
//! PCWorld, LinuxFoundation, DistroWatch, Phoronix, WindowsLatest, TechPowerUp, etc.):
//!
//! - **Portal Scopes (ItsFOSS / XDA)**: Fine-grained userland portal access controls for
//!   Camera, Microphone, Precise/Coarse Location, FilePicker, ScreenCapture, and Notifications.
//! - **Temporal Session Grants (HowToGeek / PCWorld)**: Dynamic time-bound permissions with
//!   automatic TTL expiration and one-time single-use authorization gates.
//! - **Hardware Peripheral Bounds (TechPowerUp / Phoronix)**: Granular capability limits on
//!   USB raw endpoints, Bluetooth GATT, GPU compute queues, eBPF programs, and Kernel module loading.
//! - **Enterprise Access Matrix (LinuxFoundation / DistroWatch)**: Process/App-based ACL matrix
//!   supporting Default-Deny, Explicit-Allow, One-Time-Prompt, and Sandboxed-Restricted policies.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// High-level Portal Scopes inspired by modern sandboxed Linux desktops & mobile platforms (ItsFOSS / XDA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortalPermissionScope {
    CameraAccess,
    MicrophoneAccess,
    PreciseLocation,
    CoarseLocation,
    FilePickerRead,
    FilePickerWrite,
    ScreenCapture,
    NotificationSend,
    BluetoothHidPassThrough,
    UsbRawAccess,
    EbpfProgramAttach,
    GpuComputeContext,
    KernelModuleLoad,
}

/// Authorization state for a given permission scope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionGrantState {
    /// Explicitly allowed unconditionally
    Allowed,
    /// Explicitly denied / blocked
    Denied,
    /// Temporal session grant with expiration timestamp (monotonic seconds)
    TemporalGranted { expires_at_sec: u64 },
    /// Single-use one-time authorization that auto-consumes on read
    OneTimeUse { consumed: bool },
    /// Requires dynamic user interactive prompt
    PromptUser,
    /// Restricted to sandboxed read-only fallback mode
    SandboxedRestricted,
}

/// Policy rule action in access matrix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixPolicyAction {
    DefaultDeny,
    AllowAll,
    SandboxIsolated,
    CustomScoped,
}

/// A specific app/process permission record
#[derive(Debug, Clone)]
pub struct AppPermissionRecord {
    pub app_id: String,
    pub scope: PortalPermissionScope,
    pub grant_state: PermissionGrantState,
    pub granted_at_sec: u64,
}

/// Hardware Peripheral Bounds configuration (TechPowerUp / Phoronix)
#[derive(Debug, Clone)]
pub struct HardwarePeripheralBounds {
    pub allow_usb_vendor_ids: Vec<u16>,
    pub max_gpu_compute_memory_mb: u32,
    pub allow_ebpf_program_types: Vec<u8>,
    pub allow_kernel_module_signatures_only: bool,
    pub bluetooth_gatt_read_only: bool,
}

impl Default for HardwarePeripheralBounds {
    fn default() -> Self {
        Self {
            allow_usb_vendor_ids: Vec::new(),
            max_gpu_compute_memory_mb: 512,
            allow_ebpf_program_types: Vec::new(),
            allow_kernel_module_signatures_only: true,
            bluetooth_gatt_read_only: true,
        }
    }
}

/// Fine-grained Access Control Matrix (LinuxFoundation / DistroWatch)
#[derive(Debug, Clone)]
pub struct FineGrainedAccessControlMatrix {
    pub default_action: MatrixPolicyAction,
    pub records: Vec<AppPermissionRecord>,
    pub hardware_bounds: HardwarePeripheralBounds,
}

impl FineGrainedAccessControlMatrix {
    pub fn new(default_action: MatrixPolicyAction) -> Self {
        Self {
            default_action,
            records: Vec::new(),
            hardware_bounds: HardwarePeripheralBounds::default(),
        }
    }

    /// Set permission for app and scope
    pub fn set_grant(&mut self, app_id: &str, scope: PortalPermissionScope, state: PermissionGrantState, current_time_sec: u64) {
        // Update existing record if found
        for record in &mut self.records {
            if record.app_id == app_id && record.scope == scope {
                record.grant_state = state;
                record.granted_at_sec = current_time_sec;
                return;
            }
        }
        // Insert new record
        self.records.push(AppPermissionRecord {
            app_id: String::from(app_id),
            scope,
            grant_state: state,
            granted_at_sec: current_time_sec,
        });
    }

    /// Evaluate permission grant state for app and scope
    pub fn evaluate_permission(&mut self, app_id: &str, scope: PortalPermissionScope, current_time_sec: u64) -> PermissionGrantState {
        for record in &mut self.records {
            if record.app_id == app_id && record.scope == scope {
                match record.grant_state {
                    PermissionGrantState::Allowed => return PermissionGrantState::Allowed,
                    PermissionGrantState::Denied => return PermissionGrantState::Denied,
                    PermissionGrantState::TemporalGranted { expires_at_sec } => {
                        if current_time_sec <= expires_at_sec {
                            return PermissionGrantState::TemporalGranted { expires_at_sec };
                        } else {
                            // Expired temporal grant drops back to Denied
                            record.grant_state = PermissionGrantState::Denied;
                            return PermissionGrantState::Denied;
                        }
                    }
                    PermissionGrantState::OneTimeUse { consumed } => {
                        if !consumed {
                            // Consume the single-use token
                            record.grant_state = PermissionGrantState::OneTimeUse { consumed: true };
                            return PermissionGrantState::Allowed;
                        } else {
                            return PermissionGrantState::Denied;
                        }
                    }
                    PermissionGrantState::PromptUser => return PermissionGrantState::PromptUser,
                    PermissionGrantState::SandboxedRestricted => return PermissionGrantState::SandboxedRestricted,
                }
            }
        }

        // Apply matrix default fallback policy
        match self.default_action {
            MatrixPolicyAction::AllowAll => PermissionGrantState::Allowed,
            MatrixPolicyAction::DefaultDeny => PermissionGrantState::Denied,
            MatrixPolicyAction::SandboxIsolated => PermissionGrantState::SandboxedRestricted,
            MatrixPolicyAction::CustomScoped => PermissionGrantState::PromptUser,
        }
    }

    /// Prune expired temporal grants from the matrix
    pub fn prune_expired_grants(&mut self, current_time_sec: u64) -> usize {
        let mut pruned = 0;
        for record in &mut self.records {
            if let PermissionGrantState::TemporalGranted { expires_at_sec } = record.grant_state {
                if current_time_sec > expires_at_sec {
                    record.grant_state = PermissionGrantState::Denied;
                    pruned += 1;
                }
            }
        }
        pruned
    }
}

/// Sovereign Publication-Inspired Permission Orchestrator Engine
#[derive(Debug)]
pub struct SovereignPublicationInspiredPermissionEngine {
    pub acl_matrix: FineGrainedAccessControlMatrix,
    pub total_evaluations: u64,
    pub total_blocks: u64,
    pub total_grants: u64,
}

impl SovereignPublicationInspiredPermissionEngine {
    pub fn new() -> Self {
        Self {
            acl_matrix: FineGrainedAccessControlMatrix::new(MatrixPolicyAction::DefaultDeny),
            total_evaluations: 0,
            total_blocks: 0,
            total_grants: 0,
        }
    }

    /// Authorize a portal scope request with audit metrics tracking
    pub fn check_portal_access(&mut self, app_id: &str, scope: PortalPermissionScope, current_time_sec: u64) -> bool {
        self.total_evaluations += 1;
        let grant = self.acl_matrix.evaluate_permission(app_id, scope, current_time_sec);

        match grant {
            PermissionGrantState::Allowed
            | PermissionGrantState::TemporalGranted { .. }
            | PermissionGrantState::OneTimeUse { .. } => {
                self.total_grants += 1;
                true
            }
            PermissionGrantState::SandboxedRestricted => {
                // Restricted access permitted under sandbox boundaries
                self.total_grants += 1;
                true
            }
            _ => {
                self.total_blocks += 1;
                false
            }
        }
    }

    /// Grant temporary permission for specified duration (seconds)
    pub fn grant_temporal_permission(&mut self, app_id: &str, scope: PortalPermissionScope, duration_sec: u64, current_time_sec: u64) {
        let expires_at_sec = current_time_sec.saturating_add(duration_sec);
        self.acl_matrix.set_grant(app_id, scope, PermissionGrantState::TemporalGranted { expires_at_sec }, current_time_sec);
    }

    /// Verify USB hardware peripheral bounds (TechPowerUp / Phoronix)
    pub fn check_usb_device_permission(&self, vendor_id: u16) -> bool {
        if self.acl_matrix.hardware_bounds.allow_usb_vendor_ids.is_empty() {
            // No restriction list defined -> default allow
            return true;
        }
        self.acl_matrix.hardware_bounds.allow_usb_vendor_ids.contains(&vendor_id)
    }

    /// Verify GPU memory usage against hardware limits
    pub fn check_gpu_memory_limit(&self, requested_mb: u32) -> bool {
        requested_mb <= self.acl_matrix.hardware_bounds.max_gpu_compute_memory_mb
    }
}

impl Default for SovereignPublicationInspiredPermissionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_permission_evaluation() {
        let mut engine = SovereignPublicationInspiredPermissionEngine::new();
        let app = "org.itsfoss.camera_app";

        // Default action is DefaultDeny -> false
        assert!(!engine.check_portal_access(app, PortalPermissionScope::CameraAccess, 1000));

        // Explicitly allow camera access
        engine.acl_matrix.set_grant(app, PortalPermissionScope::CameraAccess, PermissionGrantState::Allowed, 1000);
        assert!(engine.check_portal_access(app, PortalPermissionScope::CameraAccess, 1000));
    }

    #[test]
    fn test_temporal_session_grant_expiration() {
        let mut engine = SovereignPublicationInspiredPermissionEngine::new();
        let app = "com.howtogeek.screen_recorder";

        // Grant 60 seconds starting at timestamp 500
        engine.grant_temporal_permission(app, PortalPermissionScope::ScreenCapture, 60, 500);

        // Valid at time 530
        assert!(engine.check_portal_access(app, PortalPermissionScope::ScreenCapture, 530));

        // Expired at time 561
        assert!(!engine.check_portal_access(app, PortalPermissionScope::ScreenCapture, 561));
    }

    #[test]
    fn test_one_time_use_permission() {
        let mut engine = SovereignPublicationInspiredPermissionEngine::new();
        let app = "net.xda.file_manager";

        engine.acl_matrix.set_grant(
            app,
            PortalPermissionScope::FilePickerRead,
            PermissionGrantState::OneTimeUse { consumed: false },
            100,
        );

        // First attempt consumes token and returns true
        assert!(engine.check_portal_access(app, PortalPermissionScope::FilePickerRead, 100));

        // Second attempt is denied because token is consumed
        assert!(!engine.check_portal_access(app, PortalPermissionScope::FilePickerRead, 101));
    }

    #[test]
    fn test_hardware_peripheral_bounds() {
        let mut engine = SovereignPublicationInspiredPermissionEngine::new();
        engine.acl_matrix.hardware_bounds.allow_usb_vendor_ids.push(0x1234);
        engine.acl_matrix.hardware_bounds.max_gpu_compute_memory_mb = 1024;

        assert!(engine.check_usb_device_permission(0x1234));
        assert!(!engine.check_usb_device_permission(0x9999));

        assert!(engine.check_gpu_memory_limit(512));
        assert!(!engine.check_gpu_memory_limit(2048));
    }

    #[test]
    fn test_prune_expired_grants_and_policy_matrix_defaults() {
        let mut engine = SovereignPublicationInspiredPermissionEngine::new();
        let app = "org.distrowatch.scanner";

        // Test DefaultDeny policy default
        assert_eq!(
            engine.acl_matrix.evaluate_permission(app, PortalPermissionScope::PreciseLocation, 100),
            PermissionGrantState::Denied
        );

        // Grant two temporal permissions
        engine.grant_temporal_permission(app, PortalPermissionScope::PreciseLocation, 50, 100);
        engine.grant_temporal_permission(app, PortalPermissionScope::CoarseLocation, 200, 100);

        // Prune at t = 160 -> first grant expires (100 + 50 = 150 < 160)
        let pruned = engine.acl_matrix.prune_expired_grants(160);
        assert_eq!(pruned, 1);

        assert!(!engine.check_portal_access(app, PortalPermissionScope::PreciseLocation, 160));
        assert!(engine.check_portal_access(app, PortalPermissionScope::CoarseLocation, 160));
    }
}
