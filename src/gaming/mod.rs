// SPDX-License-Identifier: MIT
// SigmaOS - Gaming Module
// Gaming-related components including Steam, Wine, Proton integration

pub mod gamemode;
pub mod gamescope;
pub mod heroic;
pub mod lutris;
pub mod mangohud;
pub mod steam;
pub mod wine;

pub use gamemode::{
    gamemode_query_status, gamemode_request_end, gamemode_request_start, get_gamemode_manager,
    init_gamemode_manager, GameModeConfig, GameModeInstance, GameModeManager,
    GameOptimizationState,
};
pub use gamescope::{
    get_gamescope, init_gamescope, GamescopeConfig, GamescopeInstance, PerformanceMetrics,
};
pub use heroic::{get_heroic_launcher, init_heroic_launcher, HeroicGame, HeroicLauncher};
pub use lutris::{
    get_lutris_manager, init_lutris_manager, LutrisGame, LutrisManager, LutrisRunner,
};
pub use mangohud::{
    get_mangohud_config, init_mangohud_config, MangoHudConfig, MangoHudOption, MangoHudPosition,
};
pub use steam::{
    get_steam_manager, init_steam_manager, launch_game_with_proton, ProtonVersion, SteamGame,
    SteamManager,
};
pub use wine::{
    create_default_prefix, get_wine_manager, init_wine_manager, ProtonConfig, WindowsVersion,
    WineApplication, WineArch, WineManager, WinePrefix,
};
