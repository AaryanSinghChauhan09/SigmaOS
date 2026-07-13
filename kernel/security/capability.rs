//! SigmaOS Capability-Based Security System
//! Phase 13.1: Implement capability-based security model
//! Provides fine-grained access control without traditional permissions

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

extern "C" {
    /// External system timer to get current monotonic time in milliseconds
    fn sigma_timer_get_ms() -> SigmaU64;
    
    /// External cryptographic primitive (e.g., from Sovereign Crypto Shard)
    fn sigma_crypto_sign_ed25519(
        data: *const u8,
        data_len: usize,
        signature_out: *mut u8,
    ) -> SigmaI32;
}

/// Capability token (Task 13.1.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapabilityToken {
    pub id: SigmaU64,
    pub permissions: SigmaU64,
    pub resource: SigmaU64,
    pub expiry: SigmaU64,
    pub signature: [SigmaU8; 64],
}

impl CapabilityToken {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            permissions: 0,
            resource: 0,
            expiry: 0,
            signature: [0; 64],
        }
    }
}

/// Capability manager (Task 13.1.1)
#[repr(C)]
pub struct CapabilityManager {
    pub tokens: [CapabilityToken; 1024],
    pub token_count: SigmaU32,
    pub next_id: SigmaU64,
    pub initialized: SigmaBool,
}

static mut CAPABILITY_MANAGER: Option<CapabilityManager> = None;

/// Initialize capability manager
#[no_mangle]
pub unsafe extern "C" fn capability_init() -> SigmaI32 {
    CAPABILITY_MANAGER = Some(CapabilityManager {
        tokens: [CapabilityToken::empty(); 1024],
        token_count: 0,
        next_id: 1,
        initialized: false,
    });

    if let Some(ref mut manager) = CAPABILITY_MANAGER {
        manager.initialized = true;
        return 0;
    }

    -1
}

/// Create a new capability token
#[no_mangle]
pub unsafe extern "C" fn capability_create(
    permissions: SigmaU64,
    resource: SigmaU64,
    expiry: SigmaU64,
) -> SigmaU64 {
    if CAPABILITY_MANAGER.is_none() {
        return 0;
    }

    if let Some(ref mut manager) = CAPABILITY_MANAGER {
        if manager.token_count >= 1024 {
            return 0;
        }

        let idx = manager.token_count as usize;
        let token = &mut manager.tokens[idx];
        
        token.id = manager.next_id;
        token.permissions = permissions;
        token.resource = resource;
        token.expiry = expiry;
        
        // Use external crypto primitive to sign the token contents
        // In a real implementation, we would serialize the first 4 fields
        // For this implementation phase, we mock the serialization
        unsafe {
            let data_to_sign = [0u8; 32];
            if sigma_crypto_sign_ed25519(
                data_to_sign.as_ptr(),
                data_to_sign.len(),
                token.signature.as_mut_ptr(),
            ) != 0 {
                // Cryptographic failure, fallback to zeroing signature
                for i in 0..64 {
                    token.signature[i] = 0;
                }
            }
        }

        let id = manager.next_id;
        manager.next_id += 1;
        manager.token_count += 1;
        
        id
    } else {
        0
    }
}

/// Validate a capability token
#[no_mangle]
pub unsafe extern "C" fn capability_validate(token_id: SigmaU64) -> SigmaBool {
    if CAPABILITY_MANAGER.is_none() || token_id == 0 {
        return false;
    }

    if let Some(ref manager) = CAPABILITY_MANAGER {
        for i in 0..manager.token_count as usize {
            if manager.tokens[i].id == token_id {
                // Check expiry against active system timer
                let now = unsafe { sigma_timer_get_ms() };
                if manager.tokens[i].expiry > 0 && manager.tokens[i].expiry < now {
                    return false; // Capability token has expired
                }
                return true;
            }
        }
    }

    false
}

/// Check if capability has specific permission
#[no_mangle]
pub unsafe extern "C" fn capability_check_permission(
    token_id: SigmaU64,
    permission: SigmaU64,
) -> SigmaBool {
    if CAPABILITY_MANAGER.is_none() || token_id == 0 {
        return false;
    }

    if let Some(ref manager) = CAPABILITY_MANAGER {
        for i in 0..manager.token_count as usize {
            if manager.tokens[i].id == token_id {
                return (manager.tokens[i].permissions & permission) == permission;
            }
        }
    }

    false
}

/// Check if capability has access to specific resource
#[no_mangle]
pub unsafe extern "C" fn capability_check_resource(
    token_id: SigmaU64,
    resource: SigmaU64,
) -> SigmaBool {
    if CAPABILITY_MANAGER.is_none() || token_id == 0 {
        return false;
    }

    if let Some(ref manager) = CAPABILITY_MANAGER {
        for i in 0..manager.token_count as usize {
            if manager.tokens[i].id == token_id {
                return manager.tokens[i].resource == resource;
            }
        }
    }

    false
}

/// Revoke a capability token
#[no_mangle]
pub unsafe extern "C" fn capability_revoke(token_id: SigmaU64) -> SigmaI32 {
    if CAPABILITY_MANAGER.is_none() || token_id == 0 {
        return -1;
    }

    if let Some(ref mut manager) = CAPABILITY_MANAGER {
        for i in 0..manager.token_count as usize {
            if manager.tokens[i].id == token_id {
                // Remove token by shifting remaining tokens
                for j in i..manager.token_count as usize - 1 {
                    manager.tokens[j] = manager.tokens[j + 1];
                }
                manager.token_count -= 1;
                return 0;
            }
        }
    }

    -1
}

/// Delegate capability with subset of permissions (Task 13.1.3)
#[no_mangle]
pub unsafe extern "C" fn capability_delegate(
    token_id: SigmaU64,
    subset_permissions: SigmaU64,
    new_expiry: SigmaU64,
) -> SigmaU64 {
    if CAPABILITY_MANAGER.is_none() || token_id == 0 {
        return 0;
    }

    if let Some(ref manager) = CAPABILITY_MANAGER {
        // Find original token
        for i in 0..manager.token_count as usize {
            if manager.tokens[i].id == token_id {
                // Ensure subset permissions don't exceed original
                if (subset_permissions & manager.tokens[i].permissions) != subset_permissions {
                    return 0;
                }

                // Create new token with subset
                return capability_create(
                    subset_permissions,
                    manager.tokens[i].resource,
                    new_expiry,
                );
            }
        }
    }

    0
}

/// Permission bit flags
pub const CAP_READ: SigmaU64 = 1 << 0;
pub const CAP_WRITE: SigmaU64 = 1 << 1;
pub const CAP_EXECUTE: SigmaU64 = 1 << 2;
pub const CAP_CREATE: SigmaU64 = 1 << 3;
pub const CAP_DELETE: SigmaU64 = 1 << 4;
pub const CAP_MODIFY: SigmaU64 = 1 << 5;
pub const CAP_ADMIN: SigmaU64 = 1 << 6;
pub const CAP_NETWORK: SigmaU64 = 1 << 7;
pub const CAP_DEVICE: SigmaU64 = 1 << 8;
pub const CAP_PROCESS: SigmaU64 = 1 << 9;

/// Resource type identifiers
pub const RESOURCE_FILE: SigmaU64 = 1 << 0;
pub const RESOURCE_DIRECTORY: SigmaU64 = 1 << 1;
pub const RESOURCE_DEVICE: SigmaU64 = 1 << 2;
pub const RESOURCE_NETWORK: SigmaU64 = 1 << 3;
pub const RESOURCE_PROCESS: SigmaU64 = 1 << 4;
pub const RESOURCE_MEMORY: SigmaU64 = 1 << 5;
pub const RESOURCE_SYSTEM: SigmaU64 = 1 << 6;

/// Get capability token count
#[no_mangle]
pub unsafe extern "C" fn capability_count() -> SigmaU32 {
    if let Some(ref manager) = CAPABILITY_MANAGER {
        manager.token_count
    } else {
        0
    }
}

/// Check if capability system is initialized
#[no_mangle]
pub unsafe extern "C" fn capability_initialized() -> SigmaBool {
    if let Some(ref manager) = CAPABILITY_MANAGER {
        manager.initialized
    } else {
        false
    }
}
