// SPDX-License-Identifier: MIT
// SigmaOS Runlevel ↔ Systemd Target Symlink Mapper Engine
// Dynamic filesystem symlink parsing and SysVInit / OpenRC runlevel conversion

#![allow(dead_code)]

use std::collections::HashMap;
use std::string::{String, ToString};

/// SysVInit Runlevel Representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SysvRunlevel {
    Runlevel0 = 0, // Poweroff / Halt
    Runlevel1 = 1, // Single-User / Rescue
    Runlevel2 = 2, // Multi-User without networking
    Runlevel3 = 3, // Full Multi-User console
    Runlevel4 = 4, // Custom / User-defined
    Runlevel5 = 5, // Graphical Desktop Environment
    Runlevel6 = 6, // Reboot
}

impl SysvRunlevel {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(SysvRunlevel::Runlevel0),
            '1' | 's' | 'S' => Some(SysvRunlevel::Runlevel1),
            '2' => Some(SysvRunlevel::Runlevel2),
            '3' => Some(SysvRunlevel::Runlevel3),
            '4' => Some(SysvRunlevel::Runlevel4),
            '5' => Some(SysvRunlevel::Runlevel5),
            '6' => Some(SysvRunlevel::Runlevel6),
            _ => None,
        }
    }

    pub fn to_systemd_target(&self) -> &'static str {
        match self {
            SysvRunlevel::Runlevel0 => "poweroff.target",
            SysvRunlevel::Runlevel1 => "rescue.target",
            SysvRunlevel::Runlevel2 => "multi-user-nonet.target",
            SysvRunlevel::Runlevel3 => "multi-user.target",
            SysvRunlevel::Runlevel4 => "multi-user.target",
            SysvRunlevel::Runlevel5 => "graphical.target",
            SysvRunlevel::Runlevel6 => "reboot.target",
        }
    }
}

/// Sovereign Runlevel to Target Symlink Mapper Engine
#[derive(Debug)]
pub struct RunlevelToTargetMapper {
    pub default_symlink_path: String,
    pub active_runlevel: SysvRunlevel,
    pub previous_runlevel: Option<SysvRunlevel>,
    pub symlink_mapping: HashMap<String, String>,
}

impl RunlevelToTargetMapper {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("/etc/systemd/system/default.target".to_string(), "/lib/systemd/system/graphical.target".to_string());
        map.insert("runlevel0.target".to_string(), "poweroff.target".to_string());
        map.insert("runlevel1.target".to_string(), "rescue.target".to_string());
        map.insert("runlevel2.target".to_string(), "multi-user-nonet.target".to_string());
        map.insert("runlevel3.target".to_string(), "multi-user.target".to_string());
        map.insert("runlevel5.target".to_string(), "graphical.target".to_string());
        map.insert("runlevel6.target".to_string(), "reboot.target".to_string());

        Self {
            default_symlink_path: "/etc/systemd/system/default.target".to_string(),
            active_runlevel: SysvRunlevel::Runlevel5,
            previous_runlevel: Some(SysvRunlevel::Runlevel3),
            symlink_mapping: map,
        }
    }

    pub fn resolve_default_target(&self) -> String {
        if let Some(target) = self.symlink_mapping.get(&self.default_symlink_path) {
            target.split('/').last().unwrap_or("graphical.target").to_string()
        } else {
            "graphical.target".to_string()
        }
    }

    pub fn set_default_target_symlink(&mut self, target_name: &str) {
        let symlink_dest = format!("/lib/systemd/system/{}", target_name);
        self.symlink_mapping.insert(self.default_symlink_path.clone(), symlink_dest);
    }

    pub fn telinit_switch(&mut self, runlevel_char: char) -> Result<String, &'static str> {
        let new_runlevel = SysvRunlevel::from_char(runlevel_char)
            .ok_or("Invalid runlevel character")?;

        self.previous_runlevel = Some(self.active_runlevel);
        self.active_runlevel = new_runlevel;
        Ok(new_runlevel.to_systemd_target().to_string())
    }

    pub fn query_runlevel(&self) -> (char, char) {
        let prev = match self.previous_runlevel {
            Some(rl) => (rl as u8 + b'0') as char,
            None => 'N',
        };
        let curr = (self.active_runlevel as u8 + b'0') as char;
        (prev, curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runlevel_target_mapping_and_telinit() {
        let mut mapper = RunlevelToTargetMapper::new();
        assert_eq!(mapper.resolve_default_target(), "graphical.target");

        mapper.set_default_target_symlink("multi-user.target");
        assert_eq!(mapper.resolve_default_target(), "multi-user.target");

        let target = mapper.telinit_switch('3').unwrap();
        assert_eq!(target, "multi-user.target");

        let (prev, curr) = mapper.query_runlevel();
        assert_eq!(prev, '5');
        assert_eq!(curr, '3');
    }
}
