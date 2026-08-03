//! Systemd-Grade Init and Target State Engine for SigmaOS
//! Provides robust target dependency graphs, wants/requires properties,
//! and target states to defeat Fedora's Systemd initialization.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
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
    pub restart_policy: RestartPolicy,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub restart_count: usize,
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
            restart_policy: RestartPolicy::No,
            start_time_ms: 0,
            end_time_ms: 0,
            restart_count: 0,
        }
    }
}

pub struct SystemdEngine {
    pub units: Vec<SystemdUnit>,
    pub current_target: AtomicUsize, // stores UnitID of active target
    pub journals: Vec<[u8; 64]>,     // custom journald logging circular buffer representation
}

impl Default for SystemdEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdEngine {
    pub fn new() -> Self {
        SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
            journals: Vec::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn log_journal(&mut self, msg: &[u8]) {
        let mut entry = [0u8; 64];
        let len = msg.len().min(63);
        entry[..len].copy_from_slice(&msg[..len]);
        self.journals.push(entry);
    }

    pub fn get_unit(&self, id: UnitID) -> Option<&SystemdUnit> {
        for unit in &self.units {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn get_unit_mut(&mut self, id: UnitID) -> Option<&mut SystemdUnit> {
        for unit in &mut self.units {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn isolate_target(&mut self, target_id: UnitID) -> Result<(), &'static str> {
        let mut target_idx = None;
        for (i, unit) in self.units.iter().enumerate() {
            if unit.id == target_id && unit.unit_type == UnitType::Target {
                target_idx = Some(i);
                break;
            }
        }

        let _idx = target_idx.ok_or("Target unit not found")?;

        // Log to journald representation
        let mut msg = [0u8; 64];
        let prefix = b"Isolating target ID: ";
        msg[..prefix.len()].copy_from_slice(prefix);
        let mut id_tmp = target_id;
        let mut idx = prefix.len();
        if id_tmp == 0 {
            if idx < 64 { msg[idx] = b'0'; idx += 1; }
        } else {
            let mut digits = [0u8; 10];
            let mut d_idx = 0;
            while id_tmp > 0 && d_idx < 10 {
                digits[d_idx] = (id_tmp % 10) as u8 + b'0';
                id_tmp /= 10;
                d_idx += 1;
            }
            while d_idx > 0 && idx < 64 {
                d_idx -= 1;
                msg[idx] = digits[d_idx];
                idx += 1;
            }
        }
        self.log_journal(&msg[..idx]);

        // In Systemd, isolating a target starts that target and all its dependencies,
        // and deactivates units not required/wanted by it.
        self.units[_idx].state = UnitState::Activating;

        // Collect all required/wanted units recursively
        let mut active_set = Vec::new();
        self.collect_dependencies(target_id, &mut active_set);

        // Deactivate units not in active_set
        for unit in &mut self.units {
            if !active_set.contains(&unit.id) && unit.id != target_id {
                unit.state = UnitState::Inactive;
            } else if active_set.contains(&unit.id) {
                // Track start & end times when activating
                if unit.state != UnitState::Active {
                    unit.state = UnitState::Active;
                    if unit.start_time_ms == 0 {
                        unit.start_time_ms = 10; // simulated start timestamp
                        unit.end_time_ms = 120; // simulated end timestamp (110ms duration)
                    }
                }
            }
        }

        self.units[_idx].state = UnitState::Active;
        self.current_target.store(target_id, Ordering::SeqCst);
        Ok(())
    }

    fn collect_dependencies(&self, unit_id: UnitID, set: &mut Vec<UnitID>) {
        if set.contains(&unit_id) {
            return;
        }

        for unit in &self.units {
            if unit.id == unit_id {
                for i in 0..unit.requires.len() {
                    let req = unit.requires[i];
                    set.push(req);
                    self.collect_dependencies(req, set);
                }
                for i in 0..unit.wants.len() {
                    let want = unit.wants[i];
                    set.push(want);
                    self.collect_dependencies(want, set);
                }
            }
        }
    }

    pub fn get_active_target_id(&self) -> usize {
        self.current_target.load(Ordering::SeqCst)
    }

    /// Resolves topological ordering for starting units considering before & after relationships
    pub fn resolve_unit_order(&self, unit_ids: &[UnitID]) -> Result<Vec<UnitID>, &'static str> {
        let mut order = Vec::new();
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for i in 0..unit_ids.len() {
            let id = unit_ids[i];
            if !visited.contains(&id) {
                self.topo_visit(id, unit_ids, &mut order, &mut visited, &mut rec_stack)?;
            }
        }
        Ok(order)
    }

    fn topo_visit(
        &self,
        id: UnitID,
        all_ids: &[UnitID],
        order: &mut Vec<UnitID>,
        visited: &mut Vec<UnitID>,
        rec_stack: &mut Vec<UnitID>,
    ) -> Result<(), &'static str> {
        if rec_stack.contains(&id) {
            return Err("Dependency cycle detected");
        }
        if visited.contains(&id) {
            return Ok(());
        }

        rec_stack.push(id);

        let mut deps = Vec::new();
        if let Some(unit) = self.get_unit(id) {
            for i in 0..unit.after.len() {
                let after_id = unit.after[i];
                if all_ids.contains(&after_id) && !deps.contains(&after_id) {
                    deps.push(after_id);
                }
            }
        }
        for other in &self.units {
            if all_ids.contains(&other.id) && other.before.contains(&id) && !deps.contains(&other.id) {
                deps.push(other.id);
            }
        }

        for i in 0..deps.len() {
            let dep_id = deps[i];
            self.topo_visit(dep_id, all_ids, order, visited, rec_stack)?;
        }

        rec_stack.pop();
        visited.push(id);
        order.push(id);
        Ok(())
    }

    /// systemd-analyze blame: returns units and their startup duration, sorted descending.
    pub fn get_analyze_blame(&self) -> Vec<(UnitID, u64)> {
        let mut blame = Vec::new();
        for unit in &self.units {
            if unit.start_time_ms > 0 && unit.end_time_ms >= unit.start_time_ms {
                let duration = unit.end_time_ms - unit.start_time_ms;
                blame.push((unit.id, duration));
            }
        }

        // Sort by duration descending
        for i in 0..blame.len() {
            for j in (i + 1)..blame.len() {
                if blame[j].1 > blame[i].1 {
                    let temp = blame[i];
                    blame[i] = blame[j];
                    blame[j] = temp;
                }
            }
        }

        blame
    }

    /// Polled state monitor tick checking failed services and triggering auto-restarts.
    pub fn monitor_tick(&mut self) {
        let mut restart_list = Vec::new();

        for unit in &self.units {
            if unit.unit_type == UnitType::Service && unit.state == UnitState::Failed {
                let trigger = match unit.restart_policy {
                    RestartPolicy::Always => true,
                    RestartPolicy::OnFailure => true,
                    RestartPolicy::No => false,
                };
                if trigger {
                    restart_list.push(unit.id);
                }
            }
        }

        for i in 0..restart_list.len() {
            let id = restart_list[i];
            if let Some(unit) = self.get_unit_mut(id) {
                unit.state = UnitState::Activating;
                unit.start_time_ms = 200; // simulated restart start
                unit.end_time_ms = 450;   // simulated restart end
                unit.restart_count += 1;
                unit.state = UnitState::Active;

                // Log auto-restart
                let mut msg = [0u8; 64];
                let prefix = b"Auto-restarting failed service: ";
                msg[..prefix.len()].copy_from_slice(prefix);
                let mut idx = prefix.len();
                let mut id_tmp = id;
                if id_tmp == 0 {
                    if idx < 64 { msg[idx] = b'0'; idx += 1; }
                } else {
                    let mut digits = [0u8; 10];
                    let mut d_idx = 0;
                    while id_tmp > 0 && d_idx < 10 {
                        digits[d_idx] = (id_tmp % 10) as u8 + b'0';
                        id_tmp /= 10;
                        d_idx += 1;
                    }
                    while d_idx > 0 && idx < 64 {
                        d_idx -= 1;
                        msg[idx] = digits[d_idx];
                        idx += 1;
                    }
                }
                self.journals.push(msg);
            }
        }
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
        for unit in &engine.units {
            if unit.id == 200 || unit.id == 300 || unit.id == 100 {
                assert_eq!(unit.state, UnitState::Active);
            } else if unit.id == 400 {
                assert_eq!(unit.state, UnitState::Inactive);
            }
        }
    }

    #[test]
    fn test_systemd_engine_topological_sorting() {
        let mut engine = SystemdEngine::new();

        let mut u1 = SystemdUnit::new(1, b"u1.service", UnitType::Service);
        u1.after.push(2); // 1 starts after 2 (i.e., 2 -> 1)

        let mut u2 = SystemdUnit::new(2, b"u2.service", UnitType::Service);
        u2.before.push(3); // 2 starts before 3 (i.e., 2 -> 3)

        let u3 = SystemdUnit::new(3, b"u3.service", UnitType::Service);

        engine.register_unit(u1);
        engine.register_unit(u2);
        engine.register_unit(u3);

        let list = [1, 2, 3];
        let ordered = engine.resolve_unit_order(&list).unwrap();

        // 2 has 1 after it, and 3 before it. So 2 must start before 1 and 3.
        assert_eq!(ordered[0], 2);
    }

    #[test]
    fn test_systemd_analyze_blame() {
        let mut engine = SystemdEngine::new();

        let mut u1 = SystemdUnit::new(1, b"u1.service", UnitType::Service);
        u1.start_time_ms = 100;
        u1.end_time_ms = 150; // duration 50ms

        let mut u2 = SystemdUnit::new(2, b"u2.service", UnitType::Service);
        u2.start_time_ms = 200;
        u2.end_time_ms = 400; // duration 200ms

        let mut u3 = SystemdUnit::new(3, b"u3.service", UnitType::Service);
        u3.start_time_ms = 50;
        u3.end_time_ms = 60; // duration 10ms

        engine.register_unit(u1);
        engine.register_unit(u2);
        engine.register_unit(u3);

        let blame = engine.get_analyze_blame();
        assert_eq!(blame.len(), 3);
        assert_eq!(blame[0].0, 2); // 200ms is largest duration
        assert_eq!(blame[1].0, 1); // 50ms
        assert_eq!(blame[2].0, 3); // 10ms
    }

    #[test]
    fn test_systemd_journal_logging_and_restart_policy() {
        let mut engine = SystemdEngine::new();

        let mut u1 = SystemdUnit::new(1, b"u1.service", UnitType::Service);
        u1.state = UnitState::Failed;
        u1.restart_policy = RestartPolicy::Always;

        engine.register_unit(u1);

        assert_eq!(engine.journals.len(), 0);

        engine.monitor_tick();

        assert_eq!(engine.journals.len(), 1);
        let u1_after = engine.get_unit(1).unwrap();
        assert_eq!(u1_after.state, UnitState::Active);
        assert_eq!(u1_after.restart_count, 1);
    }
}
