//! Automation Shard — smart triggers, self-healing builds, auto-sync daemon
use std::time::{Duration, Instant};
use std::process::Command;

pub struct AutomationDaemon {
    pub interval_secs: u64,
    pub repo_root:     String,
    pub self_heal:     bool,
}

impl AutomationDaemon {
    pub fn new(root: impl Into<String>, interval: u64, heal: bool) -> Self {
        Self { interval_secs: interval, repo_root: root.into(), self_heal: heal }
    }

    pub fn sync_cycle(&self) -> bool {
        let out = Command::new("git")
            .args(["pull", "--rebase"])
            .current_dir(&self.repo_root)
            .output();
        match out {
            Ok(o) if o.status.success() => true,
            _ => false,
        }
    }

    pub fn build_affected(&self, changed_paths: &[&str]) -> Vec<String> {
        let mut targets = Vec::new();
        for path in changed_paths {
            if path.contains("kernel/") || path.contains("shards/") { targets.push("bin".into()); }
            if path.contains("web_ui/") { targets.push("web".into()); }
            if path.contains("tools/")  { targets.push("tools".into()); }
        }
        targets.sort(); targets.dedup(); targets
    }

    pub fn self_heal_build(&self, target: &str) -> bool {
        if !self.self_heal { return false; }
        eprintln!("[HEAL] Cleaning and retrying: {}", target);
        Command::new("make").arg("clean").current_dir(&self.repo_root).status().ok();
        Command::new("make").arg(target).current_dir(&self.repo_root)
            .status().map(|s| s.success()).unwrap_or(false)
    }
}
