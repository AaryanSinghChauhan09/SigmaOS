// SPDX-License-Identifier: GPL-2.0-or-later
// GhostBSD compatibility layer for SigmaOS.
// Provides OpenRC service management, desktop session init,
// and NetworkManager bridging for GhostBSD environments.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Supported GhostBSD desktop environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostDesktop {
    Mate,
    Xfce,
    Budgie,
}

/// Top-level GhostBSD configuration.
#[derive(Debug, Clone)]
pub struct GhostBsdConfig {
    pub openrc_enabled: bool,
    pub desktop: GhostDesktop,
    pub networkmanager: bool,
}

impl GhostBsdConfig {
    pub fn new(desktop: GhostDesktop) -> Self {
        Self {
            openrc_enabled: true,
            desktop,
            networkmanager: true,
        }
    }
}

/// Represents an OpenRC service with its name and runlevel.
#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub runlevel: String,
    pub running: bool,
}

impl OpenRcService {
    pub fn new(name: &str, runlevel: &str) -> Self {
        Self {
            name: name.to_string(),
            runlevel: runlevel.to_string(),
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.running {
            return Err("Service already running");
        }
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), &'static str> {
        if !self.running {
            return Err("Service not running");
        }
        self.running = false;
        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), &'static str> {
        self.running = false;
        self.running = true;
        Ok(())
    }

    pub fn status(&self) -> &'static str {
        if self.running {
            "started"
        } else {
            "stopped"
        }
    }
}

/// Manages a GhostBSD desktop session lifecycle.
pub struct GhostBsdSession {
    pub config: GhostBsdConfig,
    pub services: Vec<OpenRcService>,
    pub initialized: bool,
}

impl GhostBsdSession {
    pub fn new(config: GhostBsdConfig) -> Self {
        Self {
            config,
            services: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        self.initialized = true;
        Ok(())
    }

    pub fn start_desktop(&mut self) -> Result<String, &'static str> {
        if !self.initialized {
            return Err("Session not initialized");
        }
        let de = match self.config.desktop {
            GhostDesktop::Mate => "mate-session",
            GhostDesktop::Xfce => "startxfce4",
            GhostDesktop::Budgie => "budgie-desktop",
        };
        Ok(de.to_string())
    }

    pub fn configure_network(&mut self) -> Result<(), &'static str> {
        if !self.config.networkmanager {
            return Err("NetworkManager disabled");
        }
        let mut nm = OpenRcService::new("NetworkManager", "default");
        nm.start()?;
        self.services.push(nm);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_initializes_successfully() {
        let cfg = GhostBsdConfig::new(GhostDesktop::Mate);
        let mut session = GhostBsdSession::new(cfg);
        assert!(session.initialize().is_ok());
        assert!(session.initialized);
    }

    #[test]
    fn test_openrc_service_lifecycle() {
        let mut svc = OpenRcService::new("cupsd", "default");
        assert_eq!(svc.status(), "stopped");
        assert!(svc.start().is_ok());
        assert_eq!(svc.status(), "started");
        assert!(svc.stop().is_ok());
        assert_eq!(svc.status(), "stopped");
    }

    #[test]
    fn test_start_desktop_requires_init() {
        let cfg = GhostBsdConfig::new(GhostDesktop::Xfce);
        let mut session = GhostBsdSession::new(cfg);
        assert!(session.start_desktop().is_err());
        session.initialize().unwrap();
        let de = session.start_desktop().unwrap();
        assert_eq!(de, "startxfce4");
    }
}
