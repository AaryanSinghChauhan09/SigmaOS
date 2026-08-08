/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// and target states to defeat Fedora's Systemd initialization.

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
        SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
<<<<<<< HEAD
        };

        // Pre-register standard Ubuntu-style target system nodes
        engine.register_unit(SystemdUnit::new(9001, b"poweroff.target", UnitType::Target));
        engine.register_unit(SystemdUnit::new(9002, b"reboot.target", UnitType::Target));
        engine.register_unit(SystemdUnit::new(9003, b"emergency.target", UnitType::Target));

        engine
||||||| 23ef22a4a
            journal: Vec::new(),
        }
=======
        }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
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
}

<<<<<<< HEAD
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
||||||| 23ef22a4a
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

        visiting.remove_item(&id);
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
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
<<<<<<< HEAD
||||||| 23ef22a4a
    pub fn remove_item(&mut self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    for j in i..(self.len - 1) {
                        core::ptr::copy(self.data.add(j + 1), self.data.add(j), 1);
                    }
                    self.len -= 1;
                    return true;
                }
            }
        }
        false
    }
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(&T) -> bool {
        let mut write_idx = 0;
        for read_idx in 0..self.len {
            unsafe {
                if f(&*self.data.add(read_idx)) {
                    if write_idx != read_idx {
                        core::ptr::copy(self.data.add(read_idx), self.data.add(write_idx), 1);
                    }
                    write_idx += 1;
                } else {
                    core::ptr::drop_in_place(self.data.add(read_idx));
                }
            }
        }
        self.len = write_idx;
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
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
<<<<<<< HEAD

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
||||||| 23ef22a4a

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
=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}
