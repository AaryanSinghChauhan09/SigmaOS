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
        }
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

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

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
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
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
        graphical.wants.push(300); // wants network.target

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
}


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

