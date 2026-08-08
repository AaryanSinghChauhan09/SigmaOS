/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// and target states to defeat Fedora's Systemd initialization.

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type UnitID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Service,
    Target,
    Socket,
    Timer,
    Path,
    Mount,
    Device,
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
pub struct JournalEntry {
    pub unit_id: UnitID,
    pub message: [u8; 64],
    pub from_state: UnitState,
    pub to_state: UnitState,
}

impl JournalEntry {
    pub fn new(unit_id: UnitID, msg: &[u8], from_state: UnitState, to_state: UnitState) -> Self {
        let mut msg_arr = [0u8; 64];
        let len = msg.len().min(63);
        msg_arr[..len].copy_from_slice(&msg[..len]);
        JournalEntry {
            unit_id,
            message: msg_arr,
            from_state,
            to_state,
        }
    }
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
    pub conflicts: Vec<UnitID>,
    pub binds_to: Vec<UnitID>,
    pub part_of: Vec<UnitID>,
    pub restart_policy: RestartPolicy,
    pub restart_count: usize,
    pub startup_time_ms: u64,
    pub duration_ms: u64,
    pub is_enabled: bool,
    pub triggered_unit: Option<UnitID>,
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
            conflicts: Vec::new(),
            binds_to: Vec::new(),
            part_of: Vec::new(),
            restart_policy: RestartPolicy::No,
            restart_count: 0,
            startup_time_ms: 0,
            duration_ms: 0,
            is_enabled: true,
            triggered_unit: None,
        }
    }
}

pub struct SystemdEngine {
    pub units: Vec<SystemdUnit>,
    pub current_target: AtomicUsize, // stores UnitID of active target
    pub journal: Vec<JournalEntry>,
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
            journal: Vec::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn find_unit(&self, id: UnitID) -> Option<&SystemdUnit> {
        for unit in &self.units {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn find_unit_mut(&mut self, id: UnitID) -> Option<&mut SystemdUnit> {
        for unit in &mut self.units {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn log_journal(&mut self, unit_id: UnitID, message: &[u8], from_state: UnitState, to_state: UnitState) {
        let entry = JournalEntry::new(unit_id, message, from_state, to_state);
        self.journal.push(entry);
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
                unit.state = UnitState::Active;
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
                for &req in &unit.requires {
                    set.push(req);
                    self.collect_dependencies(req, set);
                }
                for &want in &unit.wants {
                    set.push(want);
                    self.collect_dependencies(want, set);
                }
            }
        }
    }

    pub fn get_active_target_id(&self) -> usize {
        self.current_target.load(Ordering::SeqCst)
    }

    // Topological sorting with cycle detection
    pub fn topological_sort(&self, unit_ids: &Vec<UnitID>) -> Result<Vec<UnitID>, &'static str> {
        let mut sorted = Vec::new();
        let mut visiting = Vec::new();
        let mut visited = Vec::new();

        for &id in unit_ids.iter() {
            if !visited.contains(&id) {
                self.topo_visit(id, unit_ids, &mut sorted, &mut visiting, &mut visited)?;
            }
        }
        Ok(sorted)
    }

    fn topo_visit(
        &self,
        id: UnitID,
        all_ids: &Vec<UnitID>,
        sorted: &mut Vec<UnitID>,
        visiting: &mut Vec<UnitID>,
        visited: &mut Vec<UnitID>,
    ) -> Result<(), &'static str> {
        if visiting.contains(&id) {
            return Err("Dependency cycle detected");
        }
        if visited.contains(&id) {
            return Ok(());
        }

        visiting.push(id);

        if let Some(unit) = self.find_unit(id) {
            for &other_id in all_ids.iter() {
                if other_id == id {
                    continue;
                }
                let mut is_before = false;
                if unit.after.contains(&other_id) || unit.requires.contains(&other_id) || unit.wants.contains(&other_id) {
                    is_before = true;
                }
                if let Some(other) = self.find_unit(other_id) {
                    if other.before.contains(&id) {
                        is_before = true;
                    }
                }

                if is_before {
                    self.topo_visit(other_id, all_ids, sorted, visiting, visited)?;
                }
            }
        }

        visiting.retain(|&x| x != id);
        visited.push(id);
        sorted.push(id);
        Ok(())
    }

    // systemctl controls
    pub fn systemctl_start(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (is_enabled, conflicts, requires, wants) = if let Some(unit) = self.find_unit(id) {
            if unit.state == UnitState::Active {
                return Ok(()); // Already running
            }
            (unit.is_enabled, unit.conflicts.clone(), unit.requires.clone(), unit.wants.clone())
        } else {
            return Err("Unit not found");
        };

        if !is_enabled {
            return Err("Unit is disabled");
        }

        // Handle conflicts: stop conflicting units
        for &conflict_id in conflicts.iter() {
            if let Some(conf_unit) = self.find_unit(conflict_id) {
                if conf_unit.state == UnitState::Active {
                    self.systemctl_stop(conflict_id)?;
                }
            }
        }

        // Also stop units that conflict with this unit
        let mut units_to_stop = Vec::new();
        for u in self.units.iter() {
            if u.conflicts.contains(&id) && u.state == UnitState::Active {
                units_to_stop.push(u.id);
            }
        }
        for &stop_id in units_to_stop.iter() {
            self.systemctl_stop(stop_id)?;
        }

        // Start requires dependencies (must succeed)
        for &req_id in requires.iter() {
            if let Err(_) = self.systemctl_start(req_id) {
                if let Some(u) = self.find_unit_mut(id) {
                    u.state = UnitState::Failed;
                }
                self.log_journal(id, b"Required dependency failed to start", UnitState::Inactive, UnitState::Failed);
                return Err("Required dependency failed to start");
            }
        }

        // Start wants dependencies (failures are ignored)
        for &want_id in wants.iter() {
            let _ = self.systemctl_start(want_id);
        }

        // Update unit state to Activating
        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(id, b"Unit starting/activating", UnitState::Inactive, UnitState::Activating);

        // Transition to Active, record timings
        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
            u.startup_time_ms = 100; // Mock timing for analyze compatibility
            u.duration_ms = 150;
        }
        self.log_journal(id, b"Unit started successfully", UnitState::Activating, UnitState::Active);

        Ok(())
    }

    pub fn systemctl_stop(&mut self, id: UnitID) -> Result<(), &'static str> {
        let exists = self.find_unit(id).is_some();
        if !exists {
            return Err("Unit not found");
        }

        // Update state to Deactivating
        if let Some(u) = self.find_unit_mut(id) {
            if u.state == UnitState::Inactive {
                return Ok(());
            }
            u.state = UnitState::Deactivating;
        }
        self.log_journal(id, b"Unit stopping/deactivating", UnitState::Active, UnitState::Deactivating);

        // Transition to Inactive
        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Inactive;
        }
        self.log_journal(id, b"Unit stopped successfully", UnitState::Deactivating, UnitState::Inactive);

        // BindsTo dependency handling
        let mut bound_units = Vec::new();
        for u in self.units.iter() {
            if u.binds_to.contains(&id) && u.state == UnitState::Active {
                bound_units.push(u.id);
            }
        }

        for &b_id in bound_units.iter() {
            self.systemctl_stop(b_id)?;
        }

        Ok(())
    }

    pub fn systemctl_restart(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.systemctl_stop(id)?;
        self.systemctl_start(id)?;
        Ok(())
    }

    pub fn systemctl_enable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = true;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_disable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = false;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_reload(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (state, is_active) = if let Some(u) = self.find_unit(id) {
            (u.state, u.state == UnitState::Active)
        } else {
            return Err("Unit not found");
        };
        if !is_active {
            return Err("Cannot reload inactive unit");
        }

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(id, b"Reloading unit configuration", state, UnitState::Activating);

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
        }
        self.log_journal(id, b"Unit reloaded successfully", UnitState::Activating, UnitState::Active);
        Ok(())
    }

    pub fn systemctl_status(&self, id: UnitID) -> Option<UnitState> {
        self.find_unit(id).map(|u| u.state)
    }

    // Restart Policy handling
    pub fn handle_unit_failure(&mut self, id: UnitID) -> Result<bool, &'static str> {
        let mut should_restart = false;
        let (policy, count) = if let Some(unit) = self.find_unit(id) {
            (unit.restart_policy, unit.restart_count)
        } else {
            return Err("Unit not found");
        };

        if policy == RestartPolicy::Always || (policy == RestartPolicy::OnFailure) {
            if count < 3 {
                should_restart = true;
            }
        }

        if should_restart {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.restart_count += 1;
                unit.state = UnitState::Activating;
            }
            self.log_journal(id, b"Auto-restarting failed unit based on restart policy", UnitState::Failed, UnitState::Activating);
            self.systemctl_start(id)?;
            Ok(true)
        } else {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.state = UnitState::Failed;
            }
            self.log_journal(id, b"Unit entered Failed state, restart policy not met or limit exceeded", UnitState::Inactive, UnitState::Failed);
            Ok(false)
        }
    }

    // Trigger Activations
    pub fn trigger_socket_activation(&mut self, socket_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(socket_id) {
            if unit.unit_type != UnitType::Socket {
                return Err("Unit is not a Socket");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(socket_id, b"Socket activation triggered", UnitState::Active, UnitState::Active);
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this socket")
        }
    }

    pub fn trigger_path_activation(&mut self, path_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(path_id) {
            if unit.unit_type != UnitType::Path {
                return Err("Unit is not a Path");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(path_id, b"Path modification activation triggered", UnitState::Active, UnitState::Active);
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this path")
        }
    }

    pub fn trigger_timer_activation(&mut self, timer_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(timer_id) {
            if unit.unit_type != UnitType::Timer {
                return Err("Unit is not a Timer");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(timer_id, b"Timer activation triggered", UnitState::Active, UnitState::Active);
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this timer")
        }
    }

    // systemd-analyze blame
    pub fn systemd_analyze_blame(&self) -> Vec<(UnitID, u64)> {
        let mut blame_list = Vec::new();
        for unit in self.units.iter() {
            if unit.state == UnitState::Active {
                blame_list.push((unit.id, unit.duration_ms));
            }
        }
        for i in 0..blame_list.len() {
            for j in 0..blame_list.len() - 1 - i {
                if blame_list[j].1 < blame_list[j + 1].1 {
                    let temp = blame_list[j];
                    blame_list[j] = blame_list[j + 1];
                    blame_list[j + 1] = temp;
                }
            }
        }
        blame_list
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
    fn test_systemd_topological_sort_and_cycle_detection() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.before.push(2); // a must run before b

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.before.push(3); // b must run before c

        let c = SystemdUnit::new(3, b"c.service", UnitType::Service);

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        let mut ids = Vec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);

        // Now inject a cycle
        let mut engine_cycle = SystemdEngine::new();
        let mut u1 = SystemdUnit::new(10, b"u1.service", UnitType::Service);
        u1.before.push(20);
        let mut u2 = SystemdUnit::new(20, b"u2.service", UnitType::Service);
        u2.before.push(10); // cycle: 10 -> 20 -> 10

        engine_cycle.register_unit(u1);
        engine_cycle.register_unit(u2);

        let mut cycle_ids = Vec::new();
        cycle_ids.push(10);
        cycle_ids.push(20);

        assert!(engine_cycle.topological_sort(&cycle_ids).is_err());
    }

    #[test]
    fn test_systemd_conflicts_and_binds_to() {
        let mut engine = SystemdEngine::new();

        // Conflicts: a conflicts with b. Starting a should stop b.
        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.conflicts.push(2);

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active; // b starts active

        // BindsTo: c binds to a. If a stops, c stops.
        let mut c = SystemdUnit::new(3, b"c.service", UnitType::Service);
        c.binds_to.push(1);
        c.state = UnitState::Active;

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        // Start a
        engine.systemctl_start(1).unwrap();

        // Check a is active, b is inactive (conflict resolution)
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
        assert_eq!(engine.systemctl_status(2), Some(UnitState::Inactive));

        // Stop a, check that c also stops (BindsTo dependency resolution)
        engine.systemctl_stop(1).unwrap();
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Inactive));
        assert_eq!(engine.systemctl_status(3), Some(UnitState::Inactive));
    }

    #[test]
    fn test_systemd_activation_triggers() {
        let mut engine = SystemdEngine::new();

        // Socket activations
        let mut socket = SystemdUnit::new(1, b"test.socket", UnitType::Socket);
        socket.triggered_unit = Some(10);

        let srv = SystemdUnit::new(10, b"test.service", UnitType::Service);

        engine.register_unit(socket);
        engine.register_unit(srv);

        assert_eq!(engine.systemctl_status(10), Some(UnitState::Inactive));

        // Trigger socket
        engine.trigger_socket_activation(1).unwrap();
        assert_eq!(engine.systemctl_status(10), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_restart_policy() {
        let mut engine = SystemdEngine::new();

        let mut fail_srv = SystemdUnit::new(1, b"fail.service", UnitType::Service);
        fail_srv.restart_policy = RestartPolicy::Always;

        engine.register_unit(fail_srv);

        // Fail service, should trigger restart policy
        let restarted = engine.handle_unit_failure(1).unwrap();
        assert!(restarted);
        assert_eq!(engine.find_unit(1).unwrap().restart_count, 1);
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_analyze_blame() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.state = UnitState::Active;
        a.duration_ms = 500;

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active;
        b.duration_ms = 1200;

        engine.register_unit(a);
        engine.register_unit(b);

        let blame = engine.systemd_analyze_blame();
        assert_eq!(blame.len(), 2);
        assert_eq!(blame[0].0, 2); // b.service longest
        assert_eq!(blame[0].1, 1200);
        assert_eq!(blame[1].0, 1);
        assert_eq!(blame[1].1, 500);
    }

    #[test]
    fn test_advanced_dependency_sorting() {
        let mut engine = SystemdEngine::new();

        // Setup a standard multi-target tree (no cycles)
        let mut basic = SystemdUnit::new(1, b"basic.target", UnitType::Target);
        basic.before.push(2); // basic before network

        let mut network = SystemdUnit::new(2, b"network.target", UnitType::Target);
        network.before.push(3); // network before multi-user

        let multi_user = SystemdUnit::new(3, b"multi-user.target", UnitType::Target);

        engine.register_unit(basic);
        engine.register_unit(network);
        engine.register_unit(multi_user);

        let mut ids = Vec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);
    }
}
