// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// agents/sigma_workflow_engine.rs — Automation Workflow Engine
// Implements: A local execution engine for node-based automation workflows
// (similar to n8n), triggering on system events, file changes, or schedules.
//
// Allows visual workflows (defined in JSON/YAML) to execute shell commands,
// interact with the AI agent, and modify system state.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

// ── Workflow Data Structures ───────────────────────────────────────────────

#[derive(Clone, PartialEq, Eq)]
pub enum NodeType {
    TriggerCron(String),       // e.g., "0 * * * *"
    TriggerFile(String),       // e.g., "/var/log/syslog"
    ActionShell(String),       // e.g., "systemctl restart nginx"
    ActionAiAnalyze,           // Analyze input via AI agent
    ConditionContains(String), // Checks if input contains string
    OutputLog(String),         // Write to specific log file
}

#[derive(Clone)]
pub struct WorkflowNode {
    pub id: u32,
    pub node_type: NodeType,
    pub next_nodes: Vec<u32>, // IDs of nodes to execute next
}

#[derive(Clone)]
pub struct Workflow {
    pub id: u32,
    pub name: String,
    pub nodes: Vec<WorkflowNode>,
    pub start_nodes: Vec<u32>,
    pub active: bool,
}

// ── Engine State ───────────────────────────────────────────────────────────

pub struct WorkflowEngine {
    pub initialized: bool,
    workflows: Vec<Workflow>,
    execution_queue: Vec<(u32, u32, String)>, // (Workflow ID, Node ID, Input Data)
}

static mut ENGINE: WorkflowEngine = WorkflowEngine {
    initialized: false,
    workflows: Vec::new(),
    execution_queue: Vec::new(),
};

static ENGINE_READY: AtomicBool = AtomicBool::new(false);

impl WorkflowEngine {
    pub fn init(&mut self) {
        // STUB: Load existing workflows from /etc/sigma-workflow/
        self.initialized = true;
        ENGINE_READY.store(true, Ordering::Release);
    }

    pub fn register_workflow(&mut self, workflow: Workflow) {
        self.workflows.push(workflow);
    }

    /// Triggers a workflow by ID with the given initial input data.
    pub fn trigger_workflow(&mut self, workflow_id: u32, input_data: &str) {
        if let Some(wf) = self.workflows.iter().find(|w| w.id == workflow_id) {
            if !wf.active { return; }
            for start_node_id in &wf.start_nodes {
                self.execution_queue.push((workflow_id, *start_node_id, input_data.to_string()));
            }
        }
    }

    /// Executes one step in the queue. In production, this would run in a worker thread.
    pub fn step(&mut self) -> bool {
        if self.execution_queue.is_empty() {
            return false;
        }

        let (wf_id, node_id, input) = self.execution_queue.remove(0);
        
        let (node, next_nodes) = {
            let wf = match self.workflows.iter().find(|w| w.id == wf_id) {
                Some(w) => w,
                None => return true,
            };
            
            let n = match wf.nodes.iter().find(|n| n.id == node_id) {
                Some(n) => n,
                None => return true,
            };
            
            (n.clone(), n.next_nodes.clone())
        };

        let mut output = input.clone();
        let mut proceed = true;

        match &node.node_type {
            NodeType::TriggerCron(_) | NodeType::TriggerFile(_) => {
                // Triggers just pass data through in this execution model
            }
            NodeType::ActionShell(cmd) => {
                // STUB: Execute shell command
                // In production, use standard library `Command::new` or similar
                // Here we just simulate adding to output
                output = format!("{}\n[Shell executed: {}]", input, cmd);
            }
            NodeType::ActionAiAnalyze => {
                // Bridge to AI agent
                if crate::agents::sigma_ai_agent::ai_agent_is_ready() {
                    if let Some(analysis) = crate::agents::sigma_ai_agent::ai_agent_analyze_error(&input) {
                        output = format!("{}\n[AI Analysis: {}]", input, analysis);
                    }
                } else {
                    output = format!("{}\n[AI Agent not ready]", input);
                }
            }
            NodeType::ConditionContains(substring) => {
                if !input.contains(substring) {
                    proceed = false; // Stop execution on this branch
                }
            }
            NodeType::OutputLog(file) => {
                // STUB: Write to file using VFS
                // For now, simulate logging
                output = format!("{}\n[Logged to {}]", input, file);
            }
        }

        if proceed {
            for next_id in next_nodes {
                self.execution_queue.push((wf_id, next_id, output.clone()));
            }
        }

        true // Did work
    }

    /// Process the entire queue until empty.
    pub fn run_queue(&mut self) {
        while self.step() {}
    }
}

// ── Public API ─────────────────────────────────────────────────────────────

pub fn workflow_engine_init() {
    unsafe { ENGINE.init(); }
}

pub fn workflow_register(wf: Workflow) {
    unsafe { ENGINE.register_workflow(wf); }
}

pub fn workflow_trigger(wf_id: u32, input: &str) {
    unsafe { ENGINE.trigger_workflow(wf_id, input); }
}

pub fn workflow_run_all() {
    unsafe { ENGINE.run_queue(); }
}
