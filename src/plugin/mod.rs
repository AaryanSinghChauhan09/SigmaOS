// SigmaOS OOP-based Plugin System Module

pub mod system;

pub use system::{
    ExtensionType, ManagerCapability, MarketplaceItem, Plugin, PluginCapability, PluginError,
    PluginID, PluginInfo, PluginManager, PluginMarketplace, PluginState, PluginStats, SimplePlugin,
    SimplePluginManager,
};
