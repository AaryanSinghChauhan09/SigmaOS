#![no_std]
#![no_main]

/// OOP-based Remote Desktop for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 956
/// Implements remote desktop access

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;

pub type SessionID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Error = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteError {
    Success = 0,
    NotFound = 1,
    ConnectionFailed = 2,
}

pub trait RemoteSession {
    fn id(&self) -> SessionID;
    fn host(&self) -> &[u8];
    fn state(&self) -> SessionState;
    fn set_state(&self, state: SessionState);
}

#[repr(C)]
pub struct SimpleRemoteSession {
    pub id: SessionID,
    pub host: [u8; 128],
    pub state: AtomicUsize,
}

impl SimpleRemoteSession {
    pub fn new(id: SessionID, host: &[u8]) -> Self {
        let mut host_array = [0u8; 128];
        let host_len = host.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(host.as_ptr(), host_array.as_mut_ptr(), host_len);
        }
        SimpleRemoteSession {
            id,
            host: host_array,
            state: AtomicUsize::new(SessionState::Disconnected as usize),
        }
    }
}

impl RemoteSession for SimpleRemoteSession {
    fn id(&self) -> SessionID {
        self.id
    }
    fn host(&self) -> &[u8] {
        let len = self.host.iter().position(|&b| b == 0).unwrap_or(128);
        &self.host[..len]
    }
    fn state(&self) -> SessionState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn set_state(&self, state: SessionState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait RemoteDesktop {
    fn connect(&mut self, host: &[u8], port: u16) -> Result<SessionID, RemoteError>;
    fn disconnect(&mut self, id: SessionID) -> Result<(), RemoteError>;
    fn send_input(&self, id: SessionID, input: &[u8]) -> Result<(), RemoteError>;
    fn receive_screen(&self, id: SessionID) -> Result<Vec<u8>, RemoteError>;
}

#[repr(C)]
pub struct SimpleRemoteDesktop {
    pub sessions: Vec<Option<Box<dyn RemoteSession>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRemoteDesktop {
    pub fn new() -> Self {
        SimpleRemoteDesktop {
            sessions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn get_session(&self, id: SessionID) -> Option<&dyn RemoteSession> {
        for i in 0..self.sessions.len() {
            if let Some(ref session) = self.sessions[i] {
                if session.id() == id {
                    return Some(session.as_ref());
                }
            }
        }
        None
    }
}

impl Default for SimpleRemoteDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteDesktop for SimpleRemoteDesktop {
    fn connect(&mut self, host: &[u8], _port: u16) -> Result<SessionID, RemoteError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let session = SimpleRemoteSession::new(id, host);
        self.sessions.push(Some(Box::new(session)));
        Ok(id)
    }

    fn disconnect(&mut self, id: SessionID) -> Result<(), RemoteError> {
        for i in 0..self.sessions.len() {
            if let Some(ref session) = self.sessions[i] {
                if session.id() == id {
                    session.set_state(SessionState::Disconnected);
                    return Ok(());
                }
            }
        }
        Err(RemoteError::NotFound)
    }

    fn send_input(&self, id: SessionID, _input: &[u8]) -> Result<(), RemoteError> {
        if self.get_session(id).is_some() {
            Ok(())
        } else {
            Err(RemoteError::NotFound)
        }
    }

    fn receive_screen(&self, id: SessionID) -> Result<Vec<u8>, RemoteError> {
        if self.get_session(id).is_some() {
            let mut screen = Vec::new();
            for _ in 0..1920 * 1080 * 4 {
                screen.push(0u8);
            }
            Ok(screen)
        } else {
            Err(RemoteError::NotFound)
        }
    }
}

pub trait ScreenSharing {
    fn start_sharing(&mut self) -> Result<(), RemoteError>;
    fn stop_sharing(&mut self) -> Result<(), RemoteError>;
    fn is_sharing(&self) -> bool;
}

#[repr(C)]
pub struct SimpleScreenSharing {
    pub sharing: AtomicUsize,
}

impl SimpleScreenSharing {
    pub fn new() -> Self {
        SimpleScreenSharing {
            sharing: AtomicUsize::new(0),
        }
    }
}

impl Default for SimpleScreenSharing {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenSharing for SimpleScreenSharing {
    fn start_sharing(&mut self) -> Result<(), RemoteError> {
        self.sharing.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn stop_sharing(&mut self) -> Result<(), RemoteError> {
        self.sharing.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn is_sharing(&self) -> bool {
        self.sharing.load(Ordering::SeqCst) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_desktop_connection() {
        let mut desktop = SimpleRemoteDesktop::new();
        let id = desktop.connect(b"192.168.1.50", 5900).unwrap();

        {
            let session = desktop.get_session(id).unwrap();
            assert_eq!(session.host(), b"192.168.1.50");
        }

        desktop.disconnect(id).unwrap();

        let session = desktop.get_session(id).unwrap();
        assert_eq!(session.state(), SessionState::Disconnected);
    }
}
