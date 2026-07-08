// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Mandatory Access Control (MAC)
//! SELinux/AppArmor-style context labeling and policy enforcement.
//! Provides comprehensive security policy management with audit logging.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

pub const MAX_MAC_POLICIES: usize = 256;
pub const MAC_CONTEXT_LEN: usize = 64;
pub const MAX_SUBJECTS: usize = 128;
pub const MAX_OBJECTS: usize = 256;
pub const MAX_AUDIT_LOG: usize = 512;

// ─── MAC Permission Flags ─────────────────────────────────────────────────────

pub const MAC_PERM_READ:    SigmaU32 = 0x0001;
pub const MAC_PERM_WRITE:   SigmaU32 = 0x0002;
pub const MAC_PERM_EXEC:    SigmaU32 = 0x0004;
pub const MAC_PERM_APPEND:  SigmaU32 = 0x0008;
pub const MAC_PERM_DELETE:  SigmaU32 = 0x0010;
pub const MAC_PERM_CREATE:  SigmaU32 = 0x0020;
pub const MAC_PERM_BIND:    SigmaU32 = 0x0040;
pub const MAC_PERM_CONNECT: SigmaU32 = 0x0080;
pub const MAC_PERM_LISTEN:  SigmaU32 = 0x0100;
pub const MAC_PERM_ACCEPT:  SigmaU32 = 0x0200;
pub const MAC_PERM_TRANS:   SigmaU32 = 0x0400;
pub const MAC_PERM_SETATTR: SigmaU32 = 0x0800;
pub const MAC_PERM_GETATTR: SigmaU32 = 0x1000;
pub const MAC_PERM_ALL:     SigmaU32 = 0xFFFF;

// ─── MAC Object Types ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum MacObjectType {
    File = 0,
    Directory = 1,
    Socket = 2,
    Device = 3,
    Process = 4,
    Network = 5,
    IPC = 6,
    Capability = 7,
}

// ─── Security Context ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecurityContext {
    pub user: [SigmaU8; MAC_CONTEXT_LEN],
    pub role: [SigmaU8; MAC_CONTEXT_LEN],
    pub type_: [SigmaU8; MAC_CONTEXT_LEN],
    pub level: [SigmaU8; MAC_CONTEXT_LEN],
}

// ─── MAC Policy Rule ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MacPolicy {
    pub subject_context: SecurityContext,
    pub object_context: SecurityContext,
    pub object_type: MacObjectType,
    pub permissions: SigmaU32,
    pub audit: SigmaBool,
    pub active: SigmaBool,
}

// ─── Subject/Process Context ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubjectContext {
    pub pid: SigmaU32,
    pub context: SecurityContext,
    pub capabilities: SigmaU64,
    pub valid: SigmaBool,
}

// ─── Object Context ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectContext {
    pub id: SigmaU64,
    pub context: SecurityContext,
    pub object_type: MacObjectType,
    pub valid: SigmaBool,
}

// ─── Audit Log Entry ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuditEntry {
    pub timestamp: SigmaU64,
    pub subject_pid: SigmaU32,
    pub object_id: SigmaU64,
    pub requested_perms: SigmaU32,
    pub granted: SigmaBool,
    pub denied_reason: SigmaU32,
}

// ─── MAC Enforcement Mode ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum MacEnforcementMode {
    Permissive = 0,  // Log violations but allow access
    Enforcing = 1,   // Enforce policy, deny violations
    Disabled = 2,    // MAC disabled
}

// ─── MAC State ─────────────────────────────────────────────────────────────

pub struct MacState {
    initialized: SigmaBool,
    policies: [MacPolicy; MAX_MAC_POLICIES],
    policy_count: SigmaU32,
    subjects: [SubjectContext; MAX_SUBJECTS],
    subject_count: SigmaU32,
    objects: [ObjectContext; MAX_OBJECTS],
    object_count: SigmaU32,
    audit_log: [AuditEntry; MAX_AUDIT_LOG],
    audit_index: SigmaUsize,
    enforcement_mode: MacEnforcementMode,
    default_deny: SigmaBool,
}

impl MacState {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            policies: [MacPolicy {
                subject_context: SecurityContext {
                    user: [0; MAC_CONTEXT_LEN],
                    role: [0; MAC_CONTEXT_LEN],
                    type_: [0; MAC_CONTEXT_LEN],
                    level: [0; MAC_CONTEXT_LEN],
                },
                object_context: SecurityContext {
                    user: [0; MAC_CONTEXT_LEN],
                    role: [0; MAC_CONTEXT_LEN],
                    type_: [0; MAC_CONTEXT_LEN],
                    level: [0; MAC_CONTEXT_LEN],
                },
                object_type: MacObjectType::File,
                permissions: 0,
                audit: false,
                active: false,
            }; MAX_MAC_POLICIES],
            policy_count: 0,
            subjects: [SubjectContext {
                pid: 0,
                context: SecurityContext {
                    user: [0; MAC_CONTEXT_LEN],
                    role: [0; MAC_CONTEXT_LEN],
                    type_: [0; MAC_CONTEXT_LEN],
                    level: [0; MAC_CONTEXT_LEN],
                },
                capabilities: 0,
                valid: false,
            }; MAX_SUBJECTS],
            subject_count: 0,
            objects: [ObjectContext {
                id: 0,
                context: SecurityContext {
                    user: [0; MAC_CONTEXT_LEN],
                    role: [0; MAC_CONTEXT_LEN],
                    type_: [0; MAC_CONTEXT_LEN],
                    level: [0; MAC_CONTEXT_LEN],
                },
                object_type: MacObjectType::File,
                valid: false,
            }; MAX_OBJECTS],
            object_count: 0,
            audit_log: [AuditEntry {
                timestamp: 0,
                subject_pid: 0,
                object_id: 0,
                requested_perms: 0,
                granted: false,
                denied_reason: 0,
            }; MAX_AUDIT_LOG],
            audit_index: 0,
            enforcement_mode: MacEnforcementMode::Enforcing,
            default_deny: true,
        }
    }

    pub unsafe fn init(&mut self) -> SigmaI32 {
        self.initialized = true;
        self.policy_count = 0;
        self.subject_count = 0;
        self.object_count = 0;
        self.audit_index = 0;
        self.enforcement_mode = MacEnforcementMode::Enforcing;
        self.default_deny = true;
        0
    }

    pub unsafe fn add_policy(&mut self, policy: MacPolicy) -> SigmaI32 {
        if self.policy_count >= MAX_MAC_POLICIES as SigmaU32 {
            return -1;
        }

        self.policies[self.policy_count as SigmaUsize] = policy;
        self.policy_count += 1;
        0
    }

    pub unsafe fn register_subject(&mut self, pid: SigmaU32, context: SecurityContext) -> SigmaI32 {
        if self.subject_count >= MAX_SUBJECTS as SigmaU32 {
            return -1;
        }

        self.subjects[self.subject_count as SigmaUsize] = SubjectContext {
            pid,
            context,
            capabilities: 0,
            valid: true,
        };
        self.subject_count += 1;
        0
    }

    pub unsafe fn register_object(&mut self, id: SigmaU64, context: SecurityContext, obj_type: MacObjectType) -> SigmaI32 {
        if self.object_count >= MAX_OBJECTS as SigmaU32 {
            return -1;
        }

        self.objects[self.object_count as SigmaUsize] = ObjectContext {
            id,
            context,
            object_type: obj_type,
            valid: true,
        };
        self.object_count += 1;
        0
    }

    pub unsafe fn check_access(&mut self, pid: SigmaU32, obj_id: SigmaU64, requested_perms: SigmaU32) -> SigmaI32 {
        if !self.initialized {
            return -1;
        }

        // Find subject
        let subject_ctx = match self.find_subject(pid) {
            Some(ctx) => ctx,
            None => {
                // Default to unconfined context
                SecurityContext {
                    user: [b'u'; MAC_CONTEXT_LEN],
                    role: [b'u'; MAC_CONTEXT_LEN],
                    type_: [b'u'; MAC_CONTEXT_LEN],
                    level: [b's'; MAC_CONTEXT_LEN],
                }
            }
        };

        // Find object
        let object_ctx = match self.find_object(obj_id) {
            Some(ctx) => ctx,
            None => {
                // Default to unconfined context
                ObjectContext {
                    id: obj_id,
                    context: SecurityContext {
                        user: [b'u'; MAC_CONTEXT_LEN],
                        role: [b'u'; MAC_CONTEXT_LEN],
                        type_: [b'u'; MAC_CONTEXT_LEN],
                        level: [b's'; MAC_CONTEXT_LEN],
                    },
                    object_type: MacObjectType::File,
                    valid: false,
                }
            }
        };

        // Check policies
        let mut granted = false;
        for i in 0..self.policy_count as SigmaUsize {
            if !self.policies[i].active {
                continue;
            }

            if self.contexts_match(&self.policies[i].subject_context, &subject_ctx.context) &&
               self.contexts_match(&self.policies[i].object_context, &object_ctx.context) &&
               self.policies[i].object_type == object_ctx.object_type {
                // Check if policy grants all requested permissions
                if (self.policies[i].permissions & requested_perms) == requested_perms {
                    granted = true;
                    if self.policies[i].audit {
                        self.log_audit(pid, obj_id, requested_perms, true, 0);
                    }
                    break;
                }
            }
        }

        // Default deny if no matching policy
        if !granted && self.default_deny {
            self.log_audit(pid, obj_id, requested_perms, false, 1); // No matching policy
            if self.enforcement_mode == MacEnforcementMode::Enforcing {
                return -13; // EACCES
            }
        }

        0
    }

    pub unsafe fn set_enforcement_mode(&mut self, mode: MacEnforcementMode) {
        self.enforcement_mode = mode;
    }

    pub unsafe fn get_enforcement_mode(&self) -> MacEnforcementMode {
        self.enforcement_mode
    }

    pub unsafe fn set_default_deny(&mut self, deny: SigmaBool) {
        self.default_deny = deny;
    }

    fn find_subject(&self, pid: SigmaU32) -> Option<SubjectContext> {
        for i in 0..self.subject_count as SigmaUsize {
            if self.subjects[i].valid && self.subjects[i].pid == pid {
                return Some(self.subjects[i]);
            }
        }
        None
    }

    fn find_object(&self, id: SigmaU64) -> Option<ObjectContext> {
        for i in 0..self.object_count as SigmaUsize {
            if self.objects[i].valid && self.objects[i].id == id {
                return Some(self.objects[i]);
            }
        }
        None
    }

    fn contexts_match(&self, ctx1: &SecurityContext, ctx2: &SecurityContext) -> bool {
        // Simple string matching for context fields
        // In a real implementation, this would use proper MLS/MCS rules
        self.string_match(&ctx1.user, &ctx2.user) &&
        self.string_match(&ctx1.role, &ctx2.role) &&
        self.string_match(&ctx1.type_, &ctx2.type_)
    }

    fn string_match(&self, s1: &SigmaU8, s2: &SigmaU8) -> bool {
        // Wildcard match: '*' matches anything
        if *s1 == b'*' || *s2 == b'*' {
            return true;
        }
        s1 == s2
    }

    fn log_audit(&mut self, pid: SigmaU32, obj_id: SigmaU64, perms: SigmaU32, granted: SigmaBool, reason: SigmaU32) {
        let entry = AuditEntry {
            timestamp: self.get_timestamp(),
            subject_pid: pid,
            object_id: obj_id,
            requested_perms: perms,
            granted,
            denied_reason: reason,
        };

        self.audit_log[self.audit_index] = entry;
        self.audit_index = (self.audit_index + 1) % MAX_AUDIT_LOG;
    }

    fn get_timestamp(&self) -> SigmaU64 {
        // In a real implementation, this would read from hardware timer
        0
    }

    pub unsafe fn get_audit_log(&self, entries: *mut AuditEntry, max_count: SigmaU32) -> SigmaU32 {
        if entries.is_null() {
            return 0;
        }

        let mut copied = 0;
        for i in 0..self.audit_index.min(max_count as SigmaUsize) {
            *entries.add(i) = self.audit_log[i];
            copied += 1;
        }

        copied
    }
}

static mut MAC_STATE: MacState = MacState::new();

// ─── C-ABI Interface Functions ───────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_init() -> SigmaI32 {
    MAC_STATE.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_add_policy(
    subject_user: *const SigmaU8,
    subject_role: *const SigmaU8,
    subject_type: *const SigmaU8,
    subject_level: *const SigmaU8,
    object_user: *const SigmaU8,
    object_role: *const SigmaU8,
    object_type: *const SigmaU8,
    object_level: *const SigmaU8,
    obj_type_enum: SigmaU32,
    permissions: SigmaU32,
    audit: SigmaI32,
) -> SigmaI32 {
    let mut subject_ctx = SecurityContext {
        user: [0; MAC_CONTEXT_LEN],
        role: [0; MAC_CONTEXT_LEN],
        type_: [0; MAC_CONTEXT_LEN],
        level: [0; MAC_CONTEXT_LEN],
    };
    let mut object_ctx = SecurityContext {
        user: [0; MAC_CONTEXT_LEN],
        role: [0; MAC_CONTEXT_LEN],
        type_: [0; MAC_CONTEXT_LEN],
        level: [0; MAC_CONTEXT_LEN],
    };

    copy_string(&mut subject_ctx.user, subject_user);
    copy_string(&mut subject_ctx.role, subject_role);
    copy_string(&mut subject_ctx.type_, subject_type);
    copy_string(&mut subject_ctx.level, subject_level);
    copy_string(&mut object_ctx.user, object_user);
    copy_string(&mut object_ctx.role, object_role);
    copy_string(&mut object_ctx.type_, object_type);
    copy_string(&mut object_ctx.level, object_level);

    let policy = MacPolicy {
        subject_context: subject_ctx,
        object_context: object_ctx,
        object_type: match obj_type_enum {
            0 => MacObjectType::File,
            1 => MacObjectType::Directory,
            2 => MacObjectType::Socket,
            3 => MacObjectType::Device,
            4 => MacObjectType::Process,
            5 => MacObjectType::Network,
            6 => MacObjectType::IPC,
            7 => MacObjectType::Capability,
            _ => MacObjectType::File,
        },
        permissions,
        audit: audit != 0,
        active: true,
    };

    MAC_STATE.add_policy(policy)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_register_subject(
    pid: SigmaU32,
    user: *const SigmaU8,
    role: *const SigmaU8,
    type_: *const SigmaU8,
    level: *const SigmaU8,
) -> SigmaI32 {
    let mut ctx = SecurityContext {
        user: [0; MAC_CONTEXT_LEN],
        role: [0; MAC_CONTEXT_LEN],
        type_: [0; MAC_CONTEXT_LEN],
        level: [0; MAC_CONTEXT_LEN],
    };

    copy_string(&mut ctx.user, user);
    copy_string(&mut ctx.role, role);
    copy_string(&mut ctx.type_, type_);
    copy_string(&mut ctx.level, level);

    MAC_STATE.register_subject(pid, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_register_object(
    id: SigmaU64,
    user: *const SigmaU8,
    role: *const SigmaU8,
    type_: *const SigmaU8,
    level: *const SigmaU8,
    obj_type_enum: SigmaU32,
) -> SigmaI32 {
    let mut ctx = SecurityContext {
        user: [0; MAC_CONTEXT_LEN],
        role: [0; MAC_CONTEXT_LEN],
        type_: [0; MAC_CONTEXT_LEN],
        level: [0; MAC_CONTEXT_LEN],
    };

    copy_string(&mut ctx.user, user);
    copy_string(&mut ctx.role, role);
    copy_string(&mut ctx.type_, type_);
    copy_string(&mut ctx.level, level);

    let obj_type = match obj_type_enum {
        0 => MacObjectType::File,
        1 => MacObjectType::Directory,
        2 => MacObjectType::Socket,
        3 => MacObjectType::Device,
        4 => MacObjectType::Process,
        5 => MacObjectType::Network,
        6 => MacObjectType::IPC,
        7 => MacObjectType::Capability,
        _ => MacObjectType::File,
    };

    MAC_STATE.register_object(id, ctx, obj_type)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_check_access(pid: SigmaU32, obj_id: SigmaU64, requested_perms: SigmaU32) -> SigmaI32 {
    MAC_STATE.check_access(pid, obj_id, requested_perms)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_set_enforcement_mode(mode: SigmaI32) -> SigmaI32 {
    MAC_STATE.set_enforcement_mode(match mode {
        0 => MacEnforcementMode::Permissive,
        1 => MacEnforcementMode::Enforcing,
        2 => MacEnforcementMode::Disabled,
        _ => MacEnforcementMode::Enforcing,
    });
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_get_enforcement_mode() -> SigmaI32 {
    match MAC_STATE.get_enforcement_mode() {
        MacEnforcementMode::Permissive => 0,
        MacEnforcementMode::Enforcing => 1,
        MacEnforcementMode::Disabled => 2,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_set_default_deny(deny: SigmaI32) -> SigmaI32 {
    MAC_STATE.set_default_deny(deny != 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_get_audit_log(entries: *mut AuditEntry, max_count: SigmaU32) -> SigmaU32 {
    MAC_STATE.get_audit_log(entries, max_count)
}

unsafe fn copy_string(dst: &mut [SigmaU8; MAC_CONTEXT_LEN], src: *const SigmaU8) {
    if src.is_null() {
        return;
    }
    let mut i = 0;
    while i < MAC_CONTEXT_LEN - 1 {
        let c = *src.add(i);
        dst[i] = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    dst[MAC_CONTEXT_LEN - 1] = 0;
}
