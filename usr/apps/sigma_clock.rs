// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_clock.rs — Sigma-Clock System Clock
//
// Implements a system clock application with time display,
// alarm functionality, and timezone support.
//
// Language: Rust (std for userland applications)

use std::time::{SystemTime, UNIX_EPOCH};

// ─── Alarm Structure ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Alarm {
    pub id: String,
    pub time: String,  // HH:MM
    pub label: String,
    pub enabled: bool,
    pub days: Vec<String>,  // Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

// ─── Clock Application State ───────────────────────────────────────────────────

pub struct ClockApp {
    pub timezone_offset: i32,  // Hours from UTC
    pub format_24h: bool,
    pub show_seconds: bool,
    pub alarms: Vec<Alarm>,
    pub timer_running: bool,
    pub timer_seconds: u64,
}

impl ClockApp {
    pub fn new() -> Self {
        ClockApp {
            timezone_offset: 0,
            format_24h: true,
            show_seconds: true,
            alarms: Vec::new(),
            timer_running: false,
            timer_seconds: 0,
        }
    }

    /// Get current time with timezone offset
    pub fn get_current_time(&self) -> (u32, u32, u32) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let utc_offset = self.timezone_offset as i64 * 3600;
        let local_time = (now as i64 + utc_offset).max(0) as u64;
        
        let hours = (local_time % 86400) / 3600;
        let minutes = (local_time % 3600) / 60;
        let seconds = local_time % 60;
        
        (hours as u32, minutes as u32, seconds as u32)
    }

    /// Get current date
    pub fn get_current_date(&self) -> (i32, u32, u32) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let utc_offset = self.timezone_offset as i64 * 3600;
        let local_time = (now as i64 + utc_offset).max(0) as u64;
        
        let days = local_time / 86400;
        let year = 1970 + (days / 365) as i32;
        let day_of_year = (days % 365) as u32;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;
        
        (year, month, day)
    }

    /// Format time as string
    pub fn format_time(&self, hours: u32, minutes: u32, seconds: u32) -> String {
        if self.format_24h {
            if self.show_seconds {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else {
                format!("{:02}:{:02}", hours, minutes)
            }
        } else {
            let (display_hours, period) = if hours == 0 {
                (12, "AM")
            } else if hours < 12 {
                (hours, "AM")
            } else if hours == 12 {
                (12, "PM")
            } else {
                (hours - 12, "PM")
            };
            
            if self.show_seconds {
                format!("{:02}:{:02}:{:02} {}", display_hours, minutes, seconds, period)
            } else {
                format!("{:02}:{:02} {}", display_hours, minutes, period)
            }
        }
    }

    /// Format date as string
    pub fn format_date(&self, year: i32, month: u32, day: u32) -> String {
        format!("{:04}-{:02}-{:02}", year, month, day)
    }

    /// Set timezone offset
    pub fn set_timezone(&mut self, offset: i32) {
        self.timezone_offset = offset.clamp(-12, 14);
    }

    /// Toggle 24h format
    pub fn toggle_24h(&mut self) {
        self.format_24h = !self.format_24h;
    }

    /// Toggle seconds display
    pub fn toggle_seconds(&mut self) {
        self.show_seconds = !self.show_seconds;
    }

    /// Add alarm
    pub fn add_alarm(&mut self, time: String, label: String) -> String {
        let alarm_id = format!("alarm_{}", self.alarms.len());
        
        let alarm = Alarm {
            id: alarm_id.clone(),
            time,
            label,
            enabled: true,
            days: vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string(), 
                      "Thu".to_string(), "Fri".to_string(), "Sat".to_string(), "Sun".to_string()],
        };
        
        self.alarms.push(alarm);
        alarm_id
    }

    /// Remove alarm
    pub fn remove_alarm(&mut self, alarm_id: &str) {
        self.alarms.retain(|a| a.id != alarm_id);
    }

    /// Toggle alarm
    pub fn toggle_alarm(&mut self, alarm_id: &str) {
        if let Some(alarm) = self.alarms.iter_mut().find(|a| a.id == alarm_id) {
            alarm.enabled = !alarm.enabled;
        }
    }

    /// Check if any alarm should trigger
    pub fn check_alarms(&self) -> Vec<&Alarm> {
        let (hours, minutes, _seconds) = self.get_current_time();
        let current_time = format!("{:02}:{:02}", hours, minutes);
        
        self.alarms.iter()
            .filter(|a| a.enabled && a.time == current_time)
            .collect()
    }

    /// Start timer
    pub fn start_timer(&mut self) {
        self.timer_running = true;
    }

    /// Stop timer
    pub fn stop_timer(&mut self) {
        self.timer_running = false;
    }

    /// Reset timer
    pub fn reset_timer(&mut self) {
        self.timer_seconds = 0;
        self.timer_running = false;
    }

    /// Update timer (call every second)
    pub fn update_timer(&mut self) {
        if self.timer_running {
            self.timer_seconds += 1;
        }
    }

    /// Format timer as string
    pub fn format_timer(&self) -> String {
        let hours = self.timer_seconds / 3600;
        let minutes = (self.timer_seconds % 3600) / 60;
        let seconds = self.timer_seconds % 60;
        
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    /// Get all alarms
    pub fn get_alarms(&self) -> &[Alarm] {
        &self.alarms
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut clock = ClockApp::new();
    
    println!("Sigma-Clock v0.1 - System Clock");
    
    loop {
        let (hours, minutes, seconds) = clock.get_current_time();
        let (year, month, day) = clock.get_current_date();
        
        println!("\n--- Current Time ---");
        println!("Date: {}", clock.format_date(year, month, day));
        println!("Time: {}", clock.format_time(hours, minutes, seconds));
        println!("Timezone: UTC{:+}", clock.timezone_offset);
        
        if clock.timer_running {
            println!("Timer: {}", clock.format_timer());
        }
        
        println!("\n--- Alarms ---");
        for alarm in clock.get_alarms() {
            let status = if alarm.enabled { "ON" } else { "OFF" };
            println!("[{}] {} - {} ({})", status, alarm.time, alarm.label, alarm.days.join(", "));
        }
        
        println!("\nCommands: timezone <offset>, 24h, seconds, alarm <time> <label>, remove <id>, toggle <id>, timer start/stop/reset, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "timezone" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(offset) = arg.parse::<i32>() {
                        clock.set_timezone(offset);
                        println!("Timezone set to UTC{:+}", offset);
                    }
                }
            }
            "24h" => {
                clock.toggle_24h();
                println!("24h format: {}", if clock.format_24h { "ON" } else { "OFF" });
            }
            "seconds" => {
                clock.toggle_seconds();
                println!("Seconds: {}", if clock.show_seconds { "ON" } else { "OFF" });
            }
            "alarm" => {
                if parts.len() >= 3 {
                    let time = parts[1];
                    let label = parts[2..].join(" ");
                    let alarm_id = clock.add_alarm(time.to_string(), label);
                    println!("Alarm added: {}", alarm_id);
                }
            }
            "remove" => {
                if let Some(arg) = parts.get(1) {
                    clock.remove_alarm(arg);
                    println!("Alarm removed");
                }
            }
            "toggle" => {
                if let Some(arg) = parts.get(1) {
                    clock.toggle_alarm(arg);
                    println!("Alarm toggled");
                }
            }
            "timer" => {
                if let Some(subcmd) = parts.get(1) {
                    match *subcmd {
                        "start" => {
                            clock.start_timer();
                            println!("Timer started");
                        }
                        "stop" => {
                            clock.stop_timer();
                            println!("Timer stopped");
                        }
                        "reset" => {
                            clock.reset_timer();
                            println!("Timer reset");
                        }
                        _ => {}
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
