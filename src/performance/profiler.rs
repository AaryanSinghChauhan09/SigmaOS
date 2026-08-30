extern crate alloc;

/// OOP-based Performance Profiler for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 191
/// Implements CPU and memory profiling

use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ProfileID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum ProfileType { CPU = 0, Memory = 1, IO = 2, Network = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProfilerError { Success = 0, NotFound = 1, ProfileRunning = 2 }

pub trait Profile {
    fn id(&self) -> ProfileID;
    fn profile_type(&self) -> ProfileType;
    fn start_time(&self) -> u64;
    fn end_time(&self) -> u64;
    fn set_end_time(&self, end_time: u64);
    fn duration(&self) -> u64;
    fn stop_profile(&mut self);
}

#[repr(C)]
pub struct SimpleProfile {
    pub id: ProfileID,
    pub profile_type: AtomicUsize,
    pub start_time: AtomicUsize,
    pub end_time: AtomicUsize,
}

impl SimpleProfile {
    pub fn new(id: ProfileID, profile_type: ProfileType) -> Self {
        SimpleProfile {
            id,
            profile_type: AtomicUsize::new(profile_type as usize),
            start_time: AtomicUsize::new(1000000),
            end_time: AtomicUsize::new(0),
        }
    }
}


impl Profile for SimpleProfile {
    fn id(&self) -> ProfileID { self.id }
    fn profile_type(&self) -> ProfileType { unsafe { core::mem::transmute(self.profile_type.load(Ordering::SeqCst)) } }
    fn start_time(&self) -> u64 { self.start_time.load(Ordering::SeqCst) as u64 }
    fn end_time(&self) -> u64 { self.end_time.load(Ordering::SeqCst) as u64 }
    fn set_end_time(&self, end_time: u64) { self.end_time.store(end_time as usize, Ordering::SeqCst); }
    fn duration(&self) -> u64 {
        let end = self.end_time();
        let start = self.start_time();
        if end > start { end - start } else { 0 }
    }
    fn stop_profile(&mut self) {
        self.end_time.store(2000000, Ordering::SeqCst);
    }
}

pub trait Profiler {
    fn start_profile(&mut self, profile_type: ProfileType) -> Result<ProfileID, ProfilerError>;
    fn stop_profile(&mut self, id: ProfileID) -> Result<(), ProfilerError>;
    fn get_profile(&self, id: ProfileID) -> Option<&dyn Profile>;
    fn get_cpu_usage(&self) -> f32;
    fn get_memory_usage(&self) -> f32;
}

#[repr(C)]
pub struct SimpleProfiler {
    pub profiles: Vec<Option<Box<dyn Profile>>>,
    pub cpu_usage: AtomicUsize,
    pub memory_usage: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleProfiler {
    pub fn new() -> Self {
        SimpleProfiler {
            profiles: Vec::new(),
            cpu_usage: AtomicUsize::new(0),
            memory_usage: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Profiler for SimpleProfiler {
    fn start_profile(&mut self, profile_type: ProfileType) -> Result<ProfileID, ProfilerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let profile = SimpleProfile::new(id, profile_type);
        self.profiles.push(Some(Box::new(profile)));
        Ok(id)
    }

    fn stop_profile(&mut self, id: ProfileID) -> Result<(), ProfilerError> {
        for profile_option in &mut self.profiles {
            if let Some(ref mut profile) = *profile_option {
                if profile.id() == id {
                    profile.stop_profile();
                    return Ok(());
                }
            }
        }
        Err(ProfilerError::NotFound)
    }

    fn get_profile(&self, id: ProfileID) -> Option<&dyn Profile> {
        for profile_option in &self.profiles {
            if let Some(ref profile) = *profile_option {
                if profile.id() == id { return Some(profile.as_ref()); }
            }
        }
        None
    }

    fn get_cpu_usage(&self) -> f32 { (self.cpu_usage.load(Ordering::SeqCst) as f32) / 100.0 }

    fn get_memory_usage(&self) -> f32 { (self.memory_usage.load(Ordering::SeqCst) as f32) / 100.0 }
}

pub trait CallGraph {
    fn add_node(&mut self, function: &[u8]);
    fn add_edge(&mut self, caller: &[u8], callee: &[u8]);
    fn get_hotspots(&self) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleCallGraph {
    pub nodes: Vec<[u8; 128]>,
    pub edges: Vec<([u8; 128], [u8; 128])>,
}

impl SimpleCallGraph {
    pub fn new() -> Self {
        SimpleCallGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl CallGraph for SimpleCallGraph {
    fn add_node(&mut self, function: &[u8]) {
        let mut func_array = [0u8; 128];
        let func_len = function.len().min(127);
        for i in 0..func_len {
            func_array[i] = function[i];
        }
        self.nodes.push(func_array);
    }

    fn add_edge(&mut self, caller: &[u8], callee: &[u8]) {
        let mut caller_array = [0u8; 128];
        let mut callee_array = [0u8; 128];
        let caller_len = caller.len().min(127);
        let callee_len = callee.len().min(127);
        for i in 0..caller_len { caller_array[i] = caller[i]; }
        for i in 0..callee_len { callee_array[i] = callee[i]; }
        self.edges.push((caller_array, callee_array));
    }

    fn get_hotspots(&self) -> Vec<&[u8]> {
        let mut hotspots = Vec::new();
        for node in &self.nodes {
            let len = node.iter().position(|&b| b == 0).unwrap_or(128);
            hotspots.push(&node[..len]);
        }
        hotspots
    }
}

