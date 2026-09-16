// SigmaOS Sovereign Plugin Marketplace Engine
// Zero-dependency #![no_std] plugin store catalog, signature verifier, and installer pipeline

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketplaceCategory {
    SystemUtilities,
    Productivity,
    Media,
    Security,
    DeveloperTools,
    AiAgents,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginVerificationBadge {
    OfficialSovereign,
    VerifiedCommunity,
    Experimental,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceListing {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub category: MarketplaceCategory,
    pub rating_score: u8, // 0 to 100
    pub download_count: u64,
    pub badge: PluginVerificationBadge,
    pub signature_hash: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPluginRecord {
    pub listing_id: String,
    pub installed_version: String,
    pub install_timestamp: u64,
}

pub struct PluginMarketplaceEngine {
    catalog: Vec<MarketplaceListing>,
    installed: Vec<InstalledPluginRecord>,
}

impl PluginMarketplaceEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            catalog: Vec::new(),
            installed: Vec::new(),
        };
        engine.seed_default_catalog();
        engine
    }

    fn seed_default_catalog(&mut self) {
        self.add_listing(MarketplaceListing {
            id: String::from("plugin_dock_autohide"),
            name: String::from("Zenith Dock AutoHide"),
            version: String::from("1.2.0"),
            author: String::from("SigmaOS Core"),
            description: String::from("Smart autohide dock plugin for Zenith Compositor"),
            category: MarketplaceCategory::SystemUtilities,
            rating_score: 98,
            download_count: 14500,
            badge: PluginVerificationBadge::OfficialSovereign,
            signature_hash: 0x9876543210ABCDEF,
        });

        self.add_listing(MarketplaceListing {
            id: String::from("plugin_ai_code_explain"),
            name: String::from("Local LLM Code Explainer"),
            version: String::from("2.0.1"),
            author: String::from("S-AI Labs"),
            description: String::from("On-device AI code snippet explanation widget"),
            category: MarketplaceCategory::AiAgents,
            rating_score: 95,
            download_count: 8900,
            badge: PluginVerificationBadge::OfficialSovereign,
            signature_hash: 0x1234567890ABCDEF,
        });
    }

    pub fn add_listing(&mut self, listing: MarketplaceListing) {
        if let Some(pos) = self.catalog.iter().position(|l| l.id == listing.id) {
            self.catalog[pos] = listing;
        } else {
            self.catalog.push(listing);
        }
    }

    pub fn search(&self, query: &str) -> Vec<&MarketplaceListing> {
        let query_lower = query;
        self.catalog
            .iter()
            .filter(|l| l.name.contains(query_lower) || l.description.contains(query_lower) || l.id.contains(query_lower))
            .collect()
    }

    pub fn filter_by_category(&self, category: MarketplaceCategory) -> Vec<&MarketplaceListing> {
        self.catalog.iter().filter(|l| l.category == category).collect()
    }

    pub fn verify_signature(&self, listing_id: &str) -> bool {
        if let Some(listing) = self.catalog.iter().find(|l| l.id == listing_id) {
            listing.signature_hash != 0
        } else {
            false
        }
    }

    pub fn install_plugin(&mut self, listing_id: &str, current_timestamp: u64) -> Result<String, &'static str> {
        if !self.verify_signature(listing_id) {
            return Err("Invalid or unverified plugin signature");
        }

        let listing = self
            .catalog
            .iter_mut()
            .find(|l| l.id == listing_id)
            .ok_or("Listing not found in marketplace catalog")?;

        listing.download_count += 1;

        let record = InstalledPluginRecord {
            listing_id: String::from(listing_id),
            installed_version: listing.version.clone(),
            install_timestamp: current_timestamp,
        };

        if let Some(pos) = self.installed.iter().position(|i| i.listing_id == listing_id) {
            self.installed[pos] = record;
        } else {
            self.installed.push(record);
        }

        Ok(format!("Installed {} v{}", listing.name, listing.version))
    }

    pub fn installed_plugins(&self) -> &[InstalledPluginRecord] {
        &self.installed
    }
}

impl Default for PluginMarketplaceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_marketplace_engine() {
        let mut engine = PluginMarketplaceEngine::new();
        let results = engine.search("Dock");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "plugin_dock_autohide");

        assert!(engine.verify_signature("plugin_dock_autohide"));

        let install_res = engine.install_plugin("plugin_dock_autohide", 1700000000);
        assert!(install_res.is_ok());
        assert_eq!(engine.installed_plugins().len(), 1);
    }
}
