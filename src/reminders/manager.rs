//! SigmaOS Reminders System
//!
//! Inspired by: Omarchy reminders (interactive reminder setup flow, desktop notification reminders)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SECURE_DEFAULTS, DETERMINISTIC TESTING, MEMORY_SAFETY,
//! USER_FREEDOM_AND_CONTROL, COMMAND_QUERY_SEPARATION.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Reminder priority
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReminderPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Reminder status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReminderStatus {
    Pending,
    Active,
    Completed,
    Dismissed,
    Overdue,
}

/// Reminder trigger type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReminderTrigger {
    /// Time-based reminder (at a specific timestamp)
    AtTime(u64),
    /// Duration-based reminder (in N seconds)
    InSeconds(u64),
    /// Recurring reminder (every N seconds)
    Recurring(u64),
}

/// Reminder entry
#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub trigger: ReminderTrigger,
    pub priority: ReminderPriority,
    pub status: ReminderStatus,
    pub created_at: u64,
    pub triggered_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub recurring_count: u32,
}

/// Reminder manager
pub struct ReminderManager {
    pub reminders: VecDeque<Reminder>,
    pub next_id: u64,
    pub max_reminders: usize,
}

impl ReminderManager {
    pub fn new() -> Self {
        Self {
            reminders: VecDeque::new(),
            next_id: 1,
            max_reminders: 1000,
        }
    }

    /// Create a new reminder
    pub fn create(
        &mut self,
        title: String,
        description: Option<String>,
        trigger: ReminderTrigger,
        priority: ReminderPriority,
    ) -> Result<u64, String> {
        if title.is_empty() {
            return Err("Reminder title cannot be empty".to_string());
        }

        let id = self.next_id;
        self.next_id += 1;
        let now = now_secs();

        let reminder = Reminder {
            id,
            title,
            description,
            trigger: trigger.clone(),
            priority,
            status: ReminderStatus::Pending,
            created_at: now,
            triggered_at: None,
            completed_at: None,
            recurring_count: 0,
        };

        self.reminders.push_back(reminder);

        // Enforce max limit
        while self.reminders.len() > self.max_reminders {
            self.reminders.pop_front();
        }

        Ok(id)
    }

    /// Create a time-based reminder
    pub fn create_at_time(
        &mut self,
        title: String,
        description: Option<String>,
        at_timestamp: u64,
        priority: ReminderPriority,
    ) -> Result<u64, String> {
        let now = now_secs();
        if at_timestamp < now {
            return Err("Reminder time must be in the future".to_string());
        }
        self.create(title, description, ReminderTrigger::AtTime(at_timestamp), priority)
    }

    /// Create a duration-based reminder
    pub fn create_in_seconds(
        &mut self,
        title: String,
        description: Option<String>,
        seconds: u64,
        priority: ReminderPriority,
    ) -> Result<u64, String> {
        if seconds == 0 {
            return Err("Seconds must be greater than 0".to_string());
        }
        self.create(
            title,
            description,
            ReminderTrigger::InSeconds(seconds),
            priority,
        )
    }

    /// Create a recurring reminder
    pub fn create_recurring(
        &mut self,
        title: String,
        description: Option<String>,
        interval_seconds: u64,
        priority: ReminderPriority,
    ) -> Result<u64, String> {
        if interval_seconds < 60 {
            return Err("Recurring interval must be at least 60 seconds".to_string());
        }
        self.create(
            title,
            description,
            ReminderTrigger::Recurring(interval_seconds),
            priority,
        )
    }

    /// Get reminders due now (triggered or overdue)
    pub fn get_due(&self, now: u64) -> Vec<&Reminder> {
        self.reminders
            .iter()
            .filter(|r| {
                r.status == ReminderStatus::Pending
                    && match &r.trigger {
                        ReminderTrigger::AtTime(t) => *t <= now,
                        ReminderTrigger::InSeconds(s) => {
                            r.created_at.saturating_add(*s) <= now
                        }
                        ReminderTrigger::Recurring(interval) => {
                            r.created_at.saturating_add(*interval * (r.recurring_count + 1)) <= now
                        }
                    }
            })
            .collect()
    }

    /// Mark a reminder as completed
    pub fn complete(&mut self, id: u64) -> Result<(), String> {
        for reminder in self.reminders.iter_mut() {
            if reminder.id == id {
                if reminder.status != ReminderStatus::Pending {
                    return Err(format!("Reminder {} is not pending", id));
                }
                reminder.status = ReminderStatus::Completed;
                reminder.completed_at = Some(now_secs());
                return Ok(());
            }
        }
        Err(format!("Reminder {} not found", id))
    }

    /// Dismiss a reminder
    pub fn dismiss(&mut self, id: u64) -> Result<(), String> {
        for reminder in self.reminders.iter_mut() {
            if reminder.id == id {
                reminder.status = ReminderStatus::Dismissed;
                return Ok(());
            }
        }
        Err(format!("Reminder {} not found", id))
    }

    /// Mark a reminder as overdue
    pub fn mark_overdue(&mut self, id: u64) -> Result<(), String> {
        for reminder in self.reminders.iter_mut() {
            if reminder.id == id {
                if reminder.status == ReminderStatus::Pending {
                    reminder.status = ReminderStatus::Overdue;
                }
                return Ok(());
            }
        }
        Err(format!("Reminder {} not found", id))
    }

    /// List all reminders
    pub fn list(&self) -> Vec<&Reminder> {
        self.reminders.iter().collect()
    }

    /// List pending reminders
    pub fn list_pending(&self) -> Vec<&Reminder> {
        self.reminders
            .iter()
            .filter(|r| r.status == ReminderStatus::Pending)
            .collect()
    }

    /// Get a reminder by ID
    pub fn get(&self, id: u64) -> Option<&Reminder> {
        self.reminders.iter().find(|r| r.id == id)
    }

    /// Delete a reminder
    pub fn delete(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .reminders
            .iter()
            .position(|r| r.id == id)
            .ok_or_else(|| format!("Reminder {} not found", id))?;
        self.reminders.remove(pos);
        Ok(())
    }

    /// Get reminder count
    pub fn count(&self) -> usize {
        self.reminders.len()
    }
}

impl Default for ReminderManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_reminder() {
        let mut mgr = ReminderManager::new();
        let now = now_secs();
        let id = mgr.create_at_time(
            "Test reminder".to_string(),
            Some("Test description".to_string()),
            now + 3600,
            ReminderPriority::Normal,
        )
        .unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_create_in_seconds() {
        let mut mgr = ReminderManager::new();
        let id = mgr.create_in_seconds("Test".to_string(), None, 300, ReminderPriority::Low)
            .unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_create_recurring() {
        let mut mgr = ReminderManager::new();
        let id = mgr.create_recurring("Daily".to_string(), None, 86400, ReminderPriority::Normal)
            .unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_past_time_fails() {
        let mut mgr = ReminderManager::new();
        let result = mgr.create_at_time(
            "Past".to_string(),
            None,
            now_secs() - 100,
            ReminderPriority::Normal,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_seconds_fails() {
        let mut mgr = ReminderManager::new();
        let result = mgr.create_in_seconds("Test".to_string(), None, 0, ReminderPriority::Low);
        assert!(result.is_err());
    }

    #[test]
    fn test_complete_reminder() {
        let mut mgr = ReminderManager::new();
        let now = now_secs();
        mgr.create_at_time("Test".to_string(), None, now + 3600, ReminderPriority::Normal)
            .unwrap();
        assert!(mgr.complete(1).is_ok());
        assert_eq!(mgr.list_pending().len(), 0);
    }

    #[test]
    fn test_dismiss_reminder() {
        let mut mgr = ReminderManager::new();
        let now = now_secs();
        mgr.create_at_time("Test".to_string(), None, now + 3600, ReminderPriority::Normal)
            .unwrap();
        assert!(mgr.dismiss(1).is_ok());
        assert_eq!(mgr.list_pending().len(), 0);
    }

    #[test]
    fn test_get_due() {
        let mut mgr = ReminderManager::new();
        let now = now_secs();
        mgr.create_at_time("Due now".to_string(), None, now, ReminderPriority::High)
            .unwrap();
        mgr.create_at_time("Future".to_string(), None, now + 86400, ReminderPriority::Low)
            .unwrap();
        let due = mgr.get_due(now);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].title, "Due now");
    }

    #[test]
    fn test_delete_reminder() {
        let mut mgr = ReminderManager::new();
        let now = now_secs();
        mgr.create_at_time("Test".to_string(), None, now + 3600, ReminderPriority::Normal)
            .unwrap();
        assert!(mgr.delete(1).is_ok());
        assert_eq!(mgr.count(), 0);
    }
}