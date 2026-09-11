// SigmaOS Enhanced Reminders & Task Scheduling Engine
// Zero-dependency #![no_std] task reminder manager with recurrence rules, tag categories, and snooze/defer capabilities

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecurrencePattern {
    Once,
    Daily,
    WeeklyDays(Vec<u8>), // 0 = Sun, 1 = Mon, ..., 6 = Sat
    MonthlyDay(u8),      // 1 to 31
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReminderPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReminderItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: ReminderPriority,
    pub due_timestamp: u64,
    pub recurrence: RecurrencePattern,
    pub is_completed: bool,
    pub snooze_until_timestamp: Option<u64>,
    pub tags: Vec<String>,
}

pub struct EnhancedRemindersEngine {
    reminders: Vec<ReminderItem>,
    next_id: u64,
}

impl EnhancedRemindersEngine {
    pub fn new() -> Self {
        Self {
            reminders: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_reminder(
        &mut self,
        title: &str,
        description: &str,
        category: &str,
        priority: ReminderPriority,
        due_timestamp: u64,
        recurrence: RecurrencePattern,
        tags: &[&str],
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let item = ReminderItem {
            id,
            title: String::from(title),
            description: String::from(description),
            category: String::from(category),
            priority,
            due_timestamp,
            recurrence,
            is_completed: false,
            snooze_until_timestamp: None,
            tags: tags.iter().map(|&t| String::from(t)).collect(),
        };

        self.reminders.push(item);
        id
    }

    pub fn mark_completed(&mut self, reminder_id: u64, current_timestamp: u64) -> bool {
        let (recurrence, due_timestamp) = match self.reminders.iter().find(|r| r.id == reminder_id) {
            Some(r) => (r.recurrence.clone(), r.due_timestamp),
            None => return false,
        };

        if recurrence == RecurrencePattern::Once {
            if let Some(r) = self.reminders.iter_mut().find(|r| r.id == reminder_id) {
                r.is_completed = true;
            }
        } else {
            // Calculate next due date for recurring reminder
            let next_due = match recurrence {
                RecurrencePattern::Daily => current_timestamp + 86400,
                RecurrencePattern::WeeklyDays(_) => current_timestamp + 604800,
                RecurrencePattern::MonthlyDay(_) => current_timestamp + 2592000,
                RecurrencePattern::Yearly => current_timestamp + 31536000,
                RecurrencePattern::Once => due_timestamp,
            };

            if let Some(r) = self.reminders.iter_mut().find(|r| r.id == reminder_id) {
                r.due_timestamp = next_due;
                r.snooze_until_timestamp = None;
            }
        }

        true
    }

    pub fn snooze_reminder(&mut self, reminder_id: u64, snooze_duration_sec: u64, current_timestamp: u64) -> bool {
        if let Some(r) = self.reminders.iter_mut().find(|r| r.id == reminder_id) {
            r.snooze_until_timestamp = Some(current_timestamp + snooze_duration_sec);
            true
        } else {
            false
        }
    }

    pub fn get_due_reminders(&self, current_timestamp: u64) -> Vec<&ReminderItem> {
        self.reminders
            .iter()
            .filter(|r| {
                if r.is_completed {
                    return false;
                }
                if let Some(snooze) = r.snooze_until_timestamp {
                    if current_timestamp < snooze {
                        return false;
                    }
                }
                current_timestamp >= r.due_timestamp
            })
            .collect()
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<&ReminderItem> {
        self.reminders
            .iter()
            .filter(|r| r.tags.iter().any(|t| t == tag))
            .collect()
    }

    pub fn list_reminders(&self) -> &[ReminderItem] {
        &self.reminders
    }
}

impl Default for EnhancedRemindersEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_reminders_engine() {
        let mut engine = EnhancedRemindersEngine::new();
        let now = 1700000000;

        let id = engine.add_reminder(
            "Kernel Release Review",
            "Review 6.12 patches",
            "Work",
            ReminderPriority::High,
            now - 100, // overdue
            RecurrencePattern::Daily,
            &["kernel", "release"],
        );

        assert_eq!(engine.get_due_reminders(now).len(), 1);

        // Snooze for 1 hour
        assert!(engine.snooze_reminder(id, 3600, now));
        assert_eq!(engine.get_due_reminders(now).len(), 0);

        // Advance past snooze time
        assert_eq!(engine.get_due_reminders(now + 3601).len(), 1);

        assert_eq!(engine.filter_by_tag("kernel").len(), 1);
    }
}
