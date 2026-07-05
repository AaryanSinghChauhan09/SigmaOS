/// SigmaOS: usr/apps/sigma_logic.rs
/// Visual automation node engine (OS-level Zapier logic).
/// Evaluates node graphs triggered by OS events.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;

pub const MAX_NODES: SigmaUsize = 64;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    TriggerFileCreated,
    TriggerTimerExpired,
    ActionRunAiPrompt,
    ActionSendNotification,
    ActionMoveFile,
}

#[derive(Copy, Clone)]
pub struct LogicNode {
    pub id: SigmaU32,
    pub node_type: NodeType,
    pub next_node_id: Option<SigmaU32>, // Singly-linked list of actions
    pub data_param: [u8; 32],           // E.g., File path or AI prompt
}

impl LogicNode {
    pub const fn empty() -> Self {
        LogicNode {
            id: 0,
            node_type: NodeType::TriggerTimerExpired,
            next_node_id: None,
            data_param: [0; 32],
        }
    }
}

pub struct LogicGraph {
    pub nodes: [LogicNode; MAX_NODES],
    pub count: SigmaUsize,
}

static mut AUTOMATION_GRAPH: LogicGraph = LogicGraph {
    nodes: [LogicNode::empty(); MAX_NODES],
    count: 0,
};

extern "C" {
    fn ai_submit_task(caller: SigmaU32, prio: u8, prompt: *const u8, len: SigmaUsize) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn logic_add_node(id: SigmaU32, ntype: u8, next: SigmaU32) -> SigmaI32 {
    if AUTOMATION_GRAPH.count >= MAX_NODES { return -12; } // ENOMEM
    
    let t = match ntype {
        0 => NodeType::TriggerFileCreated,
        1 => NodeType::TriggerTimerExpired,
        2 => NodeType::ActionRunAiPrompt,
        3 => NodeType::ActionSendNotification,
        _ => NodeType::ActionMoveFile,
    };
    
    let next_opt = if next == 0 { None } else { Some(next) };
    
    let idx = AUTOMATION_GRAPH.count;
    AUTOMATION_GRAPH.nodes[idx].id = id;
    AUTOMATION_GRAPH.nodes[idx].node_type = t;
    AUTOMATION_GRAPH.nodes[idx].next_node_id = next_opt;
    
    AUTOMATION_GRAPH.count += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn logic_trigger_event(event_type: u8) {
    let t = match event_type {
        0 => NodeType::TriggerFileCreated,
        _ => NodeType::TriggerTimerExpired,
    };
    
    // Find all triggers matching this event
    for i in 0..AUTOMATION_GRAPH.count {
        if AUTOMATION_GRAPH.nodes[i].node_type == t {
            // Execute the chain
            let mut curr = AUTOMATION_GRAPH.nodes[i].next_node_id;
            while let Some(next_id) = curr {
                // Find node
                for j in 0..AUTOMATION_GRAPH.count {
                    if AUTOMATION_GRAPH.nodes[j].id == next_id {
                        execute_node(&AUTOMATION_GRAPH.nodes[j]);
                        curr = AUTOMATION_GRAPH.nodes[j].next_node_id;
                        break;
                    }
                }
            }
        }
    }
}

unsafe fn execute_node(node: &LogicNode) {
    match node.node_type {
        NodeType::ActionRunAiPrompt => {
            // Submit automation prompt to AI backend
            ai_submit_task(0, 0, node.data_param.as_ptr(), 32);
        },
        NodeType::ActionSendNotification => {
            // Call into UI dash to show notification
        },
        _ => {}
    }
}