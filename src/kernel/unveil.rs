// OpenBSD Unveil Security Sandbox
// Provides filesystem path access restrictions

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Unveil permissions for a path
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnveilPermissions {
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub create: bool,
}

impl UnveilPermissions {
    pub fn empty() -> Self {
        Self {
            read: false,
            write: false,
            exec: false,
            create: false,
        }
    }

    pub fn all() -> Self {
        Self {
            read: true,
            write: true,
            exec: true,
            create: true,
        }
    }

    pub fn rw() -> Self {
        Self {
            read: true,
            write: true,
            exec: false,
            create: false,
        }
    }

    pub fn rx() -> Self {
        Self {
            read: true,
            write: false,
            exec: true,
            create: false,
        }
    }

    pub fn r() -> Self {
        Self {
            read: true,
            write: false,
            exec: false,
            create: false,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut perms = Self::empty();
        for c in s.chars() {
            match c {
                'r' => perms.read = true,
                'w' => perms.write = true,
                'x' => perms.exec = true,
                'c' => perms.create = true,
                _ => {}
            }
        }
        perms
    }

    pub fn as_str(&self) -> String {
        let mut s = String::new();
        if self.read { s.push('r'); }
        if self.write { s.push('w'); }
        if self.exec { s.push('x'); }
        if self.create { s.push('c'); }
        s
    }

    pub fn allows(&self, operation: UnveilOperation) -> bool {
        match operation {
            UnveilOperation::Read => self.read,
            UnveilOperation::Write => self.write,
            UnveilOperation::Exec => self.exec,
            UnveilOperation::Create => self.create,
        }
    }
}

/// Filesystem operations that can be restricted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnveilOperation {
    Read,
    Write,
    Exec,
    Create,
}

/// Unveil rule for a specific path
#[derive(Debug, Clone)]
pub struct UnveilRule {
    pub path: String,
    pub permissions: UnveilPermissions,
}

impl UnveilRule {
    pub fn new(path: String, permissions: UnveilPermissions) -> Self {
        Self { path, permissions }
    }

    pub fn matches(&self, requested_path: &str) -> bool {
        requested_path.starts_with(&self.path)
    }

    pub fn allows(&self, operation: UnveilOperation) -> bool {
        self.permissions.allows(operation)
    }
}

/// Unveil context for process filesystem restrictions
#[derive(Debug, Clone)]
pub struct UnveilContext {
    pub rules: Vec<UnveilRule>,
    pub unveiled: bool,
}

impl UnveilContext {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            unveiled: false,
        }
    }

    /// Unveil a path with specific permissions
    pub fn unveil(&mut self, path: String, permissions: UnveilPermissions) -> Result<(), String> {
        if self.unveiled {
            return Err("Already unveiled (locked)".to_string());
        }

        self.rules.push(UnveilRule::new(path, permissions));
        Ok(())
    }

    /// Lock the unveil context (no more unveil calls allowed)
    pub fn lock(&mut self) {
        self.unveiled = true;
    }

    /// Check if a path allows a specific operation
    pub fn allows(&self, path: &str, operation: UnveilOperation) -> bool {
        if !self.unveiled {
            return true; // Not unveiled = all allowed
        }

        // Find the most specific matching rule
        let mut best_match: Option<&UnveilRule> = None;

        for rule in &self.rules {
            if rule.matches(path) {
                match &best_match {
                    None => best_match = Some(rule),
                    Some(current) if rule.path.len() > current.path.len() => {
                        best_match = Some(rule);
                    }
                    _ => {}
                }
            }
        }

        match best_match {
            Some(rule) => rule.allows(operation),
            None => false, // Default deny
        }
    }

    /// Check if unveiled
    pub fn is_unveiled(&self) -> bool {
        self.unveiled
    }

    /// Get number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for UnveilContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Unveil manager for system-wide unveil management
pub struct UnveilManager {
    contexts: Arc<Mutex<HashMap<u64, UnveilContext>>>,
    next_context_id: Arc<Mutex<u64>>,
}

impl UnveilManager {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(Mutex::new(HashMap::new())),
            next_context_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new unveil context
    pub fn create_context(&self) -> u64 {
        let mut next_id = self.next_context_id.lock().unwrap();
        let context_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let context = UnveilContext::new();
        let mut contexts = self.contexts.lock().unwrap();
        contexts.insert(context_id, context);

        context_id
    }

    /// Get a context by ID
    pub fn get_context(&self, context_id: u64) -> Option<UnveilContext> {
        let contexts = self.contexts.lock().unwrap();
        contexts.get(&context_id).cloned()
    }

    /// Remove a context
    pub fn remove_context(&self, context_id: u64) -> Result<(), String> {
        let mut contexts = self.contexts.lock().unwrap();
        match contexts.remove(&context_id) {
            Some(_) => Ok(()),
            None => Err(format!("Context {} not found", context_id)),
        }
    }

    /// Unveil for a context
    pub fn unveil(&self, context_id: u64, path: String, permissions: UnveilPermissions) -> Result<(), String> {
        let mut contexts = self.contexts.lock().unwrap();
        match contexts.get_mut(&context_id) {
            Some(context) => context.unveil(path, permissions),
            None => Err(format!("Context {} not found", context_id)),
        }
    }

    /// Lock a context
    pub fn lock(&self, context_id: u64) -> Result<(), String> {
        let mut contexts = self.contexts.lock().unwrap();
        match contexts.get_mut(&context_id) {
            Some(context) => {
                context.lock();
                Ok(())
            }
            None => Err(format!("Context {} not found", context_id)),
        }
    }

    /// Check if a context allows an operation on a path
    pub fn allows(&self, context_id: u64, path: &str, operation: UnveilOperation) -> bool {
        match self.get_context(context_id) {
            Some(context) => context.allows(path, operation),
            None => false,
        }
    }

    /// Get number of active contexts
    pub fn context_count(&self) -> usize {
        let contexts = self.contexts.lock().unwrap();
        contexts.len()
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
    fn test_unveil_permissions_empty() {
        let perms = UnveilPermissions::empty();
        assert!(!perms.read);
        assert!(!perms.write);
        assert!(!perms.exec);
        assert!(!perms.create);
    }

    #[test]
    fn test_unveil_permissions_all() {
        let perms = UnveilPermissions::all();
        assert!(perms.read);
        assert!(perms.write);
        assert!(perms.exec);
        assert!(perms.create);
    }

    #[test]
    fn test_unveil_permissions_from_str() {
        let perms = UnveilPermissions::from_str("rwx");
        assert!(perms.read);
        assert!(perms.write);
        assert!(perms.exec);
        assert!(!perms.create);
    }

    #[test]
    fn test_unveil_permissions_as_str() {
        let perms = UnveilPermissions::from_str("rwc");
        assert_eq!(perms.as_str(), "rwc");
    }

    #[test]
    fn test_unveil_permissions_allows() {
        let perms = UnveilPermissions::from_str("rw");
        assert!(perms.allows(UnveilOperation::Read));
        assert!(perms.allows(UnveilOperation::Write));
        assert!(!perms.allows(UnveilOperation::Exec));
        assert!(!perms.allows(UnveilOperation::Create));
    }

    #[test]
    fn test_unveil_rule_creation() {
        let rule = UnveilRule::new("/home".to_string(), UnveilPermissions::rw());
        assert_eq!(rule.path, "/home");
        assert!(rule.allows(UnveilOperation::Read));
    }

    #[test]
    fn test_unveil_rule_matching() {
        let rule = UnveilRule::new("/home".to_string(), UnveilPermissions::rw());
        assert!(rule.matches("/home/user"));
        assert!(rule.matches("/home/user/file.txt"));
        assert!(!rule.matches("/etc/passwd"));
    }

    #[test]
    fn test_unveil_context_unveiled() {
        let context = UnveilContext::new();
        assert!(!context.is_unveiled());
        assert!(context.allows("/any/path", UnveilOperation::Read)); // All allowed
    }

    #[test]
    fn test_unveil_context_unveil() {
        let mut context = UnveilContext::new();
        context.unveil("/home".to_string(), UnveilPermissions::rw()).unwrap();
        context.lock();

        assert!(context.is_unveiled());
        assert!(context.allows("/home/user", UnveilOperation::Read));
        assert!(!context.allows("/etc", UnveilOperation::Read));
    }

    #[test]
    fn test_unveil_context_double_unveil() {
        let mut context = UnveilContext::new();
        context.unveil("/home".to_string(), UnveilPermissions::rw()).unwrap();
        context.lock();
        assert!(context.unveil("/tmp".to_string(), UnveilPermissions::rw()).is_err());
    }

    #[test]
    fn test_unveil_context_specificity() {
        let mut context = UnveilContext::new();
        context.unveil("/".to_string(), UnveilPermissions::r()).unwrap();
        context.unveil("/home".to_string(), UnveilPermissions::rw()).unwrap();
        context.lock();

        // More specific rule should take precedence
        assert!(context.allows("/home/user", UnveilOperation::Write));
        assert!(!context.allows("/etc", UnveilOperation::Write));
    }

    #[test]
    fn test_unveil_manager() {
        let manager = UnveilManager::new();

        let context_id = manager.create_context();
        assert_eq!(context_id, 1);

        manager.unveil(context_id, "/home".to_string(), UnveilPermissions::rw()).unwrap();
        manager.lock(context_id).unwrap();

        assert!(manager.allows(context_id, "/home/user", UnveilOperation::Read));
        assert!(!manager.allows(context_id, "/home/user", UnveilOperation::Exec));

        assert_eq!(manager.context_count(), 1);

        manager.remove_context(context_id).unwrap();
        assert_eq!(manager.context_count(), 0);
    }

    #[test]
    fn test_unveil_manager_multiple_contexts() {
        let manager = UnveilManager::new();

        let context_id1 = manager.create_context();
        let context_id2 = manager.create_context();

        manager.unveil(context_id1, "/home".to_string(), UnveilPermissions::rw()).unwrap();
        manager.lock(context_id1).unwrap();

        manager.unveil(context_id2, "/tmp".to_string(), UnveilPermissions::r()).unwrap();
        manager.lock(context_id2).unwrap();

        assert!(manager.allows(context_id1, "/home/user", UnveilOperation::Write));
        assert!(!manager.allows(context_id2, "/home/user", UnveilOperation::Write));

        assert!(manager.allows(context_id2, "/tmp/file", UnveilOperation::Read));
        assert!(!manager.allows(context_id1, "/tmp/file", UnveilOperation::Read));

        assert_eq!(manager.context_count(), 2);
    }

    #[test]
    fn test_unveil_default_deny() {
        let mut context = UnveilContext::new();
        context.lock();
        assert!(!context.allows("/any/path", UnveilOperation::Read));
    }
}
