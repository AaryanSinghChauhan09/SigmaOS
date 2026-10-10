// SPDX-License-Identifier: MIT
// SigmaOS — Omarchy Cloud Agent Environment Engine
// Inspired by Omarchy branch: cloud-agent-environment
// Zero external dependencies, Safe Rust

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

#[cfg(any(feature = "standalone_test", test))]
use std::{format, string::String, vec::Vec};
#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{format, string::String, vec::Vec};

/// Cloud provider type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
    Hetzner,
    Fly,
    Cloudflare,
    SigmaCloud,
}

impl CloudProvider {
    pub fn name(&self) -> &'static str {
        match self {
            CloudProvider::Aws => "AWS",
            CloudProvider::Gcp => "GCP",
            CloudProvider::Azure => "Azure",
            CloudProvider::Hetzner => "Hetzner",
            CloudProvider::Fly => "Fly.io",
            CloudProvider::Cloudflare => "Cloudflare",
            CloudProvider::SigmaCloud => "SigmaCloud",
        }
    }
}

/// An AI agent environment variable entry
#[derive(Debug, Clone)]
pub struct AgentEnvVar {
    pub key: String,
    pub value: String,
    pub is_secret: bool,
}

/// A cloud agent deployment target
#[derive(Debug, Clone)]
pub struct CloudAgentTarget {
    pub name: String,
    pub provider: CloudProvider,
    pub region: String,
    pub env_vars: Vec<AgentEnvVar>,
    pub cpu_limit: u32, // millicores
    pub memory_limit_mb: u32,
}

/// SigmaOS Cloud Agent Environment Engine
/// Surpasses Omarchy's cloud-agent-environment branch
pub struct OmarchyCloudAgentEnvironment {
    targets: Vec<CloudAgentTarget>,
    global_vars: Vec<AgentEnvVar>,
}

impl OmarchyCloudAgentEnvironment {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            global_vars: Vec::new(),
        }
    }

    /// Register a global environment variable shared across all targets
    pub fn set_global_var(&mut self, key: &str, value: &str, secret: bool) {
        self.global_vars.push(AgentEnvVar {
            key: String::from(key),
            value: String::from(value),
            is_secret: secret,
        });
    }

    /// Add a cloud deployment target
    pub fn add_target(&mut self, name: &str, provider: CloudProvider, region: &str) {
        self.targets.push(CloudAgentTarget {
            name: String::from(name),
            provider,
            region: String::from(region),
            env_vars: Vec::new(),
            cpu_limit: 1000, // 1 vCPU default
            memory_limit_mb: 512,
        });
    }

    /// Set resource limits for a target
    pub fn set_limits(&mut self, name: &str, cpu_mc: u32, mem_mb: u32) {
        for t in &mut self.targets {
            if t.name == name {
                t.cpu_limit = cpu_mc;
                t.memory_limit_mb = mem_mb;
                break;
            }
        }
    }

    /// Add an env var to a specific target
    pub fn set_target_var(&mut self, target: &str, key: &str, value: &str, secret: bool) {
        for t in &mut self.targets {
            if t.name == target {
                t.env_vars.push(AgentEnvVar {
                    key: String::from(key),
                    value: String::from(value),
                    is_secret: secret,
                });
                break;
            }
        }
    }

    /// Generate a fly.toml or equivalent deploy manifest
    pub fn generate_manifest(&self, target_name: &str) -> Option<String> {
        let target = self.targets.iter().find(|t| t.name == target_name)?;
        let mut out = format!(
            "# SigmaOS Cloud Agent Manifest — {}\n# Provider: {}\n# Region: {}\n\n",
            target.name,
            target.provider.name(),
            target.region
        );
        out.push_str("[env]\n");
        for v in &self.global_vars {
            if !v.is_secret {
                out.push_str(&format!("  {} = \"{}\"\n", v.key, v.value));
            }
        }
        for v in &target.env_vars {
            if !v.is_secret {
                out.push_str(&format!("  {} = \"{}\"\n", v.key, v.value));
            }
        }
        out.push_str(&format!(
            "\n[resources]\n  cpu = \"{}m\"\n  memory = \"{}Mi\"\n",
            target.cpu_limit, target.memory_limit_mb
        ));
        Some(out)
    }

    /// List all target names
    pub fn list_targets(&self) -> Vec<String> {
        self.targets.iter().map(|t| t.name.clone()).collect()
    }

    pub fn target_count(&self) -> usize {
        self.targets.len()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_agent_environment() {
        let mut env = OmarchyCloudAgentEnvironment::new();

        // Global vars
        env.set_global_var("SIGMAOS_VERSION", "0.1.0", false);
        env.set_global_var("OPENAI_API_KEY", "sk-secret", true);

        // Targets
        env.add_target("sigma-prod", CloudProvider::Hetzner, "hel1");
        env.add_target("sigma-edge", CloudProvider::Cloudflare, "global");
        env.add_target("sigma-dev", CloudProvider::Fly, "ams");
        assert_eq!(env.target_count(), 3);

        // Per-target vars
        env.set_target_var("sigma-prod", "LOG_LEVEL", "info", false);
        env.set_limits("sigma-prod", 2000, 1024);

        // Manifest generation
        let manifest = env.generate_manifest("sigma-prod").unwrap();
        assert!(manifest.contains("Hetzner"));
        assert!(manifest.contains("hel1"));
        assert!(manifest.contains("SIGMAOS_VERSION"));
        assert!(!manifest.contains("OPENAI_API_KEY")); // secret — excluded
        assert!(manifest.contains("LOG_LEVEL"));
        assert!(manifest.contains("2000m"));
        assert!(manifest.contains("1024Mi"));

        // List targets
        let names = env.list_targets();
        assert!(names.contains(&String::from("sigma-prod")));
        assert!(names.contains(&String::from("sigma-edge")));
    }
}
