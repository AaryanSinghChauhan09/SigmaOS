// SPDX-License-Identifier: MIT
// SigmaOS Fedora Pagure Dist-Git & Git-Forge Exporter Engine
// Inspired by Fedora Infrastructure Pagure, Dist-Git, Koji Buildsystem, and Fedora Lookaside Cache.

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::collections::BTreeMap;
#[cfg(not(target_os = "none"))]
use std::collections::BTreeMap;

#[cfg(target_os = "none")]
use alloc::format;
#[cfg(not(target_os = "none"))]
use std::format;

#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

#[cfg(target_os = "none")]
use alloc::vec::Vec;

// ============================================================================
// 1. Fedora Dist-Git Spec & Source Lookaside Cache Exporter
// ============================================================================

#[derive(Debug, Clone)]
pub struct DistGitSpecMetadata {
    pub package_name: String,
    pub version: String,
    pub release: String,
    pub summary: String,
    pub license: String,
    pub url: String,
    pub sources: Vec<String>,
    pub patches: Vec<String>,
    pub build_requires: Vec<String>,
    pub requires: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LookasideSourceEntry {
    pub filename: String,
    pub sha512_checksum: String,
    pub file_size_bytes: u64,
}

pub struct FedoraDistGitExporter {
    pub namespace: String, // e.g. "rpms", "modules", "flatpaks", "containers"
    pub repo_name: String,
    pub branch: String, // e.g. "rawhide", "f40", "f39"
    pub spec_metadata: Option<DistGitSpecMetadata>,
    pub lookaside_sources: Vec<LookasideSourceEntry>,
}

impl FedoraDistGitExporter {
    pub fn new(namespace: &str, repo_name: &str, branch: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            repo_name: repo_name.to_string(),
            branch: branch.to_string(),
            spec_metadata: None,
            lookaside_sources: Vec::new(),
        }
    }

    pub fn set_spec_metadata(&mut self, spec: DistGitSpecMetadata) {
        self.spec_metadata = Some(spec);
    }

    pub fn add_lookaside_source(&mut self, filename: &str, sha512: &str, size_bytes: u64) {
        self.lookaside_sources.push(LookasideSourceEntry {
            filename: filename.to_string(),
            sha512_checksum: sha512.to_string(),
            file_size_bytes: size_bytes,
        });
    }

    pub fn render_sources_file(&self) -> String {
        let mut rendered = String::new();
        for entry in &self.lookaside_sources {
            rendered.push_str(&format!(
                "SHA512 ({}) = {}\n",
                entry.filename, entry.sha512_checksum
            ));
        }
        rendered
    }

    pub fn render_spec_file(&self) -> Result<String, &'static str> {
        let spec = self
            .spec_metadata
            .as_ref()
            .ok_or("Dist-Git Spec metadata missing")?;

        let mut out = String::new();
        out.push_str(&format!("Name:           {}\n", spec.package_name));
        out.push_str(&format!("Version:        {}\n", spec.version));
        out.push_str(&format!("Release:        {}%{{?dist}}\n", spec.release));
        out.push_str(&format!("Summary:        {}\n", spec.summary));
        out.push_str(&format!("License:        {}\n", spec.license));
        out.push_str(&format!("URL:            {}\n", spec.url));

        for (idx, src) in spec.sources.iter().enumerate() {
            out.push_str(&format!("Source{}:        {}\n", idx, src));
        }

        for (idx, patch) in spec.patches.iter().enumerate() {
            out.push_str(&format!("Patch{}:         {}\n", idx, patch));
        }

        for br in &spec.build_requires {
            out.push_str(&format!("BuildRequires:  {}\n", br));
        }

        for req in &spec.requires {
            out.push_str(&format!("Requires:       {}\n", req));
        }

        out.push_str("\n%description\n");
        out.push_str(&format!(
            "Sovereign Fedora Dist-Git package build specification for {}.\n\n",
            spec.package_name
        ));

        out.push_str("%prep\n%autosetup\n\n");
        out.push_str("%build\n%cargo_build\n\n");
        out.push_str("%install\n%cargo_install\n\n");
        out.push_str("%files\n%license LICENSE\n%doc README.md\n");

        Ok(out)
    }
}

// ============================================================================
// 2. Fedora Pagure Git-Forge Metadata & Webhook API Payload Exporter
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagurePullRequestStatus {
    Open,
    Merged,
    Closed,
}

#[derive(Debug, Clone)]
pub struct PagureIssueExport {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub status: String,
    pub tags: Vec<String>,
    pub author: String,
    pub created_timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct PagurePullRequestExport {
    pub id: u32,
    pub title: String,
    pub branch_from: String,
    pub branch_to: String,
    pub status: PagurePullRequestStatus,
    pub initial_comment: String,
    pub author: String,
}

pub struct FedoraPagureExporterEngine {
    pub project_name: String,
    pub project_namespace: String,
    pub dist_git_exporter: FedoraDistGitExporter,
    pub issues: BTreeMap<u32, PagureIssueExport>,
    pub pull_requests: BTreeMap<u32, PagurePullRequestExport>,
    pub koji_target_tag: String,
}

impl FedoraPagureExporterEngine {
    pub fn new(namespace: &str, project_name: &str, branch: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            project_namespace: namespace.to_string(),
            dist_git_exporter: FedoraDistGitExporter::new(namespace, project_name, branch),
            issues: BTreeMap::new(),
            pull_requests: BTreeMap::new(),
            koji_target_tag: format!("{}-candidate", branch),
        }
    }

    pub fn export_issue(&mut self, id: u32, title: &str, content: &str, author: &str) {
        let issue = PagureIssueExport {
            id,
            title: title.to_string(),
            content: content.to_string(),
            status: "Open".to_string(),
            tags: vec!["fedora-packaging".to_string(), "sigma-os".to_string()],
            author: author.to_string(),
            created_timestamp: 1700000000,
        };
        self.issues.insert(id, issue);
    }

    pub fn export_pull_request(
        &mut self,
        id: u32,
        title: &str,
        branch_from: &str,
        branch_to: &str,
        author: &str,
    ) {
        let pr = PagurePullRequestExport {
            id,
            title: title.to_string(),
            branch_from: branch_from.to_string(),
            branch_to: branch_to.to_string(),
            status: PagurePullRequestStatus::Open,
            initial_comment: "Automated Pagure Dist-Git sync pull request".to_string(),
            author: author.to_string(),
        };
        self.pull_requests.insert(id, pr);
    }

    pub fn generate_koji_build_trigger_payload(&self) -> String {
        format!(
            "{{\"event\":\"koji_build_request\",\"package\":\"{}\",\"namespace\":\"{}\",\"tag\":\"{}\",\"git_url\":\"https://src.fedoraproject.org/{}/{}.git#branch={}\"}}",
            self.project_name,
            self.project_namespace,
            self.koji_target_tag,
            self.project_namespace,
            self.project_name,
            self.dist_git_exporter.branch
        )
    }

    pub fn generate_pagure_webhook_payload(&self, event_type: &str) -> String {
        format!(
            "{{\"project\":{{\"name\":\"{}\",\"namespace\":\"{}\"}},\"event\":\"{}\",\"issues_count\":{},\"prs_count\":{}}}",
            self.project_name,
            self.project_namespace,
            event_type,
            self.issues.len(),
            self.pull_requests.len()
        )
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fedora_dist_git_exporter() {
        let mut dist_git = FedoraDistGitExporter::new("rpms", "sigma-kernel", "rawhide");
        dist_git.add_lookaside_source(
            "sigma-kernel-1.0.0.tar.gz",
            "a1b2c3d4e5f67890123456789012345678901234567890123456789012345678",
            10485760,
        );

        let sources_file = dist_git.render_sources_file();
        assert!(sources_file.contains("SHA512 (sigma-kernel-1.0.0.tar.gz) = a1b2c3d4e5f6"));

        let spec = DistGitSpecMetadata {
            package_name: "sigma-kernel".to_string(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            summary: "Sovereign Linux-Parity Operating System Kernel".to_string(),
            license: "MIT".to_string(),
            url: "https://sigmaos.org".to_string(),
            sources: vec!["sigma-kernel-1.0.0.tar.gz".to_string()],
            patches: vec!["0001-fentry-livepatch.patch".to_string()],
            build_requires: vec!["rustc".to_string(), "cargo".to_string()],
            requires: vec!["systemd".to_string()],
        };

        dist_git.set_spec_metadata(spec);
        let spec_rendered = dist_git.render_spec_file().unwrap();
        assert!(spec_rendered.contains("Name:           sigma-kernel"));
        assert!(spec_rendered.contains("BuildRequires:  rustc"));
    }

    #[test]
    fn test_fedora_pagure_exporter_engine() {
        let mut pagure = FedoraPagureExporterEngine::new("rpms", "sigma-shell", "f40");
        pagure.export_issue(
            1,
            "Add Zsh autosuggestions feature",
            "Missing Zsh autosuggestions in shell parity",
            "jules",
        );
        pagure.export_pull_request(
            101,
            "Implement Fish/Zsh auto-suggestions",
            "feat/zsh-suggestions",
            "f40",
            "jules",
        );

        let koji_payload = pagure.generate_koji_build_trigger_payload();
        assert!(koji_payload.contains("\"package\":\"sigma-shell\""));
        assert!(koji_payload.contains("\"tag\":\"f40-candidate\""));

        let webhook_payload = pagure.generate_pagure_webhook_payload("issue.new");
        assert!(webhook_payload.contains("\"issues_count\":1"));
        assert!(webhook_payload.contains("\"prs_count\":1"));
    }
}
