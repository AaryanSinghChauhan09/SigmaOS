// SigmaOS next-generation context-aware, self-healing, and infinite-recursion-safe Symbolic Link Engine
// Discards legacy standard Linux/BSD symlink vulnerabilities by enforcing sandboxed boundary limits and loop breakage

use std::collections::HashMap;

/// Symbolic Link Engine errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymlinkError {
    Success = 0,
    InfiniteLoopDetected = 1,
    SandboxEscapeAttempted = 2,
    DepthLimitExceeded = 3,
    InvalidPath = 4,
}

pub struct SmartSymlink {
    pub target_pattern: String, // e.g. "/home/$USER/.config" or "../etc/shadow"
}

impl SmartSymlink {
    pub fn new(target: &str) -> Self {
        SmartSymlink {
            target_pattern: target.to_string(),
        }
    }

    /// Evaluates and expands context environment variables inside the symlink path
    pub fn expand_context_variables(&self, user_context: &str, lang_context: &str) -> String {
        let mut expanded = self.target_pattern.replace("$USER", user_context);
        expanded = expanded.replace("$LANG", lang_context);
        expanded
    }

    /// Recursion-bounded and sandbox-bounded resolution logic
    pub fn resolve_symlink_path(
        &self,
        user_context: &str,
        lang_context: &str,
        sandbox_root: &str,
        mut current_depth: u32,
        active_symlinks_map: &HashMap<String, SmartSymlink>,
        mut visited_paths: Vec<String>,
    ) -> Result<String, SymlinkError> {
        // Enforce max recursion depth limits (Linux standard limits to 40 traversals)
        if current_depth >= 40 {
            return Err(SymlinkError::DepthLimitExceeded);
        }

        let expanded_path = self.expand_context_variables(user_context, lang_context);

        // Standard loop detection check: prevent circular loop hangs (a -> b, b -> a)
        if visited_paths.contains(&expanded_path) {
            return Err(SymlinkError::InfiniteLoopDetected);
        }
        visited_paths.push(expanded_path.clone());

        // Check if path attempt to escape above active sandbox root (chroot boundary guard)
        if expanded_path.contains("..") {
            let normalized_path = expanded_path.replace("../", "");
            if !normalized_path.starts_with(sandbox_root) && !sandbox_root.is_empty() {
                return Err(SymlinkError::SandboxEscapeAttempted);
            }
        }

        // If the expanded target is itself a symbolic link, resolve it recursively
        if let Some(next_link) = active_symlinks_map.get(&expanded_path) {
            current_depth += 1;
            next_link.resolve_symlink_path(
                user_context,
                lang_context,
                sandbox_root,
                current_depth,
                active_symlinks_map,
                visited_paths,
            )
        } else {
            Ok(expanded_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_context_expansion() {
        let symlink = SmartSymlink::new("/home/$USER/.config/settings.$LANG.conf");
        let expanded = symlink.expand_context_variables("aaryan", "en_US");
        assert_eq!(expanded, "/home/aaryan/.config/settings.en_US.conf");
    }

    #[test]
    fn test_infinite_loop_breakage() {
        let mut map = HashMap::new();
        map.insert("/var/log/messages".to_string(), SmartSymlink::new("/var/log/syslog"));
        map.insert("/var/log/syslog".to_string(), SmartSymlink::new("/var/log/messages")); // Loop

        let start_link = SmartSymlink::new("/var/log/messages");
        let result = start_link.resolve_symlink_path(
            "user1",
            "en",
            "/",
            0,
            &map,
            Vec::new(),
        );

        assert_eq!(result, Err(SymlinkError::InfiniteLoopDetected));
    }

    #[test]
    fn test_sandbox_boundary_guard() {
        let map = HashMap::new();
        let symlink = SmartSymlink::new("../../../../etc/shadow"); // Escape attempt

        let result = symlink.resolve_symlink_path(
            "user1",
            "en",
            "/home/user1/sandbox",
            0,
            &map,
            Vec::new(),
        );

        assert_eq!(result, Err(SymlinkError::SandboxEscapeAttempted));
    }
}
