// SigmaOS Unveil - Filesystem Visibility Sandboxing Mechanism
// Inspired by OpenBSD unveil, providing fine-grained path restriction.

extern crate alloc;

use crate::klib::error::{SecurityError, SigmaError};
use alloc::string::String;
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

/// Manager enforcing filesystem visibility on a per-process basis
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

        // Parse permission characters
        let mut perms = Vec::new();
        for c in permissions.chars() {
            match c {
                'r' => perms.push(UnveilPermission::Read),
                'w' => perms.push(UnveilPermission::Write),
                'x' => perms.push(UnveilPermission::Execute),
                'c' => perms.push(UnveilPermission::Create),
                _ => return Err(SigmaError::Security(SecurityError::InvalidToken)),
            }
        }

        // If path is already present, update its permissions
        if let Some(existing) = self.restrictions.iter_mut().find(|r| r.path == path) {
            existing.permissions = perms;
        } else {
            self.restrictions.push(UnveilRestriction {
                path: String::from(path),
                permissions: perms,
            });
        }

        Ok(())
    }

    /// Validate whether a process is allowed to access the specified path with a required permission.
    /// - If no unveil restrictions are registered, access is unrestricted (permissive mode).
    /// - Once at least one path is unveiled, any path not underneath an unveiled path is blocked!
    pub fn validate_path(&self, path: &str, required: UnveilPermission) -> Result<(), SigmaError> {
        if self.restrictions.is_empty() {
            return Ok(()); // Open by default when unveil has not been used yet
        }

        // Mitigate directory traversal: reject paths containing parent directory segments
        for segment in path.split(|c| c == '/' || c == '\\') {
            if segment == ".." {
                return Err(SigmaError::Security(SecurityError::AccessDenied));
            }
        }

        // Find the most specific (longest) matching unveiled parent directory
        let mut best_match: Option<&UnveilRestriction> = None;
        for restriction in &self.restrictions {
            if path.starts_with(&restriction.path) {
                let r_len = restriction.path.len();
                // Ensure it is a valid boundary match (exact match, or followed by a separator, or suffix has slash)
                let is_boundary = path.len() == r_len
                    || path.as_bytes().get(r_len).copied() == Some(b'/')
                    || path.as_bytes().get(r_len).copied() == Some(b'\\')
                    || restriction.path.ends_with('/')
                    || restriction.path.ends_with('\\');

                if is_boundary {
                    match best_match {
                        None => best_match = Some(restriction),
                        Some(best) => {
                            if restriction.path.len() > best.path.len() {
                                best_match = Some(restriction);
                            }
                        }
                    }
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
            // Path lies completely outside any unveiled directories
            Err(SigmaError::Security(
                SecurityError::PrivilegeEscalationDetected,
            ))
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
        // Permissive by default (no unveil restrictions registered yet)
        assert!(manager
            .validate_path("/var/log/syslog", UnveilPermission::Read)
            .is_ok());
    }

    #[test]
    fn test_unveil_path_restriction() {
        let mut manager = UnveilManager::new();
        manager.unveil("/var/www", "rw").unwrap();
        manager.unveil("/tmp", "rwc").unwrap();
        manager.unveil("/etc/ssl/", "r").unwrap();

        // Path inside unveiled directories with correct permissions should pass
        assert!(manager
            .validate_path("/var/www/index.html", UnveilPermission::Read)
            .is_ok());
        assert!(manager
            .validate_path("/tmp/session.tmp", UnveilPermission::Create)
            .is_ok());

        // Path with trailing slash config should pass
        assert!(manager
            .validate_path("/etc/ssl/cert.pem", UnveilPermission::Read)
            .is_ok());

        // Path inside unveiled directory with incorrect permissions should fail
        assert!(manager
            .validate_path("/var/www/upload.cgi", UnveilPermission::Execute)
            .is_err());

        // Path completely outside unveiled directories should fail
        assert!(manager
            .validate_path("/etc/passwd", UnveilPermission::Read)
            .is_err());

        // Traversal sequences should be immediately blocked
        assert!(manager
            .validate_path("/var/www/../../etc/passwd", UnveilPermission::Read)
            .is_err());

        // Prefix bypass should fail
        assert!(manager
            .validate_path("/var/www-secret", UnveilPermission::Read)
            .is_err());
    }

    #[test]
    fn test_unveil_lock() {
        let mut manager = UnveilManager::new();
        manager.unveil("/tmp", "rw").unwrap();

        // Locking down the manager
        manager.unveil("", "").unwrap();
        assert!(manager.locked);

        // Further unveil calls should fail
        assert!(manager.unveil("/etc", "r").is_err());
    }
}
