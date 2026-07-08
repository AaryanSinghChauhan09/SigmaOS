//! SigmaOS — SigmaInit (PID 1) Init System
//! Sovereign init daemon inspired by systemd/OpenRC but fully native to SigmaOS.
//! No std, no allocator — fixed-size service table.
//!
//! BUG-009 FIX: Service dependency graph implemented via Kahn's topological sort.
//! Services now start in correct dependency order, not sequentially.
//! Each ServiceUnit carries a deps[] array; SigmaDepGraph builds an adjacency
//! list over a fixed array and performs BFS-based topo sort before boot.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;

// ── Constants ────────────────────────────────────────────────────────────────
const MAX_SERVICES:      usize = 128;
const MAX_TARGETS:       usize = 16;
const MAX_DEPS_PER_SVC:  usize = 8;
const MAX_NAME_LEN:      usize = 48;
const MAX_CMD_LEN:       usize = 128;
const MAX_ADJ_EDGES:     usize = 512; // Adjacency list edge budget

// ── Service States ───────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ServiceState {
    Inactive   = 0,
    Starting   = 1,
    Running    = 2,
    Stopping   = 3,
    Stopped    = 4,
    Failed     = 5,
    Restarting = 6,
    Masked     = 7,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ServiceType {
    Simple   = 0,  // Main process is the service
    Oneshot  = 1,  // Run once and exit
    Forking  = 2,  // Forks, parent exits
    Notify   = 3,  // Sends ready notification
    Idle     = 4,  // Run when system is idle
    Timer    = 5,  // Periodic timer service
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum RestartPolicy {
    No        = 0,
    Always    = 1,
    OnFailure = 2,
    OnAbnormal = 3,
}

// ── Boot Target (like systemd target) ────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum BootTarget {
    Emergency = 0,  // Single-user emergency shell
    Rescue    = 1,  // Single-user with basic services
    MultiUser = 2,  // Full multi-user, no GUI
    Graphical = 3,  // Full GUI desktop
    Reboot    = 4,
    Poweroff  = 5,
}

// ── Service Unit ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct ServiceUnit {
    pub name:            [U8; MAX_NAME_LEN],
    pub name_len:        usize,
    pub description:     [U8; MAX_CMD_LEN],
    pub desc_len:        usize,
    pub exec_start:      [U8; MAX_CMD_LEN],
    pub exec_start_len:  usize,
    pub exec_stop:       [U8; MAX_CMD_LEN],
    pub exec_stop_len:   usize,
    pub svc_type:        ServiceType,
    pub state:           ServiceState,
    pub restart:         RestartPolicy,
    pub pid:             U32,
    pub exit_code:       i32,
    pub start_time:      U64,
    pub restart_count:   U32,
    pub max_restarts:    U32,
    pub timeout_ms:      U32,
    pub deps:            [U32; MAX_DEPS_PER_SVC], // Service IDs this depends on
    pub dep_count:       usize,
    pub target:          BootTarget,
    pub enabled:         bool,
    pub active:          bool,
}

impl ServiceUnit {
    pub const fn empty() -> Self {
        ServiceUnit {
            name: [0u8; MAX_NAME_LEN], name_len: 0,
            description: [0u8; MAX_CMD_LEN], desc_len: 0,
            exec_start: [0u8; MAX_CMD_LEN], exec_start_len: 0,
            exec_stop: [0u8; MAX_CMD_LEN], exec_stop_len: 0,
            svc_type: ServiceType::Simple,
            state: ServiceState::Inactive,
            restart: RestartPolicy::No,
            pid: 0, exit_code: 0, start_time: 0,
            restart_count: 0, max_restarts: 3,
            timeout_ms: 30_000,
            deps: [0u32; MAX_DEPS_PER_SVC], dep_count: 0,
            target: BootTarget::MultiUser,
            enabled: false, active: false,
        }
    }

    /// Add a dependency on another service by index.
    pub fn add_dep(&mut self, dep_idx: U32) {
        if self.dep_count < MAX_DEPS_PER_SVC {
            self.deps[self.dep_count] = dep_idx;
            self.dep_count += 1;
        }
    }
}

// ── Dependency Graph (BUG-009 Fix) ───────────────────────────────────────────
//
// Implements Kahn's Algorithm for topological sorting over a fixed-capacity
// adjacency list. Uses no_std, no alloc — only fixed arrays.
//
// Representation:
//   adj_edges: [(from, to); MAX_ADJ_EDGES]  — directed edge list
//   in_degree: [usize; MAX_SERVICES]        — in-degree per node
//   topo_order: [u32; MAX_SERVICES]         — sorted output (start order)

pub struct SigmaDepGraph {
    /// Directed edges: (service_idx, must_start_after_idx)
    adj_edges:   [(u32, u32); MAX_ADJ_EDGES],
    edge_count:  usize,
    /// Count of unresolved incoming dependencies per service
    in_degree:   [usize; MAX_SERVICES],
    /// Topological order output (indices into service table)
    pub topo_order: [u32; MAX_SERVICES],
    pub topo_len:   usize,
}

impl SigmaDepGraph {
    pub const fn new() -> Self {
        Self {
            adj_edges:  [(0, 0); MAX_ADJ_EDGES],
            edge_count: 0,
            in_degree:  [0usize; MAX_SERVICES],
            topo_order: [0u32; MAX_SERVICES],
            topo_len:   0,
        }
    }

    /// Build the dependency graph from the service table.
    /// For each service X that depends on Y, we add edge Y→X
    /// (Y must start before X), incrementing in_degree[X].
    pub fn build(&mut self, services: &[ServiceUnit], svc_count: usize) {
        // Reset state
        for i in 0..MAX_SERVICES { self.in_degree[i] = 0; }
        self.edge_count = 0;
        self.topo_len   = 0;

        for x in 0..svc_count {
            for d in 0..services[x].dep_count {
                let y = services[x].deps[d] as usize;
                if y < svc_count && self.edge_count < MAX_ADJ_EDGES {
                    // Edge: y must finish before x starts
                    self.adj_edges[self.edge_count] = (y as u32, x as u32);
                    self.edge_count += 1;
                    self.in_degree[x] += 1;
                }
            }
        }
    }

    /// Run Kahn's BFS topological sort.
    /// Fills `topo_order` with service indices in valid start-order.
    /// Returns false if a cycle is detected (circular dependency).
    pub fn topo_sort(&mut self, svc_count: usize) -> bool {
        // BFS queue (fixed-size ring buffer acting as a queue)
        let mut queue:    [u32; MAX_SERVICES] = [0u32; MAX_SERVICES];
        let mut q_head:   usize = 0;
        let mut q_tail:   usize = 0;
        // Local copy of in-degrees to mutate during sort
        let mut in_deg:   [usize; MAX_SERVICES] = [0usize; MAX_SERVICES];
        for i in 0..svc_count { in_deg[i] = self.in_degree[i]; }

        // Seed queue with all nodes that have in_degree == 0
        for i in 0..svc_count {
            if in_deg[i] == 0 {
                queue[q_tail % MAX_SERVICES] = i as u32;
                q_tail += 1;
            }
        }

        self.topo_len = 0;

        while q_head < q_tail {
            let node = queue[q_head % MAX_SERVICES];
            q_head += 1;

            self.topo_order[self.topo_len] = node;
            self.topo_len += 1;

            // For each edge (node → neighbour), decrement neighbour in_degree
            for e in 0..self.edge_count {
                if self.adj_edges[e].0 == node {
                    let neighbour = self.adj_edges[e].1 as usize;
                    if in_deg[neighbour] > 0 {
                        in_deg[neighbour] -= 1;
                        if in_deg[neighbour] == 0 {
                            queue[q_tail % MAX_SERVICES] = neighbour as u32;
                            q_tail += 1;
                        }
                    }
                }
            }
        }

        // If we processed all nodes, no cycle; otherwise cycle detected
        self.topo_len == svc_count
    }
}

// ── Init System State ─────────────────────────────────────────────────────────
pub struct SigmaInit {
    pub services:           [ServiceUnit; MAX_SERVICES],
    pub service_count:      usize,
    pub current_target:     BootTarget,
    pub boot_time_ns:       U64,
    pub hostname:           [U8; 64],
    pub hostname_len:       usize,
    pub uptime_ns:          U64,
    pub reboot_requested:   bool,
    pub poweroff_requested: bool,
    /// Dependency graph — rebuilt whenever a service is registered
    pub dep_graph:          SigmaDepGraph,
}

static mut INIT: SigmaInit = SigmaInit {
    services:           [ServiceUnit::empty(); MAX_SERVICES],
    service_count:      0,
    current_target:     BootTarget::MultiUser,
    boot_time_ns:       0,
    hostname:           [0u8; 64],
    hostname_len:       0,
    uptime_ns:          0,
    reboot_requested:   false,
    poweroff_requested: false,
    dep_graph:          SigmaDepGraph::new(),
};

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialize the init system (PID 1 entry point).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start(boot_time_ns: U64) -> i32 {
    INIT.boot_time_ns       = boot_time_ns;
    INIT.current_target     = BootTarget::MultiUser;
    INIT.service_count      = 0;
    INIT.reboot_requested   = false;
    INIT.poweroff_requested = false;

    let name = b"sigmaos";
    for (i, &b) in name.iter().enumerate() {
        INIT.hostname[i] = b;
    }
    INIT.hostname_len = name.len();

    sigma_init_register_builtin();

    // Build dependency graph for all registered services
    INIT.dep_graph.build(&INIT.services, INIT.service_count);

    0
}

/// Register built-in essential services and wire their dependencies.
unsafe fn sigma_init_register_builtin() {
    // Service 0: sigma-logd  (must be first — all others depend on it)
    let logd_idx = register_service(
        b"sigma-logd", b"/sbin/sigma-logd",
        ServiceType::Simple, BootTarget::Rescue, RestartPolicy::Always,
    );

    // Service 1: sigma-udevd (device manager; depends on logd)
    let udevd_idx = register_service(
        b"sigma-udevd", b"/sbin/sigma-udevd",
        ServiceType::Simple, BootTarget::Rescue, RestartPolicy::Always,
    );
    if udevd_idx < MAX_SERVICES as u32 {
        INIT.services[udevd_idx as usize].add_dep(logd_idx);
    }

    // Service 2: sigma-networkd (depends on udevd, logd)
    let netd_idx = register_service(
        b"sigma-networkd", b"/sbin/sigma-networkd",
        ServiceType::Notify, BootTarget::MultiUser, RestartPolicy::OnFailure,
    );
    if netd_idx < MAX_SERVICES as u32 {
        INIT.services[netd_idx as usize].add_dep(logd_idx);
        INIT.services[netd_idx as usize].add_dep(udevd_idx);
    }

    // Service 3: sigma-dbus (message bus; depends on logd)
    let dbus_idx = register_service(
        b"sigma-dbus", b"/sbin/sigma-dbus",
        ServiceType::Forking, BootTarget::MultiUser, RestartPolicy::OnFailure,
    );
    if dbus_idx < MAX_SERVICES as u32 {
        INIT.services[dbus_idx as usize].add_dep(logd_idx);
    }

    // Service 4: sigma-sshd (depends on networkd, dbus)
    let sshd_idx = register_service(
        b"sigma-sshd", b"/sbin/sigma-sshd",
        ServiceType::Forking, BootTarget::MultiUser, RestartPolicy::OnFailure,
    );
    if sshd_idx < MAX_SERVICES as u32 {
        INIT.services[sshd_idx as usize].add_dep(netd_idx);
        INIT.services[sshd_idx as usize].add_dep(dbus_idx);
    }

    // Service 5: sigma-crond (depends on dbus)
    let crond_idx = register_service(
        b"sigma-crond", b"/sbin/sigma-crond",
        ServiceType::Simple, BootTarget::MultiUser, RestartPolicy::OnFailure,
    );
    if crond_idx < MAX_SERVICES as u32 {
        INIT.services[crond_idx as usize].add_dep(dbus_idx);
    }

    // Service 6: zenith-desktop (GUI; depends on networkd, dbus, udevd)
    let desk_idx = register_service(
        b"zenith-desktop", b"/sbin/zenith-desktop",
        ServiceType::Notify, BootTarget::Graphical, RestartPolicy::Always,
    );
    if desk_idx < MAX_SERVICES as u32 {
        INIT.services[desk_idx as usize].add_dep(netd_idx);
        INIT.services[desk_idx as usize].add_dep(dbus_idx);
        INIT.services[desk_idx as usize].add_dep(udevd_idx);
    }

    let _ = (logd_idx, udevd_idx, netd_idx, dbus_idx, sshd_idx, crond_idx, desk_idx);
}

unsafe fn register_service(
    name: &[U8], exec: &[U8], svc_type: ServiceType,
    target: BootTarget, restart: RestartPolicy,
) -> u32 {
    if INIT.service_count >= MAX_SERVICES { return u32::MAX; }
    let idx = INIT.service_count;
    let svc = &mut INIT.services[idx];

    let nl = name.len().min(MAX_NAME_LEN);
    for i in 0..nl { svc.name[i] = name[i]; }
    svc.name_len = nl;

    let el = exec.len().min(MAX_CMD_LEN);
    for i in 0..el { svc.exec_start[i] = exec[i]; }
    svc.exec_start_len = el;

    svc.svc_type = svc_type;
    svc.target   = target;
    svc.restart  = restart;
    svc.enabled  = true;
    svc.active   = true;
    svc.state    = ServiceState::Inactive;

    INIT.service_count += 1;
    idx as u32
}

/// Start a service by index, checking deps first.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start_service(svc_idx: U32) -> i32 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return -1; }

    let svc = &mut INIT.services[idx];
    if svc.state == ServiceState::Running { return 0; }
    if svc.state == ServiceState::Masked  { return -2; }

    for d in 0..svc.dep_count {
        let dep_idx = svc.deps[d] as usize;
        if dep_idx < INIT.service_count
            && INIT.services[dep_idx].state != ServiceState::Running
        {
            return -3; // Dependency not yet running
        }
    }

    svc.state      = ServiceState::Starting;
    // Production: fork + exec svc.exec_start here
    svc.state      = ServiceState::Running;
    svc.start_time = INIT.uptime_ns;
    0
}

/// Stop a service by index.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_stop_service(svc_idx: U32) -> i32 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return -1; }

    let svc = &mut INIT.services[idx];
    if svc.state != ServiceState::Running { return -2; }

    svc.state = ServiceState::Stopping;
    // Production: send SIGTERM, wait timeout, then SIGKILL
    svc.state = ServiceState::Stopped;
    0
}

/// Restart a service.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_restart_service(svc_idx: U32) -> i32 {
    sigma_init_stop_service(svc_idx);
    sigma_init_start_service(svc_idx)
}

/// BUG-009 FIX: Bring up all services for a target in DEPENDENCY ORDER
/// using the pre-built topological sort from SigmaDepGraph.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_reach_target(target: U8) -> i32 {
    let target = match target {
        0 => BootTarget::Emergency,
        1 => BootTarget::Rescue,
        2 => BootTarget::MultiUser,
        3 => BootTarget::Graphical,
        4 => BootTarget::Reboot,
        5 => BootTarget::Poweroff,
        _ => return -1,
    };

    INIT.current_target = target;

    match target {
        BootTarget::Reboot => {
            INIT.reboot_requested = true;
            let count = INIT.service_count;
            for i in (0..count).rev() {
                sigma_init_stop_service(i as U32);
            }
            return 0;
        }
        BootTarget::Poweroff => {
            INIT.poweroff_requested = true;
            let count = INIT.service_count;
            for i in (0..count).rev() {
                sigma_init_stop_service(i as U32);
            }
            return 0;
        }
        _ => {}
    }

    // Rebuild dependency graph then sort
    INIT.dep_graph.build(&INIT.services, INIT.service_count);
    let acyclic = INIT.dep_graph.topo_sort(INIT.service_count);

    if !acyclic {
        // Circular dependency detected — fall back to sequential start
        for i in 0..INIT.service_count {
            if INIT.services[i].enabled && (INIT.services[i].target as u8) <= (target as u8) {
                sigma_init_start_service(i as U32);
            }
        }
        return -4; // Signal cycle detected to caller
    }

    // Start services in topological order (respects all `deps` declarations)
    let order_len = INIT.dep_graph.topo_len;
    for pos in 0..order_len {
        let svc_idx = INIT.dep_graph.topo_order[pos];
        let s_idx   = svc_idx as usize;
        if s_idx < INIT.service_count
            && INIT.services[s_idx].enabled
            && (INIT.services[s_idx].target as u8) <= (target as u8)
        {
            sigma_init_start_service(svc_idx);
        }
    }

    0
}

/// Handle service exit (called when a child process exits).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_exited(pid: U32, exit_code: i32) {
    for i in 0..INIT.service_count {
        if INIT.services[i].pid == pid && INIT.services[i].state == ServiceState::Running {
            INIT.services[i].exit_code = exit_code;

            INIT.services[i].state = if exit_code != 0 {
                ServiceState::Failed
            } else {
                ServiceState::Stopped
            };

            let should_restart = match INIT.services[i].restart {
                RestartPolicy::Always    => true,
                RestartPolicy::OnFailure => exit_code != 0,
                RestartPolicy::OnAbnormal => exit_code != 0 && exit_code != 1,
                RestartPolicy::No        => false,
            };

            if should_restart && INIT.services[i].restart_count < INIT.services[i].max_restarts {
                INIT.services[i].restart_count += 1;
                INIT.services[i].state          = ServiceState::Restarting;
                sigma_init_start_service(i as U32);
            }

            break;
        }
    }
}

/// Set hostname.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_set_hostname(name: *const U8, len: usize) -> i32 {
    if name.is_null() || len == 0 || len > 64 { return -1; }
    let src = core::slice::from_raw_parts(name, len);
    for i in 0..len { INIT.hostname[i] = src[i]; }
    INIT.hostname_len = len;
    0
}

/// Get the number of registered services.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_count() -> U32 {
    INIT.service_count as U32
}

/// Get a service's state by index.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_state(svc_idx: U32) -> U8 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return 0; }
    INIT.services[idx].state as U8
}

/// Update system uptime (called from timer tick).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_update_uptime(ns: U64) {
    INIT.uptime_ns = ns;
}

/// Check if reboot is requested.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_reboot_requested() -> i32 {
    if INIT.reboot_requested { 1 } else { 0 }
}

/// Check if poweroff is requested.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_poweroff_requested() -> i32 {
    if INIT.poweroff_requested { 1 } else { 0 }
}

/// Add a dependency: service `svc_idx` depends on `dep_idx`.
/// Call after registering all services, before sigma_init_reach_target.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_add_dep(svc_idx: U32, dep_idx: U32) -> i32 {
    let s = svc_idx as usize;
    let d = dep_idx as usize;
    if s >= INIT.service_count || d >= INIT.service_count { return -1; }
    INIT.services[s].add_dep(dep_idx);
    0
}