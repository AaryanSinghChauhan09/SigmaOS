//! PipeWire Audio Graph (Stub Implementation for Phase 1)

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

/// Audio graph state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
    Stopped,
    Running,
    Suspended,
}

/// Audio graph node
#[derive(Debug, Clone)]
pub struct AudioGraph {
    pub nodes: Vec<u32>,
    pub state: GraphState,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            state: GraphState::Stopped,
        }
    }
}

/// Audio link between nodes
#[derive(Debug, Clone, Copy)]
pub struct AudioLink {
    pub source_node: u32,
    pub source_port: u32,
    pub dest_node: u32,
    pub dest_port: u32,
}

/// Audio node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Source,
    Sink,
    Filter,
}

/// Audio node in the graph
#[derive(Debug, Clone)]
pub struct AudioNode {
    pub id: u32,
    pub node_type: NodeType,
    pub name: alloc::vec::Vec<u8>,
}
