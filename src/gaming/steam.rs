// SPDX-License-Identifier: MIT
// SigmaOS - Steam Integration Framework
// Inspired by Steam client and Proton
// Provides Steam integration framework for game management and Proton compatibility

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Steam game
#[derive(Debug, Clone)]
pub struct SteamGame {
    pub app_id: u32,
    pub name: String,
    pub installed: bool,
    pub running: bool,
    pub proton_compat: bool,
    pub install_path: String,
    pub playtime: u32, // minutes
}

/// Proton version
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtonVersion {
    Proton5,
    Proton6,
    Proton7,
    Proton8,
    Proton9,
    ProtonExperimental,
    Custom(String),
}

/// Steam integration manager
pub struct SteamManager {
    pub games: BTreeMap<u32, SteamGame>,
    pub running_games: Vec<u32>,
    pub proton_version: ProtonVersion,
    pub steam_path: String,
    pub active: AtomicBool,
}

impl SteamManager {
    pub fn new() -> Self {
        Self {
            games: BTreeMap::new(),
            running_games: Vec::new(),
            proton_version: ProtonVersion::Proton9,
            steam_path: "~/.steam/steam".to_string(),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize Steam manager
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Add game
    pub fn add_game(&mut self, game: SteamGame) {
        self.games.insert(game.app_id, game);
    }
    
    /// Get game
    pub fn get_game(&self, app_id: u32) -> Option<&SteamGame> {
        self.games.get(&app_id)
    }
    
    /// Launch game
    pub fn launch_game(&mut self, app_id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&app_id).ok_or("Game not found")?;
        
        if !game.installed {
            return Err("Game not installed");
        }
        
        game.running = true;
        self.running_games.push(app_id);
        
        // In real implementation, would launch Steam command:
        // steam steam://run/{app_id}
        // Or with Proton:
        // STEAM_COMPAT_DATA_PATH="~/.steam/steam/compatdata/{app_id}" \
        //   steam steam://run/{app_id}
        
        Ok(())
    }
    
    /// Stop game
    pub fn stop_game(&mut self, app_id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&app_id).ok_or("Game not found")?;
        
        game.running = false;
        self.running_games.retain(|&id| id != app_id);
        
        // In real implementation, would send signal to game process
        
        Ok(())
    }
    
    /// Install game
    pub fn install_game(&mut self, app_id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&app_id).ok_or("Game not found")?;
        
        game.installed = true;
        
        // In real implementation, would run:
        // steam steam://install/{app_id}
        
        Ok(())
    }
    
    /// Set Proton version
    pub fn set_proton_version(&mut self, version: ProtonVersion) {
        self.proton_version = version;
    }
    
    /// Enable Proton for game
    pub fn enable_proton(&mut self, app_id: u32) -> Result<(), &'static str> {
        let game = self.games.get_mut(&app_id).ok_or("Game not found")?;
        game.proton_compat = true;
        Ok(())
    }
    
    /// Get running games
    pub fn get_running_games(&self) -> Vec<&SteamGame> {
        self.running_games.iter()
            .filter_map(|&id| self.games.get(&id))
            .collect()
    }
    
    /// Is game running
    pub fn is_game_running(&self, app_id: u32) -> bool {
        self.running_games.contains(&app_id)
    }
    
    /// Get total playtime
    pub fn get_total_playtime(&self) -> u32 {
        self.games.values().map(|g| g.playtime).sum()
    }
}

impl Default for SteamManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Steam manager
static mut STEAM_MANAGER: Option<SteamManager> = None;

/// Initialize Steam manager
pub fn init_steam_manager() {
    unsafe {
        STEAM_MANAGER = Some(SteamManager::new());
    }
}

/// Get global Steam manager
pub fn get_steam_manager() -> Option<&'static mut SteamManager> {
    unsafe {
        STEAM_MANAGER.as_mut()
    }
}

/// Launch game with Proton
pub fn launch_game_with_proton(app_id: u32, proton_version: ProtonVersion) -> Result<(), &'static str> {
    if let Some(manager) = get_steam_manager() {
        manager.set_proton_version(proton_version);
        manager.enable_proton(app_id)?;
        manager.launch_game(app_id)
    } else {
        Err("Steam manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_steam_manager_init() {
        let mut manager = SteamManager::new();
        manager.init().unwrap();
        assert!(manager.active.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_add_game() {
        let mut manager = SteamManager::new();
        let game = SteamGame {
            app_id: 730,
            name: "Counter-Strike 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/CS2".to_string(),
            playtime: 1000,
        };
        manager.add_game(game);
        assert!(manager.get_game(730).is_some());
    }
    
    #[test]
    fn test_launch_game() {
        let mut manager = SteamManager::new();
        let game = SteamGame {
            app_id: 730,
            name: "Counter-Strike 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/CS2".to_string(),
            playtime: 1000,
        };
        manager.add_game(game);
        manager.launch_game(730).unwrap();
        assert!(manager.is_game_running(730));
    }
    
    #[test]
    fn test_stop_game() {
        let mut manager = SteamManager::new();
        let game = SteamGame {
            app_id: 730,
            name: "Counter-Strike 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/CS2".to_string(),
            playtime: 1000,
        };
        manager.add_game(game);
        manager.launch_game(730).unwrap();
        manager.stop_game(730).unwrap();
        assert!(!manager.is_game_running(730));
    }
    
    #[test]
    fn test_enable_proton() {
        let mut manager = SteamManager::new();
        let game = SteamGame {
            app_id: 730,
            name: "Counter-Strike 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/CS2".to_string(),
            playtime: 1000,
        };
        manager.add_game(game);
        manager.enable_proton(730).unwrap();
        assert!(manager.get_game(730).unwrap().proton_compat);
    }
    
    #[test]
    fn test_get_running_games() {
        let mut manager = SteamManager::new();
        let game1 = SteamGame {
            app_id: 730,
            name: "Counter-Strike 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/CS2".to_string(),
            playtime: 1000,
        };
        let game2 = SteamGame {
            app_id: 570,
            name: "Dota 2".to_string(),
            installed: true,
            running: false,
            proton_compat: false,
            install_path: "/home/user/.steam/steam/steamapps/common/Dota 2".to_string(),
            playtime: 500,
        };
        manager.add_game(game1);
        manager.add_game(game2);
        manager.launch_game(730).unwrap();
        manager.launch_game(570).unwrap();
        let running = manager.get_running_games();
        assert_eq!(running.len(), 2);
    }
}
