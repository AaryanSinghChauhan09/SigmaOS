// Compile-time capability safety using Rust Phantom Types.
// Prevents privilege escalation at compile-time by enforcing context rules on the types.

use crate::security::unveil::{SecurityError, SigmaError};
use core::marker::PhantomData;

/// User-level privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserLevel;

/// Kernel-level privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelLevel;

/// Security administrator privilege marker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityAdminLevel;

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
    pub fn escalate_to_kernel(
        self,
        token: &str,
    ) -> Result<CapabilityContext<KernelLevel>, SigmaError> {
        if token == "SUPER_SECRET_KERN_TOKEN" {
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
        if token == "MASTER_ADMIN_TOKEN" {
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
            .escalate_to_kernel("SUPER_SECRET_KERN_TOKEN")
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
            .escalate_to_kernel("SUPER_SECRET_KERN_TOKEN")
            .unwrap();

        let admin_ctx = kern_ctx.escalate_to_admin("MASTER_ADMIN_TOKEN").unwrap();
        assert_eq!(
            admin_ctx.perform_admin_action(),
            "Executed administrative master reset"
        );
    }
}
