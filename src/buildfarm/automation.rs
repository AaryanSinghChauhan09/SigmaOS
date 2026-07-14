#![no_std]
#![no_main]

/// OOP-based Build Farm Automation for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 13
/// Implements scalable builders for multiple targets and architectures

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BuilderID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Architecture { X86_64 = 0, ARM64 = 1, RISCV64 = 2, PPC64 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BuilderState { Idle = 0, Building = 1, Failed = 2, Success = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BuildError { Success = 0, BuilderBusy = 1, InvalidTarget = 2, BuildFailed = 3 }

pub trait Builder {
    fn id(&self) -> BuilderID;
    fn architecture(&self) -> Architecture;
    fn state(&self) -> BuilderState;
    fn start_build(&mut self, target: &[u8]) -> Result<(), BuildError>;
    fn get_status(&self) -> BuilderState;
}

#[repr(C)]
pub struct SimpleBuilder {
    pub id: BuilderID,
    pub architecture: AtomicUsize,
    pub state: AtomicUsize,
    pub current_target: [u8; 128],
}

impl SimpleBuilder {
    pub fn new(id: BuilderID, architecture: Architecture) -> Self {
        SimpleBuilder {
            id,
            architecture: AtomicUsize::new(architecture as usize),
            state: AtomicUsize::new(BuilderState::Idle as usize),
            current_target: [0u8; 128],
        }
    }
}

impl Builder for SimpleBuilder {
    fn id(&self) -> BuilderID { self.id }
    fn architecture(&self) -> Architecture { unsafe { core::mem::transmute(self.architecture.load(Ordering::SeqCst)) } }
    fn state(&self) -> BuilderState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn start_build(&mut self, target: &[u8]) -> Result<(), BuildError> {
        if self.state.load(Ordering::SeqCst) != BuilderState::Idle as usize {
            return Err(BuildError::BuilderBusy);
        }

        let len = target.len().min(127);
        for i in 0..len {
            self.current_target[i] = target[i];
        }

        self.state.store(BuilderState::Building as usize, Ordering::SeqCst);
        Ok(())
    }

    fn get_status(&self) -> BuilderState { self.state() }
}

pub trait BuildFarm {
    fn add_builder(&mut self, builder: Box<dyn Builder>) -> Result<BuilderID, BuildError>;
    fn remove_builder(&mut self, id: BuilderID) -> Result<(), BuildError>;
    fn get_builder(&self, id: BuilderID) -> Option<&dyn Builder>;
    fn find_idle_builder(&self, architecture: Architecture) -> Option<BuilderID>;
    fn queue_build(&mut self, target: &[u8], architecture: Architecture) -> Result<(), BuildError>;
}

#[repr(C)]
pub struct SimpleBuildFarm {
    pub builders: Vec<Option<Box<dyn Builder>>>,
    pub next_id: AtomicUsize,
    pub build_queue: Vec<([u8; 128], Architecture)>,
}

impl SimpleBuildFarm {
    pub fn new() -> Self {
        SimpleBuildFarm {
            builders: Vec::new(),
            next_id: AtomicUsize::new(1),
            build_queue: Vec::new(),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let builder1 = SimpleBuilder::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::X86_64);
        self.builders.push(Some(Box::new(builder1)));

        let builder2 = SimpleBuilder::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::ARM64);
        self.builders.push(Some(Box::new(builder2)));

        let builder3 = SimpleBuilder::new(self.next_id.fetch_add(1, Ordering::SeqCst), Architecture::RISCV64);
        self.builders.push(Some(Box::new(builder3)));
    }
}

impl BuildFarm for SimpleBuildFarm {
    fn add_builder(&mut self, builder: Box<dyn Builder>) -> Result<BuilderID, BuildError> {
        let id = builder.id();
        self.builders.push(Some(builder));
        Ok(id)
    }

    fn remove_builder(&mut self, id: BuilderID) -> Result<(), BuildError> {
        for builder_option in &mut self.builders {
            if let Some(ref builder) = *builder_option {
                if builder.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BuildError::InvalidTarget)
    }

    fn get_builder(&self, id: BuilderID) -> Option<&dyn Builder> {
        for builder_option in &self.builders {
            if let Some(ref builder) = *builder_option {
                if builder.id() == id { return Some(builder.as_ref()); }
            }
        }
        None
    }

    fn find_idle_builder(&self, architecture: Architecture) -> Option<BuilderID> {
        for builder_option in &self.builders {
            if let Some(ref builder) = *builder_option {
                if builder.architecture() == architecture && builder.state() == BuilderState::Idle {
                    return Some(builder.id());
                }
            }
        }
        None
    }

    fn queue_build(&mut self, target: &[u8], architecture: Architecture) -> Result<(), BuildError> {
        let mut target_array = [0u8; 128];
        let target_len = target.len().min(127);
        for i in 0..target_len {
            target_array[i] = target[i];
        }
        self.build_queue.push((target_array, architecture));
        Ok(())
    }
}

pub trait BuildScheduler {
    fn schedule_builds(&mut self) -> Result<(), BuildError>;
    fn get_queue_size(&self) -> usize;
    fn get_active_builds(&self) -> Vec<BuilderID>;
}

#[repr(C)]
pub struct SimpleBuildScheduler {
    pub farm: SimpleBuildFarm,
}

impl SimpleBuildScheduler {
    pub fn new(farm: SimpleBuildFarm) -> Self {
        SimpleBuildScheduler { farm }
    }
}

impl BuildScheduler for SimpleBuildScheduler {
    fn schedule_builds(&mut self) -> Result<(), BuildError> {
        let mut i = 0;
        while i < self.farm.build_queue.len() {
            let (target, arch) = self.farm.build_queue[i];
            if let Some(builder_id) = self.farm.find_idle_builder(arch) {
                if let Some(builder) = self.farm.get_builder(builder_id) {
                    let builder_id = builder.id();
                    for builder_option in &mut self.farm.builders {
                        if let Some(ref mut b) = *builder_option {
                            if b.id() == builder_id {
                                b.start_build(&target)?;
                                self.farm.build_queue.remove(i);
                                break;
                            }
                        }
                    }
                }
            } else {
                i += 1;
            }
        }
        Ok(())
    }

    fn get_queue_size(&self) -> usize { self.farm.build_queue.len() }

    fn get_active_builds(&self) -> Vec<BuilderID> {
        let mut active = Vec::new();
        for builder_option in &self.farm.builders {
            if let Some(ref builder) = *builder_option {
                if builder.state() == BuilderState::Building {
                    active.push(builder.id());
                }
            }
        }
        active
    }
}

pub trait BuildArtifact {
    fn store_artifact(&mut self, builder_id: BuilderID, artifact: &[u8]) -> Result<(), BuildError>;
    fn retrieve_artifact(&self, builder_id: BuilderID) -> Option<&[u8]>;
    fn list_artifacts(&self) -> Vec<BuilderID>;
}

#[repr(C)]
pub struct SimpleBuildArtifact {
    pub artifacts: Vec<(BuilderID, [u8; 512])>,
}

impl SimpleBuildArtifact {
    pub fn new() -> Self {
        SimpleBuildArtifact {
            artifacts: Vec::new(),
        }
    }
}

impl BuildArtifact for SimpleBuildArtifact {
    fn store_artifact(&mut self, builder_id: BuilderID, artifact: &[u8]) -> Result<(), BuildError> {
        let mut artifact_array = [0u8; 512];
        let artifact_len = artifact.len().min(511);
        for i in 0..artifact_len {
            artifact_array[i] = artifact[i];
        }
        self.artifacts.push((builder_id, artifact_array));
        Ok(())
    }

    fn retrieve_artifact(&self, builder_id: BuilderID) -> Option<&[u8]> {
        for &(id, ref artifact) in &self.artifacts {
            if id == builder_id {
                let len = artifact.iter().position(|&b| b == 0).unwrap_or(512);
                return Some(&artifact[..len]);
            }
        }
        None
    }

    fn list_artifacts(&self) -> Vec<BuilderID> {
        let mut ids = Vec::new();
        for &(id, _) in &self.artifacts {
            ids.push(id);
        }
        ids
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
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
