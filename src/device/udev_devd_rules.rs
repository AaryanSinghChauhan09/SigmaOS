#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS udev & devd Device Event Rule Processing Engine
// Inspired by Linux systemd-udevd (/etc/udev/rules.d/) and FreeBSD devd (/etc/devd.conf)
// Handles hotplug event matching, /dev/ node permissions & symlink creation, and event action execution.

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

/// Hotplug action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceEventAction {
    Add,
    Remove,
    Change,
    Bind,
    Unbind,
}

/// Linux udev style rule definition
#[derive(Debug, Clone)]
pub struct UdevRule {
    pub rule_id: String,
    pub subsystem: String,
    pub sysname_pattern: String,
    pub driver_match: Option<String>,
    pub env_matches: HashMap<String, String>,
    pub symlink_name: Option<String>,
    pub mode: u32,       // e.g. 0o660
    pub group: String,   // e.g. "input", "disk", "video"
    pub run_command: Option<String>,
}

/// FreeBSD devd style rule definition
#[derive(Debug, Clone)]
pub struct DevdRule {
    pub rule_id: String,
    pub subsystem: String,
    pub type_event: String,
    pub cdev_pattern: String,
    pub action_script: String,
}

/// Dynamic Device Event Record
#[derive(Debug, Clone)]
pub struct HotplugDeviceEvent {
    pub action: DeviceEventAction,
    pub subsystem: String,
    pub sysname: String,
    pub driver: Option<String>,
    pub env: HashMap<String, String>,
}

/// Combined Linux udev & FreeBSD devd Rule Engine
#[derive(Debug, Clone)]
pub struct UdevDevdRuleEngine {
    pub udev_rules: Vec<UdevRule>,
    pub devd_rules: Vec<DevdRule>,
    pub created_symlinks: HashMap<String, String>, // symlink -> dev_node
    pub executed_actions: Vec<String>,
}

impl UdevDevdRuleEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            udev_rules: Vec::new(),
            devd_rules: Vec::new(),
            created_symlinks: HashMap::new(),
            executed_actions: Vec::new(),
        };
        engine.register_default_system_rules();
        engine
    }

    /// Registers default system udev and devd rules
    pub fn register_default_system_rules(&mut self) {
        // 99-input.rules
        self.udev_rules.push(UdevRule {
            rule_id: "99-input-mouse".to_string(),
            subsystem: "input".to_string(),
            sysname_pattern: "mouse*".to_string(),
            driver_match: None,
            env_matches: HashMap::new(),
            symlink_name: Some("input/by-id/mouse-event".to_string()),
            mode: 0o660,
            group: "input".to_string(),
            run_command: None,
        });

        // 70-persistent-net.rules
        self.udev_rules.push(UdevRule {
            rule_id: "70-persistent-net".to_string(),
            subsystem: "net".to_string(),
            sysname_pattern: "eth*".to_string(),
            driver_match: Some("e1000e".to_string()),
            env_matches: HashMap::new(),
            symlink_name: None,
            mode: 0o660,
            group: "netdev".to_string(),
            run_command: Some("/sbin/sigma-net-setup".to_string()),
        });

        // FreeBSD devd USB attach rule
        self.devd_rules.push(DevdRule {
            rule_id: "devd-usb-attach".to_string(),
            subsystem: "USB".to_string(),
            type_event: "ATTACH".to_string(),
            cdev_pattern: "ugen*".to_string(),
            action_script: "/etc/rc.d/usbd restart".to_string(),
        });
    }

    /// Processes an incoming hotplug device event against udev and devd rule tables
    pub fn process_event(&mut self, event: &HotplugDeviceEvent) -> usize {
        let mut matched = 0;

        // Evaluate udev rules
        for rule in &self.udev_rules {
            if rule.subsystem == event.subsystem {
                let pattern = rule.sysname_pattern.replace('*', "");
                if event.sysname.contains(&pattern) {
                    if let Some(drv) = &rule.driver_match {
                        if event.driver.as_ref() != Some(drv) {
                            continue;
                        }
                    }

                    matched += 1;
                    if let Some(symlink) = &rule.symlink_name {
                        let dev_path = format!("/dev/{}", event.sysname);
                        self.created_symlinks.insert(format!("/dev/{}", symlink), dev_path);
                    }

                    if let Some(cmd) = &rule.run_command {
                        self.executed_actions.push(format!("RUN: {} {}", cmd, event.sysname));
                    }
                }
            }
        }

        // Evaluate FreeBSD devd rules
        let devd_event_type = match event.action {
            DeviceEventAction::Add => "ATTACH",
            DeviceEventAction::Remove => "DETACH",
            _ => "NOTIFY",
        };

        for rule in &self.devd_rules {
            if rule.subsystem.eq_ignore_ascii_case(&event.subsystem)
                && rule.type_event == devd_event_type
            {
                matched += 1;
                self.executed_actions.push(format!("DEVD_ACTION: {}", rule.action_script));
            }
        }

        matched
    }
}

impl Default for UdevDevdRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udev_rule_matching_and_symlink() {
        let mut engine = UdevDevdRuleEngine::new();

        let mouse_event = HotplugDeviceEvent {
            action: DeviceEventAction::Add,
            subsystem: "input".to_string(),
            sysname: "mouse0".to_string(),
            driver: None,
            env: HashMap::new(),
        };

        let matches = engine.process_event(&mouse_event);
        assert!(matches >= 1);
        assert_eq!(
            engine.created_symlinks.get("/dev/input/by-id/mouse-event").unwrap(),
            "/dev/mouse0"
        );
    }

    #[test]
    fn test_devd_usb_attach_rule() {
        let mut engine = UdevDevdRuleEngine::new();

        let usb_event = HotplugDeviceEvent {
            action: DeviceEventAction::Add,
            subsystem: "usb".to_string(),
            sysname: "ugen0.1".to_string(),
            driver: None,
            env: HashMap::new(),
        };

        let matches = engine.process_event(&usb_event);
        assert!(matches >= 1);
        assert!(engine.executed_actions.iter().any(|a| a.contains("DEVD_ACTION")));
    }
}
