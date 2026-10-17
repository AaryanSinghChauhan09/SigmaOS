// SPDX-License-Identifier: MIT
// SigmaOS - Calendar
// Inspired by GNOME Calendar and Evolution
// Provides calendar and event management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Event color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Orange,
    Gray,
}

/// Event reminder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reminder {
    None,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    OneDay,
    OneWeek,
}

/// Recurrence rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Event
#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_time: i64,
    pub end_time: i64,
    pub all_day: bool,
    pub color: EventColor,
    pub reminder: Reminder,
    pub recurrence: Recurrence,
}

/// Calendar
#[derive(Debug, Clone)]
pub struct Calendar {
    pub name: String,
    pub color: EventColor,
    pub events: Vec<Event>,
    pub visible: bool,
}

/// Date
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Calendar manager
pub struct CalendarManager {
    pub calendars: BTreeMap<u32, Calendar>,
    pub selected_date: Date,
    pub view_month: u32,
    pub view_year: i32,
    next_calendar_id: AtomicU32,
    next_event_id: AtomicU32,
}

impl CalendarManager {
    pub fn new() -> Self {
        let mut calendars = BTreeMap::new();

        // Default calendar
        let id = 1;
        calendars.insert(
            id,
            Calendar {
                name: "Personal".to_string(),
                color: EventColor::Blue,
                events: Vec::new(),
                visible: true,
            },
        );

        Self {
            calendars,
            selected_date: Date {
                year: 2024,
                month: 1,
                day: 1,
            },
            view_month: 1,
            view_year: 2024,
            next_calendar_id: AtomicU32::new(2),
            next_event_id: AtomicU32::new(1),
        }
    }

    /// Add calendar
    pub fn add_calendar(&mut self, name: String, color: EventColor) -> u32 {
        let id = self.next_calendar_id.fetch_add(1, Ordering::Relaxed);
        let calendar = Calendar {
            name,
            color,
            events: Vec::new(),
            visible: true,
        };
        self.calendars.insert(id, calendar);
        id
    }

    /// Remove calendar
    pub fn remove_calendar(&mut self, id: u32) -> Result<(), &'static str> {
        if id == 1 {
            return Err("Cannot remove default calendar");
        }
        if !self.calendars.contains_key(&id) {
            return Err("Calendar not found");
        }
        self.calendars.remove(&id);
        Ok(())
    }

    /// Add event
    pub fn add_event(&mut self, calendar_id: u32, event: Event) -> Result<(), &'static str> {
        let calendar = self
            .calendars
            .get_mut(&calendar_id)
            .ok_or("Calendar not found")?;
        calendar.events.push(event);
        Ok(())
    }

    /// Remove event
    pub fn remove_event(&mut self, calendar_id: u32, event_id: u32) -> Result<(), &'static str> {
        let calendar = self
            .calendars
            .get_mut(&calendar_id)
            .ok_or("Calendar not found")?;
        calendar.events.retain(|e| e.id != event_id);
        Ok(())
    }

    /// Get events for date
    pub fn get_events_for_date(&self, date: Date) -> Vec<&Event> {
        let mut events = Vec::new();
        for calendar in self.calendars.values() {
            if !calendar.visible {
                continue;
            }
            for event in &calendar.events {
                if self.event_matches_date(event, date) {
                    events.push(event);
                }
            }
        }
        events.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        events
    }

    /// Check if event matches date
    fn event_matches_date(&self, event: &Event, date: Date) -> bool {
        // Simplified - in real implementation would check timestamps
        true
    }

    /// Get events for month
    pub fn get_events_for_month(&self, year: i32, month: u32) -> Vec<&Event> {
        let mut events = Vec::new();
        for calendar in self.calendars.values() {
            if !calendar.visible {
                continue;
            }
            for event in &calendar.events {
                // Simplified - in real implementation would check month/year
                events.push(event);
            }
        }
        events
    }

    /// Set selected date
    pub fn set_selected_date(&mut self, date: Date) {
        self.selected_date = date;
    }

    /// Set view month
    pub fn set_view_month(&mut self, year: i32, month: u32) {
        self.view_year = year;
        self.view_month = month;
    }

    /// Navigate to next month
    pub fn next_month(&mut self) {
        if self.view_month == 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
    }

    /// Navigate to previous month
    pub fn previous_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
    }

    /// Go to today
    pub fn go_to_today(&mut self) {
        // Simplified - in real implementation would get current date
        self.view_year = 2024;
        self.view_month = 1;
        self.selected_date = Date {
            year: 2024,
            month: 1,
            day: 1,
        };
    }

    /// Get days in month
    pub fn get_days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Check if leap year
    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Get day of week for date
    pub fn get_day_of_week(&self, date: Date) -> u32 {
        // Simplified - in real implementation would use proper algorithm
        0
    }

    /// Toggle calendar visibility
    pub fn toggle_calendar_visibility(&mut self, id: u32) -> Result<(), &'static str> {
        let calendar = self.calendars.get_mut(&id).ok_or("Calendar not found")?;
        calendar.visible = !calendar.visible;
        Ok(())
    }

    /// Get visible calendars
    pub fn get_visible_calendars(&self) -> Vec<&Calendar> {
        self.calendars.values().filter(|c| c.visible).collect()
    }

    /// Get calendar count
    pub fn calendar_count(&self) -> usize {
        self.calendars.len()
    }

    /// Get total event count
    pub fn event_count(&self) -> usize {
        self.calendars.values().map(|c| c.events.len()).sum()
    }
}

/// Global calendar manager
static mut CALENDAR_MANAGER: Option<CalendarManager> = None;

/// Initialize calendar manager
pub fn init_calendar() {
    unsafe {
        CALENDAR_MANAGER = Some(CalendarManager::new());
    }
}

/// Get calendar manager
pub fn get_calendar() -> &'static mut CalendarManager {
    unsafe {
        CALENDAR_MANAGER
            .as_mut()
            .expect("Calendar manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_manager_creation() {
        let cm = CalendarManager::new();
        assert_eq!(cm.calendar_count(), 1);
    }

    #[test]
    fn test_add_calendar() {
        let mut cm = CalendarManager::new();
        let id = cm.add_calendar("Work".to_string(), EventColor::Green);
        assert_eq!(cm.calendar_count(), 2);
    }

    #[test]
    fn test_next_month() {
        let mut cm = CalendarManager::new();
        cm.next_month();
        assert_eq!(cm.view_month, 2);
    }

    #[test]
    fn test_previous_month() {
        let mut cm = CalendarManager::new();
        cm.view_month = 3;
        cm.previous_month();
        assert_eq!(cm.view_month, 2);
    }

    #[test]
    fn test_days_in_month() {
        let cm = CalendarManager::new();
        assert_eq!(cm.get_days_in_month(2024, 1), 31);
        assert_eq!(cm.get_days_in_month(2024, 4), 30);
    }

    #[test]
    fn test_leap_year() {
        let cm = CalendarManager::new();
        assert!(cm.is_leap_year(2024));
        assert!(!cm.is_leap_year(2023));
    }
}
