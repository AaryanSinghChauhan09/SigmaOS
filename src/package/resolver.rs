#![no_std]
#![no_main]

/// OOP-based Dependency Resolver Engine for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 5
/// Implements deterministic solver with conflict diagnostics

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PackageID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResolverError { Success = 0, Conflict = 1, NotFound = 2, Cycle = 3 }

pub trait Dependency {
    fn package_id(&self) -> PackageID;
    fn dependencies(&self) -> Vec<PackageID>;
    fn version(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleDependency {
    pub package_id: PackageID,
    pub deps: Vec<PackageID>,
    pub version: [u8; 32],
}

impl SimpleDependency {
    pub fn new(id: PackageID, version: &[u8]) -> Self {
        let mut version_array = [0u8; 32];
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleDependency {
            package_id: id,
            deps: Vec::new(),
            version: version_array,
        }
    }
}

impl Dependency for SimpleDependency {
    fn package_id(&self) -> PackageID { self.package_id }
    fn dependencies(&self) -> Vec<PackageID> { self.deps.clone() }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        &self.version[..len]
    }
}

pub trait DependencyResolver {
    fn add_dependency(&mut self, dep: Box<dyn Dependency>) -> Result<(), ResolverError>;
    fn resolve(&self, target: PackageID) -> Result<Vec<PackageID>, ResolverError>;
    fn detect_conflicts(&self, target: PackageID) -> Vec<(PackageID, PackageID)>;
    fn detect_cycles(&self) -> Vec<PackageID>;
}

#[repr(C)]
pub struct SimpleDependencyResolver {
    pub dependencies: Vec<Option<Box<dyn Dependency>>>,
    pub resolved: Vec<PackageID>,
}

impl SimpleDependencyResolver {
    pub fn new() -> Self {
        SimpleDependencyResolver {
            dependencies: Vec::new(),
            resolved: Vec::new(),
        }
    }
}

impl DependencyResolver for SimpleDependencyResolver {
    fn add_dependency(&mut self, dep: Box<dyn Dependency>) -> Result<(), ResolverError> {
        self.dependencies.push(Some(dep));
        Ok(())
    }

    fn resolve(&self, target: PackageID) -> Result<Vec<PackageID>, ResolverError> {
        let mut resolved = Vec::new();
        let mut visited = Vec::new();

        self.visit(target, &mut resolved, &mut visited)?;

        Ok(resolved)
    }

    fn detect_conflicts(&self, target: PackageID) -> Vec<(PackageID, PackageID)> {
        let mut conflicts = Vec::new();
        let mut all_deps = Vec::new();

        for dep_option in &self.dependencies {
            if let Some(ref dep) = *dep_option {
                if dep.package_id() == target {
                    all_deps = dep.dependencies();
                    break;
                }
            }
        }

        for &dep_id in &all_deps {
            for dep_option in &self.dependencies {
                if let Some(ref dep) = *dep_option {
                    if dep.package_id() == dep_id {
                        for &sub_dep in &dep.dependencies() {
                            if all_deps.contains(&sub_dep) {
                                conflicts.push((dep_id, sub_dep));
                            }
                        }
                    }
                }
            }
        }

        conflicts
    }

    fn detect_cycles(&self) -> Vec<PackageID> {
        let mut cycles = Vec::new();
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for dep_option in &self.dependencies {
            if let Some(ref dep) = *dep_option {
                let id = dep.package_id();
                if !visited.contains(&id) {
                    if self.has_cycle(id, &mut visited, &mut rec_stack) {
                        cycles.push(id);
                    }
                }
            }
        }

        cycles
    }
}

impl SimpleDependencyResolver {
    fn visit(&self, id: PackageID, resolved: &mut Vec<PackageID>, visited: &mut Vec<PackageID>) -> Result<(), ResolverError> {
        if visited.contains(&id) {
            return Err(ResolverError::Cycle);
        }

        visited.push(id);

        for dep_option in &self.dependencies {
            if let Some(ref dep) = *dep_option {
                if dep.package_id() == id {
                    for &dep_id in &dep.dependencies() {
                        self.visit(dep_id, resolved, visited)?;
                    }
                }
            }
        }

        if !resolved.contains(&id) {
            resolved.push(id);
        }

        Ok(())
    }

    fn has_cycle(&self, id: PackageID, visited: &mut Vec<PackageID>, rec_stack: &mut Vec<PackageID>) -> bool {
        visited.push(id);
        rec_stack.push(id);

        for dep_option in &self.dependencies {
            if let Some(ref dep) = *dep_option {
                if dep.package_id() == id {
                    for &dep_id in &dep.dependencies() {
                        if !visited.contains(&dep_id) {
                            if self.has_cycle(dep_id, visited, rec_stack) {
                                return true;
                            }
                        } else if rec_stack.contains(&dep_id) {
                            return true;
                        }
                    }
                }
            }
        }

        rec_stack.pop();
        false
    }
}

pub trait VersionConstraint {
    fn satisfies(&self, version: &[u8]) -> bool;
    fn to_string(&self) -> &[u8];
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ConstraintType { Exact = 0, Greater = 1, Less = 2, GreaterEqual = 3, LessEqual = 4 }

#[repr(C)]
pub struct SimpleVersionConstraint {
    pub constraint_type: ConstraintType,
    pub version: [u8; 32],
}

impl SimpleVersionConstraint {
    pub fn new(constraint_type: ConstraintType, version: &[u8]) -> Self {
        let mut version_array = [0u8; 32];
        let version_len = version.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(version.as_ptr(), version_array.as_mut_ptr(), version_len);
        }
        SimpleVersionConstraint {
            constraint_type,
            version: version_array,
        }
    }
}

impl VersionConstraint for SimpleVersionConstraint {
    fn satisfies(&self, version: &[u8]) -> bool {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(32);
        let constraint_version = &self.version[..len];

        match self.constraint_type {
            ConstraintType::Exact => constraint_version == version,
            ConstraintType::Greater => version > constraint_version,
            ConstraintType::Less => version < constraint_version,
            ConstraintType::GreaterEqual => version >= constraint_version,
            ConstraintType::LessEqual => version <= constraint_version,
        }
    }

    fn to_string(&self) -> &[u8] {
        match self.constraint_type {
            ConstraintType::Exact => b"==",
            ConstraintType::Greater => b">",
            ConstraintType::Less => b"<",
            ConstraintType::GreaterEqual => b">=",
            ConstraintType::LessEqual => b"<=",
        }
    }
}

pub trait ConflictResolver {
    fn resolve_conflict(&mut self, pkg1: PackageID, pkg2: PackageID) -> Result<PackageID, ResolverError>;
    fn get_resolution_strategy(&self) -> ResolutionStrategy;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResolutionStrategy { Newest = 0, Oldest = 1, Manual = 2 }

#[repr(C)]
pub struct SimpleConflictResolver {
    pub strategy: AtomicUsize,
}

impl SimpleConflictResolver {
    pub fn new(strategy: ResolutionStrategy) -> Self {
        SimpleConflictResolver {
            strategy: AtomicUsize::new(strategy as usize),
        }
    }
}

impl ConflictResolver for SimpleConflictResolver {
    fn resolve_conflict(&mut self, pkg1: PackageID, pkg2: PackageID) -> Result<PackageID, ResolverError> {
        let strategy = unsafe { core::mem::transmute(self.strategy.load(Ordering::SeqCst)) };
        match strategy {
            ResolutionStrategy::Newest => Ok(pkg1.max(pkg2)),
            ResolutionStrategy::Oldest => Ok(pkg1.min(pkg2)),
            ResolutionStrategy::Manual => Err(ResolverError::Conflict),
        }
    }

    fn get_resolution_strategy(&self) -> ResolutionStrategy {
        unsafe { core::mem::transmute(self.strategy.load(Ordering::SeqCst)) }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
