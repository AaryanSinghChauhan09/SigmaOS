// SPDX-License-Identifier: MIT
// SigmaOS — Omarchy Docker Rootless Engine
// Inspired by Omarchy branch: docker-no-group
// Zero external dependencies, Safe Rust

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

#[cfg(any(feature = "standalone_test", test))]
use std::{format, string::String, vec::Vec};
#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{format, string::String, vec::Vec};

/// Docker runtime mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockerMode {
    /// Traditional mode — requires docker group (security risk)
    Privileged,
    /// Rootless mode — no docker group, user-namespace isolation
    Rootless,
    /// Podman-compatible rootless (OCI)
    PodmanRootless,
}

/// Container namespace isolation level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamespaceLevel {
    /// All namespaces: user, pid, net, mnt, uts, ipc
    Full,
    /// Network only
    NetworkOnly,
    /// User + PID (minimal)
    UserPid,
}

/// A rootless container configuration record
#[derive(Debug, Clone)]
pub struct RootlessContainerConfig {
    pub name: String,
    pub image: String,
    pub mode: DockerMode,
    pub ns_level: NamespaceLevel,
    pub uid_map: String, // "0 100000 65536"
    pub gid_map: String,
    pub network_mode: String,
}

/// SigmaOS Docker Rootless Engine — surpasses Omarchy docker-no-group
pub struct OmarchyDockerRootlessEngine {
    mode: DockerMode,
    containers: Vec<RootlessContainerConfig>,
    uid_base: u32,
}

impl OmarchyDockerRootlessEngine {
    pub fn new() -> Self {
        Self {
            mode: DockerMode::Rootless,
            containers: Vec::new(),
            uid_base: 100_000,
        }
    }

    /// Set the runtime mode
    pub fn set_mode(&mut self, mode: DockerMode) {
        self.mode = mode;
    }

    /// Register a rootless container
    pub fn register_container(&mut self, name: &str, image: &str) {
        let uid_map = format!("0 {} 65536", self.uid_base);
        let gid_map = format!("0 {} 65536", self.uid_base);
        self.containers.push(RootlessContainerConfig {
            name: String::from(name),
            image: String::from(image),
            mode: self.mode,
            ns_level: NamespaceLevel::Full,
            uid_map,
            gid_map,
            network_mode: String::from("slirp4netns"),
        });
    }

    /// Generate the `~/.config/docker/daemon.json` content for rootless mode
    pub fn generate_daemon_json(&self) -> String {
        match self.mode {
            DockerMode::Rootless => String::from(
                r#"{"userns-remap":"default","no-new-privileges":true,"log-driver":"json-file","log-opts":{"max-size":"10m","max-file":"3"}}"#,
            ),
            DockerMode::PodmanRootless => String::from(
                r#"{"runtime":"crun","network_backend":"netavark","userns":"keep-id"}"#,
            ),
            DockerMode::Privileged => String::from(r#"{"log-driver":"json-file"}"#),
        }
    }

    /// Generate subuid/subgid entries for /etc/subuid and /etc/subgid
    pub fn generate_subuid_entry(&self, username: &str) -> String {
        format!("{}:{}:65536", username, self.uid_base)
    }

    /// Generate systemd user unit for dockerd rootless
    pub fn generate_systemd_user_unit(&self) -> String {
        String::from(
            "[Unit]\nDescription=Docker Application Container Engine (Rootless)\n\
            [Service]\nType=simple\nExecStart=/usr/bin/dockerd-rootless.sh\nRestart=on-failure\n\
            [Install]\nWantedBy=default.target\n",
        )
    }

    /// Get the run command for a registered container (rootless-compatible)
    pub fn get_run_command(&self, name: &str) -> Option<String> {
        for c in &self.containers {
            if c.name == name {
                return Some(format!(
                    "docker run --rm --network {} --userns=keep-id {} {}",
                    c.network_mode, c.image, c.name
                ));
            }
        }
        None
    }

    pub fn container_count(&self) -> usize {
        self.containers.len()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_rootless_engine() {
        let mut engine = OmarchyDockerRootlessEngine::new();
        assert_eq!(engine.mode, DockerMode::Rootless);

        // Register containers
        engine.register_container("sigma-app", "sigmaos/app:latest");
        engine.register_container("sigma-db", "postgres:16-alpine");
        assert_eq!(engine.container_count(), 2);

        // Check daemon.json
        let json = engine.generate_daemon_json();
        assert!(json.contains("userns-remap"));
        assert!(json.contains("no-new-privileges"));

        // Check subuid entry
        let subuid = engine.generate_subuid_entry("sigma");
        assert!(subuid.starts_with("sigma:100000:65536"));

        // Check systemd unit
        let unit = engine.generate_systemd_user_unit();
        assert!(unit.contains("dockerd-rootless"));

        // Check run command
        let cmd = engine.get_run_command("sigma-app").unwrap();
        assert!(cmd.contains("slirp4netns"));
        assert!(cmd.contains("userns=keep-id"));

        // Switch to Podman mode
        engine.set_mode(DockerMode::PodmanRootless);
        let json2 = engine.generate_daemon_json();
        assert!(json2.contains("crun"));
    }
}
