// SPDX-License-Identifier: MIT
// SigmaOS Rolling Release System - Arch Linux Inspired
// Implements rolling release update mechanisms following Arch's philosophy

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Release channel information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseChannel {
    pub name: String,
    pub version: String,
    pub stability: ReleaseStability,
    pub release_date: u64,
    pub end_of_life: Option<u64>,
    pub is_active: bool,
}

/// Release stability levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseStability {
    Experimental,
    Testing,
    Stable,
    Lts,
}

/// Package update information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageUpdate {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub update_type: UpdateType,
    pub changelog: Option<String>,
    pub download_url: Option<String>,
    pub requires_restart: bool,
}

/// Types of package updates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    Security,
    Bugfix,
    Feature,
    Major,
    Dependency,
}

/// System update information
#[derive(Debug, Clone)]
pub struct SystemUpdate {
    pub updates: Vec<PackageUpdate>,
    pub total_size: u64,
    pub estimated_time: Option<u64>,
    pub requires_restart: bool,
    pub update_timestamp: u64,
}

/// News feed for releases
#[derive(Debug, Clone)]
pub struct NewsFeed {
    pub id: String,
    pub title: String,
    pub content: String,
    pub date: u64,
    pub tags: Vec<String>,
    pub severity: NewsSeverity,
}

/// News severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewsSeverity {
    Info,
    Warning,
    Critical,
    Security,
}

/// Rolling release manager
#[derive(Debug, Clone)]
pub struct RollingReleaseManager {
    pub channels: BTreeMap<String, ReleaseChannel>,
    pub active_channel: String,
    pub current_version: String,
    pub last_update_check: Option<u64>,
    pub news_feed: Vec<NewsFeed>,
    pub auto_update: bool,
    pub check_interval: u64,
    pub notify_updates: bool,
}

impl RollingReleaseManager {
    /// Create a new rolling release manager
    pub fn new() -> Self {
        let mut channels = BTreeMap::new();

        // Default channels inspired by Arch
        channels.insert("stable".to_string(), ReleaseChannel {
            name: "stable".to_string(),
            version: "2024.11.01".to_string(),
            stability: ReleaseStability::Stable,
            release_date: 1730419200, // Nov 1, 2024
            end_of_life: None,
            is_active: true,
        });

        channels.insert("testing".to_string(), ReleaseChannel {
            name: "testing".to_string(),
            version: "2024.11.08".to_string(),
            stability: ReleaseStability::Testing,
            release_date: 1731024000, // Nov 8, 2024
            end_of_life: None,
            is_active: true,
        });

        channels.insert("experimental".to_string(), ReleaseChannel {
            name: "experimental".to_string(),
            version: "2024.11.09".to_string(),
            stability: ReleaseStability::Experimental,
            release_date: 1731110400, // Nov 9, 2024
            end_of_life: None,
            is_active: true,
        });

        Self {
            channels,
            active_channel: "stable".to_string(),
            current_version: "2024.11.01".to_string(),
            last_update_check: None,
            news_feed: Vec::new(),
            auto_update: false,
            check_interval: 86400, // 24 hours
            notify_updates: true,
        }
    }

    /// Get active channel
    pub fn get_active_channel(&self) -> Option<&ReleaseChannel> {
        self.channels.get(&self.active_channel)
    }

    /// Set active channel
    pub fn set_active_channel(&mut self, channel: String) -> Result<(), String> {
        if self.channels.contains_key(&channel) {
            self.active_channel = channel;
            Ok(())
        } else {
            Err(format!("Channel '{}' not found", channel))
        }
    }

    /// Check for updates
    pub fn check_updates(&mut self) -> Result<Vec<PackageUpdate>, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();

        // Check if we should check for updates
        if let Some(last_check) = self.last_update_check {
            if now - last_check < self.check_interval {
                return Ok(Vec::new()); // Too soon to check again
            }
        }

        self.last_update_check = Some(now);

        // Simulate checking for updates
        // In real implementation, would query package repositories
        let updates = Vec::new();

        if self.notify_updates && !updates.is_empty() {
            self.notify_updates_available(&updates);
        }

        Ok(updates)
    }

    /// Perform system update
    pub fn perform_update(&mut self, updates: Vec<PackageUpdate>) -> Result<SystemUpdate, String> {
        if updates.is_empty() {
            return Err("No updates available".to_string());
        }

        let total_size: u64 = updates.iter()
            .map(|u| u.new_version.len() as u64 * 1024) // Simulate size calculation
            .sum();

        let requires_restart = updates.iter().any(|u| u.requires_restart);

        let system_update = SystemUpdate {
            updates: updates.clone(),
            total_size,
            estimated_time: Some(total_size / 1024 / 1024), // Rough estimate in seconds
            requires_restart,
            update_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| format!("Time error: {}", e))?
                .as_secs(),
        };

        // Update current version if it's a major update
        if let Some(major_update) = updates.iter().find(|u| u.update_type == UpdateType::Major) {
            self.current_version = major_update.new_version.clone();
        }

        Ok(system_update)
    }

    /// Add news feed item
    pub fn add_news(&mut self, news: NewsFeed) {
        self.news_feed.push(news);
        // Keep only recent news (last 100 items)
        if self.news_feed.len() > 100 {
            self.news_feed.remove(0);
        }
    }

    /// Get recent news
    pub fn get_recent_news(&self, limit: usize) -> Vec<&NewsFeed> {
        let mut news: Vec<&NewsFeed> = self.news_feed.iter().collect();
        news.sort_by(|a, b| b.date.cmp(&a.date));
        news.truncate(limit);
        news
    }

    /// Get news by severity
    pub fn get_news_by_severity(&self, severity: &NewsSeverity) -> Vec<&NewsFeed> {
        self.news_feed
            .iter()
            .filter(|n| &n.severity == severity)
            .collect()
    }

    /// Check if system needs update
    pub fn needs_update(&self) -> bool {
        if let Some(last_check) = self.last_update_check {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now - last_check > self.check_interval
        } else {
            true
        }
    }

    /// Get update status
    pub fn get_update_status(&self) -> UpdateStatus {
        UpdateStatus {
            last_check: self.last_update_check,
            next_check: self.last_update_check.map(|t| t + self.check_interval),
            auto_update: self.auto_update,
            active_channel: self.active_channel.clone(),
            current_version: self.current_version.clone(),
        }
    }

    /// Notify about available updates
    fn notify_updates_available(&self, updates: &[PackageUpdate]) {
        if self.notify_updates {
            println!(":: {} updates available", updates.len());
            for update in updates.iter().take(5) {
                println!("   {} -> {}", update.name, update.new_version);
            }
            if updates.len() > 5 {
                println!("   ... and {} more", updates.len() - 5);
            }
        }
    }

    /// Add a new release channel
    pub fn add_channel(&mut self, channel: ReleaseChannel) {
        self.channels.insert(channel.name.clone(), channel);
    }

    /// Remove a release channel
    pub fn remove_channel(&mut self, name: &str) -> Result<(), String> {
        if name == self.active_channel {
            return Err("Cannot remove active channel".to_string());
        }
        if self.channels.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Channel '{}' not found", name))
        }
    }

    /// Get all channels
    pub fn get_channels(&self) -> Vec<&ReleaseChannel> {
        self.channels.values().collect()
    }

    /// Get channel by name
    pub fn get_channel(&self, name: &str) -> Option<&ReleaseChannel> {
        self.channels.get(name)
    }

    /// Set auto-update
    pub fn set_auto_update(&mut self, enabled: bool) {
        self.auto_update = enabled;
    }

    /// Set check interval
    pub fn set_check_interval(&mut self, interval: u64) {
        self.check_interval = interval;
    }

    /// Set notify updates
    pub fn set_notify_updates(&mut self, enabled: bool) {
        self.notify_updates = enabled;
    }
}

impl Default for RollingReleaseManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Update status information
#[derive(Debug, Clone)]
pub struct UpdateStatus {
    pub last_check: Option<u64>,
    pub next_check: Option<u64>,
    pub auto_update: bool,
    pub active_channel: String,
    pub current_version: String,
}

/// Update checker for specific package types
#[derive(Debug, Clone)]
pub struct UpdateChecker {
    pub package_manager: String,
    pub repositories: Vec<String>,
    pub check_signature: bool,
    pub verify_checksums: bool,
}

impl UpdateChecker {
    /// Create a new update checker
    pub fn new(package_manager: impl Into<String>) -> Self {
        Self {
            package_manager: package_manager.into(),
            repositories: Vec::new(),
            check_signature: true,
            verify_checksums: true,
        }
    }

    /// Add repository
    pub fn add_repository(mut self, repo: impl Into<String>) -> Self {
        self.repositories.push(repo.into());
        self
    }

    /// Enable signature checking
    pub fn check_signature(mut self, enable: bool) -> Self {
        self.check_signature = enable;
        self
    }

    /// Enable checksum verification
    pub fn verify_checksums(mut self, enable: bool) -> Self {
        self.verify_checksums = enable;
        self
    }

    /// Check for package updates
    pub fn check_package_updates(&self, package_name: &str) -> Result<Vec<PackageUpdate>, String> {
        // Simulate checking for updates
        // In real implementation, would query package manager
        Ok(Vec::new())
    }

    /// Check for system updates
    pub fn check_system_updates(&self) -> Result<Vec<PackageUpdate>, String> {
        // Simulate checking for updates
        // In real implementation, would query all repositories
        Ok(Vec::new())
    }
}

/// Rolling release news generator
pub struct NewsGenerator;

impl NewsGenerator {
    /// Generate news for a new release
    pub fn generate_release_news(channel: &ReleaseChannel) -> NewsFeed {
        NewsFeed {
            id: format!("release-{}", channel.version),
            title: format!("SigmaOS {} Release", channel.version),
            content: format!(
                "A new {} release is available.\n\nVersion: {}\nStability: {:?}\nRelease Date: {}",
                channel.name,
                channel.version,
                channel.stability,
                channel.release_date
            ),
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tags: vec!["release".to_string(), channel.stability.to_string().to_lowercase()],
            severity: NewsSeverity::Info,
        }
    }

    /// Generate security advisory news
    pub fn generate_security_news(title: &str, description: &str, severity: NewsSeverity) -> NewsFeed {
        NewsFeed {
            id: format!("security-{}", SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()),
            title: title.to_string(),
            content: description.to_string(),
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tags: vec!["security".to_string()],
            severity,
        }
    }

    /// Generate maintenance news
    pub fn generate_maintenance_news(title: &str, description: &str) -> NewsFeed {
        NewsFeed {
            id: format!("maintenance-{}", SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()),
            title: title.to_string(),
            content: description.to_string(),
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tags: vec!["maintenance".to_string()],
            severity: NewsSeverity::Info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_channel() {
        let channel = ReleaseChannel {
            name: "stable".to_string(),
            version: "2024.11.01".to_string(),
            stability: ReleaseStability::Stable,
            release_date: 1730419200,
            end_of_life: None,
            is_active: true,
        };

        assert_eq!(channel.name, "stable");
        assert_eq!(channel.stability, ReleaseStability::Stable);
    }

    #[test]
    fn test_rolling_release_manager() {
        let mut manager = RollingReleaseManager::new();

        assert_eq!(manager.active_channel, "stable");
        assert!(manager.get_active_channel().is_some());

        // Test channel switching
        assert!(manager.set_active_channel("testing".to_string()).is_ok());
        assert_eq!(manager.active_channel, "testing");

        // Test invalid channel
        assert!(manager.set_active_channel("invalid".to_string()).is_err());
    }

    #[test]
    fn test_update_checking() {
        let mut manager = RollingReleaseManager::new();
        manager.set_check_interval(0); // Check immediately

        let updates = manager.check_updates().unwrap();
        assert!(updates.is_empty()); // No updates in simulation
    }

    #[test]
    fn test_news_feed() {
        let mut manager = RollingReleaseManager::new();

        let news = NewsFeed {
            id: "test-1".to_string(),
            title: "Test News".to_string(),
            content: "This is a test news item".to_string(),
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tags: vec!["test".to_string()],
            severity: NewsSeverity::Info,
        };

        manager.add_news(news);

        let recent = manager.get_recent_news(10);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].title, "Test News");
    }

    #[test]
    fn test_news_generator() {
        let channel = ReleaseChannel {
            name: "stable".to_string(),
            version: "2024.11.01".to_string(),
            stability: ReleaseStability::Stable,
            release_date: 1730419200,
            end_of_life: None,
            is_active: true,
        };

        let news = NewsGenerator::generate_release_news(&channel);
        assert!(news.title.contains("2024.11.01"));
        assert_eq!(news.severity, NewsSeverity::Info);
    }

    #[test]
    fn test_update_status() {
        let manager = RollingReleaseManager::new();
        let status = manager.get_update_status();

        assert_eq!(status.active_channel, "stable");
        assert_eq!(status.current_version, "2024.11.01");
        assert!(!status.auto_update);
    }
}
