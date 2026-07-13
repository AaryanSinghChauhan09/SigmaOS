// sigma_nl_cli.rs
// SigmaOS NL→CLI Intent Parser — bare-metal Rust implementation
// Replaces sigma_nl_cli.c
// Zero external library dependencies
// Uses only core library

#![no_std]
#![allow(dead_code)]

use core::ffi::c_char;
use core::slice;

// ── Intent Table ───────────────────────────────────────────────────────────
#[repr(C)]
pub struct IntentMapping {
    pub phrase: &'static str,
    pub command: &'static str,
    pub description: &'static str,
}

const INTENTS: [IntentMapping; 11] = [
    IntentMapping {
        phrase: "install",
        command: "sigpkg install",
        description: "Install a package",
    },
    IntentMapping {
        phrase: "remove",
        command: "sigpkg remove",
        description: "Remove a package",
    },
    IntentMapping {
        phrase: "update",
        command: "sigpkg update",
        description: "Update all packages",
    },
    IntentMapping {
        phrase: "rollback",
        command: "sigpkg rollback",
        description: "Rollback last transaction",
    },
    IntentMapping {
        phrase: "list packages",
        command: "sigpkg list",
        description: "List installed packages",
    },
    IntentMapping {
        phrase: "check kernel",
        command: "uname -r",
        description: "Print kernel version",
    },
    IntentMapping {
        phrase: "disk usage",
        command: "df -h",
        description: "Show disk usage",
    },
    IntentMapping {
        phrase: "cpu info",
        command: "cat /proc/cpuinfo",
        description: "Show CPU info",
    },
    IntentMapping {
        phrase: "memory",
        command: "free -h",
        description: "Show memory usage",
    },
    IntentMapping {
        phrase: "shutdown",
        command: "shutdown -h now",
        description: "Shut down the system",
    },
    IntentMapping {
        phrase: "reboot",
        command: "reboot",
        description: "Reboot the system",
    },
];

// ── C string to Rust string conversion ─────────────────────────────────────
unsafe fn c_str_to_str(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    
    let slice = slice::from_raw_parts(ptr as *const u8, len);
    core::str::from_utf8(slice).ok()
}

// ── Simple substring check (strstr equivalent) ───────────────────────────────
fn contains(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

// ── Intent Matcher ────────────────────────────────────────────────────────
pub fn match_intent(input: &str) -> Option<&'static str> {
    for intent in INTENTS.iter() {
        if contains(input, intent.phrase) {
            return Some(intent.command);
        }
    }
    None
}

// ── Dry-Run Sandbox ───────────────────────────────────────────────────────
pub fn dry_run(command: &str) {
    // In a real implementation, this would print to console
    // For no_std, we just have the logic
    let _ = command;
}

// ── Process input string ───────────────────────────────────────────────────
pub fn process_input(input: &str) -> Result<&'static str, &'static str> {
    let trimmed = input.trim();
    
    if trimmed == "exit" || trimmed == "quit" {
        return Err("exit");
    }
    
    match match_intent(trimmed) {
        Some(cmd) => {
            dry_run(cmd);
            Ok(cmd)
        }
        None => Err("Intent not recognized"),
    }
}

// ── Main entry point for no_std context ───────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_nl_cli_process(input_ptr: *const c_char) -> i32 {
    if input_ptr.is_null() {
        return 1; // Error: null pointer
    }
    
    if let Some(input) = unsafe { c_str_to_str(input_ptr) } {
        match process_input(input) {
            Ok(_) => 0, // Success
            Err("exit") => 2, // Exit signal
            Err(_) => 1, // Error
        }
    } else {
        1 // Error: invalid string
    }
}
