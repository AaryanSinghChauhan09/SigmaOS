// sigma_init.rs — sigmad Service Manager (runit/OpenRC-inspired)
// The PID 1 init system for SigmaOS. Manages service supervision trees,
// dependency ordering, health checks, and automatic restart policies.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── Service Definitions ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Restarting,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
    OnAbnormal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    Oneshot,
    Daemon,
    Forking,
    Notify,
    Timer,
}

#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub service_type: ServiceType,
    pub exec_start: String,
    pub exec_stop: String,
    pub depends_on: Vec<String>,
    pub wanted_by: String,
    pub restart_policy: RestartPolicy,
    pub restart_delay_ms: u32,
    pub max_restarts: u8,
    pub current_restarts: u8,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub health_check_cmd: Option<String>,
    pub health_interval_ms: u32,
}

// ── Service Supervision Tree ────────────────────────────────────────────────

#[derive(Debug)]
pub struct SupervisionTree {
    pub services: Vec<ServiceUnit>,
    pub boot_target: String,
}

impl SupervisionTree {
    pub fn new() -> Self {
        SupervisionTree {
            services: Vec::new(),
            boot_target: String::from("multi-user.target"),
        }
    }

    pub fn register(&mut self, service: ServiceUnit) {
        self.services.push(service);
    }

    pub fn boot_order(&self) -> Result<Vec<&ServiceUnit>, &'static str> {
        let mut result: Vec<&ServiceUnit> = Vec::new();
        let mut visited: Vec<String> = Vec::new();

        fn visit<'a>(
            name: &str,
            services: &'a [ServiceUnit],
            visited: &mut Vec<String>,
            result: &mut Vec<&'a ServiceUnit>,
        ) -> Result<(), &'static str> {
            if visited.contains(&String::from(name)) {
                return Ok(());
            }
            let svc = services
                .iter()
                .find(|s| s.name.as_str() == name)
                .ok_or("Service not found")?;
            for dep in &svc.depends_on {
                visit(dep.as_str(), services, visited, result)?;
            }
            visited.push(String::from(name));
            result.push(svc);
            Ok(())
        }

        for svc in &self.services {
            visit(&svc.name, &self.services, &mut visited, &mut result)?;
        }
        Ok(result)
    }

    pub fn start_service(&mut self, name: &str) -> Result<u32, &'static str> {
        let svc = self.services.iter_mut().find(|s| s.name.as_str() == name)
            .ok_or("Service not found")?;
        if svc.state == ServiceState::Running {
            return Err("Service already running");
        }
        svc.state = ServiceState::Starting;
        let pid = 1000 + (svc.name.len() as u32);
        svc.pid = Some(pid);
        svc.state = ServiceState::Running;
        Ok(pid)
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        let svc = self.services.iter_mut().find(|s| s.name.as_str() == name)
            .ok_or("Service not found")?;
        svc.state = ServiceState::Stopping;
        svc.pid = None;
        svc.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn handle_failure(&mut self, name: &str) -> Result<(), &'static str> {
        let svc = self.services.iter_mut().find(|s| s.name.as_str() == name)
            .ok_or("Service not found")?;
        match svc.restart_policy {
            RestartPolicy::Never => { svc.state = ServiceState::Failed; }
            _ => {
                if svc.current_restarts < svc.max_restarts {
                    svc.current_restarts += 1;
                    svc.state = ServiceState::Restarting;
                } else {
                    svc.state = ServiceState::Failed;
                }
            }
        }
        Ok(())
    }
}

pub fn create_default_services() -> SupervisionTree {
    let mut tree = SupervisionTree::new();
    tree.register(ServiceUnit {
        name: String::from("sigma-dbus"),
        service_type: ServiceType::Daemon,
        exec_start: String::from("/usr/bin/sigma-dbus-daemon --system"),
        exec_stop: String::from("/usr/bin/sigma-dbus-daemon --shutdown"),
        depends_on: Vec::new(),
        wanted_by: String::from("multi-user.target"),
        restart_policy: RestartPolicy::Always,
        restart_delay_ms: 1000, max_restarts: 10, current_restarts: 0,
        state: ServiceState::Stopped, pid: None,
        health_check_cmd: Some(String::from("sigma-dbus-check")),
        health_interval_ms: 5000,
    });
    tree.register(ServiceUnit {
        name: String::from("sigma-networkd"),
        service_type: ServiceType::Daemon,
        exec_start: String::from("/usr/bin/sigma-networkd"),
        exec_stop: String::from("/usr/bin/sigma-networkd --stop"),
        depends_on: alloc::vec![String::from("sigma-dbus")],
        wanted_by: String::from("multi-user.target"),
        restart_policy: RestartPolicy::OnFailure,
        restart_delay_ms: 2000, max_restarts: 5, current_restarts: 0,
        state: ServiceState::Stopped, pid: None,
        health_check_cmd: Some(String::from("ping -c1 127.0.0.1")),
        health_interval_ms: 10000,
    });
    tree.register(ServiceUnit {
        name: String::from("sigma-logd"),
        service_type: ServiceType::Daemon,
        exec_start: String::from("/usr/bin/sigma-logd"),
        exec_stop: String::from("/usr/bin/sigma-logd --stop"),
        depends_on: Vec::new(),
        wanted_by: String::from("multi-user.target"),
        restart_policy: RestartPolicy::Always,
        restart_delay_ms: 500, max_restarts: 20, current_restarts: 0,
        state: ServiceState::Stopped, pid: None,
        health_check_cmd: None, health_interval_ms: 30000,
    });
    tree.register(ServiceUnit {
        name: String::from("sigma-ai-agent"),
        service_type: ServiceType::Daemon,
        exec_start: String::from("/usr/bin/sigma-ai-agent --daemon"),
        exec_stop: String::from("/usr/bin/sigma-ai-agent --stop"),
        depends_on: alloc::vec![String::from("sigma-dbus"), String::from("sigma-logd")],
        wanted_by: String::from("multi-user.target"),
        restart_policy: RestartPolicy::OnFailure,
        restart_delay_ms: 3000, max_restarts: 3, current_restarts: 0,
        state: ServiceState::Stopped, pid: None,
        health_check_cmd: Some(String::from("sigma-ai-agent --health")),
        health_interval_ms: 15000,
    });
    tree.register(ServiceUnit {
        name: String::from("sigma-zenith-compositor"),
        service_type: ServiceType::Notify,
        exec_start: String::from("/usr/bin/sigma-zenith --wayland"),
        exec_stop: String::from("/usr/bin/sigma-zenith --exit"),
        depends_on: alloc::vec![String::from("sigma-dbus"), String::from("sigma-logd")],
        wanted_by: String::from("graphical.target"),
        restart_policy: RestartPolicy::OnAbnormal,
        restart_delay_ms: 2000, max_restarts: 3, current_restarts: 0,
        state: ServiceState::Stopped, pid: None,
        health_check_cmd: None, health_interval_ms: 10000,
    });
    tree
}
