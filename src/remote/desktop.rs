#![no_std]
#![no_main]

extern crate alloc;

use core::mem;
/// OOP-based Remote Desktop for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 956
/// Implements remote desktop access
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;
use alloc::boxed::Box;

pub type SessionID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum SessionState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Error = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    fn state(&self) -> SessionState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn set_state(&self, state: SessionState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait RemoteDesktop {
    fn connect(&mut self, host: &[u8], port: u16) -> Result<SessionID, RemoteError>;
    fn disconnect(&mut self, id: SessionID) -> Result<(), RemoteError>;
    fn send_input(&self, id: SessionID, input: &[u8]) -> Result<(), RemoteError>;
    fn receive_screen(&self, id: SessionID) -> Result<Vec<u8>, RemoteError>;
    fn get_session(&self, id: SessionID) -> Option<&dyn RemoteSession>;
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
}

impl RemoteDesktop for SimpleRemoteDesktop {
    fn connect(&mut self, host: &[u8], _port: u16) -> Result<SessionID, RemoteError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let session = SimpleRemoteSession::new(id, host);
        self.sessions.push(Some(Box::new(session)));
        Ok(id)
    }

    fn disconnect(&mut self, id: SessionID) -> Result<(), RemoteError> {
        for session_option in &mut self.sessions {
            if let Some(ref mut session) = *session_option {
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

    fn get_session(&self, id: SessionID) -> Option<&dyn RemoteSession> {
        for session_option in &self.sessions {
            if let Some(ref session) = *session_option {
                if session.id() == id {
                    return Some(session.as_ref());
                }
            }
        }
        None
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


pub struct InputAuthGate;
impl InputAuthGate {
    pub fn new() -> Self { Self }
}

pub struct PqcVideoCipher;
impl PqcVideoCipher {
    pub fn new() -> Self { Self }
}

pub struct SigmaRendezvous;
impl SigmaRendezvous {
    pub fn new() -> Self { Self }
}
