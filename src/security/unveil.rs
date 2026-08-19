// SigmaOS Unveil & Landlock - Filesystem Visibility & Sandboxing Subsystem
// Inspired by OpenBSD unveil and Linux Landlock, providing fine-grained path restrictions.

extern crate alloc;

#[cfg(not(test))]
use crate::klib::error::{SecurityError, SigmaError};

#[cfg(test)]
pub use test_stub::*;

#[cfg(test)]
mod test_stub {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SecurityError {
        AccessDenied,
        PrivilegeEscalationDetected,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SigmaError {
        Security(SecurityError),
    }
}
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Filesystem access permissions for unveiled paths
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnveilPermission {
    Read,    // 'r'
    Write,   // 'w'
    Execute, // 'x'
    Create,  // 'c'
}

/// A specific path restriction mapping
#[derive(Debug, Clone)]
pub struct UnveilRestriction {
    pub path: String,
    pub permissions: Vec<UnveilPermission>,
}

/// Manager enforcing filesystem visibility and Landlock-style rule inheritance on a per-process basis
pub struct UnveilManager {
    pub restrictions: Vec<UnveilRestriction>,
    pub locked: bool,
}

impl UnveilManager {
    /// Create a new, unrestricted unveil manager
    pub fn new() -> Self {
        Self {
            restrictions: Vec::new(),
            locked: false,
        }
    }

    /// Register a path visibility constraint.
    /// - If path and permissions are both empty, the unveil system is locked (no further changes allowed).
    /// - If path is already unveiled, updates its permission list.
    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), SigmaError> {
        if self.locked {
            return Err(SigmaError::Security(SecurityError::AccessDenied));
        }

        if path.is_empty() && permissions.is_empty() {
            self.locked = true;
            return Ok(());
        }

        let parsed_perms = Self::parse_permissions(permissions);

        if let Some(existing) = self.restrictions.iter_mut().find(|r| r.path == path) {
            existing.permissions = parsed_perms;
        } else {
            self.restrictions.push(UnveilRestriction {
                path: path.to_string(),
                permissions: parsed_perms,
            });
        }

        Ok(())
    }

    /// Landlock-style unveil_at: Register a constraint relative to a parent dir_fd / base_path
    pub fn unveil_at(&mut self, base_path: &str, relative_path: &str, permissions: &str) -> Result<(), SigmaError> {
        let full_path = if base_path.ends_with('/') {
            format!("{}{}", base_path, relative_path)
        } else {
            format!("{}/{}", base_path, relative_path)
        };
        self.unveil(&full_path, permissions)
    }

    /// Helper to parse character flags into UnveilPermissions
    fn parse_permissions(perms: &str) -> Vec<UnveilPermission> {
        let mut vec = Vec::new();
        for ch in perms.chars() {
            match ch {
                'r' => vec.push(UnveilPermission::Read),
                'w' => vec.push(UnveilPermission::Write),
                'x' => vec.push(UnveilPermission::Execute),
                'c' => vec.push(UnveilPermission::Create),
                _ => {}
            }
        }
        vec
    }

    /// Validate whether `required` permission is granted for `path`.
    pub fn validate_path(&self, path: &str, required: UnveilPermission) -> Result<(), SigmaError> {
        if self.restrictions.is_empty() {
            return Ok(()); // Permissive default
        }

        let mut best_match: Option<&UnveilRestriction> = None;

        // Zero-allocation path matching: avoids heap-allocating `format!("{}/", restriction.path)`
        // per iteration in high-frequency path validation loops.
        for restriction in &self.restrictions {
            let r_path = &restriction.path;
            let is_match = path == r_path || (
                path.starts_with(r_path) && (
                    r_path.ends_with('/') ||
                    path[r_path.len()..].starts_with('/') ||
                    path[r_path.len()..].starts_with('\\')
                )
            );

            if is_match {
                if let Some(best) = best_match {
                    if r_path.len() > best.path.len() {
                        best_match = Some(restriction);
                    }
                } else {
                    best_match = Some(restriction);
                }
            }
        }

        if let Some(restriction) = best_match {
            if restriction.permissions.contains(&required) {
                Ok(())
            } else {
                Err(SigmaError::Security(SecurityError::AccessDenied))
            }
        } else {
            Err(SigmaError::Security(SecurityError::PrivilegeEscalationDetected))
        }
    }
}

impl Default for UnveilManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unveil_permissive_default() {
        let manager = UnveilManager::new();
        assert!(manager
            .validate_path("/var/log/syslog", UnveilPermission::Read)
            .is_ok());
    }

    #[test]
    fn test_unveil_path_restriction() {
        let mut manager = UnveilManager::new();
        manager.unveil("/var/www", "rw").unwrap();
        manager.unveil("/tmp", "rwc").unwrap();

        assert!(manager
            .validate_path("/var/www/index.html", UnveilPermission::Read)
            .is_ok());
        assert!(manager
            .validate_path("/tmp/session.tmp", UnveilPermission::Create)
            .is_ok());

        assert!(manager
            .validate_path("/var/www/upload.cgi", UnveilPermission::Execute)
            .is_err());

        assert!(manager
            .validate_path("/etc/passwd", UnveilPermission::Read)
            .is_err());

        // Verify prefix bypasses are properly blocked
        assert!(manager
            .validate_path("/var/www-secret", UnveilPermission::Read)
            .is_err());
    }

    #[test]
    fn test_unveil_at_landlock_inheritance() {
        let mut manager = UnveilManager::new();
        manager.unveil_at("/etc", "nginx", "r").unwrap();

        assert!(manager
            .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Read)
            .is_ok());
        assert!(manager
            .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Write)
            .is_err());
    }

    #[test]
    fn test_unveil_lock() {
        let mut manager = UnveilManager::new();
        manager.unveil("/tmp", "rw").unwrap();

        manager.unveil("", "").unwrap();
        assert!(manager.locked);

        assert!(manager.unveil("/etc", "r").is_err());
    }
}
