// SPDX-License-Identifier: MIT
// SigmaOS - Gaming Module
// Gaming-related components including Steam, Wine, Proton integration

pub mod steam;

pub use steam::{
    SteamGame, SteamManager, ProtonVersion,
    init_steam_manager, get_steam_manager,
    launch_game_with_proton,
};
