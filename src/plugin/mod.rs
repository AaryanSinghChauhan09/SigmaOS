// SigmaOS OOP-based Plugin System Module

pub mod framework;
pub mod marketplace;
pub mod system;

pub use framework::{
    PluginCapability as SovereignPluginCapability, PluginManifest, PluginSandboxedContext,
    SovereignPluginFramework,
};
pub use marketplace::{
    InstalledPluginRecord, MarketplaceCategory, MarketplaceListing, PluginMarketplaceEngine,
    PluginVerificationBadge,
};
pub use system::{
    ExtensionType, ManagerCapability, MarketplaceItem, Plugin, PluginCapability, PluginError,
    PluginID, PluginInfo, PluginManager, PluginMarketplace, PluginState, PluginStats, SimplePlugin,
    SimplePluginManager,
};
