// OpenBSD Pledge Security Sandbox
// Provides syscall promise-based security restrictions

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// OpenBSD pledge promises (subset of available promises)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PledgePromise {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Dpath,
    Tty,
    Recvfd,
    Sendfd,
    Exec,
    Proc,
    Id,
    Setuid,
    Setgid,
    Setfgid,
    Setresuid,
    Setresgid,
    Getpw,
    Timer,
    Dns,
    Unix,
    Flock,
    Fattr,
    Inet,
    Mcast,
    Route,
    Audio,
    Video,
    Bpf,
    Unveil,
    Error,
    ProtExec,
    Ps,
    Vminfo,
    Idle,
    Pf,
    Wifi,
}

impl PledgePromise {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stdio" => Some(PledgePromise::Stdio),
            "rpath" => Some(PledgePromise::Rpath),
            "wpath" => Some(PledgePromise::Wpath),
            "cpath" => Some(PledgePromise::Cpath),
            "dpath" => Some(PledgePromise::Dpath),
            "tty" => Some(PledgePromise::Tty),
            "recvfd" => Some(PledgePromise::Recvfd),
            "sendfd" => Some(PledgePromise::Sendfd),
            "exec" => Some(PledgePromise::Exec),
            "proc" => Some(PledgePromise::Proc),
            "id" => Some(PledgePromise::Id),
            "setuid" => Some(PledgePromise::Setuid),
            "setgid" => Some(PledgePromise::Setgid),
            "setfgid" => Some(PledgePromise::Setfgid),
            "setresuid" => Some(PledgePromise::Setresuid),
            "setresgid" => Some(PledgePromise::Setresgid),
            "getpw" => Some(PledgePromise::Getpw),
            "timer" => Some(PledgePromise::Timer),
            "dns" => Some(PledgePromise::Dns),
            "unix" => Some(PledgePromise::Unix),
            "flock" => Some(PledgePromise::Flock),
            "fattr" => Some(PledgePromise::Fattr),
            "inet" => Some(PledgePromise::Inet),
            "mcast" => Some(PledgePromise::Mcast),
            "route" => Some(PledgePromise::Route),
            "audio" => Some(PledgePromise::Audio),
            "video" => Some(PledgePromise::Video),
            "bpf" => Some(PledgePromise::Bpf),
            "unveil" => Some(PledgePromise::Unveil),
            "error" => Some(PledgePromise::Error),
            "prot_exec" => Some(PledgePromise::ProtExec),
            "ps" => Some(PledgePromise::Ps),
            "vminfo" => Some(PledgePromise::Vminfo),
            "idle" => Some(PledgePromise::Idle),
            "pf" => Some(PledgePromise::Pf),
            "wifi" => Some(PledgePromise::Wifi),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PledgePromise::Stdio => "stdio",
            PledgePromise::Rpath => "rpath",
            PledgePromise::Wpath => "wpath",
            PledgePromise::Cpath => "cpath",
            PledgePromise::Dpath => "dpath",
            PledgePromise::Tty => "tty",
            PledgePromise::Recvfd => "recvfd",
            PledgePromise::Sendfd => "sendfd",
            PledgePromise::Exec => "exec",
            PledgePromise::Proc => "proc",
            PledgePromise::Id => "id",
            PledgePromise::Setuid => "setuid",
            PledgePromise::Setgid => "setgid",
            PledgePromise::Setfgid => "setfgid",
            PledgePromise::Setresuid => "setresuid",
            PledgePromise::Setresgid => "setresgid",
            PledgePromise::Getpw => "getpw",
            PledgePromise::Timer => "timer",
            PledgePromise::Dns => "dns",
            PledgePromise::Unix => "unix",
            PledgePromise::Flock => "flock",
            PledgePromise::Fattr => "fattr",
            PledgePromise::Inet => "inet",
            PledgePromise::Mcast => "mcast",
            PledgePromise::Route => "route",
            PledgePromise::Audio => "audio",
            PledgePromise::Video => "video",
            PledgePromise::Bpf => "bpf",
            PledgePromise::Unveil => "unveil",
            PledgePromise::Error => "error",
            PledgePromise::ProtExec => "prot_exec",
            PledgePromise::Ps => "ps",
            PledgePromise::Vminfo => "vminfo",
            PledgePromise::Idle => "idle",
            PledgePromise::Pf => "pf",
            PledgePromise::Wifi => "wifi",
        }
    }
}

/// Pledge context for process syscall restrictions
#[derive(Debug, Clone)]
pub struct PledgeContext {
    pub promises: HashSet<PledgePromise>,
    pub pledged: bool,
}

impl PledgeContext {
    pub fn new() -> Self {
        Self {
            promises: HashSet::new(),
            pledged: false,
        }
    }

    /// Pledge with specific promises
    pub fn pledge(&mut self, promises: &[PledgePromise]) -> Result<(), String> {
        if self.pledged {
            return Err("Already pledged".to_string());
        }

        self.promises = promises.iter().cloned().collect();
        self.pledged = true;
        Ok(())
    }

    /// Check if a promise is granted
    pub fn has_promise(&self, promise: PledgePromise) -> bool {
        if !self.pledged {
            return true; // No pledge = all allowed
        }
        self.promises.contains(&promise)
    }

    /// Check if any of the given promises are granted
    pub fn has_any_promise(&self, promises: &[PledgePromise]) -> bool {
        if !self.pledged {
            return true;
        }
        promises.iter().any(|p| self.promises.contains(p))
    }

    /// Parse promises from a space-separated string
    pub fn parse_promises(s: &str) -> Vec<PledgePromise> {
        s.split_whitespace()
            .filter_map(|p| PledgePromise::from_str(p))
            .collect()
    }

    /// Check if pledged
    pub fn is_pledged(&self) -> bool {
        self.pledged
    }
}

impl Default for PledgeContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Pledge manager for system-wide pledge management
pub struct PledgeManager {
    contexts: Arc<Mutex<HashMap<u64, PledgeContext>>>,
    next_context_id: Arc<Mutex<u64>>,
}

impl PledgeManager {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(Mutex::new(HashMap::new())),
            next_context_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new pledge context
    pub fn create_context(&self) -> u64 {
        let mut next_id = self.next_context_id.lock().unwrap();
        let context_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let context = PledgeContext::new();
        let mut contexts = self.contexts.lock().unwrap();
        contexts.insert(context_id, context);

        context_id
    }

    /// Get a context by ID
    pub fn get_context(&self, context_id: u64) -> Option<PledgeContext> {
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

    /// Pledge for a context
    pub fn pledge(&self, context_id: u64, promises: &[PledgePromise]) -> Result<(), String> {
        let mut contexts = self.contexts.lock().unwrap();
        match contexts.get_mut(&context_id) {
            Some(context) => context.pledge(promises),
            None => Err(format!("Context {} not found", context_id)),
        }
    }

    /// Check if a context has a promise
    pub fn has_promise(&self, context_id: u64, promise: PledgePromise) -> bool {
        match self.get_context(context_id) {
            Some(context) => context.has_promise(promise),
            None => false,
        }
    }

    /// Get number of active contexts
    pub fn context_count(&self) -> usize {
        let contexts = self.contexts.lock().unwrap();
        contexts.len()
    }
}

impl Default for PledgeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pledge_promise_from_str() {
        assert_eq!(PledgePromise::from_str("stdio"), Some(PledgePromise::Stdio));
        assert_eq!(PledgePromise::from_str("rpath"), Some(PledgePromise::Rpath));
        assert_eq!(PledgePromise::from_str("invalid"), None);
    }

    #[test]
    fn test_pledge_promise_as_str() {
        assert_eq!(PledgePromise::Stdio.as_str(), "stdio");
        assert_eq!(PledgePromise::Rpath.as_str(), "rpath");
    }

    #[test]
    fn test_pledge_context_unpledged() {
        let context = PledgeContext::new();
        assert!(!context.is_pledged());
        assert!(context.has_promise(PledgePromise::Stdio)); // All allowed
    }

    #[test]
    fn test_pledge_context_pledge() {
        let mut context = PledgeContext::new();
        context.pledge(&[PledgePromise::Stdio, PledgePromise::Rpath]).unwrap();

        assert!(context.is_pledged());
        assert!(context.has_promise(PledgePromise::Stdio));
        assert!(context.has_promise(PledgePromise::Rpath));
        assert!(!context.has_promise(PledgePromise::Wpath));
    }

    #[test]
    fn test_pledge_context_double_pledge() {
        let mut context = PledgeContext::new();
        context.pledge(&[PledgePromise::Stdio]).unwrap();
        assert!(context.pledge(&[PledgePromise::Rpath]).is_err());
    }

    #[test]
    fn test_pledge_context_parse_promises() {
        let promises = PledgeContext::parse_promises("stdio rpath wpath");
        assert_eq!(promises.len(), 3);
        assert!(promises.contains(&PledgePromise::Stdio));
        assert!(promises.contains(&PledgePromise::Rpath));
        assert!(promises.contains(&PledgePromise::Wpath));
    }

    #[test]
    fn test_pledge_context_has_any_promise() {
        let mut context = PledgeContext::new();
        context.pledge(&[PledgePromise::Stdio]).unwrap();

        assert!(context.has_any_promise(&[PledgePromise::Stdio, PledgePromise::Rpath]));
        assert!(!context.has_any_promise(&[PledgePromise::Rpath, PledgePromise::Wpath]));
    }

    #[test]
    fn test_pledge_manager() {
        let manager = PledgeManager::new();

        let context_id = manager.create_context();
        assert_eq!(context_id, 1);

        manager.pledge(context_id, &[PledgePromise::Stdio]).unwrap();

        assert!(manager.has_promise(context_id, PledgePromise::Stdio));
        assert!(!manager.has_promise(context_id, PledgePromise::Rpath));

        assert_eq!(manager.context_count(), 1);

        manager.remove_context(context_id).unwrap();
        assert_eq!(manager.context_count(), 0);
    }

    #[test]
    fn test_pledge_manager_multiple_contexts() {
        let manager = PledgeManager::new();

        let context_id1 = manager.create_context();
        let context_id2 = manager.create_context();

        manager.pledge(context_id1, &[PledgePromise::Stdio]).unwrap();
        manager.pledge(context_id2, &[PledgePromise::Rpath]).unwrap();

        assert!(manager.has_promise(context_id1, PledgePromise::Stdio));
        assert!(!manager.has_promise(context_id2, PledgePromise::Stdio));

        assert!(manager.has_promise(context_id2, PledgePromise::Rpath));
        assert!(!manager.has_promise(context_id1, PledgePromise::Rpath));

        assert_eq!(manager.context_count(), 2);
    }

    #[test]
    fn test_pledge_promise_hash() {
        let mut set = HashSet::new();
        set.insert(PledgePromise::Stdio);
        set.insert(PledgePromise::Rpath);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&PledgePromise::Stdio));
        assert!(set.contains(&PledgePromise::Rpath));
    }
}
