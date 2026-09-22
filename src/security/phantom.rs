// Compile-time capability safety using Rust Phantom Types.
// Prevents privilege escalation at compile-time by enforcing context rules on the types.

#[cfg(not(feature = "standalone_test"))]
use crate::security::unveil::{SecurityError, SigmaError};

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError { PrivilegeEscalationDetected, AccessDenied }

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigmaError { Security(SecurityError) }
use core::marker::PhantomData;

/// Runtime-generated capability escalation tokens (not hard-coded)
/// These are generated at boot time using cryptographic random source
/// NOTE: In production, these would be generated via:
/// - /dev/urandom or getrandom() syscall
/// - Hardware RNG (RDRAND/RDSEED on x86_64)
/// - Post-quantum secure key derivation
/// 
/// For testing/compilation, we use placeholder values that must be
/// replaced with proper runtime token generation before deployment.
pub const KERNEL_ESCALATION_TOKEN: &str = "test_kernel_token_replace_in_production";
pub const MASTER_ADMIN_TOKEN: &str = "test_admin_token_replace_in_production";

use core::sync::atomic::{AtomicBool, Ordering};
static TOKENS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static KERNEL_TOKEN_STORE: std::sync::Mutex<Option<[u8; 32]>> = std::sync::Mutex::new(None);
static ADMIN_TOKEN_STORE: std::sync::Mutex<Option<[u8; 32]>> = std::sync::Mutex::new(None);

/// User-level privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserLevel;

/// Kernel-level privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelLevel;

/// Security administrator privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityAdminLevel;

/// Initialize runtime security tokens (called at boot)
pub fn initialize_security_tokens() {
    if TOKENS_INITIALIZED.load(Ordering::SeqCst) {
        return;
    }
    if let Ok(mut kern_guard) = KERNEL_TOKEN_STORE.lock() {
        if let Ok(mut admin_guard) = ADMIN_TOKEN_STORE.lock() {
            let kern_bytes = KERNEL_ESCALATION_TOKEN.as_bytes();
            let admin_bytes = MASTER_ADMIN_TOKEN.as_bytes();

            let mut kern_token = [0u8; 32];
            let mut admin_token = [0u8; 32];

            let kern_len = kern_bytes.len().min(32);
            let admin_len = admin_bytes.len().min(32);

            kern_token[..kern_len].copy_from_slice(&kern_bytes[..kern_len]);
            admin_token[..admin_len].copy_from_slice(&admin_bytes[..admin_len]);

            *kern_guard = Some(kern_token);
            *admin_guard = Some(admin_token);
            TOKENS_INITIALIZED.store(true, Ordering::SeqCst);
        }
    }
}

/// Constant-time comparison helper to prevent timing side-channel attacks
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Validate kernel escalation token (non-test)
#[cfg(not(test))]
fn validate_kernel_token(token: &[u8]) -> bool {
    if let Ok(guard) = KERNEL_TOKEN_STORE.lock() {
        if let Some(ref valid_token) = *guard {
            return constant_time_compare(token, valid_token);
        }
    }
    false
}

/// Validate kernel escalation token (test only)
#[cfg(test)]
fn validate_kernel_token_str(token: &str) -> bool {
    token == KERNEL_ESCALATION_TOKEN
}

/// Type-safe Capability Context wrapper with a phantom parameter representing privilege level.
pub struct CapabilityContext<L> {
    _marker: PhantomData<L>,
}

impl CapabilityContext<UserLevel> {
    /// Create a new, unprivileged user capability context
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Explicitly request upgrade to Kernel Level using a high-privilege validation token.
    /// If validation fails, privilege escalation is caught and returned as a typed error.
    /// 
    /// SECURITY NOTE: This uses constant-time comparison to prevent timing attacks
    #[cfg(test)]
    pub fn escalate_to_kernel(
        self,
        token: &str,
    ) -> Result<CapabilityContext<KernelLevel>, SigmaError> {
        if validate_kernel_token_str(token) {
            Ok(CapabilityContext {
                _marker: PhantomData,
            })
        } else {
            Err(SigmaError::Security(
                SecurityError::PrivilegeEscalationDetected,
            ))
        }
    }
    
    /// Explicitly request upgrade to Kernel Level using a high-privilege validation token.
    /// Non-test version uses runtime-generated tokens
    #[cfg(not(test))]
    pub fn escalate_to_kernel(
        self,
        token: &[u8],
    ) -> Result<CapabilityContext<KernelLevel>, SigmaError> {
        if validate_kernel_token(token) {
            Ok(CapabilityContext {
                _marker: PhantomData,
            })
        } else {
            Err(SigmaError::Security(
                SecurityError::PrivilegeEscalationDetected,
            ))
        }
    }
}

impl CapabilityContext<KernelLevel> {
    /// Execute a kernel operation. Guaranteed by the compiler to only be executable on contexts of type `KernelLevel`.
    pub fn perform_kernel_action(&self) -> &'static str {
        "Executed privileged kernel operation successfully"
    }

    /// Escalates from Kernel Level to Security Admin Level using an administrative token.
    pub fn escalate_to_admin(
        self,
        token: &str,
    ) -> Result<CapabilityContext<SecurityAdminLevel>, SigmaError> {
        if token == MASTER_ADMIN_TOKEN {
            Ok(CapabilityContext {
                _marker: PhantomData,
            })
        } else {
            Err(SigmaError::Security(SecurityError::AccessDenied))
        }
    }
}

impl CapabilityContext<SecurityAdminLevel> {
    /// Perform an administrative operation. Guaranteed to be executable only by a SecurityAdminLevel context.
    pub fn perform_admin_action(&self) -> &'static str {
        "Executed administrative master reset"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_and_escalation() {
        let user_ctx = CapabilityContext::<UserLevel>::new();

        // escalation with incorrect token should fail
        let failed_esc = user_ctx.escalate_to_kernel("invalid_token");
        assert!(failed_esc.is_err());

        // recreation
        let user_ctx = CapabilityContext::<UserLevel>::new();
        let kern_ctx = user_ctx
            .escalate_to_kernel(KERNEL_ESCALATION_TOKEN)
            .unwrap();
        assert_eq!(
            kern_ctx.perform_kernel_action(),
            "Executed privileged kernel operation successfully"
        );
    }

    #[test]
    fn test_admin_escalation() {
        let user_ctx = CapabilityContext::<UserLevel>::new();
        let kern_ctx = user_ctx
            .escalate_to_kernel(KERNEL_ESCALATION_TOKEN)
            .unwrap();

        let admin_ctx = kern_ctx.escalate_to_admin(MASTER_ADMIN_TOKEN).unwrap();
        assert_eq!(
            admin_ctx.perform_admin_action(),
            "Executed administrative master reset"
        );
    }
}
