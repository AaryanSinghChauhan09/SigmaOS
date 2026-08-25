// SigmaOS Kernel Persona Containers & Syscall Graph
// Encapsulates lightweight legacy kernel version mimicry and graph-based dynamic syscall mapping

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersonaVersion {
    Linux26,
    Linux3x,
    Linux4x,
    Linux5x,
}

#[derive(Debug, Clone)]
pub struct KernelPersonaContainer {
    pub version: PersonaVersion,
    pub active_processes: usize,
    pub is_isolated: bool,
}

impl KernelPersonaContainer {
    pub fn new(version: PersonaVersion) -> Self {
        KernelPersonaContainer {
            version,
            active_processes: 0,
            is_isolated: true,
        }
    }

    pub fn enter_persona(&mut self) {
        self.active_processes += 1;
    }

    pub fn exit_persona(&mut self) {
        if self.active_processes > 0 {
            self.active_processes -= 1;
        }
    }
}

// =========================================================================
// SYSCALL EVOLUTION GRAPH IMPLEMENTATION
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyscallCategory {
    File,
    Network,
    Process,
}

pub struct SyscallNode {
    pub num: u32,
    pub category: SyscallCategory,
    pub name: String,
    pub deprecated: bool,
}

pub struct SyscallGraph {
    pub nodes: BTreeMap<u32, SyscallNode>,
    pub translation_edges: BTreeMap<u32, u32>, // maps old syscall number -> modern syscall number
}

impl SyscallGraph {
    pub fn new() -> Self {
        let mut graph = SyscallGraph {
            nodes: BTreeMap::new(),
            translation_edges: BTreeMap::new(),
        };
        // Seed default legacy-to-modern syscall mappings
        graph.add_syscall(
            1,
            SyscallCategory::Process,
            "sys_exit_legacy".to_string(),
            true,
        );
        graph.add_syscall(
            60,
            SyscallCategory::Process,
            "sys_exit_modern".to_string(),
            false,
        );
        graph.add_translation(1, 60); // Translate old exit (1) to modern exit (60)

        graph
    }

    pub fn add_syscall(
        &mut self,
        num: u32,
        category: SyscallCategory,
        name: String,
        deprecated: bool,
    ) {
        self.nodes.insert(
            num,
            SyscallNode {
                num,
                category,
                name,
                deprecated,
            },
        );
    }

    pub fn add_translation(&mut self, from: u32, to: u32) {
        self.translation_edges.insert(from, to);
    }

    pub fn translate_syscall(&self, sys_num: u32) -> u32 {
        if let Some(&target) = self.translation_edges.get(&sys_num) {
            target
        } else {
            sys_num // Fallback to same syscall number if no translation is needed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_persona_container() {
        let mut container = KernelPersonaContainer::new(PersonaVersion::Linux26);
        assert_eq!(container.active_processes, 0);
        container.enter_persona();
        assert_eq!(container.active_processes, 1);
        container.exit_persona();
        assert_eq!(container.active_processes, 0);
    }

    #[test]
    fn test_syscall_graph_routing() {
        let graph = SyscallGraph::new();
        let routed = graph.translate_syscall(1); // old exit (1) should route to 60
        assert_eq!(routed, 60);

        let unrouted = graph.translate_syscall(3); // old read (3) has no translation
        assert_eq!(unrouted, 3);
    }
}
