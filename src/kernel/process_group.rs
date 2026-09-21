// Linux-inspired process group and session management
// Provides process group and session abstraction for process control

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Process group
#[derive(Debug, Clone)]
pub struct ProcessGroup {
    pub pgid: i32,
    pub leader_pid: i32,
    pub members: Vec<i32>,
}

impl ProcessGroup {
    pub fn new(pgid: i32, leader_pid: i32) -> Self {
        Self {
            pgid,
            leader_pid,
            members: vec![leader_pid],
        }
    }

    /// Add process to group
    pub fn add_process(&mut self, pid: i32) {
        if !self.members.contains(&pid) {
            self.members.push(pid);
        }
    }

    /// Remove process from group
    pub fn remove_process(&mut self, pid: i32) {
        self.members.retain(|&p| p != pid);
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Check if process is in group
    pub fn contains(&self, pid: i32) -> bool {
        self.members.contains(&pid)
    }
}

/// Session
#[derive(Debug, Clone)]
pub struct Session {
    pub sid: i32,
    pub leader_pid: i32,
    pub process_groups: HashMap<i32, ProcessGroup>,
}

impl Session {
    pub fn new(sid: i32, leader_pid: i32) -> Self {
        let mut process_groups = HashMap::new();
        let pgid = leader_pid;
        process_groups.insert(pgid, ProcessGroup::new(pgid, leader_pid));

        Self {
            sid,
            leader_pid,
            process_groups,
        }
    }

    /// Create process group
    pub fn create_process_group(&mut self, pgid: i32, leader_pid: i32) -> Result<(), String> {
        if self.process_groups.contains_key(&pgid) {
            return Err(format!("Process group {} already exists", pgid));
        }

        self.process_groups.insert(pgid, ProcessGroup::new(pgid, leader_pid));
        Ok(())
    }

    /// Get process group
    pub fn get_process_group(&self, pgid: i32) -> Option<&ProcessGroup> {
        self.process_groups.get(&pgid)
    }

    /// Remove process group
    pub fn remove_process_group(&mut self, pgid: i32) -> Result<(), String> {
        match self.process_groups.remove(&pgid) {
            Some(_) => Ok(()),
            None => Err(format!("Process group {} not found", pgid)),
        }
    }

    /// Get process group count
    pub fn process_group_count(&self) -> usize {
        self.process_groups.len()
    }
}

/// Process group and session manager
pub struct ProcessGroupSessionManager {
    pub sessions: Arc<Mutex<HashMap<i32, Session>>>,
    pub next_sid: Arc<Mutex<i32>>,
}

impl ProcessGroupSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_sid: Arc::new(Mutex::new(1)),
        }
    }

    /// Create session
    pub fn create_session(&self, leader_pid: i32) -> i32 {
        let mut next_sid = self.next_sid.lock().unwrap();
        let sid = *next_sid;
        *next_sid += 1;
        drop(next_sid);

        let session = Session::new(sid, leader_pid);
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(sid, session);

        sid
    }

    /// Get session
    pub fn get_session(&self, sid: i32) -> Option<Session> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(&sid).cloned()
    }

    /// Remove session
    pub fn remove_session(&self, sid: i32) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.remove(&sid) {
            Some(_) => Ok(()),
            None => Err(format!("Session {} not found", sid)),
        }
    }

    /// Create process group in session
    pub fn create_process_group(&self, sid: i32, pgid: i32, leader_pid: i32) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.get_mut(&sid) {
            Some(session) => session.create_process_group(pgid, leader_pid),
            None => Err(format!("Session {} not found", sid)),
        }
    }

    /// Get process group from session
    pub fn get_process_group(&self, sid: i32, pgid: i32) -> Option<ProcessGroup> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(&sid).and_then(|s| s.get_process_group(pgid).cloned())
    }

    /// Remove process group from session
    pub fn remove_process_group(&self, sid: i32, pgid: i32) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.get_mut(&sid) {
            Some(session) => session.remove_process_group(pgid),
            None => Err(format!("Session {} not found", sid)),
        }
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        let sessions = self.sessions.lock().unwrap();
        sessions.len()
    }
}

impl Default for ProcessGroupSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_group() {
        let pg = ProcessGroup::new(100, 1000);
        assert_eq!(pg.pgid, 100);
        assert_eq!(pg.leader_pid, 1000);
        assert_eq!(pg.member_count(), 1);
    }

    #[test]
    fn test_process_group_add_process() {
        let mut pg = ProcessGroup::new(100, 1000);
        pg.add_process(1001);
        pg.add_process(1002);

        assert_eq!(pg.member_count(), 3);
    }

    #[test]
    fn test_process_group_add_duplicate() {
        let mut pg = ProcessGroup::new(100, 1000);
        pg.add_process(1000);
        pg.add_process(1000);

        assert_eq!(pg.member_count(), 1);
    }

    #[test]
    fn test_process_group_remove_process() {
        let mut pg = ProcessGroup::new(100, 1000);
        pg.add_process(1001);
        pg.remove_process(1001);

        assert_eq!(pg.member_count(), 1);
    }

    #[test]
    fn test_process_group_contains() {
        let mut pg = ProcessGroup::new(100, 1000);
        pg.add_process(1001);

        assert!(pg.contains(1000));
        assert!(pg.contains(1001));
        assert!(!pg.contains(9999));
    }

    #[test]
    fn test_session() {
        let session = Session::new(1, 1000);
        assert_eq!(session.sid, 1);
        assert_eq!(session.leader_pid, 1000);
        assert_eq!(session.process_group_count(), 1);
    }

    #[test]
    fn test_session_create_process_group() {
        let mut session = Session::new(1, 1000);
        session.create_process_group(200, 2000).unwrap();

        assert_eq!(session.process_group_count(), 2);
    }

    #[test]
    fn test_session_create_duplicate_process_group() {
        let mut session = Session::new(1, 1000);
        session.create_process_group(200, 2000).unwrap();
        assert!(session.create_process_group(200, 3000).is_err());
    }

    #[test]
    fn test_session_get_process_group() {
        let session = Session::new(1, 1000);
        let pg = session.get_process_group(1000);
        assert!(pg.is_some());
        assert_eq!(pg.unwrap().pgid, 1000);
    }

    #[test]
    fn test_session_remove_process_group() {
        let mut session = Session::new(1, 1000);
        session.create_process_group(200, 2000).unwrap();
        session.remove_process_group(200).unwrap();

        assert_eq!(session.process_group_count(), 1);
    }

    #[test]
    fn test_process_group_session_manager() {
        let manager = ProcessGroupSessionManager::new();

        let sid = manager.create_session(1000);
        assert_eq!(sid, 1);
        assert_eq!(manager.session_count(), 1);
    }

    #[test]
    fn test_process_group_session_manager_create_pg() {
        let manager = ProcessGroupSessionManager::new();

        let sid = manager.create_session(1000);
        manager.create_process_group(sid, 200, 2000).unwrap();

        assert_eq!(manager.get_process_group(sid, 200).unwrap().pgid, 200);
    }

    #[test]
    fn test_process_group_session_manager_invalid() {
        let manager = ProcessGroupSessionManager::new();
        assert!(manager.get_session(999).is_none());
        assert!(manager.create_process_group(999, 200, 2000).is_err());
    }
}
