// src/notification/mod.rs
// Notification system for SigmaOS

#![no_std]

pub mod notification_system;

pub use notification_system::{
    presets, Action, ActionType, DndConfig, Notification, NotificationSystem, Priority, Urgency,
};
