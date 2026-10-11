// SigmaOS Logind Session & Seat Management Subsystem
// Reference: systemd-logind, sd-login API (freedesktop.org)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    Unspecified,
    Tty,
    X11,
    Wayland,
    Mir,
    Web,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionClass {
    User,
    Greeter,
    LockScreen,
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Online,
    Closing,
    Active,
}

/// User session tracked by systemd-logind
#[derive(Debug, Clone)]
pub struct LogindSession {
    pub id: String,
    pub uid: u32,
    pub user_name: String,
    pub seat: String,
    pub vtnr: u32,
    pub session_type: SessionType,
    pub session_class: SessionClass,
    pub state: SessionState,
    pub idle_hint: bool,
}

/// Hardware seat (e.g. "seat0")
#[derive(Debug, Clone)]
pub struct LogindSeat {
    pub name: String,
    pub active_session_id: Option<String>,
    pub can_multi_session: bool,
    pub can_tty: bool,
    pub can_graphical: bool,
    pub devices: Vec<String>, // sysfs device paths bound to seat
}

/// Logind Manager
pub struct LogindManager {
    seats: BTreeMap<String, LogindSeat>,
    sessions: BTreeMap<String, LogindSession>,
    next_session_id: u32,
}

impl LogindManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            seats: BTreeMap::new(),
            sessions: BTreeMap::new(),
            next_session_id: 1,
        };

        // Initialize default seat0
        mgr.seats.insert(
            String::from("seat0"),
            LogindSeat {
                name: String::from("seat0"),
                active_session_id: None,
                can_multi_session: true,
                can_tty: true,
                can_graphical: true,
                devices: Vec::new(),
            },
        );

        mgr
    }

    /// Creates a new user session on a specified seat
    pub fn create_session(
        &mut self,
        uid: u32,
        user_name: &str,
        seat: &str,
        vtnr: u32,
        session_type: SessionType,
        session_class: SessionClass,
    ) -> String {
        let sid = alloc::format!("c{}", self.next_session_id);
        self.next_session_id += 1;

        let session = LogindSession {
            id: sid.clone(),
            uid,
            user_name: String::from(user_name),
            seat: String::from(seat),
            vtnr,
            session_type,
            session_class,
            state: SessionState::Active,
            idle_hint: false,
        };

        self.sessions.insert(sid.clone(), session);

        // Update active session on seat
        if let Some(seat_obj) = self.seats.get_mut(seat) {
            seat_obj.active_session_id = Some(sid.clone());
        }

        sid
    }

    /// Closes a session
    pub fn close_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.state = SessionState::Closing;
            true
        } else {
            false
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<&LogindSession> {
        self.sessions.get(session_id)
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}
