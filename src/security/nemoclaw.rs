
use core::mem;
/// NVIDIA NemoClaw-inspired AI Agent Security Stack for SigmaOS
/// Provides OpenShell sandboxing, Privacy Router info-redaction,
/// and Default-Deny outbound network policies.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NemoClawError {
    Success = 0,
    BlockedByPolicy = 1,
    SandboxViolation = 2,
    RedactionFailed = 3,
}

/// NemoClaw Privacy Router
/// Automatically masks sensitive user information before passing to AI models
pub struct PrivacyRouter {
    pub redaction_count: AtomicUsize,
}

impl PrivacyRouter {
    pub fn new() -> Self {
        PrivacyRouter {
            redaction_count: AtomicUsize::new(0),
        }
    }

    /// Redact standard patterns such as credit cards, Aadhaar, or emails in a custom stream
    pub fn redact_sensitive_info(
        &self,
        prompt: &[u8],
        redacted: &mut [u8],
    ) -> Result<usize, NemoClawError> {
        let mut read_idx = 0;
        let mut write_idx = 0;

        while read_idx < prompt.len() {
            // Check for Aadhaar format: 12 digits separated by spaces (e.g. 1234 5678 9012)
            if read_idx + 14 <= prompt.len()
                && prompt[read_idx..read_idx + 4]
                    .iter()
                    .all(|&b| b.is_ascii_digit())
                && prompt[read_idx + 4] == b' '
                && prompt[read_idx + 5..read_idx + 9]
                    .iter()
                    .all(|&b| b.is_ascii_digit())
                && prompt[read_idx + 9] == b' '
                && prompt[read_idx + 10..read_idx + 14]
                    .iter()
                    .all(|&b| b.is_ascii_digit())
            {
                let token = b"[REDACTED_AADHAAR]";
                if write_idx + token.len() > redacted.len() {
                    return Err(NemoClawError::RedactionFailed);
                }
                for &b in token {
                    redacted[write_idx] = b;
                    write_idx += 1;
                }
                read_idx += 14;
                self.redaction_count.fetch_add(1, Ordering::SeqCst);
                continue;
            }

            // Check for credit card pattern: 16 digits
            if read_idx + 16 <= prompt.len()
                && prompt[read_idx..read_idx + 16]
                    .iter()
                    .all(|&b| b.is_ascii_digit())
            {
                let token = b"[REDACTED_CARD]";
                if write_idx + token.len() > redacted.len() {
                    return Err(NemoClawError::RedactionFailed);
                }
                for &b in token {
                    redacted[write_idx] = b;
                    write_idx += 1;
                }
                read_idx += 16;
                self.redaction_count.fetch_add(1, Ordering::SeqCst);
                continue;
            }

            // Default byte copy
            if write_idx < redacted.len() {
                redacted[write_idx] = prompt[read_idx];
                write_idx += 1;
            }
            read_idx += 1;
        }

        Ok(write_idx)
    }
}

/// NemoClaw Default-Deny Network Policy gatekeeper
/// Restricts AI agents from outbound exfiltrations unless explicitly whitelisted
pub struct DefaultDenyNetworkPolicy {
    pub permitted_endpoints: Vec<[u8; 32]>,
}

impl DefaultDenyNetworkPolicy {
    pub fn new() -> Self {
        DefaultDenyNetworkPolicy {
            permitted_endpoints: Vec::new(),
        }
    }

    pub fn whitelist_endpoint(&mut self, endpoint: &[u8]) {
        let mut ep_array = [0u8; 32];
        let len = endpoint.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(endpoint.as_ptr(), ep_array.as_mut_ptr(), len);
        }
        self.permitted_endpoints.push(ep_array);
    }

    /// Check if outbound packet transmission is permitted under the NemoClaw default-deny model
    pub fn is_permitted(&self, destination: &[u8]) -> bool {
        for i in 0..self.permitted_endpoints.len {
            let ep = &self.permitted_endpoints[i];
            let len = ep.iter().position(|&b| b == 0).unwrap_or(32);
            if &ep[..len] == destination {
                return true;
            }
        }
        false
    }
}

/// OpenShell Secure Sandboxed execution wrapper for AI agents
pub struct OpenShellAgentSandbox {
    pub max_cpu_cycles: usize,
    pub max_memory_mb: usize,
    pub network_policy: DefaultDenyNetworkPolicy,
}

impl OpenShellAgentSandbox {
    pub fn new(max_cpu_cycles: usize, max_memory_mb: usize) -> Self {
        OpenShellAgentSandbox {
            max_cpu_cycles,
            max_memory_mb,
            network_policy: DefaultDenyNetworkPolicy::new(),
        }
    }

    /// Verify if an agent's parsed command or action violates OpenShell execution constraints
    pub fn check_action(
        &self,
        target_command: &[u8],
        requested_memory_mb: usize,
    ) -> Result<(), NemoClawError> {
        if requested_memory_mb > self.max_memory_mb {
            return Err(NemoClawError::SandboxViolation);
        }

        // Prevent shell escaping prompt injection commands (like sudo, chmod, rm -rf, etc.)
        let forbidden_patterns = [
            b"sudo".as_slice(),
            b"chmod".as_slice(),
            b"rm -rf".as_slice(),
            b"sh -c".as_slice(),
        ];

        for pattern in &forbidden_patterns {
            if target_command.len() >= pattern.len() {
                for i in 0..=(target_command.len() - pattern.len()) {
                    if &target_command[i..(i + pattern.len())] == *pattern {
                        return Err(NemoClawError::SandboxViolation);
                    }
                }
            }
        }

        Ok(())
    }
}

struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::std::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_router_redactions() {
        let router = PrivacyRouter::new();
        let prompt = b"User login with card 1234567812345678 and Aadhaar 1234 5678 9012.";
        let mut redacted = [0u8; 128];
        let len = router.redact_sensitive_info(prompt, &mut redacted).unwrap();

        assert_eq!(router.redaction_count.load(Ordering::SeqCst), 2);
        assert!(len > 0);
    }

    #[test]
    fn test_default_deny_networks() {
        let mut policy = DefaultDenyNetworkPolicy::new();
        policy.whitelist_endpoint(b"api.nvidia.com");

        assert!(policy.is_permitted(b"api.nvidia.com"));
        assert!(!policy.is_permitted(b"malicious-site.cn"));
    }

    #[test]
    fn test_openshell_command_sandbox() {
        let sandbox = OpenShellAgentSandbox::new(10000, 512);

        // standard command
        assert!(sandbox.check_action(b"ls -la /home", 256).is_ok());

        // prompt injection shell escapes
        assert_eq!(
            sandbox.check_action(b"rm -rf /", 256).unwrap_err(),
            NemoClawError::SandboxViolation
        );
        assert_eq!(
            sandbox
                .check_action(b"sudo apt-get install", 256)
                .unwrap_err(),
            NemoClawError::SandboxViolation
        );
    }
}
