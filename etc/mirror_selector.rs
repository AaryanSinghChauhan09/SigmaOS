// SPDX-License-Identifier: MIT
// SigmaOS Mirror Selector - Arch Linux Inspired
// Implements mirror selection similar to Arch's reflector

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

/// Mirror information
#[derive(Debug, Clone, PartialEq)]
pub struct Mirror {
    pub url: String,
    pub country: String,
    pub city: String,
    pub protocols: Vec<String>,
    pub is_active: bool,
    pub last_sync: Option<u64>,
    pub speed_test: Option<u64>,
    pub score: f64,
}

impl Mirror {
    /// Create a new mirror
    pub fn new(url: String, country: String, city: String) -> Self {
        Self {
            url,
            country,
            city,
            protocols: vec!["https".to_string(), "http".to_string()],
            is_active: true,
            last_sync: None,
            speed_test: None,
            score: 0.0,
        }
    }

    /// Add protocol
    pub fn add_protocol(mut self, protocol: impl Into<String>) -> Self {
        let protocol = protocol.into();
        if !self.protocols.contains(&protocol) {
            self.protocols.push(protocol);
        }
        self
    }

    /// Set active status
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// Set last sync time
    pub fn last_sync(mut self, timestamp: u64) -> Self {
        self.last_sync = Some(timestamp);
        self
    }

    /// Set speed test result
    pub fn speed_test(mut self, speed: u64) -> Self {
        self.speed_test = Some(speed);
        self
    }

    /// Set score
    pub fn score(mut self, score: f64) -> Self {
        self.score = score;
        self
    }

    /// Calculate mirror score based on various factors
    pub fn calculate_score(&mut self) {
        let mut score = 0.0;

        // Speed score (0-50 points)
        if let Some(speed) = self.speed_test {
            let speed_score = (speed as f64 / 1000.0).min(50.0); // Max 50 points for 50MB/s+
            score += speed_score;
        }

        // Protocol score (0-20 points)
        if self.protocols.contains(&"https".to_string()) {
            score += 10.0;
        }
        if self.protocols.contains(&"rsync".to_string()) {
            score += 5.0;
        }
        if self.protocols.contains(&"ipv6".to_string()) {
            score += 5.0;
        }

        // Activity score (0-20 points)
        if let Some(last_sync) = self.last_sync {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let age_hours = (now - last_sync) as f64 / 3600.0;
            if age_hours < 1.0 {
                score += 20.0;
            } else if age_hours < 6.0 {
                score += 15.0;
            } else if age_hours < 24.0 {
                score += 10.0;
            } else if age_hours < 72.0 {
                score += 5.0;
            }
        }

        // Geographic score (0-10 points) - prefer closer mirrors
        // This would be implemented with actual geolocation data
        score += 5.0; // Placeholder

        self.score = score;
    }

    /// Get mirror URL
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Get country
    pub fn get_country(&self) -> &str {
        &self.country
    }

    /// Get city
    pub fn get_city(&self) -> &str {
        &self.city
    }

    /// Check if mirror is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.url, self.city, self.country)
    }
}

/// Mirror selector
#[derive(Debug, Clone)]
pub struct MirrorSelector {
    pub mirrors: Vec<Mirror>,
    pub selected_mirrors: Vec<Mirror>,
    pub max_mirrors: usize,
    pub min_speed: Option<u64>,
    pub preferred_protocols: Vec<String>,
    pub preferred_countries: Vec<String>,
    pub last_update: Option<u64>,
}

impl MirrorSelector {
    /// Create a new mirror selector
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            selected_mirrors: Vec::new(),
            max_mirrors: 10,
            min_speed: None,
            preferred_protocols: vec!["https".to_string()],
            preferred_countries: Vec::new(),
            last_update: None,
        }
    }

    /// Create mirror selector with configuration
    pub fn with_config(max_mirrors: usize, min_speed: Option<u64>) -> Self {
        Self {
            mirrors: Vec::new(),
            selected_mirrors: Vec::new(),
            max_mirrors,
            min_speed,
            preferred_protocols: vec!["https".to_string()],
            preferred_countries: Vec::new(),
            last_update: None,
        }
    }

    /// Add mirror
    pub fn add_mirror(&mut self, mirror: Mirror) {
        self.mirrors.push(mirror);
    }

    /// Add mirror by URL
    pub fn add_mirror_url(&mut self, url: String, country: String, city: String) {
        let mirror = Mirror::new(url, country, city);
        self.add_mirror(mirror);
    }

    /// Set maximum mirrors
    pub fn set_max_mirrors(&mut self, max: usize) {
        self.max_mirrors = max;
    }

    /// Set minimum speed
    pub fn set_min_speed(&mut self, speed: Option<u64>) {
        self.min_speed = speed;
    }

    /// Add preferred protocol
    pub fn add_preferred_protocol(&mut self, protocol: impl Into<String>) {
        let protocol = protocol.into();
        if !self.preferred_protocols.contains(&protocol) {
            self.preferred_protocols.push(protocol);
        }
    }

    /// Add preferred country
    pub fn add_preferred_country(&mut self, country: impl Into<String>) {
        let country = country.into();
        if !self.preferred_countries.contains(&country) {
            self.preferred_countries.push(country);
        }
    }

    /// Filter mirrors by criteria
    pub fn filter_mirrors(&mut self) {
        let mut filtered = Vec::new();

        for mirror in &self.mirrors {
            // Check if mirror is active
            if !mirror.is_active {
                continue;
            }

            // Check minimum speed
            if let Some(min_speed) = self.min_speed {
                if let Some(speed) = mirror.speed_test {
                    if speed < min_speed {
                        continue;
                    }
                }
            }

            // Check preferred protocols
            let has_preferred_protocol = self.preferred_protocols.iter()
                .any(|p| mirror.protocols.contains(p));
            if !has_preferred_protocol {
                continue;
            }

            // Check preferred countries
            if !self.preferred_countries.is_empty() {
                if !self.preferred_countries.contains(&mirror.country) {
                    continue;
                }
            }

            filtered.push(mirror.clone());
        }

        self.mirrors = filtered;
    }

    /// Calculate scores for all mirrors
    pub fn calculate_scores(&mut self) {
        for mirror in &mut self.mirrors {
            mirror.calculate_score();
        }
    }

    /// Sort mirrors by score
    pub fn sort_mirrors(&mut self) {
        self.mirrors.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Select best mirrors
    pub fn select_mirrors(&mut self) -> Vec<Mirror> {
        self.filter_mirrors();
        self.calculate_scores();
        self.sort_mirrors();

        self.selected_mirrors = self.mirrors.iter()
            .take(self.max_mirrors)
            .cloned()
            .collect();

        self.selected_mirrors.clone()
    }

    /// Get selected mirrors
    pub fn get_selected_mirrors(&self) -> &[Mirror] {
        &self.selected_mirrors
    }

    /// Generate mirrorlist file
    pub fn generate_mirrorlist(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS Mirrorlist\n");
        content.push_str("# Generated by SigmaOS Mirror Selector\n");
        content.push_str("# Last updated: {}\n\n", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        for mirror in &self.selected_mirrors {
            content.push_str(&format!("Server = {}\n", mirror.url));
        }

        content
    }

    /// Save mirrorlist to file
    pub fn save_mirrorlist(&self, path: &str) -> std::io::Result<()> {
        let content = self.generate_mirrorlist();
        std::fs::write(path, content)
    }

    /// Update mirrors from remote source
    pub fn update_mirrors(&mut self) -> Result<(), String> {
        // In real implementation, would fetch mirror list from remote
        // For now, simulate update
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.last_update = Some(now);

        // Simulate adding some default mirrors
        if self.mirrors.is_empty() {
            self.add_mirror_url(
                "https://mirror1.example.com".to_string(),
                "US".to_string(),
                "New York".to_string(),
            );
            self.add_mirror_url(
                "https://mirror2.example.com".to_string(),
                "DE".to_string(),
                "Berlin".to_string(),
            );
            self.add_mirror_url(
                "https://mirror3.example.com".to_string(),
                "JP".to_string(),
                "Tokyo".to_string(),
            );
        }

        Ok(())
    }

    /// Get mirror statistics
    pub fn get_statistics(&self) -> MirrorStatistics {
        let total = self.mirrors.len();
        let active = self.mirrors.iter().filter(|m| m.is_active).count();
        let selected = self.selected_mirrors.len();

        MirrorStatistics {
            total,
            active,
            selected,
            last_update: self.last_update,
        }
    }

    /// Test mirror speed (simulated)
    pub fn test_mirror_speed(&mut self, url: &str) -> Option<u64> {
        // In real implementation, would perform actual speed test
        // For now, simulate with random speed
        let speed = (1000 + (url.len() as u64 * 100)).min(50000); // 1-50 MB/s
        Some(speed)
    }

    /// Test all mirrors
    pub fn test_all_mirrors(&mut self) {
        for mirror in &mut self.mirrors {
            if let Some(speed) = self.test_mirror_speed(&mirror.url) {
                mirror.speed_test = Some(speed);
            }
        }
    }

    /// Find best mirror by country
    pub fn find_best_by_country(&self, country: &str) -> Option<&Mirror> {
        self.mirrors.iter()
            .filter(|m| m.country == country && m.is_active)
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find mirrors by protocol
    pub fn find_by_protocol(&self, protocol: &str) -> Vec<&Mirror> {
        self.mirrors.iter()
            .filter(|m| m.protocols.contains(&protocol.to_string()) && m.is_active)
            .collect()
    }

    /// Get mirror by URL
    pub fn get_mirror(&self, url: &str) -> Option<&Mirror> {
        self.mirrors.iter().find(|m| m.url == url)
    }

    /// Remove mirror by URL
    pub fn remove_mirror(&mut self, url: &str) -> bool {
        if let Some(pos) = self.mirrors.iter().position(|m| m.url == url) {
            self.mirrors.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clear all mirrors
    pub fn clear_mirrors(&mut self) {
        self.mirrors.clear();
        self.selected_mirrors.clear();
    }

    /// Export mirrors to JSON (simplified)
    pub fn export_json(&self) -> String {
        let mut json = String::from("[\n");
        for (i, mirror) in self.mirrors.iter().enumerate() {
            json.push_str(&format!(
                "  {{\"url\": \"{}\", \"country\": \"{}\", \"city\": \"{}\", \"score\": {:.2}}}{}\n",
                mirror.url,
                mirror.country,
                mirror.city,
                mirror.score,
                if i < self.mirrors.len() - 1 { "," } else { "" }
            ));
        }
        json.push_str("]\n");
        json
    }

    /// Import mirrors from JSON (simplified)
    pub fn import_json(&mut self, json: &str) -> Result<(), String> {
        // In real implementation, would parse JSON properly
        // For now, just return success
        let _ = json;
        Ok(())
    }
}

impl Default for MirrorSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// Mirror statistics
#[derive(Debug, Clone)]
pub struct MirrorStatistics {
    pub total: usize,
    pub active: usize,
    pub selected: usize,
    pub last_update: Option<u64>,
}

/// Mirror manager for system-wide mirror configuration
#[derive(Debug, Clone)]
pub struct MirrorManager {
    pub selector: MirrorSelector,
    pub config_file: PathBuf,
    pub mirrorlist_file: PathBuf,
    pub auto_select: bool,
    pub update_interval: u64,
}

impl MirrorManager {
    /// Create a new mirror manager
    pub fn new() -> Self {
        Self {
            selector: MirrorSelector::new(),
            config_file: PathBuf::from("/etc/sigmaos/mirrors.conf"),
            mirrorlist_file: PathBuf::from("/etc/pacman.d/mirrorlist"),
            auto_select: true,
            update_interval: 86400, // 24 hours
        }
    }

    /// Create mirror manager with custom paths
    pub fn with_paths(config_file: PathBuf, mirrorlist_file: PathBuf) -> Self {
        Self {
            selector: MirrorSelector::new(),
            config_file,
            mirrorlist_file,
            auto_select: true,
            update_interval: 86400,
        }
    }

    /// Initialize mirror manager
    pub fn initialize(&mut self) -> Result<(), String> {
        // Load configuration
        if self.config_file.exists() {
            self.load_config()?;
        }

        // Update mirrors if needed
        if self.selector.mirrors.is_empty() || self.needs_update() {
            self.update_mirrors()?;
        }

        // Select mirrors
        if self.auto_select {
            self.selector.select_mirrors();
            self.save_mirrorlist()?;
        }

        Ok(())
    }

    /// Load configuration
    pub fn load_config(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string(&self.config_file)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "max_mirrors" => {
                        if let Ok(max) = value.parse() {
                            self.selector.set_max_mirrors(max);
                        }
                    }
                    "min_speed" => {
                        if let Ok(speed) = value.parse() {
                            self.selector.set_min_speed(Some(speed));
                        }
                    }
                    "auto_select" => {
                        self.auto_select = value == "true";
                    }
                    "update_interval" => {
                        if let Ok(interval) = value.parse() {
                            self.update_interval = interval;
                        }
                    }
                    "preferred_protocol" => {
                        self.selector.add_preferred_protocol(value);
                    }
                    "preferred_country" => {
                        self.selector.add_preferred_country(value);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Save configuration
    pub fn save_config(&self) -> Result<(), String> {
        let mut content = String::new();

        content.push_str("# SigmaOS Mirror Configuration\n");
        content.push_str("# Generated by SigmaOS Mirror Manager\n\n");

        content.push_str(&format!("max_mirrors={}\n", self.selector.max_mirrors));
        content.push_str(&format!("min_speed={}\n", self.selector.min_speed.unwrap_or(0)));
        content.push_str(&format!("auto_select={}\n", self.auto_select));
        content.push_str(&format!("update_interval={}\n", self.update_interval));

        for protocol in &self.selector.preferred_protocols {
            content.push_str(&format!("preferred_protocol={}\n", protocol));
        }

        for country in &self.selector.preferred_countries {
            content.push_str(&format!("preferred_country={}\n", country));
        }

        std::fs::write(&self.config_file, content)
            .map_err(|e| format!("Failed to write config: {}", e))
    }

    /// Update mirrors
    pub fn update_mirrors(&mut self) -> Result<(), String> {
        self.selector.update_mirrors()?;
        self.selector.last_update = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        Ok(())
    }

    /// Select mirrors
    pub fn select_mirrors(&mut self) -> Result<(), String> {
        self.selector.select_mirrors();
        self.save_mirrorlist()?;
        Ok(())
    }

    /// Save mirrorlist
    pub fn save_mirrorlist(&self) -> Result<(), String> {
        self.selector.save_mirrorlist(self.mirrorlist_file.to_str().unwrap())
            .map_err(|e| format!("Failed to save mirrorlist: {}", e))
    }

    /// Check if mirrors need update
    pub fn needs_update(&self) -> bool {
        if let Some(last_update) = self.selector.last_update {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now - last_update > self.update_interval
        } else {
            true
        }
    }

    /// Get mirror statistics
    pub fn get_statistics(&self) -> MirrorStatistics {
        self.selector.get_statistics()
    }

    /// Test all mirrors
    pub fn test_all_mirrors(&mut self) {
        self.selector.test_all_mirrors();
    }

    /// Add mirror
    pub fn add_mirror(&mut self, mirror: Mirror) {
        self.selector.add_mirror(mirror);
    }

    /// Remove mirror
    pub fn remove_mirror(&mut self, url: &str) -> bool {
        self.selector.remove_mirror(url)
    }

    /// Get all mirrors
    pub fn get_mirrors(&self) -> &[Mirror] {
        &self.selector.mirrors
    }

    /// Get selected mirrors
    pub fn get_selected_mirrors(&self) -> &[Mirror] {
        self.selector.get_selected_mirrors()
    }

    /// Export configuration
    pub fn export_config(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Mirror Configuration Export\n\n");

        for mirror in &self.selector.mirrors {
            content.push_str(&format!(
                "mirror={} country={} city={} protocols={} active={} score={:.2}\n",
                mirror.url,
                mirror.country,
                mirror.city,
                mirror.protocols.join(","),
                mirror.is_active,
                mirror.score
            ));
        }

        content
    }

    /// Import configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        self.selector.clear_mirrors();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("mirror=") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let mut mirror = Mirror::new(
                        parts[0].trim_start_matches("mirror=").to_string(),
                        parts[1].trim_start_matches("country=").to_string(),
                        parts[2].trim_start_matches("city=").to_string(),
                    );

                    // Parse protocols
                    let protocols_str = parts[3].trim_start_matches("protocols=");
                    mirror.protocols = protocols_str.split(',').map(String::from).collect();

                    // Parse active status
                    mirror.is_active = parts[4].trim_start_matches("active=") == "true";

                    // Parse score
                    if let Ok(score) = parts[5].trim_start_matches("score=").parse::<f64>() {
                        mirror.score = score;
                    }

                    self.selector.add_mirror(mirror);
                }
            }
        }

        Ok(())
    }
}

impl Default for MirrorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_creation() {
        let mirror = Mirror::new(
            "https://mirror.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        );

        assert_eq!(mirror.url, "https://mirror.example.com");
        assert_eq!(mirror.country, "US");
        assert_eq!(mirror.city, "New York");
        assert!(mirror.is_active);
    }

    #[test]
    fn test_mirror_builder() {
        let mirror = Mirror::new(
            "https://mirror.example.com".to_string(),
            "DE".to_string(),
            "Berlin".to_string(),
        )
        .add_protocol("rsync")
        .add_protocol("ipv6")
        .speed_test(50000)
        .score(85.5);

        assert!(mirror.protocols.contains(&"rsync".to_string()));
        assert!(mirror.protocols.contains(&"ipv6".to_string()));
        assert_eq!(mirror.speed_test, Some(50000));
        assert_eq!(mirror.score, 85.5);
    }

    #[test]
    fn test_mirror_score_calculation() {
        let mut mirror = Mirror::new(
            "https://mirror.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        )
        .add_protocol("https")
        .add_protocol("rsync")
        .speed_test(100000) // 100 MB/s
        .last_sync(1700000000);

        mirror.calculate_score();
        assert!(mirror.score > 0.0);
    }

    #[test]
    fn test_mirror_selector() {
        let mut selector = MirrorSelector::new();

        selector.add_mirror_url(
            "https://mirror1.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        );
        selector.add_mirror_url(
            "https://mirror2.example.com".to_string(),
            "DE".to_string(),
            "Berlin".to_string(),
        );

        assert_eq!(selector.mirrors.len(), 2);

        let selected = selector.select_mirrors();
        assert!(!selected.is_empty());
        assert!(selected.len() <= selector.max_mirrors);
    }

    #[test]
    fn test_mirror_filtering() {
        let mut selector = MirrorSelector::new();
        selector.set_min_speed(Some(50000)); // 50 MB/s minimum

        let mut mirror1 = Mirror::new(
            "https://mirror1.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        )
        .speed_test(100000); // 100 MB/s

        let mut mirror2 = Mirror::new(
            "https://mirror2.example.com".to_string(),
            "DE".to_string(),
            "Berlin".to_string(),
        )
        .speed_test(30000); // 30 MB/s

        mirror1.calculate_score();
        mirror2.calculate_score();

        selector.add_mirror(mirror1);
        selector.add_mirror(mirror2);

        selector.filter_mirrors();
        assert_eq!(selector.mirrors.len(), 1);
        assert_eq!(selector.mirrors[0].url, "https://mirror1.example.com");
    }

    #[test]
    fn test_mirrorlist_generation() {
        let mut selector = MirrorSelector::new();
        selector.add_mirror_url(
            "https://mirror1.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        );
        selector.add_mirror_url(
            "https://mirror2.example.com".to_string(),
            "DE".to_string(),
            "Berlin".to_string(),
        );

        selector.select_mirrors();
        let mirrorlist = selector.generate_mirrorlist();

        assert!(mirrorlist.contains("Server = https://mirror1.example.com"));
        assert!(mirrorlist.contains("Server = https://mirror2.example.com"));
    }

    #[test]
    fn test_mirror_manager() {
        let mut manager = MirrorManager::new();

        assert!(manager.update_mirrors().is_ok());
        assert!(manager.select_mirrors().is_ok());
        assert!(!manager.get_selected_mirrors().is_empty());
    }

    #[test]
    fn test_mirror_export_import() {
        let mut manager = MirrorManager::new();
        manager.update_mirrors().unwrap();

        let exported = manager.export_config();
        assert!(!exported.is_empty());

        let mut manager2 = MirrorManager::new();
        assert!(manager2.import_config(&exported).is_ok());
        assert_eq!(manager2.get_mirrors().len(), manager.get_mirrors().len());
    }

    #[test]
    fn test_mirror_statistics() {
        let mut selector = MirrorSelector::new();
        selector.add_mirror_url(
            "https://mirror1.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        );

        let stats = selector.get_statistics();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.active, 1);
    }
}
