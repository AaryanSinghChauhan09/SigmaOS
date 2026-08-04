//! Sovereign OliveTin web dashboard simulation engine for SigmaOS
//! Provides a safe, incredibly clean, lightweight web control panel
//! allowing administrators to expose pre-defined shell commands with parameter variables
//! in a sandboxed, ease-of-use environment under #![no_std].

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use crate::klib::{Vec, HashMap};

#[derive(Debug, Clone)]
pub struct OliveTinAction {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub command_template: String,
    pub parameter_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OliveTinExecutionLog {
    pub action_id: usize,
    pub timestamp_ms: u64,
    pub arguments: Vec<String>,
    pub exit_code: i32,
    pub output: String,
}

pub struct SovereignOliveTinEngine {
    pub actions: Vec<OliveTinAction>,
    pub execution_logs: Vec<OliveTinExecutionLog>,
}

impl SovereignOliveTinEngine {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            execution_logs: Vec::new(),
        }
    }

    pub fn register_action(
        &mut self,
        id: usize,
        title: &str,
        description: &str,
        command_template: &str,
        parameter_names: &[&str],
    ) {
        let params = parameter_names.iter().map(|&p| p.to_string()).collect();
        self.actions.push(OliveTinAction {
            id,
            title: title.to_string(),
            description: description.to_string(),
            command_template: command_template.to_string(),
            parameter_names: params,
        });
    }

    /// Evaluates action parameter interpolation and executes the pre-defined command
    pub fn execute_action(&mut self, action_id: usize, args: &[&str]) -> Result<String, &'static str> {
        let mut action_opt = None;
        for i in 0..self.actions.len() {
            if self.actions[i].id == action_id {
                action_opt = Some(self.actions[i].clone());
                break;
            }
        }

        let action = action_opt.ok_or("Action not found")?;

        if args.len() < action.parameter_names.len() {
            return Err("Missing required argument parameters");
        }

        // Interpolate parameters: replace `{{param_name}}` with supplied argument
        let mut executed_command = action.command_template.clone();
        for i in 0..action.parameter_names.len() {
            let placeholder = alloc::format!("{{{{{}}}}}", action.parameter_names[i]);
            executed_command = executed_command.replace(&placeholder, args[i]);
        }

        // Simulate safe sandboxed execution outputs
        let output = if executed_command.contains("reboot") {
            "System transitioning to runlevel 6 (rebooting)...".to_string()
        } else if executed_command.contains("backup") {
            alloc::format!("Successfully backed up target directory: {}", args[0])
        } else if executed_command.contains("ping") {
            alloc::format!("Sovereign Ping to {} succeeded. 0% packet loss.", args[0])
        } else {
            alloc::format!("Pre-defined action executed command: {}", executed_command)
        };

        let log = OliveTinExecutionLog {
            action_id,
            timestamp_ms: 1000,
            arguments: args.iter().map(|&a| a.to_string()).collect(),
            exit_code: 0,
            output: output.clone(),
        };
        self.execution_logs.push(log);

        Ok(output)
    }

    /// Generates a lightweight, highly-polished, responsive HTML dashboard representing the OliveTin UI
    pub fn render_dashboard_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html><head>");
        html.push_str("<title>SigmaOS OliveTin Control Panel</title>");
        html.push_str("<style>body{font-family:sans-serif;background:#0d1117;color:#c9d1d9;padding:2rem;}");
        html.push_str(".container{max-width:800px;margin:0 auto;}");
        html.push_str(".card{background:#161b22;border:1px solid #30363d;padding:1.5rem;margin-bottom:1rem;border-radius:6px;}");
        html.push_str("button{background:#238636;color:white;border:none;padding:0.5rem 1rem;border-radius:4px;cursor:pointer;}");
        html.push_str("</style></head><body><div class=\"container\">");
        html.push_str("<h1>⚡ OliveTin Action Dashboard</h1><p>Expose pre-defined safe commands as simple web buttons.</p>");

        for i in 0..self.actions.len() {
            let act = &self.actions[i];
            html.push_str("<div class=\"card\">");
            html.push_str("<h3>");
            html.push_str(&act.title);
            html.push_str("</h3><p>");
            html.push_str(&act.description);
            html.push_str("</p><p><code>");
            html.push_str(&act.command_template);
            html.push_str("</code></p>");
            html.push_str("<button>Run Action</button>");
            html.push_str("</div>");
        }

        html.push_str("</div></body></html>");
        html
    }
}

impl Default for SovereignOliveTinEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_olivetin_engine() {
        let mut ot = SovereignOliveTinEngine::new();
        ot.register_action(
            100,
            "Backup Folder",
            "Triggers safe system backups to partition",
            "sigma-backup --dir {{dir_path}} --encrypt",
            &["dir_path"],
        );

        assert_eq!(ot.actions.len(), 1);
        assert_eq!(ot.actions[0].title, "Backup Folder");

        // Execute action with missing args -> fail
        assert!(ot.execute_action(100, &[]).is_err());

        // Execute successfully with parameter interpolation
        let out = ot.execute_action(100, &["/home/jules"]).unwrap();
        assert!(out.contains("/home/jules"));
        assert_eq!(ot.execution_logs.len(), 1);
        assert_eq!(ot.execution_logs[0].action_id, 100);

        // Render dashboard verification
        let html = ot.render_dashboard_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Backup Folder"));
    }
}
