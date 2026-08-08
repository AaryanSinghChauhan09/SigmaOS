/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// and target states to defeat Fedora's Systemd initialization.
/// Enhanced with standard Ubuntu-style target states (reboot.target, poweroff.target,
/// emergency.target) and structured service controls (reload, restart, status checks).

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;

pub type UnitID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Service,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub id: UnitID,
    pub name: [u8; 32],
    pub unit_type: UnitType,
    pub state: UnitState,
    pub requires: Vec<UnitID>,
    pub wants: Vec<UnitID>,
    pub before: Vec<UnitID>,
    pub after: Vec<UnitID>,
}

impl SystemdUnit {
    pub fn new(id: UnitID, name: &[u8], unit_type: UnitType) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        SystemdUnit {
            id,
            name: name_arr,
            unit_type,
            state: UnitState::Inactive,
            requires: Vec::new(),
            wants: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}

pub struct SystemdEngine {
    pub units: Vec<SystemdUnit>,
    pub current_target: AtomicUsize, // stores UnitID of active target
}

impl Default for SystemdEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdEngine {
    pub fn new() -> Self {
        let mut engine = SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
        };

        // Pre-register standard Ubuntu-style target system nodes
        engine.register_unit(SystemdUnit::new(9001, b"poweroff.target", UnitType::Target));
        engine.register_unit(SystemdUnit::new(9002, b"reboot.target", UnitType::Target));
        engine.register_unit(SystemdUnit::new(9003, b"emergency.target", UnitType::Target));

        engine
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn isolate_target(&mut self, target_id: UnitID) -> Result<(), &'static str> {
        let mut target_idx = None;
        for i in 0..self.units.len() {
            if self.units[i].id == target_id && self.units[i].unit_type == UnitType::Target {
                target_idx = Some(i);
                break;
            }
        }

        let idx = target_idx.ok_or("Target unit not found")?;

        // In Systemd, isolating a target starts that target and all its dependencies,
        // and deactivates units not required/wanted by it.
        self.units[idx].state = UnitState::Activating;

        // Collect all required/wanted units recursively
        let mut active_set = Vec::new();
        self.collect_dependencies(target_id, &mut active_set);

        // Deactivate units not in active_set
        for i in 0..self.units.len() {
            let unit_id = self.units[i].id;
            if !active_set.contains(&unit_id) && unit_id != target_id {
                self.units[i].state = UnitState::Inactive;
            } else if active_set.contains(&unit_id) {
                self.units[i].state = UnitState::Active;
            }
        }

        self.units[idx].state = UnitState::Active;
        self.current_target.store(target_id, Ordering::SeqCst);
        Ok(())
    }

    fn collect_dependencies(&self, unit_id: UnitID, set: &mut Vec<UnitID>) {
        if set.contains(&unit_id) {
            return;
        }

        for i in 0..self.units.len() {
            let unit = &self.units[i];
            if unit.id == unit_id {
                for j in 0..unit.requires.len() {
                    let req = unit.requires[j];
                    set.push(req);
                    self.collect_dependencies(req, set);
                }
                for j in 0..unit.wants.len() {
                    let want = unit.wants[j];
                    set.push(want);
                    self.collect_dependencies(want, set);
                }
            }
        }
    }

    pub fn get_active_target_id(&self) -> usize {
        self.current_target.load(Ordering::SeqCst)
    }

    /// Ubuntu-style restart control
    pub fn restart_unit(&mut self, unit_id: UnitID) -> Result<(), &'static str> {
        for i in 0..self.units.len() {
            if self.units[i].id == unit_id {
                self.units[i].state = UnitState::Deactivating;
                self.units[i].state = UnitState::Inactive;
                self.units[i].state = UnitState::Activating;
                self.units[i].state = UnitState::Active;
                return Ok(());
            }
        }
        Err("Unit not found")
    }

    /// Ubuntu-style reload control
    pub fn reload_unit(&mut self, unit_id: UnitID) -> Result<(), &'static str> {
        for i in 0..self.units.len() {
            if self.units[i].id == unit_id {
                if self.units[i].state != UnitState::Active {
                    return Err("Cannot reload inactive unit");
                }
                // Simulate configuration reloading safely
                return Ok(());
            }
        }
        Err("Unit not found")
    }

    /// Retrieve active status of a supervised service unit
    pub fn get_unit_status(&self, unit_id: UnitID) -> Option<UnitState> {
        for i in 0..self.units.len() {
            if self.units[i].id == unit_id {
                return Some(self.units[i].state);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_engine_dependency_isolation() {
        let mut engine = SystemdEngine::new();

        // 1. Create graphical.target
        let mut graphical = SystemdUnit::new(100, b"graphical.target", UnitType::Target);
        graphical.requires.push(200); // requires multi-user.target
        graphical.wants.push(300);    // wants network.target

        // 2. Create multi-user.target
        let multi_user = SystemdUnit::new(200, b"multi-user.target", UnitType::Target);

        // 3. Create network.target
        let network = SystemdUnit::new(300, b"network.target", UnitType::Target);

        // 4. Create an independent service
        let service = SystemdUnit::new(400, b"apache.service", UnitType::Service);

        engine.register_unit(graphical);
        engine.register_unit(multi_user);
        engine.register_unit(network);
        engine.register_unit(service);

        // Isolate graphical.target
        engine.isolate_target(100).unwrap();

        assert_eq!(engine.get_active_target_id(), 100);

        // dependencies should be active
        for i in 0..engine.units.len() {
            let unit = &engine.units[i];
            if unit.id == 200 || unit.id == 300 || unit.id == 100 {
                assert_eq!(unit.state, UnitState::Active);
            } else if unit.id == 400 {
                assert_eq!(unit.state, UnitState::Inactive);
            }
        }
    }

    #[test]
    fn test_ubuntu_systemd_controls_and_targets() {
        let mut engine = SystemdEngine::new();

        // Verify standard Ubuntu target state registration
        assert_eq!(engine.get_unit_status(9001).unwrap(), UnitState::Inactive); // poweroff.target
        assert_eq!(engine.get_unit_status(9002).unwrap(), UnitState::Inactive); // reboot.target
        assert_eq!(engine.get_unit_status(9003).unwrap(), UnitState::Inactive); // emergency.target

        // Verify service reload on inactive fails
        let mut service = SystemdUnit::new(420, b"nginx.service", UnitType::Service);
        engine.register_unit(service);
        assert!(engine.reload_unit(420).is_err());

        // Restart transitions state
        assert!(engine.restart_unit(420).is_ok());
        assert_eq!(engine.get_unit_status(420).unwrap(), UnitState::Active);

        // Reload succeeds on active service
        assert!(engine.reload_unit(420).is_ok());
    }
}
