//! SigmaSystemTools - System Utilities Collection
//!
//! Provides comprehensive system utilities inspired by Linux Mint and Omarchy.
//! Supports system monitoring, cleanup, maintenance, and user-friendly tools.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Tool category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCategory {
    System,
    Disk,
    Network,
    Security,
    Maintenance,
    Backup,
;

/// System tool
#[derive(Debug, Clone)]
pub struct SystemTool {
    pub id: String,
    pub name: String,
    pub category: ToolCategory,
    pub description: String,
    pub executable_path: String,
    pub installed: bool,
    pub enabled: bool,
;

impl SystemTool {
    pub fn new(id: String, name: String, category: ToolCategory, description: String, executable_path: String) -> Self {
        Self {
            id,
            name,
            category,
            description,
            executable_path,
            installed: false,
            enabled: true,
        }
    }

    pub fn set_installed(&mut self, installed: bool) {
        self.installed = installed;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_available(&self) -> bool {
        self.installed && self.enabled
    }
;

/// Maintenance task
#[derive(Debug, Clone)]
pub struct MaintenanceTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub frequency: String, // daily, weekly, monthly
    pub last_run: Option<String>,
    pub next_run: String,
    pub enabled: bool,
;

impl MaintenanceTask {
    pub fn new(id: String, name: String, description: String, frequency: String, next_run: String) -> Self {
        Self {
            id,
            name,
            description,
            frequency,
            last_run: None,
            next_run,
            enabled: true,
        }
    }

    pub fn set_last_run(&mut self, last_run: String) {
        self.last_run = Some(last_run);
    }

    pub fn set_next_run(&mut self, next_run: String) {
        self.next_run = next_run;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_due(&self, current_time: &str) -> bool {
        self.enabled && current_time >= &self.next_run
    }
;

/// System cleanup action
#[derive(Debug, Clone)]
pub struct CleanupAction {
    pub id: String,
    pub name: String,
    pub target_path: String,
    pub action_type: String, // cache, temp, logs, thumbnails
    pub size_freed: u64,
;

impl CleanupAction {
    pub fn new(id: String, name: String, target_path: String, action_type: String) -> Self {
        Self {
            id,
            name,
            target_path,
            action_type,
            size_freed: 0,
        }
    }

    pub fn set_size_freed(&mut self, size: u64) {
        self.size_freed = size;
    }
;

/// SigmaSystemTools - System utilities manager
#[derive(Debug)]
pub struct SigmaSystemTools {
    tools: BTreeMap<String, SystemTool>,
    maintenance_tasks: BTreeMap<String, MaintenanceTask>,
    cleanup_actions: Vec<CleanupAction>,
;

impl SigmaSystemTools {
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
            maintenance_tasks: BTreeMap::new(),
            cleanup_actions: Vec::new(),
        }
    }

    pub fn add_tool(&mut self, tool: SystemTool) {
        self.tools.insert(tool.id.clone(), tool);
    }

    pub fn remove_tool(&mut self, id: &str) -> bool {
        self.tools.remove(id).is_some()
    }

    pub fn get_tool(&self, id: &str) -> Option<&SystemTool> {
        self.tools.get(id)
    }

    pub fn get_tool_mut(&mut self, id: &str) -> Option<&mut SystemTool> {
        self.tools.get_mut(id)
    }

    pub fn add_maintenance_task(&mut self, task: MaintenanceTask) {
        self.maintenance_tasks.insert(task.id.clone(), task);
    }

    pub fn remove_maintenance_task(&mut self, id: &str) -> bool {
        self.maintenance_tasks.remove(id).is_some()
    }

    pub fn get_maintenance_task(&self, id: &str) -> Option<&MaintenanceTask> {
        self.maintenance_tasks.get(id)
    }

    pub fn get_maintenance_task_mut(&mut self, id: &str) -> Option<&mut MaintenanceTask> {
        self.maintenance_tasks.get_mut(id)
    }

    pub fn add_cleanup_action(&mut self, action: CleanupAction) {
        self.cleanup_actions.push(action);
    }

    pub fn get_due_tasks(&self, current_time: &str) -> Vec<&MaintenanceTask> {
        self.maintenance_tasks.values()
            .filter(|task| task.is_due(current_time))
            .collect()
    }

    pub fn list_tools(&self) -> Vec<&SystemTool> {
        self.tools.values().collect()
    }

    pub fn list_tools_by_category(&self, category: ToolCategory) -> Vec<&SystemTool> {
        self.tools.values()
            .filter(|tool| tool.category == category)
            .collect()
    }

    pub fn list_available_tools(&self) -> Vec<&SystemTool> {
        self.tools.values()
            .filter(|tool| tool.is_available())
            .collect()
    }

    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }

    pub fn task_count(&self) -> usize {
        self.maintenance_tasks.len()
    }

    pub fn cleanup_size(&self) -> u64 {
        self.cleanup_actions.iter().map(|action| action.size_freed).sum()
    }
;

impl Default for SigmaSystemTools {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaSystemTools instance
static mut SIGMASYSTEMTOOLS_INSTANCE: Option<SigmaSystemTools> = None;

/// Initialize the global SigmaSystemTools instance
pub fn init_sigmasystemtools() {
    unsafe {
        SIGMASYSTEMTOOLS_INSTANCE = Some(SigmaSystemTools::new());
    }
;

/// Get the global SigmaSystemTools instance
pub fn get_sigmasystemtools() -> Option<&'static SigmaSystemTools> {
    unsafe {
        SIGMASYSTEMTOOLS_INSTANCE.as_ref()
    }
;

/// Get the global SigmaSystemTools instance (mutable)
pub fn get_sigmasystemtools_mut() -> Option<&'static mut SigmaSystemTools> {
    unsafe {
        SIGMASYSTEMTOOLS_INSTANCE.as_mut()
    }
;

/// Convenience function to add a tool
pub fn add_tool(tool: SystemTool) -> bool {
    if let Some(tools) = get_sigmasystemtools_mut() {
        tools.add_tool(tool);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_tool_creation() {
        let tool = SystemTool::new(
            "tool1".to_string(),
            "Disk Cleaner".to_string(),
            ToolCategory::Disk,
            "Cleans disk space".to_string(),
            "/usr/bin/cleaner".to_string(),
        );
        assert_eq!(tool.id, "tool1");
        assert_eq!(tool.category, ToolCategory::Disk);
        assert!(!tool.installed);
    }

    #[test]
    fn test_system_tool_modification() {
        let mut tool = SystemTool::new(
            "tool1".to_string(),
            "Test".to_string(),
            ToolCategory::System,
            "Test tool".to_string(),
            "/path/tool".to_string(),
        );
        tool.set_installed(true);
        tool.set_enabled(true);
        assert!(tool.is_available());
    }

    #[test]
    fn test_maintenance_task_creation() {
        let task = MaintenanceTask::new(
            "task1".to_string(),
            "System Update".to_string(),
            "Update system packages".to_string(),
            "weekly".to_string(),
            "2024-01-01".to_string(),
        );
        assert_eq!(task.id, "task1");
        assert!(task.enabled);
    }

    #[test]
    fn test_maintenance_task_due() {
        let mut task = MaintenanceTask::new(
            "task1".to_string(),
            "Test".to_string(),
            "Test task".to_string(),
            "daily".to_string(),
            "2024-01-01".to_string(),
        );
        assert!(task.is_due("2024-01-02"));
        assert!(!task.is_due("2023-12-31"));
    }

    #[test]
    fn test_cleanup_action_creation() {
        let action = CleanupAction::new(
            "action1".to_string(),
            "Cache Cleanup".to_string(),
            "/var/cache".to_string(),
            "cache".to_string(),
        );
        assert_eq!(action.id, "action1");
        assert_eq!(action.action_type, "cache");
    }

    #[test]
    fn test_sigmasystemtools_creation() {
        let tools = SigmaSystemTools::new();
        assert_eq!(tools.tool_count(), 0);
        assert_eq!(tools.task_count(), 0);
    }

    #[test]
    fn test_sigmasystemtools_management() {
        let mut tools = SigmaSystemTools::new();
        let tool = SystemTool::new(
            "tool1".to_string(),
            "Test".to_string(),
            ToolCategory::System,
            "Test".to_string(),
            "/path".to_string(),
        );
        tools.add_tool(tool);
        assert_eq!(tools.tool_count(), 1);
    }
;
