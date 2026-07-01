// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Power Daemon Daemon Utility (Rust, no_std)
//! Replaces: bin/powerd/main.cpp
//! =========================================================================

#[derive(Copy, Clone, PartialEq)]
pub enum PowerState {
    Active,
    Sleep,
    Hibernate,
    Shutdown,
}

pub struct PowerDaemon {
    current_state: PowerState,
    battery_level: u8,
}

impl PowerDaemon {
    pub const fn new() -> Self {
        Self {
            current_state: PowerState::Active,
            battery_level: 100,
        }
    }

    pub fn update_battery(&mut self, level: u8) {
        self.battery_level = level;
        if self.battery_level < 5 {
            self.request_state_change(PowerState::Shutdown);
        } else if self.battery_level < 15 {
            self.request_state_change(PowerState::Sleep);
        }
    }

    pub fn request_state_change(&mut self, new_state: PowerState) -> bool {
        self.current_state = new_state;
        // Trigger ACPI state changes via Sovereign Syscall gate
        true
    }

    pub fn current_state(&self) -> PowerState {
        self.current_state
    }

    pub fn class_name(&self) -> &'static str {
        "PowerDaemon"
    }
}
