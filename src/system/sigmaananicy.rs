//! SigmaAnanicy - Process Nice Management
//!
//! Inspired by Ananicy (Omarchy Linux), this module provides automatic process
//! nice/renice management for improved system responsiveness and CPU scheduling.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Process priority class enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityClass {
    Realtime,
    High,
    AboveNormal,
    Normal,
    BelowNormal,
    Low,
    Idle,
    Unknown,
;

/// Process category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessCategory {
    Background,
    Interactive,
    Game,
    Media,
    System,
    Custom,
    Unknown,
;

/// Process nice data structure
#[derive(Debug, Clone)]
pub struct ProcessNice {
    pub process_id: String,
    pub process_name: String,
    pub category: ProcessCategory,
    pub priority_class: PriorityClass,
    pub nice_value: i32,
    pub is_managed: bool,
;

impl ProcessNice {
    /// Create a new process nice entry
    pub fn new(
        process_id: String,
        process_name: String,
        category: ProcessCategory,
        priority_class: PriorityClass,
    ) -> Self {
        let nice_value = Self::priority_to_nice(priority_class);
        Self {
            process_id,
            process_name,
            category,
            priority_class,
            nice_value,
            is_managed: false,
        }
    }

    /// Convert priority class to nice value
    fn priority_to_nice(priority: PriorityClass) -> i32 {
        match priority {
            PriorityClass::Realtime => -20,
            PriorityClass::High => -10,
            PriorityClass::AboveNormal => -5,
            PriorityClass::Normal => 0,
            PriorityClass::BelowNormal => 5,
            PriorityClass::Low => 10,
            PriorityClass::Idle => 19,
            PriorityClass::Unknown => 0,
        }
    }

    /// Set managed state
    pub fn set_managed(&mut self, managed: bool) {
        self.is_managed = managed;
    }

    /// Update nice value
    pub fn update_nice(&mut self, nice: i32) {
        self.nice_value = nice;
    }

    /// Check if process is managed
    pub fn is_process_managed(&self) -> bool {
        self.is_managed
    }
;

/// Nice rule for automatic process classification
#[derive(Debug, Clone)]
pub struct NiceRule {
    pub rule_id: String,
    pub pattern: String,
    pub category: ProcessCategory,
    pub priority_class: PriorityClass,
    pub enabled: bool,
;

impl NiceRule {
    /// Create a new nice rule
    pub fn new(
        rule_id: String,
        pattern: String,
        category: ProcessCategory,
        priority_class: PriorityClass,
    ) -> Self {
        Self {
            rule_id,
            pattern,
            category,
            priority_class,
            enabled: true,
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

    /// Check if process matches rule
    pub fn matches(&self, process_name: &str) -> bool {
        process_name.contains(&self.pattern)
    }
;

/// Ananicy manager
pub struct SigmaAnanicy {
    processes: BTreeMap<String, ProcessNice>,
    rules: BTreeMap<String, NiceRule>,
    running: AtomicBool,
;

impl SigmaAnanicy {
    /// Create a new Ananicy manager
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            rules: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the Ananicy manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the Ananicy manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a process
    pub fn add_process(&mut self, process: ProcessNice) -> bool {
        let pid = process.process_id.clone();
        self.processes.insert(pid, process);
        true
    }

    /// Remove a process
    pub fn remove_process(&mut self, process_id: &str) -> Option<ProcessNice> {
        self.processes.remove(process_id)
    }

    /// Get a process by ID
    pub fn get_process(&self, process_id: &str) -> Option<&ProcessNice> {
        self.processes.get(process_id)
    }

    /// List all processes
    pub fn list_processes(&self) -> Vec<&ProcessNice> {
        self.processes.values().collect()
    }

    /// List processes by category
    pub fn list_processes_by_category(&self, category: ProcessCategory) -> Vec<&ProcessNice> {
        self.processes
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// List managed processes
    pub fn list_managed_processes(&self) -> Vec<&ProcessNice> {
        self.processes
            .values()
            .filter(|p| p.is_process_managed())
            .collect()
    }

    /// Auto-classify process using rules
    pub fn auto_classify(
        &mut self,
        process_id: &str,
        process_name: &str,
    ) -> Option<ProcessCategory> {
        for rule in self.rules.values() {
            if rule.is_enabled() && rule.matches(process_name) {
                return Some(rule.category);
            }
        }
        None
    }

    /// Set process nice value
    pub fn set_process_nice(&mut self, process_id: &str, nice: i32) -> bool {
        if let Some(process) = self.processes.get_mut(process_id) {
            process.update_nice(nice);
            process.set_managed(true);
            true
        } else {
            false
        }
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: NiceRule) -> bool {
        let rule_id = rule.rule_id.clone();
        self.rules.insert(rule_id, rule);
        true
    }

    /// Remove a rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Option<NiceRule> {
        self.rules.remove(rule_id)
    }

    /// Get a rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&NiceRule> {
        self.rules.get(rule_id)
    }

    /// List all rules
    pub fn list_rules(&self) -> Vec<&NiceRule> {
        self.rules.values().collect()
    }

    /// List enabled rules
    pub fn list_enabled_rules(&self) -> Vec<&NiceRule> {
        self.rules.values().filter(|r| r.is_enabled()).collect()
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
;

impl Default for SigmaAnanicy {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaAnanicy instance (placeholder for actual global state)
static mut SIGMAANANICY_INSTANCE: Option<SigmaAnanicy> = None;

/// Initialize the global SigmaAnanicy instance
pub fn init_sigmaananicy() {
    unsafe {
        SIGMAANANICY_INSTANCE = Some(SigmaAnanicy::new());
    }
;

/// Get the global SigmaAnanicy instance
pub fn get_sigmaananicy() -> Option<&'static SigmaAnanicy> {
    unsafe { SIGMAANANICY_INSTANCE.as_ref() }
;

/// Get mutable global SigmaAnanicy instance
pub fn get_sigmaananicy_mut() -> Option<&'static mut SigmaAnanicy> {
    unsafe { SIGMAANANICY_INSTANCE.as_mut() }
;

/// Convenience function to add a process
pub fn add_process(process: ProcessNice) -> bool {
    if let Some(ananicy) = get_sigmaananicy_mut() {
        ananicy.add_process(process)
    } else {
        false
    }
;

/// Convenience function to set process nice
pub fn set_process_nice(process_id: &str, nice: i32) -> bool {
    if let Some(ananicy) = get_sigmaananicy_mut() {
        ananicy.set_process_nice(process_id, nice)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_nice_creation() {
        let process = ProcessNice::new(
            "proc1".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        );

        assert_eq!(process.process_id, "proc1");
        assert_eq!(process.category, ProcessCategory::Interactive);
        assert_eq!(process.nice_value, 0);
    }

    #[test]
    fn test_priority_to_nice() {
        assert_eq!(ProcessNice::priority_to_nice(PriorityClass::Realtime), -20);
        assert_eq!(ProcessNice::priority_to_nice(PriorityClass::High), -10);
        assert_eq!(ProcessNice::priority_to_nice(PriorityClass::Normal), 0);
        assert_eq!(ProcessNice::priority_to_nice(PriorityClass::Low), 10);
        assert_eq!(ProcessNice::priority_to_nice(PriorityClass::Idle), 19);
    }

    #[test]
    fn test_nice_rule_creation() {
        let rule = NiceRule::new(
            "rule1".to_string(),
            "steam".to_string(),
            ProcessCategory::Game,
            PriorityClass::High,
        );

        assert_eq!(rule.rule_id, "rule1");
        assert_eq!(rule.category, ProcessCategory::Game);
        assert!(rule.is_enabled());
    }

    #[test]
    fn test_nice_rule_matches() {
        let rule = NiceRule::new(
            "rule1".to_string(),
            "steam".to_string(),
            ProcessCategory::Game,
            PriorityClass::High,
        );

        assert!(rule.matches("steam-runtime"));
        assert!(!rule.matches("firefox"));
    }

    #[test]
    fn test_sigmaananicy_creation() {
        let ananicy = SigmaAnanicy::new();
        assert!(!ananicy.is_running());
        assert_eq!(ananicy.process_count(), 0);
        assert_eq!(ananicy.rule_count(), 0);
    }

    #[test]
    fn test_sigmaananicy_start_stop() {
        let mut ananicy = SigmaAnanicy::new();
        ananicy.start();
        assert!(ananicy.is_running());
        ananicy.stop();
        assert!(!ananicy.is_running());
    }

    #[test]
    fn test_sigmaananicy_add_process() {
        let mut ananicy = SigmaAnanicy::new();
        let process = ProcessNice::new(
            "proc1".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        );

        let added = ananicy.add_process(process);
        assert!(added);
        assert_eq!(ananicy.process_count(), 1);
    }

    #[test]
    fn test_sigmaananicy_add_rule() {
        let mut ananicy = SigmaAnanicy::new();
        let rule = NiceRule::new(
            "rule1".to_string(),
            "steam".to_string(),
            ProcessCategory::Game,
            PriorityClass::High,
        );

        let added = ananicy.add_rule(rule);
        assert!(added);
        assert_eq!(ananicy.rule_count(), 1);
    }

    #[test]
    fn test_sigmaananicy_auto_classify() {
        let mut ananicy = SigmaAnanicy::new();

        ananicy.add_rule(NiceRule::new(
            "rule1".to_string(),
            "steam".to_string(),
            ProcessCategory::Game,
            PriorityClass::High,
        ));

        let category = ananicy.auto_classify("proc1", "steam-runtime");
        assert_eq!(category, Some(ProcessCategory::Game));
    }

    #[test]
    fn test_sigmaananicy_set_process_nice() {
        let mut ananicy = SigmaAnanicy::new();

        ananicy.add_process(ProcessNice::new(
            "proc1".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        ));

        let set = ananicy.set_process_nice("proc1", -5);
        assert!(set);

        let process = ananicy.get_process("proc1").unwrap();
        assert_eq!(process.nice_value, -5);
        assert!(process.is_process_managed());
    }

    #[test]
    fn test_sigmaananicy_list_by_category() {
        let mut ananicy = SigmaAnanicy::new();

        ananicy.add_process(ProcessNice::new(
            "proc1".to_string(),
            "steam".to_string(),
            ProcessCategory::Game,
            PriorityClass::High,
        ));

        ananicy.add_process(ProcessNice::new(
            "proc2".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        ));

        let games = ananicy.list_processes_by_category(ProcessCategory::Game);
        assert_eq!(games.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaananicy();

        let ananicy = get_sigmaananicy();
        assert!(ananicy.is_some());

        let mut ananicy_mut = get_sigmaananicy_mut();
        assert!(ananicy_mut.is_some());

        if let Some(ananicy) = ananicy_mut {
            ananicy.start();
            assert!(ananicy.is_running());
        }
    }

    #[test]
    fn test_convenience_add_process() {
        init_sigmaananicy();

        let process = ProcessNice::new(
            "proc1".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        );

        let added = add_process(process);
        assert!(added);

        let ananicy = get_sigmaananicy().unwrap();
        assert_eq!(ananicy.process_count(), 1);
    }

    #[test]
    fn test_convenience_set_process_nice() {
        init_sigmaananicy();

        let process = ProcessNice::new(
            "proc1".to_string(),
            "firefox".to_string(),
            ProcessCategory::Interactive,
            PriorityClass::Normal,
        );

        add_process(process);
        let set = set_process_nice("proc1", -5);
        assert!(set);

        let ananicy = get_sigmaananicy().unwrap();
        let proc = ananicy.get_process("proc1").unwrap();
        assert_eq!(proc.nice_value, -5);
    }
;
