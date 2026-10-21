// src/reminder/mod.rs
// Reminder subsystem for SigmaOS, inspired by Omarchy 4.0.5
// (`omancy-reminder`: set/show/clear lightweight desktop notification reminders).

pub mod reminder_store;

pub use reminder_store::{
    DEFAULT_REMINDER_CAPACITY, Reminder, ReminderStatus, ReminderStore,
};
