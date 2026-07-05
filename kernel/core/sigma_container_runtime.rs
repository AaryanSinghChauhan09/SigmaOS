// SigmaOS — OCI Container Runtime (sigma-pod)
// Sovereign implementation — no external dependencies
// Implements: OCI spec, namespaces, cgroups, image layers, overlay FS
#![no_std]
#![allow(dead_code)]

// ─── OCI Image Spec ──────────────────────────────────────────────────────────
pub const MAX_LAYERS:   usize = 32;
pub const MAX_ENVVARS:  usize = 64;
pub const MAX_MOUNTS:   usize = 16;
pub const MAX_CAPS:     usize = 64;
pub const MAX_CONTAINERS: usize = 256;

pub const OCI_VERSION: &[u8] = b"1.0.0";

// ─── Container Namespaces ────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub struct Namespaces {
    pub pid:  bool,
    pub net:  bool,
    pub mnt:  bool,
    pub uts:  bool,
    pub ipc:  bool,
    pub user: bool,
    pub cgroup: bool,
    pub time: bool,
}

impl Namespaces {
    pub fn all_isolated() -> Self {
        Namespaces { pid:true, net:true, mnt:true, uts:true, ipc:true, user:true, cgroup:true, time:true }
    }
    pub fn host_net() -> Self {
        let mut n = Self::all_isolated();
        n.net = false;  // share host network
        n
    }
}

// ─── Cgroup Resources ────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct CgroupSpec {
    pub cpu_shares:   u32,   // relative weight (default 1024)
    pub cpu_quota_us: i64,   // -1 = unlimited
    pub mem_limit:    u64,   // bytes, 0 = unlimited
    pub mem_swap_limit: u64,
    pub pids_limit:   u32,   // 0 = unlimited
    pub blkio_weight: u16,   // 10–1000
    pub cpu_cpus:     [u8; 16], // CPU affinity set (bitmap)
}

impl CgroupSpec {
    pub const fn default() -> Self {
        CgroupSpec {
            cpu_shares: 1024, cpu_quota_us: -1,
            mem_limit: 0, mem_swap_limit: 0,
            pids_limit: 0, blkio_weight: 500,
            cpu_cpus: [0xFFu8; 16],
        }
    }
    pub const fn minimal() -> Self {
        CgroupSpec { cpu_shares: 512, cpu_quota_us: 100000, mem_limit: 64*1024*1024, ..Self::default() }
    }
}

// ─── OCI Mount ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct OciMount {
    pub destination: [u8; 128],
    pub mount_type:  [u8; 16],
    pub source:      [u8; 128],
    pub options:     [u8; 64],
}

impl OciMount {
    pub const fn empty() -> Self {
        OciMount {
            destination: [0u8; 128], mount_type: [0u8; 16],
            source: [0u8; 128], options: [0u8; 64],
        }
    }
    pub fn proc() -> Self {
        let mut m = Self::empty();
        m.destination[..5].copy_from_slice(b"/proc");
        m.mount_type[..4].copy_from_slice(b"proc");
        m.source[..4].copy_from_slice(b"proc");
        m
    }
    pub fn sysfs() -> Self {
        let mut m = Self::empty();
        m.destination[..4].copy_from_slice(b"/sys");
        m.mount_type[..6].copy_from_slice(b"sysfs");
        m.source[..6].copy_from_slice(b"sysfs");
        m
    }
    pub fn tmpfs(dst: &[u8], size_mb: u32) -> Self {
        let mut m = Self::empty();
        let dlen = dst.len().min(127);
        m.destination[..dlen].copy_from_slice(&dst[..dlen]);
        m.mount_type[..5].copy_from_slice(b"tmpfs");
        m.source[..5].copy_from_slice(b"tmpfs");
        // options: size=NNm
        let size_str = u32_to_ascii(size_mb);
        m.options[..5].copy_from_slice(b"size=");
        m.options[5..5+size_str.1].copy_from_slice(&size_str.0[..size_str.1]);
        m.options[5+size_str.1] = b'm';
        m
    }
}

fn u32_to_ascii(n: u32) -> ([u8;10], usize) {
    let mut buf = [0u8; 10];
    if n == 0 { buf[0] = b'0'; return (buf, 1); }
    let mut v = n;
    let mut i = 0;
    let mut tmp = [0u8; 10];
    while v > 0 { tmp[i] = b'0' + (v % 10) as u8; v /= 10; i += 1; }
    for j in 0..i { buf[j] = tmp[i - 1 - j]; }
    (buf, i)
}

// ─── Image Layer ─────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct ImageLayer {
    pub digest: [u8; 64],  // sha256 hex
    pub size:   u64,
    pub phys:   u64,       // physical address of unpacked layer
    pub ready:  bool,
}

impl ImageLayer {
    pub const fn empty() -> Self {
        ImageLayer { digest: [0u8; 64], size: 0, phys: 0, ready: false }
    }
}

// ─── Container Spec ──────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct ContainerSpec {
    pub id:       [u8; 64],
    pub image:    [u8; 128],
    pub cmd:      [u8; 256],
    pub workdir:  [u8; 128],
    pub envvars:  [[u8; 128]; MAX_ENVVARS],
    pub n_env:    u8,
    pub mounts:   [OciMount; MAX_MOUNTS],
    pub n_mounts: u8,
    pub ns:       Namespaces,
    pub cgroups:  CgroupSpec,
    pub layers:   [ImageLayer; MAX_LAYERS],
    pub n_layers: u8,
    pub readonly_rootfs: bool,
    pub privileged:      bool,
}

impl ContainerSpec {
    pub const fn new() -> Self {
        const EMPTY_ENV: [u8; 128] = [0u8; 128];
        const EMPTY_MNT: OciMount = OciMount::empty();
        const EMPTY_LYR: ImageLayer = ImageLayer::empty();
        ContainerSpec {
            id: [0u8; 64], image: [0u8; 128], cmd: [0u8; 256], workdir: [0u8; 128],
            envvars: [EMPTY_ENV; MAX_ENVVARS], n_env: 0,
            mounts: [EMPTY_MNT; MAX_MOUNTS], n_mounts: 0,
            ns: Namespaces::all_isolated(),
            cgroups: CgroupSpec::default(),
            layers: [EMPTY_LYR; MAX_LAYERS], n_layers: 0,
            readonly_rootfs: true, privileged: false,
        }
    }
}

// ─── Container State ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ContainerState { Created, Running, Paused, Stopped, Deleted }

#[derive(Clone, Copy)]
pub struct Container {
    pub spec:     ContainerSpec,
    pub state:    ContainerState,
    pub pid:      u32,         // init PID inside container
    pub host_pid: u32,         // corresponding host PID
    pub exit_code: i32,
    pub created_ns: u64,       // nanoseconds since boot
    pub started_ns: u64,
    pub finished_ns: u64,
}

impl Container {
    pub const fn new(spec: ContainerSpec) -> Self {
        Container {
            spec, state: ContainerState::Created,
            pid: 0, host_pid: 0, exit_code: 0,
            created_ns: 0, started_ns: 0, finished_ns: 0,
        }
    }
    pub fn is_running(&self) -> bool { self.state == ContainerState::Running }
    pub fn uptime_ns(&self, now_ns: u64) -> u64 {
        if self.started_ns == 0 { 0 } else { now_ns.saturating_sub(self.started_ns) }
    }
}

// ─── Container Runtime ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy)]
pub enum RuntimeError {
    TooManyContainers, NotFound, AlreadyRunning, NotRunning,
    InvalidSpec, LayerError, NamespaceError, CgroupError,
}

pub struct ContainerRuntime {
    pub containers: [Container; MAX_CONTAINERS],
    pub count:      usize,
    pub started:    u64,
}

impl ContainerRuntime {
    pub const fn new() -> Self {
        const EMPTY: Container = Container::new(ContainerSpec::new());
        ContainerRuntime { containers: [EMPTY; MAX_CONTAINERS], count: 0, started: 0 }
    }

    pub fn create(&mut self, spec: ContainerSpec) -> Result<usize, RuntimeError> {
        if self.count >= MAX_CONTAINERS { return Err(RuntimeError::TooManyContainers); }
        let idx = self.count;
        self.containers[idx] = Container::new(spec);
        self.containers[idx].created_ns = self.started;
        self.count += 1;
        Ok(idx)
    }

    pub fn start(&mut self, idx: usize, now_ns: u64) -> Result<u32, RuntimeError> {
        if idx >= self.count { return Err(RuntimeError::NotFound); }
        let c = &mut self.containers[idx];
        if c.state == ContainerState::Running { return Err(RuntimeError::AlreadyRunning); }
        c.state      = ContainerState::Running;
        c.started_ns = now_ns;
        c.host_pid   = 1000 + idx as u32; // stub PID assignment
        c.pid        = 1;
        Ok(c.host_pid)
    }

    pub fn stop(&mut self, idx: usize, now_ns: u64, exit_code: i32) -> Result<(), RuntimeError> {
        if idx >= self.count { return Err(RuntimeError::NotFound); }
        let c = &mut self.containers[idx];
        if c.state != ContainerState::Running { return Err(RuntimeError::NotRunning); }
        c.state       = ContainerState::Stopped;
        c.finished_ns = now_ns;
        c.exit_code   = exit_code;
        Ok(())
    }

    pub fn pause(&mut self, idx: usize) -> Result<(), RuntimeError> {
        if idx >= self.count { return Err(RuntimeError::NotFound); }
        if self.containers[idx].state != ContainerState::Running {
            return Err(RuntimeError::NotRunning);
        }
        self.containers[idx].state = ContainerState::Paused;
        Ok(())
    }

    pub fn resume(&mut self, idx: usize) -> Result<(), RuntimeError> {
        if idx >= self.count { return Err(RuntimeError::NotFound); }
        if self.containers[idx].state != ContainerState::Paused {
            return Err(RuntimeError::NotRunning);
        }
        self.containers[idx].state = ContainerState::Running;
        Ok(())
    }

    pub fn delete(&mut self, idx: usize) -> Result<(), RuntimeError> {
        if idx >= self.count { return Err(RuntimeError::NotFound); }
        let c = &mut self.containers[idx];
        if c.state == ContainerState::Running { return Err(RuntimeError::AlreadyRunning); }
        c.state = ContainerState::Deleted;
        Ok(())
    }

    pub fn running_count(&self) -> usize {
        self.containers[..self.count].iter().filter(|c| c.is_running()).count()
    }

    pub fn find_by_id(&self, id: &[u8]) -> Option<usize> {
        for (i, c) in self.containers[..self.count].iter().enumerate() {
            let ilen = id.len().min(64);
            if &c.spec.id[..ilen] == &id[..ilen] { return Some(i); }
        }
        None
    }
}

// ─── CRI (Container Runtime Interface) for Kubernetes ────────────────────────
pub struct CriRuntime {
    pub runtime: ContainerRuntime,
    pub pods:    [PodSandbox; 64],
    pub n_pods:  usize,
}

#[derive(Clone, Copy)]
pub struct PodSandbox {
    pub id:          [u8; 64],
    pub namespace:   [u8; 64],
    pub name:        [u8; 64],
    pub uid:         [u8; 64],
    pub containers:  [usize; 16],  // container indices
    pub n_containers: u8,
    pub state:       PodState,
    pub pod_ip:      u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PodState { Ready, NotReady, Terminated }

impl PodSandbox {
    pub const fn new() -> Self {
        PodSandbox {
            id: [0u8; 64], namespace: [0u8; 64], name: [0u8; 64], uid: [0u8; 64],
            containers: [0usize; 16], n_containers: 0,
            state: PodState::NotReady, pod_ip: 0,
        }
    }
}

impl CriRuntime {
    pub const fn new() -> Self {
        const EMPTY_POD: PodSandbox = PodSandbox::new();
        CriRuntime {
            runtime: ContainerRuntime::new(),
            pods: [EMPTY_POD; 64], n_pods: 0,
        }
    }
    pub fn run_pod_sandbox(&mut self) -> Option<usize> {
        if self.n_pods >= 64 { return None; }
        let idx = self.n_pods;
        self.pods[idx] = PodSandbox::new();
        self.pods[idx].state = PodState::NotReady;
        self.n_pods += 1;
        Some(idx)
    }
    pub fn pod_ready_count(&self) -> usize {
        self.pods[..self.n_pods].iter().filter(|p| p.state == PodState::Ready).count()
    }
}
