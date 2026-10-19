//! SigmaNanny - Parental Controls and Content Filtering
//!
//! Inspired by Linux Mint's MintNanny, this module provides comprehensive parental
//! controls including content filtering, time limits, and usage tracking.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Filter category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterCategory {
    AdultContent,
    Gambling,
    Violence,
    Drugs,
    SocialMedia,
    Streaming,
    Gaming,
    Custom,
    Unknown,
}

/// Filter action enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterAction {
    Block,
    Warn,
    Allow,
    LogOnly,
    Unknown,
}

/// Time limit type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeLimitType {
    Daily,
    Weekly,
    Monthly,
    Unknown,
}

/// Usage record data structure
#[derive(Debug, Clone)]
pub struct UsageRecord {
    pub user_id: String,
    pub application: String,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_seconds: u64,
}

impl UsageRecord {
    /// Create a new usage record
    pub fn new(user_id: String, application: String, start_time: u64, end_time: u64) -> Self {
        let duration = if end_time > start_time {
            end_time - start_time
        } else {
            0
        };

        Self {
            user_id,
            application,
            start_time,
            end_time,
            duration_seconds: duration,
        }
    }
}

/// Content filter rule data structure
#[derive(Debug, Clone)]
pub struct FilterRule {
    pub rule_id: String,
    pub category: FilterCategory,
    pub pattern: String,
    pub action: FilterAction,
    pub enabled: bool,
    pub description: String,
}

impl FilterRule {
    /// Create a new filter rule
    pub fn new(
        rule_id: String,
        category: FilterCategory,
        pattern: String,
        action: FilterAction,
        description: String,
    ) -> Self {
        Self {
            rule_id,
            category,
            pattern,
            action,
            enabled: true,
            description,
        }
    }

    /// Enable rule
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable rule
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if rule is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Time limit data structure
#[derive(Debug, Clone)]
pub struct TimeLimit {
    pub limit_id: String,
    pub user_id: String,
    pub application: String,
    pub limit_type: TimeLimitType,
    pub limit_seconds: u64,
    pub used_seconds: u64,
    pub enabled: bool,
}

impl TimeLimit {
    /// Create a new time limit
    pub fn new(
        limit_id: String,
        user_id: String,
        application: String,
        limit_type: TimeLimitType,
        limit_seconds: u64,
    ) -> Self {
        Self {
            limit_id,
            user_id,
            application,
            limit_type,
            limit_seconds,
            used_seconds: 0,
            enabled: true,
        }
    }

    /// Add usage time
    pub fn add_usage(&mut self, seconds: u64) {
        self.used_seconds += seconds;
    }

    /// Reset usage
    pub fn reset_usage(&mut self) {
        self.used_seconds = 0;
    }

    /// Check if limit is exceeded
    pub fn is_exceeded(&self) -> bool {
        self.used_seconds >= self.limit_seconds
    }

    /// Get remaining time
    pub fn remaining_seconds(&self) -> u64 {
        if self.used_seconds >= self.limit_seconds {
            0
        } else {
            self.limit_seconds - self.used_seconds
        }
    }

    /// Enable limit
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable limit
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if limit is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Parental controls manager
pub struct SigmaNanny {
    filter_rules: BTreeMap<String, FilterRule>,
    time_limits: BTreeMap<String, TimeLimit>,
    usage_records: Vec<UsageRecord>,
    running: AtomicBool,
}

impl SigmaNanny {
    /// Create a new parental controls manager
    pub fn new() -> Self {
        Self {
            filter_rules: BTreeMap::new(),
            time_limits: BTreeMap::new(),
            usage_records: Vec::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the parental controls manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the parental controls manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a filter rule
    pub fn add_filter_rule(&mut self, rule: FilterRule) -> bool {
        let rule_id = rule.rule_id.clone();
        self.filter_rules.insert(rule_id, rule);
        true
    }

    /// Remove a filter rule
    pub fn remove_filter_rule(&mut self, rule_id: &str) -> Option<FilterRule> {
        self.filter_rules.remove(rule_id)
    }

    /// Get a filter rule by ID
    pub fn get_filter_rule(&self, rule_id: &str) -> Option<&FilterRule> {
        self.filter_rules.get(rule_id)
    }

    /// List all filter rules
    pub fn list_filter_rules(&self) -> Vec<&FilterRule> {
        self.filter_rules.values().collect()
    }

    /// List filter rules by category
    pub fn list_filter_rules_by_category(&self, category: FilterCategory) -> Vec<&FilterRule> {
        self.filter_rules
            .values()
            .filter(|r| r.category == category)
            .collect()
    }

    /// List enabled filter rules
    pub fn list_enabled_filter_rules(&self) -> Vec<&FilterRule> {
        self.filter_rules.values().filter(|r| r.enabled).collect()
    }

    /// Enable a filter rule
    pub fn enable_filter_rule(&mut self, rule_id: &str) -> bool {
        if let Some(rule) = self.filter_rules.get_mut(rule_id) {
            rule.enable();
            true
        } else {
            false
        }
    }

    /// Disable a filter rule
    pub fn disable_filter_rule(&mut self, rule_id: &str) -> bool {
        if let Some(rule) = self.filter_rules.get_mut(rule_id) {
            rule.disable();
            true
        } else {
            false
        }
    }

    /// Check content against filter rules
    pub fn check_content(&self, content: &str) -> FilterAction {
        for rule in self.filter_rules.values() {
            if !rule.enabled {
                continue;
            }

            if content.contains(&rule.pattern) {
                return rule.action;
            }
        }

        FilterAction::Allow
    }

    /// Add a time limit
    pub fn add_time_limit(&mut self, limit: TimeLimit) -> bool {
        let limit_id = limit.limit_id.clone();
        self.time_limits.insert(limit_id, limit);
        true
    }

    /// Remove a time limit
    pub fn remove_time_limit(&mut self, limit_id: &str) -> Option<TimeLimit> {
        self.time_limits.remove(limit_id)
    }

    /// Get a time limit by ID
    pub fn get_time_limit(&self, limit_id: &str) -> Option<&TimeLimit> {
        self.time_limits.get(limit_id)
    }

    /// List all time limits
    pub fn list_time_limits(&self) -> Vec<&TimeLimit> {
        self.time_limits.values().collect()
    }

    /// List time limits by user
    pub fn list_time_limits_by_user(&self, user_id: &str) -> Vec<&TimeLimit> {
        self.time_limits
            .values()
            .filter(|l| l.user_id == user_id)
            .collect()
    }

    /// List time limits by application
    pub fn list_time_limits_by_application(&self, application: &str) -> Vec<&TimeLimit> {
        self.time_limits
            .values()
            .filter(|l| l.application == application)
            .collect()
    }

    /// Add usage to time limit
    pub fn add_usage(&mut self, limit_id: &str, seconds: u64) -> bool {
        if let Some(limit) = self.time_limits.get_mut(limit_id) {
            limit.add_usage(seconds);
            true
        } else {
            false
        }
    }

    /// Reset time limit usage
    pub fn reset_time_limit(&mut self, limit_id: &str) -> bool {
        if let Some(limit) = self.time_limits.get_mut(limit_id) {
            limit.reset_usage();
            true
        } else {
            false
        }
    }

    /// Check if time limit is exceeded
    pub fn check_time_limit(&self, limit_id: &str) -> bool {
        if let Some(limit) = self.time_limits.get(limit_id) {
            limit.is_exceeded()
        } else {
            false
        }
    }

    /// Record usage
    pub fn record_usage(&mut self, record: UsageRecord) {
        self.usage_records.push(record);
    }

    /// Get usage records by user
    pub fn get_usage_by_user(&self, user_id: &str) -> Vec<&UsageRecord> {
        self.usage_records
            .iter()
            .filter(|r| r.user_id == user_id)
            .collect()
    }

    /// Get usage records by application
    pub fn get_usage_by_application(&self, application: &str) -> Vec<&UsageRecord> {
        self.usage_records
            .iter()
            .filter(|r| r.application == application)
            .collect()
    }

    /// Get total usage time for user
    pub fn get_total_usage_time(&self, user_id: &str) -> u64 {
        self.usage_records
            .iter()
            .filter(|r| r.user_id == user_id)
            .map(|r| r.duration_seconds)
            .sum()
    }

    /// Get filter rule count
    pub fn filter_rule_count(&self) -> usize {
        self.filter_rules.len()
    }

    /// Get time limit count
    pub fn time_limit_count(&self) -> usize {
        self.time_limits.len()
    }

    /// Get usage record count
    pub fn usage_record_count(&self) -> usize {
        self.usage_records.len()
    }
}

impl Default for SigmaNanny {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNanny instance (placeholder for actual global state)
static mut SIGMANANNY_INSTANCE: Option<SigmaNanny> = None;

/// Initialize the global SigmaNanny instance
pub fn init_sigmananny() {
    unsafe {
        SIGMANANNY_INSTANCE = Some(SigmaNanny::new());
    }
}

/// Get the global SigmaNanny instance
pub fn get_sigmananny() -> Option<&'static SigmaNanny> {
    unsafe { SIGMANANNY_INSTANCE.as_ref() }
}

/// Get mutable global SigmaNanny instance
pub fn get_sigmananny_mut() -> Option<&'static mut SigmaNanny> {
    unsafe { SIGMANANNY_INSTANCE.as_mut() }
}

/// Convenience function to add a filter rule
pub fn add_filter_rule(rule: FilterRule) -> bool {
    if let Some(nanny) = get_sigmananny_mut() {
        nanny.add_filter_rule(rule)
    } else {
        false
    }
}

/// Convenience function to check content
pub fn check_content(content: &str) -> FilterAction {
    if let Some(nanny) = get_sigmananny() {
        nanny.check_content(content)
    } else {
        FilterAction::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_rule_creation() {
        let rule = FilterRule::new(
            "rule1".to_string(),
            FilterCategory::AdultContent,
            "adult".to_string(),
            FilterAction::Block,
            "Block adult content".to_string(),
        );

        assert_eq!(rule.rule_id, "rule1");
        assert_eq!(rule.category, FilterCategory::AdultContent);
        assert_eq!(rule.action, FilterAction::Block);
        assert!(rule.is_enabled());
    }

    #[test]
    fn test_filter_rule_enable_disable() {
        let mut rule = FilterRule::new(
            "rule1".to_string(),
            FilterCategory::Gambling,
            "casino".to_string(),
            FilterAction::Block,
            "Block gambling sites".to_string(),
        );

        assert!(rule.is_enabled());
        rule.disable();
        assert!(!rule.is_enabled());
        rule.enable();
        assert!(rule.is_enabled());
    }

    #[test]
    fn test_time_limit_creation() {
        let limit = TimeLimit::new(
            "limit1".to_string(),
            "user1".to_string(),
            "game".to_string(),
            TimeLimitType::Daily,
            3600,
        );

        assert_eq!(limit.limit_id, "limit1");
        assert_eq!(limit.user_id, "user1");
        assert_eq!(limit.limit_seconds, 3600);
        assert_eq!(limit.used_seconds, 0);
        assert!(!limit.is_exceeded());
    }

    #[test]
    fn test_time_limit_usage() {
        let mut limit = TimeLimit::new(
            "limit1".to_string(),
            "user1".to_string(),
            "game".to_string(),
            TimeLimitType::Daily,
            3600,
        );

        limit.add_usage(1800);
        assert_eq!(limit.used_seconds, 1800);
        assert!(!limit.is_exceeded());

        limit.add_usage(1800);
        assert!(limit.is_exceeded());
    }

    #[test]
    fn test_time_limit_remaining() {
        let mut limit = TimeLimit::new(
            "limit1".to_string(),
            "user1".to_string(),
            "game".to_string(),
            TimeLimitType::Daily,
            3600,
        );

        assert_eq!(limit.remaining_seconds(), 3600);
        limit.add_usage(1800);
        assert_eq!(limit.remaining_seconds(), 1800);
        limit.add_usage(1800);
        assert_eq!(limit.remaining_seconds(), 0);
    }

    #[test]
    fn test_usage_record_creation() {
        let record = UsageRecord::new("user1".to_string(), "browser".to_string(), 1000, 2000);

        assert_eq!(record.user_id, "user1");
        assert_eq!(record.application, "browser");
        assert_eq!(record.duration_seconds, 1000);
    }

    #[test]
    fn test_sigmananny_creation() {
        let nanny = SigmaNanny::new();
        assert!(!nanny.is_running());
        assert_eq!(nanny.filter_rule_count(), 0);
        assert_eq!(nanny.time_limit_count(), 0);
    }

    #[test]
    fn test_sigmananny_start_stop() {
        let mut nanny = SigmaNanny::new();
        nanny.start();
        assert!(nanny.is_running());
        nanny.stop();
        assert!(!nanny.is_running());
    }

    #[test]
    fn test_sigmananny_add_filter_rule() {
        let mut nanny = SigmaNanny::new();
        let rule = FilterRule::new(
            "rule1".to_string(),
            FilterCategory::AdultContent,
            "adult".to_string(),
            FilterAction::Block,
            "Block adult content".to_string(),
        );

        let added = nanny.add_filter_rule(rule);
        assert!(added);
        assert_eq!(nanny.filter_rule_count(), 1);
    }

    #[test]
    fn test_sigmananny_check_content() {
        let mut nanny = SigmaNanny::new();
        nanny.add_filter_rule(FilterRule::new(
            "rule1".to_string(),
            FilterCategory::AdultContent,
            "adult".to_string(),
            FilterAction::Block,
            "Block adult content".to_string(),
        ));

        let action = nanny.check_content("adult site");
        assert_eq!(action, FilterAction::Block);

        let action = nanny.check_content("safe site");
        assert_eq!(action, FilterAction::Allow);
    }

    #[test]
    fn test_sigmananny_add_time_limit() {
        let mut nanny = SigmaNanny::new();
        let limit = TimeLimit::new(
            "limit1".to_string(),
            "user1".to_string(),
            "game".to_string(),
            TimeLimitType::Daily,
            3600,
        );

        let added = nanny.add_time_limit(limit);
        assert!(added);
        assert_eq!(nanny.time_limit_count(), 1);
    }

    #[test]
    fn test_sigmananny_add_usage() {
        let mut nanny = SigmaNanny::new();
        nanny.add_time_limit(TimeLimit::new(
            "limit1".to_string(),
            "user1".to_string(),
            "game".to_string(),
            TimeLimitType::Daily,
            3600,
        ));

        let added = nanny.add_usage("limit1", 1800);
        assert!(added);

        let limit = nanny.get_time_limit("limit1").unwrap();
        assert_eq!(limit.used_seconds, 1800);
    }

    #[test]
    fn test_sigmananny_record_usage() {
        let mut nanny = SigmaNanny::new();
        let record = UsageRecord::new("user1".to_string(), "browser".to_string(), 1000, 2000);

        nanny.record_usage(record);
        assert_eq!(nanny.usage_record_count(), 1);
    }

    #[test]
    fn test_sigmananny_get_usage_by_user() {
        let mut nanny = SigmaNanny::new();
        nanny.record_usage(UsageRecord::new(
            "user1".to_string(),
            "browser".to_string(),
            1000,
            2000,
        ));

        nanny.record_usage(UsageRecord::new(
            "user1".to_string(),
            "game".to_string(),
            2000,
            3000,
        ));

        nanny.record_usage(UsageRecord::new(
            "user2".to_string(),
            "browser".to_string(),
            1000,
            2000,
        ));

        let user1_usage = nanny.get_usage_by_user("user1");
        assert_eq!(user1_usage.len(), 2);

        let user2_usage = nanny.get_usage_by_user("user2");
        assert_eq!(user2_usage.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmananny();

        let nanny = get_sigmananny();
        assert!(nanny.is_some());

        let mut nanny_mut = get_sigmananny_mut();
        assert!(nanny_mut.is_some());

        if let Some(nanny) = nanny_mut {
            nanny.start();
            assert!(nanny.is_running());
        }
    }

    #[test]
    fn test_convenience_add_filter_rule() {
        init_sigmananny();

        let rule = FilterRule::new(
            "rule1".to_string(),
            FilterCategory::AdultContent,
            "adult".to_string(),
            FilterAction::Block,
            "Block adult content".to_string(),
        );

        let added = add_filter_rule(rule);
        assert!(added);

        let nanny = get_sigmananny().unwrap();
        assert_eq!(nanny.filter_rule_count(), 1);
    }

    #[test]
    fn test_convenience_check_content() {
        init_sigmananny();

        let rule = FilterRule::new(
            "rule1".to_string(),
            FilterCategory::AdultContent,
            "adult".to_string(),
            FilterAction::Block,
            "Block adult content".to_string(),
        );

        add_filter_rule(rule);
        let action = check_content("adult site");
        assert_eq!(action, FilterAction::Block);
    }
}
