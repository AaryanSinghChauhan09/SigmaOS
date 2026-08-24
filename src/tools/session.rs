//! Session Manager (systemd-logind Inspiration)
//! Session tracking, seat management, and device assignment

#![no_std]

extern crate alloc;

use crate::klib::{Vec};
use alloc::string::{String, ToString};

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    TTY,
    X11,
    Wayland,
    Unspecified,
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Online,
    Active,
    Closing,
}

/// Session
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: u32,
    pub seat: String,
    pub session_type: SessionType,
    pub state: SessionState,
    pub display: String,
    pub leader_pid: u32,
}

impl Session {
    pub fn new(id: &str, user_id: u32, seat: &str) -> Self {
        Self {
            id: id.to_string(),
            user_id,
            seat: seat.to_string(),
            session_type: SessionType::Unspecified,
            state: SessionState::Online,
            display: String::new(),
            leader_pid: 0,
        }
    }

    pub fn set_session_type(&mut self, session_type: SessionType) {
        self.session_type = session_type;
    }

    pub fn set_display(&mut self, display: &str) {
        self.display = display.to_string();
    }

    pub fn activate(&mut self) {
        self.state = SessionState::Active;
    }

    pub fn close(&mut self) {
        self.state = SessionState::Closing;
    }
}

/// Seat
#[derive(Debug, Clone)]
pub struct Seat {
    pub name: String,
    pub sessions: Vec<String>,
    pub devices: Vec<String>,
}

impl Seat {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sessions: Vec::new(),
            devices: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session_id: &str) {
        self.sessions.push(session_id.to_string());
    }

    pub fn remove_session(&mut self, session_id: &str) {
        self.sessions.retain(|s| s != session_id);
    }

    pub fn add_device(&mut self, device: &str) {
        self.devices.push(device.to_string());
    }
}

/// User session
#[derive(Debug, Clone)]
pub struct UserSession {
    pub user_id: u32,
    pub username: String,
    pub sessions: Vec<String>,
    pub display: String,
}

impl UserSession {
    pub fn new(user_id: u32, username: &str) -> Self {
        Self {
            user_id,
            username: username.to_string(),
            sessions: Vec::new(),
            display: String::new(),
        }
    }

    pub fn add_session(&mut self, session_id: &str) {
        self.sessions.push(session_id.to_string());
    }
}

/// Login manager
pub struct LoginManager {
    pub sessions: Vec<Session>,
    pub seats: Vec<Seat>,
    pub users: Vec<UserSession>,
}

impl LoginManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            seats: Vec::new(),
            users: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    pub fn add_seat(&mut self, seat: Seat) {
        self.seats.push(seat);
    }

    pub fn add_user(&mut self, user: UserSession) {
        self.users.push(user);
    }

    pub fn get_session(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|s| s.id == id)
    }

    pub fn get_seat(&mut self, name: &str) -> Option<&mut Seat> {
        self.seats.iter_mut().find(|s| s.name == name)
    }

    pub fn get_user(&mut self, user_id: u32) -> Option<&mut UserSession> {
        self.users.iter_mut().find(|u| u.user_id == user_id)
    }

    pub fn create_session(&mut self, user_id: u32, seat: &str) -> Result<String, SessionError> {
        let session_id = format!("session-{}", self.sessions.len());
        let mut session = Session::new(&session_id, user_id, seat);
        session.activate();
        
        self.add_session(session);
        
        // Add to seat
        if let Some(seat_obj) = self.get_seat(seat) {
            seat_obj.add_session(&session_id);
        }
        
        // Add to user
        if let Some(user) = self.get_user(user_id) {
            user.add_session(&session_id);
        }
        
        Ok(session_id)
    }

    pub fn terminate_session(&mut self, session_id: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session(session_id) {
            session.close();
            
            // Remove from seat
            for seat in &mut self.seats {
                seat.remove_session(session_id);
            }
            
            // Remove from user
            for user in &mut self.users {
                user.sessions.retain(|s| s != session_id);
            }
            
            Ok(())
        } else {
            Err(SessionError::SessionNotFound)
        }
    }

    pub fn activate_session(&mut self, session_id: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session(session_id) {
            session.activate();
            Ok(())
        } else {
            Err(SessionError::SessionNotFound)
        }
    }

    pub fn get_active_sessions(&self) -> Vec<&Session> {
        self.sessions.iter().filter(|s| s.state == SessionState::Active).collect()
    }

    pub fn get_sessions_by_user(&self, user_id: u32) -> Vec<&Session> {
        self.sessions.iter().filter(|s| s.user_id == user_id).collect()
    }
}

/// Device
#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub session: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    DRM,
    Input,
    Audio,
    USB,
}

impl Device {
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        Self {
            name: name.to_string(),
            device_type,
            session: None,
        }
    }

    pub fn assign_to_session(&mut self, session_id: &str) {
        self.session = Some(session_id.to_string());
    }
}

/// Session device
#[derive(Debug, Clone)]
pub struct SessionDevice {
    pub session_id: String,
    pub device_name: String,
    pub permissions: DevicePermissions,
}

#[derive(Debug, Clone)]
pub struct DevicePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// Device manager
pub struct DeviceManager {
    pub devices: Vec<Device>,
    pub session_devices: Vec<SessionDevice>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            session_devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn assign_device_to_session(&mut self, device_name: &str, session_id: &str) -> Result<(), SessionError> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == device_name) {
            device.assign_to_session(session_id);
            Ok(())
        } else {
            Err(SessionError::DeviceNotFound)
        }
    }

    pub fn get_devices_by_session(&self, session_id: &str) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.session.as_ref() == Some(&session_id.to_string())).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    SessionNotFound,
    SeatNotFound,
    UserNotFound,
    DeviceNotFound,
    PermissionDenied,
}

impl Default for LoginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let session = Session::new("session-1", 1000, "seat0");
        assert_eq!(session.user_id, 1000);
    }

    #[test]
    fn test_seat() {
        let mut seat = Seat::new("seat0");
        seat.add_session("session-1");
        assert_eq!(seat.sessions.len(), 1);
    }

    #[test]
    fn test_login_manager() {
        let mut manager = LoginManager::new();
        let session = Session::new("session-1", 1000, "seat0");
        manager.add_session(session);
        assert_eq!(manager.sessions.len(), 1);
    }

    #[test]
    fn test_create_session() {
        let mut manager = LoginManager::new();
        let seat = Seat::new("seat0");
        manager.add_seat(seat);
        let user = UserSession::new(1000, "user");
        manager.add_user(user);
        let session_id = manager.create_session(1000, "seat0").unwrap();
        assert!(manager.get_session(&session_id).is_some());
    }
}