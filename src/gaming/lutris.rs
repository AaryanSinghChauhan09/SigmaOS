// SPDX-License-Identifier: MIT
// SigmaOS - Lutris Integration
// Inspired by Lutris (https://github.com/lutris/lutris)
// Provides Lutris game manager integration for Wine games

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Lutris game
#[derive(Debug, Clone)]
pub struct LutrisGame {
    pub id: u32,
    pub name: String,
    pub runner: LutrisRunner,
    pub installed: bool,
    pub running: bool,
    pub playtime: u32, // minutes
}

/// Lutris runner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LutrisRunner {
    Wine,
    Proton,
    Steam,
    Native,
    RetroArch,
    Mame,
    Dosbox,
    ScummVM,
}

/// Lutris manager
pub struct LutrisManager {
    pub games: BTreeMap<u32, LutrisGame>,
    pub runners: Vec<LutrisRunner>,
    pub active: AtomicBool,
}

impl LutrisManager {
    pub fn new() -> Self {
        Self {
            games: BTreeMap::new(),
            runners: vec![
                LutrisRunner::Wine,
                LutrisRunner::Proton,
                LutrisRunner::Steam,
                LutrisRunner::Native,
                LutrisRunner::RetroArch,
                LutrisRunner::Mame,
                LutrisRunner::Dosbox,
                LutrisRunner::ScummVM,
            ],
            active: AtomicBool::new(false),
        }
    }

    /// Initialize Lutris manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Add game
    pub fn add_game(&mut self, game: LutrisGame) {
        self.games.insert(game.id, game);
    }

    /// Get game
    pub fn get_game(&self, id: u32) -> Option<&LutrisGame> {
        self.games.get(&id)
    }

    /// Launch game
    pub fn launch_game(&mut self, id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;

        if !game.installed {
            return Err("Game not installed");
        }

        game.running = true;

        // In real implementation, would run:
        // lutris lutris:rungame/{id}

        Ok(())
    }

    /// Stop game
    pub fn stop_game(&mut self, id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;

        game.running = false;

        Ok(())
    }

    /// Install game
    pub fn install_game(&mut self, id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&id).ok_or("Game not found")?;

        game.installed = true;

        Ok(())
    }

    /// Get games by runner
    pub fn get_games_by_runner(&self, runner: LutrisRunner) -> Vec<&LutrisGame> {
        self.games.values().filter(|g| g.runner == runner).collect()
    }

    /// Get installed games
    pub fn get_installed_games(&self) -> Vec<&LutrisGame> {
        self.games.values().filter(|g| g.installed).collect()
    }

    /// Get running games
    pub fn get_running_games(&self) -> Vec<&LutrisGame> {
        self.games.values().filter(|g| g.running).collect()
    }
}

impl Default for LutrisManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Lutris manager
static mut LUTRIS_MANAGER: Option<LutrisManager> = None;

/// Initialize Lutris manager
pub fn init_lutris_manager() {
    unsafe {
        LUTRIS_MANAGER = Some(LutrisManager::new());
    }
}

/// Get global Lutris manager
pub fn get_lutris_manager() -> Option<&'static mut LutrisManager> {
    unsafe { LUTRIS_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lutris_manager_init() {
        let mut manager = LutrisManager::new();
        manager.init();
        assert!(manager.active.load(Ordering::Relaxed));
    }

    #[test]
    fn test_add_game() {
        let mut manager = LutrisManager::new();
        let game = LutrisGame {
            id: 1,
            name: "Test Game".to_string(),
            runner: LutrisRunner::Wine,
            installed: true,
            running: false,
            playtime: 0,
        };
        manager.add_game(game);
        assert!(manager.get_game(1).is_some());
    }

    #[test]
    fn test_launch_game() {
        let mut manager = LutrisManager::new();
        let game = LutrisGame {
            id: 1,
            name: "Test Game".to_string(),
            runner: LutrisRunner::Wine,
            installed: true,
            running: false,
            playtime: 0,
        };
        manager.add_game(game);
        manager.launch_game(1).unwrap();
        assert!(manager.get_game(1).unwrap().running);
    }

    #[test]
    fn test_get_games_by_runner() {
        let mut manager = LutrisManager::new();
        let game1 = LutrisGame {
            id: 1,
            name: "Wine Game".to_string(),
            runner: LutrisRunner::Wine,
            installed: true,
            running: false,
            playtime: 0,
        };
        let game2 = LutrisGame {
            id: 2,
            name: "Steam Game".to_string(),
            runner: LutrisRunner::Steam,
            installed: true,
            running: false,
            playtime: 0,
        };
        manager.add_game(game1);
        manager.add_game(game2);
        let wine_games = manager.get_games_by_runner(LutrisRunner::Wine);
        assert_eq!(wine_games.len(), 1);
    }
}
