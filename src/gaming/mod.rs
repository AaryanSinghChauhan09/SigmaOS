// SPDX-License-Identifier: MIT
// SigmaOS - Gaming Module
// Gaming-related components including Steam, Wine, Proton integration

pub mod steam;
pub mod wine;
pub mod gamemode;
pub mod mangohud;
pub mod lutris;
pub mod heroic;
pub mod gamescope;

pub use steam::{
    SteamGame, SteamManager, ProtonVersion,
    init_steam_manager, get_steam_manager,
    launch_game_with_proton,
};
pub use wine::{
    WinePrefix, WineArch, WindowsVersion, ProtonConfig, WineApplication, WineManager,
    init_wine_manager, get_wine_manager, create_default_prefix,
};
pub use gamemode::{
    GameModeInstance, GameModeManager, GameOptimizationState, GameModeConfig,
    init_gamemode_manager, get_gamemode_manager,
    gamemode_request_start, gamemode_request_end, gamemode_query_status,
};
pub use mangohud::{
    MangoHudConfig, MangoHudOption, MangoHudPosition,
    init_mangohud_config, get_mangohud_config,
};
pub use lutris::{
    LutrisGame, LutrisRunner, LutrisManager,
    init_lutris_manager, get_lutris_manager,
};
pub use heroic::{
    HeroicGame, HeroicLauncher,
    init_heroic_launcher, get_heroic_launcher,
};
pub use gamescope::{
    GamescopeInstance, GamescopeConfig, PerformanceMetrics,
    init_gamescope, get_gamescope,
};
