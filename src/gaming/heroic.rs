// SPDX-License-Identifier: MIT
// SigmaOS - Heroic Games Launcher Integration
// Inspired by Heroic Games Launcher (https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher)
// Provides Epic Games Store client integration

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Heroic game
#[derive(Debug, Clone)]
pub struct HeroicGame {
    pub app_name: String,
    pub title: String,
    pub installed: bool,
    pub running: bool,
    pub playtime: u32, // minutes
}

/// Heroic launcher
pub struct HeroicLauncher {
    pub games: BTreeMap<String, HeroicGame>,
    pub active: AtomicBool,
}

impl HeroicLauncher {
    pub fn new() -> Self {
        Self {
            games: BTreeMap::new(),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize Heroic launcher
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Add game
    pub fn add_game(&mut self, game: HeroicGame) {
        self.games.insert(game.app_name.clone(), game);
    }

    /// Get game
    pub fn get_game(&self, app_name: &str) -> Option<&HeroicGame> {
        self.games.get(app_name)
    }

    /// Launch game
    pub fn launch_game(&mut self, app_name: &str) -> Result<(), &'static str> {
        let game = self.games.get_mut(app_name).ok_or("Game not found")?;

        if !game.installed {
            return Err("Game not installed");
        }

        game.running = true;

        // In real implementation, would run:
        // heroic --launch {app_name}

        Ok(())
    }

    /// Stop game
    pub fn stop_game(&mut self, app_name: &str) -> Result<(), &'static str> {
        let game = self.games.get_mut(app_name).ok_or("Game not found")?;
        game.running = false;
        Ok(())
    }

    /// Install game
    pub fn install_game(&mut self, app_name: &str) -> Result<(), &'static str> {
        let game = self.games.get_mut(app_name).ok_or("Game not found")?;
        game.installed = true;
        Ok(())
    }

    /// Get installed games
    pub fn get_installed_games(&self) -> Vec<&HeroicGame> {
        self.games.values().filter(|g| g.installed).collect()
    }
}

impl Default for HeroicLauncher {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Heroic launcher
static mut HEROIC_LAUNCHER: Option<HeroicLauncher> = None;

/// Initialize Heroic launcher
pub fn init_heroic_launcher() {
    unsafe {
        HEROIC_LAUNCHER = Some(HeroicLauncher::new());
    }
}

/// Get global Heroic launcher
pub fn get_heroic_launcher() -> Option<&'static mut HeroicLauncher> {
    unsafe { HEROIC_LAUNCHER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heroic_launcher_init() {
        let mut launcher = HeroicLauncher::new();
        launcher.init();
        assert!(launcher.active.load(Ordering::Relaxed));
    }

    #[test]
    fn test_add_game() {
        let mut launcher = HeroicLauncher::new();
        let game = HeroicGame {
            app_name: "TestGame".to_string(),
            title: "Test Game".to_string(),
            installed: true,
            running: false,
            playtime: 0,
        };
        launcher.add_game(game);
        assert!(launcher.get_game("TestGame").is_some());
    }

    #[test]
    fn test_launch_game() {
        let mut launcher = HeroicLauncher::new();
        let game = HeroicGame {
            app_name: "TestGame".to_string(),
            title: "Test Game".to_string(),
            installed: true,
            running: false,
            playtime: 0,
        };
        launcher.add_game(game);
        launcher.launch_game("TestGame").unwrap();
        assert!(launcher.get_game("TestGame").unwrap().running);
    }
}
