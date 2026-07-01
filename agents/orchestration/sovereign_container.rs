// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Container (Rust, no_std)
//! Replaces: agents/orchestration/SovereignContainer.cpp
//!           agents/orchestration/SovereignContainerRuntime.cpp
//! =========================================================================

const MAX_CONTAINERS: usize = 32;
const MAX_NAME_LEN: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Exited,
}

#[derive(Copy, Clone)]
pub struct ContainerConfig {
    pub memory_limit_mb: u32,
    pub cpu_weight: u8,
    pub read_only_root: bool,
}

impl ContainerConfig {
    pub const fn default() -> Self {
        Self { memory_limit_mb: 256, cpu_weight: 10, read_only_root: false }
    }
}

pub struct SovereignContainer {
    pub id: u32,
    pub state: ContainerState,
    pub config: ContainerConfig,
}

impl SovereignContainer {
    pub const fn new(id: u32, config: ContainerConfig) -> Self {
        Self { id, state: ContainerState::Created, config }
    }

    pub fn start(&mut self) {
        self.state = ContainerState::Running;
    }

    pub fn pause(&mut self) {
        if self.state == ContainerState::Running {
            self.state = ContainerState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = ContainerState::Exited;
    }

    pub fn class_name(&self) -> &'static str {
        "SovereignContainer"
    }
}

/// Container Runtime — manages lifecycle of all sovereign containers
pub struct SovereignContainerRuntime {
    containers: [Option<SovereignContainer>; MAX_CONTAINERS],
    count: usize,
    next_id: u32,
}

impl SovereignContainerRuntime {
    pub const fn new() -> Self {
        Self {
            containers: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ],
            count: 0,
            next_id: 1,
        }
    }

    pub fn create(&mut self, config: ContainerConfig) -> Option<u32> {
        if self.count >= MAX_CONTAINERS {
            return None;
        }
        let id = self.next_id;
        self.containers[self.count] = Some(SovereignContainer::new(id, config));
        self.count += 1;
        self.next_id += 1;
        Some(id)
    }

    pub fn start(&mut self, id: u32) -> bool {
        for i in 0..self.count {
            if let Some(ref mut c) = self.containers[i] {
                if c.id == id {
                    c.start();
                    return true;
                }
            }
        }
        false
    }

    pub fn stop(&mut self, id: u32) -> bool {
        for i in 0..self.count {
            if let Some(ref mut c) = self.containers[i] {
                if c.id == id {
                    c.stop();
                    return true;
                }
            }
        }
        false
    }

    pub fn running_count(&self) -> usize {
        let mut n = 0;
        for i in 0..self.count {
            if let Some(ref c) = self.containers[i] {
                if c.state == ContainerState::Running {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn class_name(&self) -> &'static str {
        "SovereignContainerRuntime"
    }
}
