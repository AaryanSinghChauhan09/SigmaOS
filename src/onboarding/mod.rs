// src/onboarding/mod.rs
// First-boot onboarding for SigmaOS
//
// The onboarding wizard is implemented in Nim for:
// - Rapid UI development
// - Type safety
// - Memory safety
// - Easy integration with the compositor
//
// See: onboarding_wizard.nim

#![no_std]

// Note: Nim module exports are handled by the build system
// The onboarding wizard will be compiled as a standalone binary
// and invoked by the init system on first boot.

/// Check if onboarding has been completed
pub fn is_onboarding_complete() -> bool {
    // In real implementation, would check for marker file
    // e.g., /etc/sigmaos/onboarding-complete
    false
}

/// Mark onboarding as complete
pub fn mark_onboarding_complete() -> Result<(), ()> {
    // In real implementation, would create marker file
    Ok(())
}

/// Launch onboarding wizard
pub fn launch_wizard() -> Result<(), ()> {
    // In real implementation, would spawn onboarding_wizard binary
    Ok(())
}
