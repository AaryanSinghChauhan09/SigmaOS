/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// parallel topological service startup, restart policies, environment files,
/// and instance templates to match modern Linux distributions.

use core::sync::atomic::{AtomicUsize, Ordering};

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

    // Linux-inspired additions
    pub restart_policy: RestartPolicy,
    pub restart_sec_ms: u64,
    pub restart_count: usize,
    pub max_restarts: usize,
    pub is_template: bool,
    pub instance_name: [u8; 16],
    pub env_vars: Vec<([u8; 16], [u8; 32])>, // simulated key-value environment file
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
            restart_sec_ms: 100,
            restart_count: 0,
            max_restarts: 3,
            is_template: false,
            instance_name: [0u8; 16],
            env_vars: Vec::new(),
        }
    }

    /// Instantiate a templated unit (getty@.service -> getty@tty1.service)
    pub fn instantiate(&self, id: UnitID, instance: &[u8]) -> Self {
        let mut inst = self.clone();
        inst.id = id;
        inst.is_template = false;

        let inst_len = instance.len().min(15);
        inst.instance_name[..inst_len].copy_from_slice(&instance[..inst_len]);

        // Find index of '@' and build new name
        let name_len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        let mut at_pos = name_len;
        for i in 0..name_len {
            if self.name[i] == b'@' {
                at_pos = i;
                break;
            }
        }

        let mut new_name = [0u8; 32];
        let mut cursor = 0;

        // Copy before '@' (e.g. "getty")
        for i in 0..at_pos {
            new_name[cursor] = self.name[i];
            cursor += 1;
        }

        // Copy '@' and instance (e.g. "@tty1")
        new_name[cursor] = b'@';
        cursor += 1;
        for i in 0..inst_len {
            if cursor < 31 {
                new_name[cursor] = instance[i];
                cursor += 1;
            }
        }

        // Copy rest after '@' if any in template (e.g. ".service")
        let ext_start = at_pos + 1;
        if ext_start < name_len {
            for i in ext_start..name_len {
                if cursor < 31 {
                    new_name[cursor] = self.name[i];
                    cursor += 1;
                }
            }
        }

        inst.name = new_name;
        inst
    }

    /// Set restart policy (systemd Restart=)
    pub fn with_restart_policy(mut self, policy: RestartPolicy, delay_ms: u64, max_attempts: usize) -> Self {
        self.restart_policy = policy;
        self.restart_sec_ms = delay_ms;
        self.max_restarts = max_attempts;
        self
    }

    /// Set environment variable (systemd Environment= or EnvironmentFile=)
    pub fn with_env_var(&mut self, key: &[u8], val: &[u8]) {
        let mut key_arr = [0u8; 16];
        let mut val_arr = [0u8; 32];
        key_arr[..key.len().min(15)].copy_from_slice(&key[..key.len().min(15)]);
        val_arr[..val.len().min(31)].copy_from_slice(&val[..val.len().min(31)]);
        self.env_vars.push((key_arr, val_arr));
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
        SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    /// Simulates running the systemd recovery manager on service failure
    pub fn handle_service_failure(&mut self, id: UnitID) -> Result<UnitState, &'static str> {
        let mut unit_idx = None;
        for (i, unit) in self.units.iter().enumerate() {
            if unit.id == id && unit.unit_type == UnitType::Service {
                unit_idx = Some(i);
                break;
            }
        }

        let idx = unit_idx.ok_or("Service unit not found")?;
        self.units[idx].state = UnitState::Failed;

        let policy = self.units[idx].restart_policy;
        if policy == RestartPolicy::Always || policy == RestartPolicy::OnFailure {
            if self.units[idx].restart_count < self.units[idx].max_restarts {
                self.units[idx].restart_count += 1;
                self.units[idx].state = UnitState::Activating;
                self.units[idx].state = UnitState::Active;
                return Ok(UnitState::Active);
            }
        }

        Ok(UnitState::Failed)
    }

    /// Isolates a target: stops unneeded units, activates dependencies
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

    /// Parallel startup order: returns chronological execution blocks (topological ordering)
    /// respects 'Before=' and 'After=' systemd dependency specifications
    pub fn resolve_parallel_startup_blocks(&self) -> Vec<Vec<UnitID>> {
        let mut dependency_levels = Vec::new();
        let mut resolved = Vec::new();

        // Standard Kahn's topological sort level-by-level
        loop {
            let mut current_level = Vec::new();
            for unit in &self.units {
                if resolved.contains(&unit.id) {
                    continue;
                }

                // Check if all units this must run After (after dependency) are resolved
                let mut can_resolve = true;
                for &after_id in &unit.after {
                    if !resolved.contains(&after_id) {
                        // Check if after_id actually exists in our engine
                        let mut exists = false;
                        for u in &self.units {
                            if u.id == after_id {
                                exists = true;
                                break;
                            }
                        }
                        if exists {
                            can_resolve = false;
                            break;
                        }
                    }
                }

                // Check if any unresolved unit must run Before this (before dependency)
                if can_resolve {
                    for other in &self.units {
                        if !resolved.contains(&other.id) && other.before.contains(&unit.id) {
                            can_resolve = false;
                            break;
                        }
                    }
                }

                if can_resolve {
                    current_level.push(unit.id);
                }
            }

            if current_level.is_empty() {
                break;
            }

            for &id in &current_level {
                resolved.push(id);
            }
            dependency_levels.push(current_level);
        }

        dependency_levels
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
}

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter { vec: self, index: 0 }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut { data: self.data, len: self.len, index: 0, _marker: core::marker::PhantomData }
    }
    pub fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
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
    fn test_systemd_parallel_startup_with_ordering() {
        let mut engine = SystemdEngine::new();

        // Setup typical systemd dependencies:
        // network.service -> db.service -> web.service
        // db.service and logging.service can run in parallel (no order between them)
        let mut logging = SystemdUnit::new(1, b"logging.service", UnitType::Service);

        let mut network = SystemdUnit::new(2, b"network.service", UnitType::Service);

        let mut db = SystemdUnit::new(3, b"db.service", UnitType::Service);
        db.after.push(2); // Runs after network

        let mut web = SystemdUnit::new(4, b"web.service", UnitType::Service);
        web.after.push(3); // Runs after db
        web.after.push(1); // Runs after logging

        engine.register_unit(logging);
        engine.register_unit(network);
        engine.register_unit(db);
        engine.register_unit(web);

        let levels = engine.resolve_parallel_startup_blocks();

        // Assertions checking level execution hierarchy
        assert_eq!(levels.len(), 3);

        // Level 1: logging.service (1) and network.service (2) can run in parallel
        assert!(levels[0].contains(&1));
        assert!(levels[0].contains(&2));

        // Level 2: db.service (3) runs after network is resolved
        assert!(levels[1].contains(&3));

        // Level 3: web.service (4) runs after both db and logging are resolved
        assert_eq!(levels[2][0], 4);
    }

    #[test]
    fn test_systemd_restart_policies() {
        let mut engine = SystemdEngine::new();
        let service = SystemdUnit::new(1, b"failing.service", UnitType::Service)
            .with_restart_policy(RestartPolicy::OnFailure, 50, 2);
        engine.register_unit(service);

        // First failure triggers automatic restart
        let state1 = engine.handle_service_failure(1).unwrap();
        assert_eq!(state1, UnitState::Active);
        assert_eq!(engine.units[0].restart_count, 1);

        // Second failure triggers automatic restart
        let state2 = engine.handle_service_failure(1).unwrap();
        assert_eq!(state2, UnitState::Active);
        assert_eq!(engine.units[0].restart_count, 2);

        // Third failure exceeds max_restarts -> remains Failed
        let state3 = engine.handle_service_failure(1).unwrap();
        assert_eq!(state3, UnitState::Failed);
    }

    #[test]
    fn test_systemd_template_instantiation() {
        let template = SystemdUnit::new(10, b"getty@.service", UnitType::Service);
        assert_eq!(template.id, 10);

        let instance = template.instantiate(11, b"tty1");
        assert_eq!(instance.id, 11);

        let name_str = std::str::from_utf8(&instance.name).unwrap().trim_end_matches('\0');
        assert_eq!(name_str, "getty@tty1.service");
    }
}
