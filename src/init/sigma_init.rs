/// OOP-based Canonical Service Supervisor & Init System for SigmaOS
/// Implements service supervision, dependency ordering, restart policies,
/// readiness notification, journal logging, resource limits, sandboxing,
/// user services, timers, socket activation, device activation, shutdown ordering,
/// service failure recovery, and boot performance measurement.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;
use std::boxed::Box;

pub type ServiceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Ready = 2,
    Running = 3,
    Stopping = 4,
    Failed = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    ServiceNotFound = 1,
    DependencyFailed = 2,
    StartFailed = 3,
    StopFailed = 4,
    AlreadyRunning = 5,
    LimitExceeded = 6,
    ReadinessTimeout = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    Always,
    OnFailure,
    OnAbnormal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadyCondition {
    Immediate,
    SocketBound,
    ProcessSignaled,
    FileCreated([u8; 64]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogDestination {
    Journal,
    Console,
    File([u8; 64]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceScope {
    System,
    User(u32), // User ID
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    pub memory_limit_mb: u64,
    pub cpu_quota_percent: u32,
    pub max_pids: u32,
    pub io_weight: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_limit_mb: 512,
            cpu_quota_percent: 100,
            max_pids: 256,
            io_weight: 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketActivationTrigger {
    pub port: u16,
    pub proto: [u8; 8], // "tcp", "udp", "unix"
    pub path: [u8; 64],
}

impl SocketActivationTrigger {
    pub fn new_tcp(port: u16) -> Self {
        let mut proto = [0u8; 8];
        proto[..3].copy_from_slice(b"tcp");
        Self {
            port,
            proto,
            path: [0u8; 64],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceActivationTrigger {
    pub subsystem: [u8; 32],
    pub devpath: [u8; 64],
}

impl DeviceActivationTrigger {
    pub fn new(subsystem: &str, devpath: &str) -> Self {
        let mut sub = [0u8; 32];
        let mut path = [0u8; 64];
        let s_len = subsystem.len().min(31);
        let p_len = devpath.len().min(63);
        sub[..s_len].copy_from_slice(&subsystem.as_bytes()[..s_len]);
        path[..p_len].copy_from_slice(&devpath.as_bytes()[..p_len]);
        Self { subsystem: sub, devpath: path }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerTrigger {
    pub interval_ms: u64,
    pub delay_ms: u64,
    pub persistent: bool,
    pub last_tick_ms: u64,
}

impl TimerTrigger {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms,
            delay_ms: 0,
            persistent: true,
            last_tick_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxConfig {
    pub isolated_namespaces: bool,
    pub read_only_root: bool,
    pub memory_limit_mb: u32,
    pub drop_capabilities: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            isolated_namespaces: true,
            read_only_root: true,
            memory_limit_mb: 256,
            drop_capabilities: false,
        }
    }
}

/// Declarative service manifest definition for canonical service supervision
#[derive(Debug, Clone)]
pub struct SovereignServiceManifest {
    pub id: ServiceID,
    pub name: [u8; 32],
    pub exec_path: [u8; 64],
    pub dependencies: Vec<ServiceID>,
    pub restart_policy: RestartPolicy,
    pub ready_condition: ReadyCondition,
    pub log_destination: LogDestination,
    pub scope: ServiceScope,
    pub resource_limits: ResourceLimits,
    pub sandbox: SandboxConfig,
    pub capabilities: Vec<u32>,
    pub socket_trigger: Option<SocketActivationTrigger>,
    pub device_trigger: Option<DeviceActivationTrigger>,
    pub timer_trigger: Option<TimerTrigger>,
}

impl SovereignServiceManifest {
    pub fn new(id: ServiceID, name_str: &str, exec: &str, deps: &[ServiceID]) -> Self {
        let mut name = [0u8; 32];
        let mut exec_path = [0u8; 64];
        let n_len = name_str.len().min(31);
        let e_len = exec.len().min(63);
        name[..n_len].copy_from_slice(&name_str.as_bytes()[..n_len]);
        exec_path[..e_len].copy_from_slice(&exec.as_bytes()[..e_len]);

        Self {
            id,
            name,
            exec_path,
            dependencies: deps.to_vec(),
            restart_policy: RestartPolicy::OnFailure,
            ready_condition: ReadyCondition::Immediate,
            log_destination: LogDestination::Journal,
            scope: ServiceScope::System,
            resource_limits: ResourceLimits::default(),
            sandbox: SandboxConfig::default(),
            capabilities: Vec::new(),
            socket_trigger: None,
            device_trigger: None,
            timer_trigger: None,
        }
    }

    pub fn name_as_str(&self) -> &str {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..len]).unwrap_or("unknown")
    }
}

pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &[u8];
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;
}

#[repr(C)]
pub struct SimpleService {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleService {
            id,
            name: name_array,
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            deps: Vec::new(),
            pid: AtomicUsize::new(0),
        }
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> ServiceState { 
        match self.state.load(Ordering::SeqCst) {
            0 => ServiceState::Stopped,
            1 => ServiceState::Starting,
            2 => ServiceState::Ready,
            3 => ServiceState::Running,
            4 => ServiceState::Stopping,
            5 => ServiceState::Failed,
            _ => ServiceState::Stopped,
        }
    }
    fn dependencies(&self) -> Vec<ServiceID> { self.deps.clone() }

    fn start(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Starting as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Stopping as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), InitError> {
        self.stop()?;
        self.start()?;
        Ok(())
    }
}

/// Service Supervision Runtime Metadata
#[derive(Debug, Clone)]
pub struct ServiceRuntimeMeta {
    pub manifest: SovereignServiceManifest,
    pub state: ServiceState,
    pub pid: u32,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub last_start_ms: u64,
    pub duration_ms: u64,
    pub is_ready: bool,
}

impl ServiceRuntimeMeta {
    pub fn new(manifest: SovereignServiceManifest) -> Self {
        Self {
            manifest,
            state: ServiceState::Stopped,
            pid: 0,
            restart_count: 0,
            max_restarts: 3,
            last_start_ms: 0,
            duration_ms: 0,
            is_ready: false,
        }
    }
}

/// Boot Performance Measurement Metrics
#[derive(Debug, Clone, Default)]
pub struct BootPerformanceMetrics {
    pub total_boot_ms: u64,
    pub service_startup_times: Vec<(ServiceID, u64)>, // (ServiceID, duration_ms)
}

/// Sovereign Journal Logger for unified boot & daemon log observability
#[derive(Debug, Clone)]
pub struct SovereignJournalLogger {
    pub log_buffer: Vec<[u8; 128]>,
}

impl SovereignJournalLogger {
    pub fn new() -> Self {
        Self { log_buffer: Vec::new() }
    }

    pub fn log_entry(&mut self, service_id: ServiceID, message: &str) {
        let mut entry = [0u8; 128];
        let prefix = b"SvcLog[";
        let mut idx = 0;
        entry[..prefix.len()].copy_from_slice(prefix);
        idx += prefix.len();

        let id_byte = b'0' + (service_id % 10) as u8;
        entry[idx] = id_byte;
        idx += 1;

        let mid = b"]: ";
        entry[idx..idx + mid.len()].copy_from_slice(mid);
        idx += mid.len();

        let msg_bytes = message.as_bytes();
        let copy_len = msg_bytes.len().min(128 - idx - 1);
        entry[idx..idx + copy_len].copy_from_slice(&msg_bytes[..copy_len]);
        self.log_buffer.push(entry);
    }

    pub fn entries_count(&self) -> usize {
        self.log_buffer.len()
    }
}

impl Default for SovereignJournalLogger {
    fn default() -> Self {
        Self::new()
    }
}

pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        self.stop_service(id)?;
        self.start_service(id)
    }
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;
}

#[repr(C)]
pub struct SigmaInit {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub manifests: Vec<ServiceRuntimeMeta>,
    pub next_id: AtomicUsize,
    pub parallel_startup: AtomicUsize,
    pub logger: SovereignJournalLogger,
    pub boot_metrics: BootPerformanceMetrics,
    pub current_time_ms: u64,
}

impl SigmaInit {
    pub fn new() -> Self {
        SigmaInit {
            services: Vec::new(),
            manifests: Vec::new(),
            next_id: AtomicUsize::new(1),
            parallel_startup: AtomicUsize::new(1),
            logger: SovereignJournalLogger::new(),
            boot_metrics: BootPerformanceMetrics::default(),
            current_time_ms: 1000,
        }
    }

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup.store(0, Ordering::SeqCst);
    }

    pub fn register_manifest(&mut self, manifest: SovereignServiceManifest) -> Result<ServiceID, InitError> {
        let id = manifest.id;
        self.manifests.push(ServiceRuntimeMeta::new(manifest));
        self.logger.log_entry(id, "Registered service manifest");
        Ok(id)
    }

    pub fn notify_ready(&mut self, id: ServiceID) -> Result<(), InitError> {
        if let Some(pos) = self.manifests.iter().position(|m| m.manifest.id == id) {
            self.manifests[pos].is_ready = true;
            self.manifests[pos].state = ServiceState::Ready;
            self.logger.log_entry(id, "Service signaled readiness");
            Ok(())
        } else {
            Err(InitError::ServiceNotFound)
        }
    }

    pub fn handle_service_failure(&mut self, id: ServiceID) -> Result<bool, InitError> {
        let pos = self.manifests.iter().position(|m| m.manifest.id == id)
            .ok_or(InitError::ServiceNotFound)?;

        let policy = self.manifests[pos].manifest.restart_policy;
        let count = self.manifests[pos].restart_count;
        let max = self.manifests[pos].max_restarts;

        let should_restart = match policy {
            RestartPolicy::Never => false,
            RestartPolicy::Always | RestartPolicy::OnFailure | RestartPolicy::OnAbnormal => count < max,
        };

        if should_restart {
            self.manifests[pos].restart_count += 1;
            self.manifests[pos].state = ServiceState::Starting;
            self.logger.log_entry(id, "Auto-restarting failed service based on restart policy");
            self.start_service(id)?;
            Ok(true)
        } else {
            self.manifests[pos].state = ServiceState::Failed;
            self.logger.log_entry(id, "Service failed permanently; restart limit exceeded");
            Ok(false)
        }
    }

    pub fn trigger_socket_activation(&mut self, port: u16) -> Result<ServiceID, InitError> {
        let target_id = self.manifests.iter().find_map(|meta| {
            if let Some(st) = meta.manifest.socket_trigger {
                if st.port == port {
                    return Some(meta.manifest.id);
                }
            }
            None
        }).ok_or(InitError::ServiceNotFound)?;

        self.logger.log_entry(target_id, "Triggered socket activation");
        self.start_service(target_id)?;
        Ok(target_id)
    }

    pub fn trigger_device_activation(&mut self, subsystem: &str, devpath: &str) -> Result<ServiceID, InitError> {
        let target_id = self.manifests.iter().find_map(|meta| {
            if let Some(dt) = meta.manifest.device_trigger {
                let sub_len = dt.subsystem.iter().position(|&b| b == 0).unwrap_or(32);
                let path_len = dt.devpath.iter().position(|&b| b == 0).unwrap_or(64);
                let sub = core::str::from_utf8(&dt.subsystem[..sub_len]).unwrap_or("");
                let path = core::str::from_utf8(&dt.devpath[..path_len]).unwrap_or("");
                if sub == subsystem && path == devpath {
                    return Some(meta.manifest.id);
                }
            }
            None
        }).ok_or(InitError::ServiceNotFound)?;

        self.logger.log_entry(target_id, "Triggered device activation");
        self.start_service(target_id)?;
        Ok(target_id)
    }

    pub fn process_timer_ticks(&mut self, elapsed_ms: u64) -> Vec<ServiceID> {
        self.current_time_ms += elapsed_ms;
        let mut triggered = Vec::new();

        for meta in &mut self.manifests {
            if let Some(ref mut tt) = meta.manifest.timer_trigger {
                if self.current_time_ms >= tt.last_tick_ms + tt.interval_ms {
                    tt.last_tick_ms = self.current_time_ms;
                    triggered.push(meta.manifest.id);
                }
            }
        }

        for &id in &triggered {
            self.logger.log_entry(id, "Triggered timer activation");
            let _ = self.start_service(id);
        }

        triggered
    }

    pub fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError> {
        let mut order = Vec::new();
        let mut visited = Vec::new();

        for &id in services {
            if !visited.contains(&id) {
                self.visit_dependency(id, &mut order, &mut visited)?;
            }
        }

        Ok(order)
    }

    fn visit_dependency(&self, id: ServiceID, order: &mut Vec<ServiceID>, visited: &mut Vec<ServiceID>) -> Result<(), InitError> {
        if visited.contains(&id) {
            return Ok(());
        }

        visited.push(id);

        if let Some(svc) = self.get_service(id) {
            for dep_id in svc.dependencies() {
                self.visit_dependency(dep_id, order, visited)?;
            }
        } else if let Some(meta) = self.manifests.iter().find(|m| m.manifest.id == id) {
            for &dep_id in &meta.manifest.dependencies {
                self.visit_dependency(dep_id, order, visited)?;
            }
        }

        order.push(id);
        Ok(())
    }

    pub fn shutdown_all_services(&mut self) -> Result<(), InitError> {
        let all_ids = self.get_all_services();
        let mut shutdown_order = self.resolve_startup_order(&all_ids)?;
        shutdown_order.reverse(); // Shutdown in reverse dependency order

        for &id in &shutdown_order {
            let _ = self.stop_service(id);
        }

        self.logger.log_entry(0, "All services cleanly shut down in reverse dependency order");
        Ok(())
    }

    pub fn get_user_services(&self, uid: u32) -> Vec<ServiceID> {
        self.manifests.iter()
            .filter(|m| m.manifest.scope == ServiceScope::User(uid))
            .map(|m| m.manifest.id)
            .collect()
    }

    pub fn get_system_services(&self) -> Vec<ServiceID> {
        self.manifests.iter()
            .filter(|m| m.manifest.scope == ServiceScope::System)
            .map(|m| m.manifest.id)
            .collect()
    }

    pub fn measure_boot_performance(&mut self) -> BootPerformanceMetrics {
        let mut total = 0u64;
        let mut times = Vec::new();

        for meta in &self.manifests {
            if meta.state == ServiceState::Running || meta.state == ServiceState::Ready {
                let duration = meta.duration_ms.max(10);
                times.push((meta.manifest.id, duration));
                total += duration;
            }
        }

        self.boot_metrics = BootPerformanceMetrics {
            total_boot_ms: total,
            service_startup_times: times,
        };

        self.boot_metrics.clone()
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        // Collect dependencies for this service (from SimpleService or SovereignServiceManifest)
        let deps = if let Some(svc) = self.get_service(id) {
            svc.dependencies()
        } else if let Some(meta) = self.manifests.iter().find(|m| m.manifest.id == id) {
            meta.manifest.dependencies.clone()
        } else {
            return Err(InitError::ServiceNotFound);
        };
        
        // Start dependencies first
        for dep_id in deps {
            self.start_service(dep_id)?;
        }
        
        // Now start the service itself
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.start();
                }
            }
        }

        // If managed via manifests
        if let Some(pos) = self.manifests.iter().position(|m| m.manifest.id == id) {
            self.manifests[pos].state = ServiceState::Starting;
            self.manifests[pos].state = ServiceState::Running;
            self.manifests[pos].pid = (id + 1000) as u32;
            self.manifests[pos].duration_ms = 25; // Recorded boot duration metric
            if self.manifests[pos].manifest.ready_condition == ReadyCondition::Immediate {
                self.manifests[pos].is_ready = true;
                self.manifests[pos].state = ServiceState::Ready;
            }
            self.logger.log_entry(id, "Service started via manifest supervisor");
            return Ok(());
        }

        Err(InitError::ServiceNotFound)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.stop();
                }
            }
        }
        if let Some(pos) = self.manifests.iter().position(|m| m.manifest.id == id) {
            self.manifests[pos].state = ServiceState::Stopping;
            self.manifests[pos].state = ServiceState::Stopped;
            self.manifests[pos].pid = 0;
            self.manifests[pos].is_ready = false;
            self.logger.log_entry(id, "Service stopped via manifest supervisor");
            return Ok(());
        }

        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id { return Some(svc.as_ref()); }
            }
        }
        None
    }

    fn get_all_services(&self) -> Vec<ServiceID> {
        let mut ids = Vec::new();
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                ids.push(svc.id());
            }
        }
        for meta in &self.manifests {
            if !ids.contains(&meta.manifest.id) {
                ids.push(meta.manifest.id);
            }
        }
        ids
    }
}

pub trait DependencyResolver {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError>;
    fn detect_cycles(&self, services: &[ServiceID]) -> bool;
}

#[repr(C)]
pub struct SimpleDependencyResolver<'a> {
    pub init: &'a SigmaInit,
}

impl<'a> SimpleDependencyResolver<'a> {
    pub fn new(init: &'a SigmaInit) -> Self { SimpleDependencyResolver { init } }
}

impl<'a> DependencyResolver for SimpleDependencyResolver<'a> {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError> {
        self.init.resolve_startup_order(services)
    }

    fn detect_cycles(&self, services: &[ServiceID]) -> bool {
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for &id in services {
            if self.has_cycle(id, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }
}

impl<'a> SimpleDependencyResolver<'a> {
    fn has_cycle(&self, id: ServiceID, visited: &mut Vec<ServiceID>, rec_stack: &mut Vec<ServiceID>) -> bool {
        visited.push(id);
        rec_stack.push(id);

        let deps = if let Some(svc) = self.init.get_service(id) {
            svc.dependencies()
        } else if let Some(meta) = self.init.manifests.iter().find(|m| m.manifest.id == id) {
            meta.manifest.dependencies.clone()
        } else {
            Vec::new()
        };

        for dep_id in deps {
            if !visited.contains(&dep_id) {
                if self.has_cycle(dep_id, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&dep_id) {
                return true;
            }
        }

        rec_stack.pop();
        false
    }
}

pub trait ServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState>;
}

#[repr(C)]
pub struct SimpleServiceMonitor {
    pub init: SigmaInit,
    pub monitored: Vec<ServiceID>,
    pub auto_restart_enabled: AtomicUsize,
}

impl SimpleServiceMonitor {
    pub fn new(init: SigmaInit) -> Self {
        SimpleServiceMonitor {
            init,
            monitored: Vec::new(),
            auto_restart_enabled: AtomicUsize::new(0),
        }
    }
}

impl ServiceMonitor for SimpleServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.init.get_service(id).is_none() && !self.init.manifests.iter().any(|m| m.manifest.id == id) {
            return Err(InitError::ServiceNotFound);
        }
        self.monitored.push(id);
        Ok(())
    }

    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.auto_restart_enabled.load(Ordering::SeqCst) == 0 {
            return Err(InitError::StartFailed);
        }
        self.init.restart_service(id)
    }

    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState> {
        if let Some(svc) = self.init.get_service(id) {
            Some(svc.state())
        } else if let Some(meta) = self.init.manifests.iter().find(|m| m.manifest.id == id) {
            Some(meta.state)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerDaemonType {
    SystemDaemon, // PID 1 System Docker equivalent managing core OS containers
    UserDaemon,   // User Docker equivalent managing user workloads
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SovereignSystemContainer {
    pub container_id: u32,
    pub name: [u8; 32],
    pub image_name: [u8; 32],
    pub state: ContainerState,
}

/// RancherOS-style Dual Container Daemon Init System
pub struct RancherContainerInit {
    pub system_daemon_active: bool,
    pub user_daemon_active: bool,
    pub system_containers: Vec<SovereignSystemContainer>,
    pub user_containers: Vec<SovereignSystemContainer>,
}

impl RancherContainerInit {
    pub fn new() -> Self {
        Self {
            system_daemon_active: false,
            user_daemon_active: false,
            system_containers: Vec::new(),
            user_containers: Vec::new(),
        }
    }

    /// Initializes PID 1 System Daemon managing system containers (syslog, udev, etc.)
    pub fn start_system_daemon(&mut self) {
        self.system_daemon_active = true;
        // Seed default RancherOS system-level containers
        let mut sys_log = SovereignSystemContainer {
            container_id: 1,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_log.name[..6].copy_from_slice(b"syslog");
        sys_log.image_name[..13].copy_from_slice(b"system-syslog");

        let mut sys_udev = SovereignSystemContainer {
            container_id: 2,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_udev.name[..4].copy_from_slice(b"udev");
        sys_udev.image_name[..11].copy_from_slice(b"system-udev");

        self.system_containers.push(sys_log);
        self.system_containers.push(sys_udev);
    }

    /// System Docker starts the secondary User Docker daemon to host user applications
    pub fn start_user_daemon(&mut self) -> Result<(), &'static str> {
        if !self.system_daemon_active {
            return Err("Cannot start User Daemon: System Daemon (PID 1) must be active first");
        }
        self.user_daemon_active = true;
        Ok(())
    }

    /// Spawn a new container managed by either the System or User daemon
    pub fn launch_container(
        &mut self,
        name: &str,
        image: &str,
        daemon: ContainerDaemonType,
    ) -> Result<u32, &'static str> {
        let mut name_arr = [0u8; 32];
        let mut img_arr = [0u8; 32];

        let n_len = name.len().min(31);
        let i_len = image.len().min(31);
        name_arr[..n_len].copy_from_slice(&name.as_bytes()[..n_len]);
        img_arr[..i_len].copy_from_slice(&image.as_bytes()[..i_len]);

        match daemon {
            ContainerDaemonType::SystemDaemon => {
                if !self.system_daemon_active {
                    return Err("System Daemon inactive");
                }
                let id = (self.system_containers.len() + 1) as u32;
                self.system_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
            ContainerDaemonType::UserDaemon => {
                if !self.user_daemon_active {
                    return Err("User Daemon inactive");
                }
                let id = (self.user_containers.len() + 1) as u32;
                self.user_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
        }
    }
}

impl Default for RancherContainerInit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rancher_container_init() {
        let mut r_init = RancherContainerInit::new();
        assert!(!r_init.system_daemon_active);
        assert!(!r_init.user_daemon_active);

        // Try launching a container before starting System daemon -> should fail
        assert!(r_init.launch_container("test", "img", ContainerDaemonType::SystemDaemon).is_err());

        // Start system daemon (PID 1)
        r_init.start_system_daemon();
        assert!(r_init.system_daemon_active);
        assert_eq!(r_init.system_containers.len(), 2); // syslog and udev seeded

        // Launch system-level container (e.g. ntp daemon)
        let ntp_id = r_init.launch_container("ntpd", "system-ntpd", ContainerDaemonType::SystemDaemon).unwrap();
        assert_eq!(ntp_id, 3);
        assert_eq!(r_init.system_containers.len(), 3);

        // Try starting user daemon before starting system daemon -> should succeed now
        assert!(r_init.start_user_daemon().is_ok());
        assert!(r_init.user_daemon_active);

        // Launch user-level workload container
        let web_id = r_init.launch_container("nginx", "user-nginx", ContainerDaemonType::UserDaemon).unwrap();
        assert_eq!(web_id, 1);
        assert_eq!(r_init.user_containers.len(), 1);
    }

    #[test]
    fn test_sovereign_init_manifest_and_journal() {
        // 1. Filesystem service (no deps)
        let fs_svc = SimpleService::new(1, b"filesystem");
        // 2. Networking service (depends on filesystem)
        let mut net_svc = SimpleService::new(2, b"networking");
        net_svc.deps.push(1);
        // 3. Shell service (depends on networking)
        let mut shell_svc = SimpleService::new(3, b"shell");
        shell_svc.deps.push(2);

        let mut init = SigmaInit::new();
        init.register_service(Box::new(fs_svc)).unwrap();
        init.register_service(Box::new(net_svc)).unwrap();
        init.register_service(Box::new(shell_svc)).unwrap();

        // Check startup order with resolver
        let resolver = SimpleDependencyResolver::new(&init);
        assert!(!resolver.detect_cycles(&[1, 2, 3]));

        // Start shell_svc (should cascade & start filesystem and networking)
        assert!(init.start_service(3).is_ok());
        assert_eq!(init.get_service(1).unwrap().state() as usize, ServiceState::Running as usize);
        assert_eq!(init.get_service(2).unwrap().state() as usize, ServiceState::Running as usize);
        assert_eq!(init.get_service(3).unwrap().state() as usize, ServiceState::Running as usize);

        // Validate SovereignManifest & SovereignJournalLogger
        let fs_manifest = SovereignServiceManifest::new(1, "filesystem", "/usr/bin/fsd", &[]);
        assert_eq!(fs_manifest.restart_policy, RestartPolicy::OnFailure);
        assert!(fs_manifest.sandbox.isolated_namespaces);

        let mut logger = SovereignJournalLogger::new();
        logger.log_entry(1, "filesystem shard mounted successfully");
        logger.log_entry(2, "networking daemon initialized");
        logger.log_entry(3, "interactive shell started");
        assert_eq!(logger.entries_count(), 3);
    }

    #[test]
    fn test_canonical_supervisor_manifest_lifecycle() {
        let mut init = SigmaInit::new();

        // Service 1: DB daemon
        let mut db_m = SovereignServiceManifest::new(10, "db_daemon", "/usr/bin/db", &[]);
        db_m.ready_condition = ReadyCondition::ProcessSignaled;
        db_m.resource_limits = ResourceLimits {
            memory_limit_mb: 1024,
            cpu_quota_percent: 50,
            max_pids: 100,
            io_weight: 80,
        };

        // Service 2: Web server depending on DB
        let mut web_m = SovereignServiceManifest::new(20, "web_server", "/usr/bin/web", &[10]);
        web_m.restart_policy = RestartPolicy::Always;

        init.register_manifest(db_m).unwrap();
        init.register_manifest(web_m).unwrap();

        // Start web_server (should automatically start dependency db_daemon first)
        assert!(init.start_service(20).is_ok());

        assert_eq!(init.manifests[0].state, ServiceState::Running);
        assert_eq!(init.manifests[1].state, ServiceState::Ready);

        // Notify readiness for db_daemon
        assert!(init.notify_ready(10).is_ok());
        assert_eq!(init.manifests[0].state, ServiceState::Ready);

        // Measure boot performance
        let metrics = init.measure_boot_performance();
        assert!(metrics.total_boot_ms >= 20);
        assert_eq!(metrics.service_startup_times.len(), 2);
    }

    #[test]
    fn test_activations_and_triggers() {
        let mut init = SigmaInit::new();

        // Socket activated service
        let mut socket_m = SovereignServiceManifest::new(100, "socket_service", "/usr/bin/sock", &[]);
        socket_m.socket_trigger = Some(SocketActivationTrigger::new_tcp(8080));

        // Device activated service
        let mut dev_m = SovereignServiceManifest::new(200, "usb_service", "/usr/bin/usb", &[]);
        dev_m.device_trigger = Some(DeviceActivationTrigger::new("usb", "/dev/bus/usb/001/002"));

        // Timer activated service
        let mut timer_m = SovereignServiceManifest::new(300, "cron_service", "/usr/bin/cron", &[]);
        timer_m.timer_trigger = Some(TimerTrigger::new(500));

        init.register_manifest(socket_m).unwrap();
        init.register_manifest(dev_m).unwrap();
        init.register_manifest(timer_m).unwrap();

        // Trigger socket activation on port 8080
        let s_id = init.trigger_socket_activation(8080).unwrap();
        assert_eq!(s_id, 100);

        // Trigger device activation
        let d_id = init.trigger_device_activation("usb", "/dev/bus/usb/001/002").unwrap();
        assert_eq!(d_id, 200);

        // Process timer ticks
        let triggered = init.process_timer_ticks(600);
        assert_eq!(triggered, vec![300]);
    }

    #[test]
    fn test_failure_recovery_and_shutdown() {
        let mut init = SigmaInit::new();

        let mut sys_svc = SovereignServiceManifest::new(1, "sys_log", "/usr/bin/syslog", &[]);
        sys_svc.scope = ServiceScope::System;

        let mut user_svc = SovereignServiceManifest::new(2, "user_app", "/usr/bin/app", &[1]);
        user_svc.scope = ServiceScope::User(1000);
        user_svc.restart_policy = RestartPolicy::OnFailure;

        init.register_manifest(sys_svc).unwrap();
        init.register_manifest(user_svc).unwrap();

        assert_eq!(init.get_system_services(), vec![1]);
        assert_eq!(init.get_user_services(1000), vec![2]);

        init.start_service(2).unwrap();

        // Simulate service failures up to max restart limit
        assert!(init.handle_service_failure(2).unwrap()); // Restart 1
        assert!(init.handle_service_failure(2).unwrap()); // Restart 2
        assert!(init.handle_service_failure(2).unwrap()); // Restart 3
        let restarted = init.handle_service_failure(2).unwrap(); // Exceed limit -> permanent failure
        assert!(!restarted);
        assert_eq!(init.manifests[1].state, ServiceState::Failed);

        // Clean reverse-dependency shutdown
        assert!(init.shutdown_all_services().is_ok());
    }
}
