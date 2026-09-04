#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// SigmaOS Calendar App
// OOP-based calendar with events, reminders, and scheduling

#[cfg(not(test))]
use crate::klib::BTreeMap;
use core::time::Duration;
#[cfg(test)]
use std::collections::BTreeMap;
// SystemTime not in no_std

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Meeting,
    Appointment,
    Task,
    Reminder,
    Birthday,
    Holiday,
    Other,
}

/// Recurrence pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurrencePattern {
    None,
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Yearly,
    Custom(u32), // Custom days
}

/// Event status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
    Completed,
}

/// Fedora-inspired Sub-Calendar Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubCalendarCategory {
    FedoraQa,
    FesCo,
    Council,
    RelEng,
    Mindshare,
    Personal,
    Community,
}

/// Sub-Calendar container
#[derive(Debug, Clone)]
pub struct SubCalendar {
    pub id: String,
    pub name: String,
    pub category: SubCalendarCategory,
    pub color_hex: String,
    pub is_visible: bool,
}

/// Calendar event
#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_time: u64,
    pub end_time: u64,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub recurrence: RecurrencePattern,
    pub status: EventStatus,
    pub reminder_minutes_before: Vec<u32>,
    pub color: String,
    pub sub_calendar_id: Option<String>,
}

impl CalendarEvent {
    /// Render event in RFC 5545 iCalendar (.ics) format
    pub fn export_icalendar_format(&self) -> String {
        let mut ics = String::new();
        ics.push_str("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//SigmaOS//Fedocal Engine//EN\r\nBEGIN:VEVENT\r\n");
        ics.push_str(&format!("UID:{}\r\n", self.id));
        ics.push_str(&format!("SUMMARY:{}\r\n", self.title));
        ics.push_str(&format!("DESCRIPTION:{}\r\n", self.description));
        ics.push_str(&format!("DTSTART:{}\r\n", self.start_time));
        ics.push_str(&format!("DTEND:{}\r\n", self.end_time));
        if let Some(ref loc) = self.location {
            ics.push_str(&format!("LOCATION:{}\r\n", loc));
        }
        for attendee in &self.attendees {
            ics.push_str(&format!("ATTENDEE:MAILTO:{}\r\n", attendee));
        }
        ics.push_str("END:VEVENT\r\nEND:VCALENDAR\r\n");
        ics
    }
}

/// Calendar view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarView {
    Day,
    Week,
    Month,
    Year,
    Agenda,
}

/// Day info
#[derive(Debug, Clone)]
pub struct DayInfo {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub events: Vec<CalendarEvent>,
    pub is_weekend: bool,
    pub is_holiday: bool,
}

/// OOP trait for calendar storage
pub trait CalendarStorage {
    /// Add event
    fn add_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError>;
    /// Remove event
    fn remove_event(&mut self, event_id: &str) -> Result<(), CalendarError>;
    /// Update event
    fn update_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError>;
    /// Get event
    fn get_event(&self, event_id: &str) -> Option<CalendarEvent>;
    /// Get all events
    fn get_all_events(&self) -> Vec<CalendarEvent>;
    /// Get events in range
    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<CalendarEvent>;
}

/// In-memory calendar storage
pub struct InMemoryCalendarStorage {
    events: BTreeMap<String, CalendarEvent>,
}

impl InMemoryCalendarStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }
}

impl CalendarStorage for InMemoryCalendarStorage {
    fn add_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError> {
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    fn remove_event(&mut self, event_id: &str) -> Result<(), CalendarError> {
        self.events
            .remove(event_id)
            .ok_or_else(|| CalendarError::EventNotFound(event_id.to_string()))?;
        Ok(())
    }

    fn update_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError> {
        if !self.events.contains_key(&event.id) {
            return Err(CalendarError::EventNotFound(event.id.clone()));
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    fn get_event(&self, event_id: &str) -> Option<CalendarEvent> {
        self.events.get(event_id).cloned()
    }

    fn get_all_events(&self) -> Vec<CalendarEvent> {
        self.events.values().cloned().collect()
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<CalendarEvent> {
        self.events
            .values()
            .filter(|e| e.start_time >= start && e.start_time <= end)
            .cloned()
            .collect()
    }
}

/// OOP-based Calendar App
pub struct CalendarApp {
    storage: Box<dyn CalendarStorage>,
    current_view: CalendarView,
    current_date: u64,
    timezones: BTreeMap<String, String>,
}

impl CalendarApp {
    pub fn new(storage: Box<dyn CalendarStorage>) -> Self {
        let now = 1700000000u64;

        Self {
            storage,
            current_view: CalendarView::Month,
            current_date: now,
            timezones: BTreeMap::new(),
        }
    }

    /// Set current view
    pub fn with_view(mut self, view: CalendarView) -> Self {
        self.current_view = view;
        self
    }

    /// Add event
    pub fn add_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError> {
        self.storage.add_event(event)
    }

    /// Remove event
    pub fn remove_event(&mut self, event_id: &str) -> Result<(), CalendarError> {
        self.storage.remove_event(event_id)
    }

    /// Update event
    pub fn update_event(&mut self, event: CalendarEvent) -> Result<(), CalendarError> {
        self.storage.update_event(event)
    }

    /// Get event
    pub fn get_event(&self, event_id: &str) -> Option<CalendarEvent> {
        self.storage.get_event(event_id)
    }

    /// Get all events
    pub fn get_all_events(&self) -> Vec<CalendarEvent> {
        self.storage.get_all_events()
    }

    /// Get events for day
    pub fn get_events_for_day(&self, timestamp: u64) -> Vec<CalendarEvent> {
        let day_start = timestamp - (timestamp % 86400);
        let day_end = day_start + 86400;
        self.storage.get_events_in_range(day_start, day_end)
    }

    /// Get events for week
    pub fn get_events_for_week(&self, timestamp: u64) -> Vec<CalendarEvent> {
        let week_start = timestamp - (timestamp % 86400) - ((timestamp / 86400 % 7) as u64 * 86400);
        let week_end = week_start + (7 * 86400);
        self.storage.get_events_in_range(week_start, week_end)
    }

    /// Get events for month
    pub fn get_events_for_month(&self, year: u32, month: u32) -> Vec<CalendarEvent> {
        // Simplified month calculation
        let month_start = self.calculate_month_start(year, month);
        let month_end = month_start + (31 * 86400); // Max days
        self.storage.get_events_in_range(month_start, month_end)
    }

    /// Calculate month start timestamp
    fn calculate_month_start(&self, year: u32, month: u32) -> u64 {
        // Simplified calculation - in real implementation would use proper date library
        let days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut total_days = (year - 1970) * 365;
        for m in 1..month {
            total_days += days_per_month[(m - 1) as usize];
        }
        u64::from(total_days) * 86400
    }

    /// Get upcoming events
    pub fn get_upcoming_events(&self, count: usize) -> Vec<CalendarEvent> {
        let now = 1700000000u64;

        let mut events: Vec<CalendarEvent> = self
            .storage
            .get_all_events()
            .into_iter()
            .filter(|e| e.start_time >= now)
            .collect();

        events.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        events.into_iter().take(count).collect()
    }

    /// Search events
    pub fn search_events(&self, query: &str) -> Vec<CalendarEvent> {
        let query_lower = query.to_lowercase();
        self.storage
            .get_all_events()
            .into_iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower)
                    || e.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get events by type
    pub fn get_events_by_type(&self, event_type: EventType) -> Vec<CalendarEvent> {
        self.storage
            .get_all_events()
            .into_iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Get current view
    pub fn current_view(&self) -> CalendarView {
        self.current_view
    }

    /// Set current view
    pub fn set_view(&mut self, view: CalendarView) {
        self.current_view = view;
    }

    /// Get current date
    pub fn current_date(&self) -> u64 {
        self.current_date
    }

    /// Set current date
    pub fn set_current_date(&mut self, timestamp: u64) {
        self.current_date = timestamp;
    }

    /// Navigate to next period
    pub fn navigate_next(&mut self) {
        match self.current_view {
            CalendarView::Day => self.current_date += 86400,
            CalendarView::Week => self.current_date += 7 * 86400,
            CalendarView::Month => self.current_date += 30 * 86400,
            CalendarView::Year => self.current_date += 365 * 86400,
            CalendarView::Agenda => self.current_date += 86400,
        }
    }

    /// Navigate to previous period
    pub fn navigate_previous(&mut self) {
        match self.current_view {
            CalendarView::Day => self.current_date -= 86400,
            CalendarView::Week => self.current_date -= 7 * 86400,
            CalendarView::Month => self.current_date -= 30 * 86400,
            CalendarView::Year => self.current_date -= 365 * 86400,
            CalendarView::Agenda => self.current_date -= 86400,
        }
    }

    /// Go to today
    pub fn go_to_today(&mut self) {
        self.current_date = 1700000000u64;
    }

    /// Detect meeting schedule collisions
    pub fn detect_event_collisions(&self, event: &CalendarEvent) -> Vec<CalendarEvent> {
        self.storage
            .get_all_events()
            .into_iter()
            .filter(|e| {
                e.id != event.id
                    && e.status != EventStatus::Cancelled
                    && ((event.start_time >= e.start_time && event.start_time < e.end_time)
                        || (event.end_time > e.start_time && event.end_time <= e.end_time)
                        || (event.start_time <= e.start_time && event.end_time >= e.end_time))
            })
            .collect()
    }
}

impl Default for CalendarApp {
    fn default() -> Self {
        Self::new(Box::new(InMemoryCalendarStorage::new())).with_view(CalendarView::Month)
    }
}

/// Calendar errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    EventNotFound(String),
    InvalidTimeRange,
    StorageError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_event() {
        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Meeting".to_string(),
            description: "Test meeting".to_string(),
            event_type: EventType::Meeting,
            start_time: 1234567890,
            end_time: 1234571490,
            location: Some("Office".to_string()),
            attendees: vec!["user@example.com".to_string()],
            recurrence: RecurrencePattern::None,
            status: EventStatus::Confirmed,
            reminder_minutes_before: vec![15],
            color: "#FF0000".to_string(),
            sub_calendar_id: Some("fedora-qa".to_string()),
        };
        assert_eq!(event.title, "Meeting");
        let ics = event.export_icalendar_format();
        assert!(ics.contains("BEGIN:VCALENDAR"));
        assert!(ics.contains("SUMMARY:Meeting"));
    }

    #[test]
    fn test_in_memory_calendar_storage() {
        let storage = InMemoryCalendarStorage::new();
        assert_eq!(storage.get_all_events().len(), 0);
    }

    #[test]
    fn test_calendar_app() {
        let app = CalendarApp::default();
        assert_eq!(app.current_view(), CalendarView::Month);
    }

    #[test]
    fn test_add_event() {
        let mut app = CalendarApp::default();
        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Meeting".to_string(),
            description: "Test meeting".to_string(),
            event_type: EventType::Meeting,
            start_time: 1234567890,
            end_time: 1234571490,
            location: Some("Office".to_string()),
            attendees: vec!["user@example.com".to_string()],
            recurrence: RecurrencePattern::None,
            status: EventStatus::Confirmed,
            reminder_minutes_before: vec![15],
            color: "#FF0000".to_string(),
            sub_calendar_id: None,
        };
        app.add_event(event).unwrap();
        assert_eq!(app.get_all_events().len(), 1);
    }

    #[test]
    fn test_search_events() {
        let mut app = CalendarApp::default();
        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Team Meeting".to_string(),
            description: "Weekly team sync".to_string(),
            event_type: EventType::Meeting,
            start_time: 1234567890,
            end_time: 1234571490,
            location: Some("Office".to_string()),
            attendees: vec!["user@example.com".to_string()],
            recurrence: RecurrencePattern::None,
            status: EventStatus::Confirmed,
            reminder_minutes_before: vec![15],
            color: "#FF0000".to_string(),
            sub_calendar_id: None,
        };
        app.add_event(event).unwrap();
        let results = app.search_events("team");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_detect_event_collisions() {
        let mut app = CalendarApp::default();
        let event1 = CalendarEvent {
            id: "m1".to_string(),
            title: "FEsCo Meeting".to_string(),
            description: "FEsCo weekly meeting".to_string(),
            event_type: EventType::Meeting,
            start_time: 1700000000,
            end_time: 1700003600,
            location: Some("#fedora-meeting".to_string()),
            attendees: vec![],
            recurrence: RecurrencePattern::Weekly,
            status: EventStatus::Confirmed,
            reminder_minutes_before: vec![10],
            color: "#0000FF".to_string(),
            sub_calendar_id: Some("fesco".to_string()),
        };
        app.add_event(event1).unwrap();

        let event2 = CalendarEvent {
            id: "m2".to_string(),
            title: "Overlapping Meeting".to_string(),
            description: "Conflict".to_string(),
            event_type: EventType::Meeting,
            start_time: 1700001800,
            end_time: 1700005400,
            location: None,
            attendees: vec![],
            recurrence: RecurrencePattern::None,
            status: EventStatus::Confirmed,
            reminder_minutes_before: vec![],
            color: "#FF0000".to_string(),
            sub_calendar_id: None,
        };

        let collisions = app.detect_event_collisions(&event2);
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].id, "m1");
    }
}
