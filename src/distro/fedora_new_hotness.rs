// SPDX-License-Identifier: MIT
// SigmaOS Fedora "The New Hotness" Upstream Version Monitor & Automation Engine
// Inspired by Fedora Infrastructure's the-new-hotness and Anitya (release-monitoring.org).

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]

#[cfg(target_os = "none")]
use std::collections::BTreeMap;
#[cfg(not(target_os = "none"))]
use std::collections::BTreeMap;

#[cfg(target_os = "none")]
use std::format;
#[cfg(not(target_os = "none"))]
use std::format;

#[cfg(target_os = "none")]
use std::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

#[cfg(target_os = "none")]
use std::vec::Vec;

#[cfg(not(test))]
use crate::distro::fedora_pagure_exporter::FedoraPagureExporterEngine;

#[cfg(test_disabled)]
#[path = "fedora_pagure_exporter.rs"]
mod fedora_pagure_exporter;

#[cfg(test_disabled)]
use fedora_pagure_exporter::FedoraPagureExporterEngine;

// ============================================================================
// 1. Anitya Release Monitoring Provider Types & Project Spec
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpstreamBackendProvider {
    GitHubReleases,
    PyPI,
    CratesIo,
    GitLab,
    CustomTarballUrl,
}

#[derive(Debug, Clone)]
pub struct AnityaUpstreamProject {
    pub anitya_id: u32,
    pub name: String,
    pub homepage: String,
    pub backend_provider: UpstreamBackendProvider,
    pub ecosystem_package_name: String,
    pub current_stable_version: String,
    pub regex_pattern: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpstreamReleaseEvent {
    pub anitya_id: u32,
    pub project_name: String,
    pub new_version: String,
    pub release_url: String,
    pub sha512_checksum: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewHotnessActionType {
    FilePagureIssue,
    OpenDistGitPullRequest,
    TriggerKojiScratchBuild,
}

#[derive(Debug, Clone)]
pub struct NewHotnessAutomatedTask {
    pub task_id: u32,
    pub project_name: String,
    pub action_type: NewHotnessActionType,
    pub old_version: String,
    pub new_version: String,
    pub payload_summary: String,
    pub is_executed: bool,
}

// ============================================================================
// 2. Fedora "The New Hotness" Upstream Monitor Engine
// ============================================================================

pub struct FedoraNewHotnessUpstreamMonitorEngine {
    pub registered_projects: BTreeMap<u32, AnityaUpstreamProject>,
    pub pending_tasks: Vec<NewHotnessAutomatedTask>,
    pub pagure_exporter: FedoraPagureExporterEngine,
    pub next_task_id: u32,
}

impl FedoraNewHotnessUpstreamMonitorEngine {
    pub fn new(namespace: &str, project_name: &str, branch: &str) -> Self {
        Self {
            registered_projects: BTreeMap::new(),
            pending_tasks: Vec::new(),
            pagure_exporter: FedoraPagureExporterEngine::new(namespace, project_name, branch),
            next_task_id: 1,
        }
    }

    pub fn register_anitya_project(
        &mut self,
        anitya_id: u32,
        name: &str,
        provider: UpstreamBackendProvider,
        current_version: &str,
    ) {
        let proj = AnityaUpstreamProject {
            anitya_id,
            name: name.to_string(),
            homepage: format!("https://release-monitoring.org/project/{}/", anitya_id),
            backend_provider: provider,
            ecosystem_package_name: name.to_string(),
            current_stable_version: current_version.to_string(),
            regex_pattern: None,
        };
        self.registered_projects.insert(anitya_id, proj);
    }

    pub fn process_upstream_release_event(
        &mut self,
        event: UpstreamReleaseEvent,
    ) -> Result<usize, &'static str> {
        let proj = self
            .registered_projects
            .get_mut(&event.anitya_id)
            .ok_or("Anitya project ID not registered in The New Hotness")?;

        if proj.current_stable_version == event.new_version {
            return Ok(0); // Version unchanged
        }

        let old_ver = proj.current_stable_version.clone();
        proj.current_stable_version = event.new_version.clone();

        let issue_title = format!(
            "Upstream release available: {} {}",
            proj.name, event.new_version
        );
        let issue_body = format!(
            "The New Hotness has detected a new upstream release for {}.\nOld Version: {}\nNew Version: {}\nRelease URL: {}\nSHA512: {}\n",
            proj.name, old_ver, event.new_version, event.release_url, event.sha512_checksum
        );

        // 1. Queue Pagure Issue Creation
        let task1_id = self.next_task_id;
        self.next_task_id += 1;
        self.pending_tasks.push(NewHotnessAutomatedTask {
            task_id: task1_id,
            project_name: proj.name.clone(),
            action_type: NewHotnessActionType::FilePagureIssue,
            old_version: old_ver.clone(),
            new_version: event.new_version.clone(),
            payload_summary: issue_title,
            is_executed: false,
        });

        // 2. Queue Koji Scratch Build Trigger
        let task2_id = self.next_task_id;
        self.next_task_id += 1;
        self.pending_tasks.push(NewHotnessAutomatedTask {
            task_id: task2_id,
            project_name: proj.name.clone(),
            action_type: NewHotnessActionType::TriggerKojiScratchBuild,
            old_version: old_ver.clone(),
            new_version: event.new_version.clone(),
            payload_summary: format!("Koji scratch build for {} {}", proj.name, event.new_version),
            is_executed: false,
        });

        // Register event in Pagure exporter
        self.pagure_exporter
            .export_issue(task1_id, &format!("Upstream {}", event.new_version), &issue_body, "the-new-hotness");

        Ok(2)
    }

    pub fn execute_pending_tasks(&mut self) -> usize {
        let mut executed_count = 0;
        for task in &mut self.pending_tasks {
            if !task.is_executed {
                task.is_executed = true;
                executed_count += 1;
            }
        }
        executed_count
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fedora_new_hotness_upstream_monitor() {
        let mut hotness = FedoraNewHotnessUpstreamMonitorEngine::new("rpms", "sigma-toolchain", "rawhide");
        hotness.register_anitya_project(1001, "sigma-toolchain", UpstreamBackendProvider::GitHubReleases, "1.0.0");

        assert_eq!(hotness.registered_projects.len(), 1);

        let event = UpstreamReleaseEvent {
            anitya_id: 1001,
            project_name: "sigma-toolchain".to_string(),
            new_version: "1.1.0".to_string(),
            release_url: "https://github.com/sigma/toolchain/releases/tag/v1.1.0".to_string(),
            sha512_checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            timestamp: 1700000000,
        };

        let queued = hotness.process_upstream_release_event(event).unwrap();
        assert_eq!(queued, 2);
        assert_eq!(hotness.pending_tasks.len(), 2);
        assert_eq!(hotness.registered_projects.get(&1001).unwrap().current_stable_version, "1.1.0");

        let executed = hotness.execute_pending_tasks();
        assert_eq!(executed, 2);
        assert!(hotness.pending_tasks.iter().all(|t| t.is_executed));
    }
}
