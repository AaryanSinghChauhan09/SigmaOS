// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Cline autonomous dual-stage coding agent
// Principles: DEFENSIVE PROGRAMMING, COMMAND-QUERY SEPARATION, FAIL-FAST
use alloc::{string::String, vec::Vec};
#[derive(Debug,Clone)] pub enum ClineAction { ReadFile { path: String }, WriteFile { path: String, content: String }, RunCommand { cmd: String, sandbox: bool }, BrowserOpen { url: String }, ApiCall { endpoint: String, body: String }, SearchFiles { pattern: String }, ListDirectory { path: String } }
#[derive(Debug,Clone,PartialEq)] pub enum ActionSafety { Safe, NeedsReview, Dangerous { reason: String } }
impl ClineAction {
    pub fn classify(&self) -> ActionSafety {
        match self {
            Self::ReadFile { .. } | Self::ListDirectory { .. } | Self::BrowserOpen { .. } => ActionSafety::Safe,
            Self::SearchFiles { .. } => ActionSafety::Safe,
            Self::ApiCall { .. } => ActionSafety::NeedsReview,
            Self::WriteFile { path, .. } => if path.starts_with("/etc") || path.starts_with("/sys") { ActionSafety::Dangerous { reason: String::from("system file modification") } } else { ActionSafety::NeedsReview },
            Self::RunCommand { cmd, .. } => { if cmd.contains("rm -rf") || cmd.contains("dd if=") || cmd.contains(":(){ :|:& };:") { ActionSafety::Dangerous { reason: String::from("destructive command") } } else { ActionSafety::NeedsReview } }
        }
    }
}
#[derive(Debug,Clone)] pub struct DiffHunk { pub start_line: u32, pub end_line: u32, pub removed: Vec<String>, pub added: Vec<String> }
#[derive(Debug,Clone)] pub struct ClineDiff { pub path: String, pub hunks: Vec<DiffHunk> }
#[derive(Debug,Clone)] pub struct ActionPlan { pub objective: String, pub steps: Vec<ClineAction>, pub explanation: String }
pub struct ClineAgent { pub approved_actions: Vec<ActionSafety>, pub executed_count: u32 }
impl ClineAgent {
    pub fn new() -> Self { Self { approved_actions: Vec::new(), executed_count: 0 } }
    pub fn plan(&self, objective: &str) -> ActionPlan {
        let steps = if objective.contains("read") { alloc::vec![ClineAction::ReadFile { path: String::from("src/main.rs") }] }
        else if objective.contains("write") { alloc::vec![ClineAction::WriteFile { path: String::from("output.rs"), content: String::from("// generated") }] }
        else { alloc::vec![ClineAction::RunCommand { cmd: String::from("echo done"), sandbox: true }] };
        ActionPlan { objective: objective.into(), steps, explanation: String::from("Automated plan generated") }
    }
    pub fn classify_action(&self, action: &ClineAction) -> ActionSafety { action.classify() }
    pub fn preview_file_changes(&self, plan: &ActionPlan) -> Vec<ClineDiff> {
        plan.steps.iter().filter_map(|s| if let ClineAction::WriteFile { path, content } = s {
            Some(ClineDiff { path: path.clone(), hunks: alloc::vec![DiffHunk { start_line: 1, end_line: 1, removed: Vec::new(), added: alloc::vec![content.clone()] }] })
        } else { Option::None }).collect()
    }
    pub fn execute_plan(&mut self, plan: &ActionPlan) -> Vec<Result<String,String>> {
        plan.steps.iter().map(|step| { self.executed_count += 1; match self.classify_action(step) { ActionSafety::Dangerous { reason } => Err(reason), _ => Ok(String::from("ok")) } }).collect()
    }
    pub fn approve_and_execute(&mut self, plan: &ActionPlan, approved: bool) -> Result<Vec<String>,&'static str> {
        if !approved { return Err("plan not approved"); }
        let results: Vec<String> = plan.steps.iter().filter_map(|s| match self.classify_action(s) { ActionSafety::Dangerous { .. } => Option::None, _ => Some(String::from("executed")) }).collect();
        self.executed_count += results.len() as u32;
        Ok(results)
    }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_plan_creation() { let agent = ClineAgent::new(); let plan = agent.plan("read the main file"); assert!(!plan.steps.is_empty()); }
    #[test] fn test_safe_classification() { let agent = ClineAgent::new(); let a = ClineAction::ReadFile { path: String::from("src/lib.rs") }; assert_eq!(agent.classify_action(&a), ActionSafety::Safe); }
    #[test] fn test_dangerous_command() { let agent = ClineAgent::new(); let a = ClineAction::RunCommand { cmd: String::from("rm -rf /"), sandbox: false }; assert!(matches!(agent.classify_action(&a), ActionSafety::Dangerous { .. })); }
    #[test] fn test_sys_file_dangerous() { let agent = ClineAgent::new(); let a = ClineAction::WriteFile { path: String::from("/etc/passwd"), content: String::new() }; assert!(matches!(agent.classify_action(&a), ActionSafety::Dangerous { .. })); }
    #[test] fn test_approve_execute() { let mut agent = ClineAgent::new(); let plan = agent.plan("run something"); let res = agent.approve_and_execute(&plan, true).unwrap(); assert!(!res.is_empty()); }
}
