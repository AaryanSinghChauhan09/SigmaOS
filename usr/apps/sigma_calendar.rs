// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_calendar.rs — Sigma-Calendar Calendar
//
// Implements a calendar application with date navigation,
// event management, and reminder functionality.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Event Structures ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,  // YYYY-MM-DD
    pub time: String,  // HH:MM
    pub location: Option<String>,
    pub reminder: bool,
}

// ─── Calendar State ─────────────────────────────────────────────────────────

pub struct Calendar {
    pub current_year: i32,
    pub current_month: u32,
    pub current_day: u32,
    pub events: HashMap<String, Vec<Event>>, // Key: YYYY-MM-DD
    pub selected_date: Option<String>,
}

impl Calendar {
    pub fn new() -> Self {
        Calendar {
            current_year: 2026,
            current_month: 7,
            current_day: 4,
            events: HashMap::new(),
            selected_date: None,
        }
    }

    /// Check if year is leap year
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Get days in month
    pub fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Get day of week for first day of month (0 = Sunday, 6 = Saturday)
    pub fn first_day_of_month(&self, year: i32, month: u32) -> u32 {
        // Zeller's congruence algorithm
        let m = if month < 3 { month + 12 } else { month };
        let y = if month < 3 { year - 1 } else { year };
        
        let k = y % 100;
        let j = y / 100;
        
        let h = (1 + (13 * (m + 1) as i32) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        
        ((h + 5) % 7) as u32 // Convert to 0 = Sunday
    }

    /// Navigate to next month
    pub fn next_month(&mut self) {
        self.current_month += 1;
        if self.current_month > 12 {
            self.current_month = 1;
            self.current_year += 1;
        }
    }

    /// Navigate to previous month
    pub fn previous_month(&mut self) {
        if self.current_month == 1 {
            self.current_month = 12;
            self.current_year -= 1;
        } else {
            self.current_month -= 1;
        }
    }

    /// Go to today
    pub fn go_to_today(&mut self) {
        self.current_year = 2026;
        self.current_month = 7;
        self.current_day = 4;
    }

    /// Select date
    pub fn select_date(&mut self, year: i32, month: u32, day: u32) {
        let date_key = format!("{:04}-{:02}-{:02}", year, month, day);
        self.selected_date = Some(date_key);
    }

    /// Add event
    pub fn add_event(&mut self, event: Event) {
        let date_key = format!("{:04}-{:02}-{:02}", 
            self.current_year, self.current_month, event.date.split('-').last().unwrap_or(&event.date));
        
        self.events.entry(date_key.clone()).or_insert_with(Vec::new).push(event);
    }

    /// Remove event
    pub fn remove_event(&mut self, date: &str, event_id: &str) {
        if let Some(events) = self.events.get_mut(date) {
            events.retain(|e| e.id != event_id);
        }
    }

    /// Get events for date
    pub fn get_events_for_date(&self, date: &str) -> &[Event] {
        self.events.get(date).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get events for current month
    pub fn get_month_events(&self) -> Vec<(&String, &Event)> {
        let mut events = Vec::new();
        let month_prefix = format!("{:04}-{:02}", self.current_year, self.current_month);
        
        for (date_key, date_events) in &self.events {
            if date_key.starts_with(&month_prefix) {
                for event in date_events {
                    events.push((date_key, event));
                }
            }
        }
        
        events.sort_by(|a, b| a.1.time.cmp(&b.1.time));
        events
    }

    /// Format date key
    pub fn format_date_key(&self, day: u32) -> String {
        format!("{:04}-{:02}-{:02}", self.current_year, self.current_month, day)
    }

    /// Get current date string
    pub fn get_current_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.current_year, self.current_month, self.current_day)
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut calendar = Calendar::new();
    
    println!("Sigma-Calendar v0.1 - Calendar");
    println!("Current: {}", calendar.get_current_date());
    
    loop {
        println!("\n--- Calendar: {:04} {:02} ---", calendar.current_year, calendar.current_month);
        println!("Su Mo Tu We Th Fr Sa");
        
        let first_day = calendar.first_day_of_month(calendar.current_year, calendar.current_month);
        let days_in_month = calendar.days_in_month(calendar.current_year, calendar.current_month);
        
        // Print leading spaces
        for _ in 0..first_day {
            print!("   ");
        }
        
        // Print days
        for day in 1..=days_in_month {
            let date_key = calendar.format_date_key(day);
            let has_events = !calendar.get_events_for_date(&date_key).is_empty();
            let marker = if has_events { "*" } else { " " };
            let current_marker = if day == calendar.current_day { ">" } else { " " };
            print!("{}{:2}{} ", current_marker, day, marker);
            
            if (day + first_day as u32) % 7 == 0 {
                println!();
            }
        }
        
        println!("\n--- Events this month ---");
        for (date, event) in calendar.get_month_events() {
            println!("{} {} @ {} - {}", date, event.time, event.title, event.location.as_deref().unwrap_or("No location"));
        }
        
        println!("\nCommands: next, prev, today, select <day>, add, list <day>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "next" => {
                calendar.next_month();
            }
            "prev" => {
                calendar.previous_month();
            }
            "today" => {
                calendar.go_to_today();
            }
            "select" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(day) = arg.parse::<u32>() {
                        calendar.current_day = day;
                        calendar.select_date(calendar.current_year, calendar.current_month, day);
                    }
                }
            }
            "add" => {
                println!("Add new event");
                print!("Title: ");
                std::io::stdout().flush().unwrap();
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                
                print!("Time (HH:MM): ");
                std::io::stdout().flush().unwrap();
                let mut time = String::new();
                std::io::stdin().read_line(&mut time).unwrap();
                
                print!("Location (optional): ");
                std::io::stdout().flush().unwrap();
                let mut location = String::new();
                std::io::stdin().read_line(&mut location).unwrap();
                
                print!("Description: ");
                std::io::stdout().flush().unwrap();
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).unwrap();
                
                let event = Event {
                    id: format!("event_{}", calendar.events.len()),
                    title: title.trim().to_string(),
                    description: description.trim().to_string(),
                    date: calendar.format_date_key(calendar.current_day),
                    time: time.trim().to_string(),
                    location: if location.trim().is_empty() { None } else { Some(location.trim().to_string()) },
                    reminder: false,
                };
                
                calendar.add_event(event);
                println!("Event added");
            }
            "list" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(day) = arg.parse::<u32>() {
                        let date_key = calendar.format_date_key(day);
                        println!("--- Events for {} ---", date_key);
                        for event in calendar.get_events_for_date(&date_key) {
                            println!("{} @ {} - {}", event.time, event.title, event.description);
                        }
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
