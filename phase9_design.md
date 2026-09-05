# Phase 9 Design: Advanced Namespace & Compatibility Features

**Version**: 1.0
**Status**: APPROVED
**Architecture**: Modular, following Phase 8 patterns

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                   SigmaOS Phase 9 Stack                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Advanced Syscall Filtering (F6)                        │
│  ├─ seccomp_ebpf.rs                                    │
│  └─ BPF integration layer                              │
│                                                         │
│  eBPF Virtual Machine (F4)                              │
│  ├─ ebpf_vm.rs (interpreter)                           │
│  ├─ BPF instruction set                                │
│  ├─ Helper function registry                           │
│  └─ Program verification & caching                     │
│                                                         │
│  Extended Cgroups (F5)                                  │
│  ├─ cgroup_controllers.rs                              │
│  ├─ Device controller                                  │
│  ├─ Hugetlb controller                                 │
│  └─ Pids/Rdma/NetCls controllers                       │
│                                                         │
│  User Namespace (F3)                                    │
│  ├─ user_namespace.rs                                  │
│  ├─ UID/GID mapping tables                             │
│  ├─ subuid/subgid support                              │
│  └─ Capability isolation                               │
│                                                         │
│  Network Namespace (F2)                                 │
│  ├─ network_namespace.rs                               │
│  ├─ Per-namespace interface registry                   │
│  ├─ Routing tables                                     │
│  ├─ Virtual bridge (vbrx)                              │
│  └─ Firewall rules                                     │
│                                                         │
│  UTS Namespace (F1)                                     │
│  ├─ uts_namespace.rs                                   │
│  ├─ Hostname storage                                   │
│  ├─ Domainname storage                                 │
│  └─ sys_sethostname/gethostname                        │
│                                                         │
│  Phase 8 Foundation                                     │
│  └─ Namespaces, File Monitor, Resource Limits, Security, Events
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Module Design

### 1. UTS Namespace (src/kernel/uts_namespace.rs)

**Structure**:
```rust
pub struct UtsNamespace {
    id: NamespaceId,
    hostname: String,
    domainname: String,
    nodename: String,
    release: String,
    version: String,
    machine: String,
    parent_ns: Option<NamespaceId>,
}

pub struct UtsNamespaceManager {
    namespaces: Arc<Mutex<HashMap<NamespaceId, Arc<UtsNamespace>>>>,
    id_counter: Arc<AtomicU64>,
}
```

**Key Functions**:
- `create_uts_namespace(parent: Option<NamespaceId>) -> NamespaceId`
- `set_hostname(ns_id, hostname) -> Result`
- `get_hostname(ns_id) -> String`
- `sys_sethostname(fd, hostname_ptr, len) -> i32`
- `sys_gethostname(fd, hostname_ptr, len) -> i32`

**Integration Points**:
- Register with existing namespace manager
- Support CLONE_NEWUTS flag
- Link with PID namespace hierarchy

---

### 2. Network Namespace (src/net/network_namespace.rs)

**Structure**:
```rust
pub struct NetworkNamespace {
    id: NamespaceId,
    interfaces: Arc<Mutex<HashMap<String, NetworkInterface>>>,
    routing_table: Arc<Mutex<Vec<Route>>>,
    firewall_rules: Arc<Mutex<Vec<FirewallRule>>>,
    virtual_bridge: Arc<Mutex<VirtualBridge>>,
    parent_ns: Option<NamespaceId>,
}

pub struct NetworkInterface {
    name: String,
    ip_addr: IpAddr,
    gateway: IpAddr,
    mtu: u16,
    flags: u32,
}

pub struct VirtualBridge {
    name: String,
    namespaces: Vec<NamespaceId>,
    veth_pairs: HashMap<(NamespaceId, NamespaceId), (u32, u32)>,
}
```

**Key Functions**:
- `create_network_namespace(parent: Option<NamespaceId>) -> NamespaceId`
- `add_interface(ns_id, iface) -> Result`
- `add_route(ns_id, route) -> Result`
- `add_firewall_rule(ns_id, rule) -> Result`
- `create_veth_pair(ns1, ns2) -> Result<(fd1, fd2)>`
- `sys_socket(ns_id, domain, type, proto) -> i32`

**Virtual Bridge Design**:
```rust
pub struct VirtualBridge {
    // Connects multiple network namespaces
    // Automatically forwards packets between peer namespaces
    // Maintains veth device pairs for communication
}
```

**Integration Points**:
- Register with namespace manager
- Support CLONE_NEWNET flag
- Integrate with existing socket layer
- Per-namespace firewall integration

---

### 3. User Namespace (src/security/user_namespace.rs)

**Structure**:
```rust
pub struct UserNamespace {
    id: NamespaceId,
    uid_map: Arc<Mutex<Vec<(u32, u32, u32)>>>, // inner, outer, count
    gid_map: Arc<Mutex<Vec<(u32, u32, u32)>>>,
    capabilities: Arc<Mutex<HashMap<u32, CapabilitySet>>>,
    parent_ns: Option<NamespaceId>,
}

pub struct CapabilitySet {
    inheritable: u64,
    permitted: u64,
    effective: u64,
}
```

**Key Functions**:
- `create_user_namespace(parent: Option<NamespaceId>) -> NamespaceId`
- `set_uid_map(ns_id, map_str) -> Result`
- `set_gid_map(ns_id, map_str) -> Result`
- `parse_subuid(path) -> Result<Vec<(uid, start, count)>>`
- `parse_subgid(path) -> Result<Vec<(gid, start, count)>>`
- `map_uid_host_to_ns(ns_id, host_uid) -> u32`
- `map_uid_ns_to_host(ns_id, ns_uid) -> u32`

**Integration Points**:
- Register with namespace manager
- Support CLONE_NEWUSER flag
- Integrate with capability system
- Per-process UID/GID translation

---

### 4. eBPF Virtual Machine (src/kernel/ebpf_vm.rs)

**Structure**:
```rust
pub struct BpfVm {
    registers: [u64; 11], // R0-R10
    stack: Vec<u8>,
    heap: HashMap<u64, Vec<u8>>,
    pc: u64,
}

pub enum BpfInstruction {
    Load64Imm { dst: u8, imm: u64 },
    Load64Mem { dst: u8, src: u8, offset: i16 },
    Store64Mem { dst: u8, src: u8, offset: i16 },
    Arithmetic { op: u8, dst: u8, src: u8 },
    Jump { op: u8, dst: u8, src: u8, off: i16, imm: u32 },
    Call { func_id: u32 },
    Return,
}

pub struct BpfProgram {
    instructions: Vec<BpfInstruction>,
    entry_point: u64,
    verified: bool,
    cached_result: Option<u64>,
}

pub struct BpfHelper {
    id: u32,
    func: Box<dyn Fn(&mut BpfVm) -> u64 + Send + Sync>,
}
```

**Key Functions**:
- `create_bpf_vm() -> BpfVm`
- `load_bpf_program(bytes) -> Result<BpfProgram>`
- `verify_bpf_program(program) -> Result`
- `execute_bpf(program, context) -> u64`
- `register_bpf_helper(id, func) -> Result`
- `sys_bpf(cmd, attr, size) -> i32`

**Helper Functions** (registered):
- `bpf_probe_read(vm, addr, len) -> i64`
- `bpf_ktime_get_ns(vm) -> u64`
- `bpf_get_current_pid(vm) -> u64`
- `bpf_get_current_uid(vm) -> u32`
- `bpf_get_sysctl(vm, name) -> u64`

**Instruction Execution**:
```rust
fn execute_instruction(&mut self, instr: &BpfInstruction) -> Result<bool> {
    match instr {
        Load64Imm { dst, imm } => { self.registers[*dst as usize] = *imm; },
        Call { func_id } => { /* call helper */ },
        Jump { ... } => { /* conditional jump */ },
        Return => { return Ok(false); }, // Stop execution
        // ...
    }
    Ok(true)
}
```

**Integration Points**:
- Integration with seccomp (F6 uses BPF)
- Integration with event system (BPF hooks)
- Integration with network stack (BPF filters)
- Syscall: `sys_bpf()` for program loading

---

### 5. Extended Cgroups Controllers (src/kernel/cgroup_controllers.rs)

**Structure**:
```rust
pub trait CgroupController: Send + Sync {
    fn name(&self) -> &str;
    fn setup(&mut self, cg: &mut Cgroup) -> Result<()>;
    fn apply_limits(&self, cg: &Cgroup) -> Result<()>;
    fn get_stats(&self, cg: &Cgroup) -> Result<ControllerStats>;
}

pub struct DeviceController {
    rules: HashMap<(u32, u32), AccessMode>,
}

pub struct HugetlbController {
    page_sizes: HashMap<u64, HugetlbLimit>,
}

pub struct RdmaController {
    limits: HashMap<String, u64>,
}

pub struct PidsController {
    max_pids: Option<u64>,
    current_pids: AtomicU64,
}

pub struct NetClsController {
    classid: Option<u32>,
}
```

**Key Functions**:
- `register_cgroup_controller(name, controller) -> Result`
- `apply_device_limits(cg, rules) -> Result`
- `apply_hugetlb_limits(cg, page_size, limit) -> Result`
- `apply_pids_limits(cg, max) -> Result`
- `get_device_stats(cg) -> Result<DeviceStats>`

**Integration Points**:
- Register with cgroups v2 hierarchy (Phase 8)
- Extend existing controller interface
- Per-process enforcement
- Per-cgroup statistics

---

### 6. Advanced Syscall Filtering (src/security/seccomp_ebpf.rs)

**Structure**:
```rust
pub struct BpfSeccompFilter {
    program: Arc<BpfProgram>,
    default_action: SeccompAction,
}

pub struct SeccompManager {
    filters: Arc<Mutex<HashMap<u32, BpfSeccompFilter>>>,
}

pub enum SeccompAction {
    Allow,
    Deny(i32),
    Kill,
    Trace,
    Log,
}
```

**Key Functions**:
- `load_bpf_seccomp_filter(pid, program) -> Result`
- `check_syscall_bpf(filter, syscall_id, args) -> SeccompAction`
- `sys_seccomp(op, flags, uargs) -> i32`
- `build_bpf_filter_chain(rules) -> BpfProgram`

**Integration Points**:
- Uses eBPF VM (F4)
- Extends seccomp framework (Phase 8)
- Per-process filter application
- Syscall interception hooks

---

## Syscall Additions

| Syscall | Involved Features |
|---------|------------------|
| `sys_sethostname(2)` | F1 (UTS) |
| `sys_gethostname(2)` | F1 (UTS) |
| `sys_socket(2)` w/ NEWNET | F2 (Network) |
| `sys_sethostname()` per-NS | F1 (UTS) |
| `sys_clone(CLONE_NEWUTS)` | F1 (UTS) |
| `sys_clone(CLONE_NEWNET)` | F2 (Network) |
| `sys_clone(CLONE_NEWUSER)` | F3 (User) |
| `sys_bpf()` | F4 (eBPF) |
| `sys_seccomp()` w/ BPF | F6 (Advanced) |

---

## Data Flow: Process Isolation

```
New Process (clone with CLONE_NEWUTS | CLONE_NEWNET | CLONE_NEWUSER)
│
├─ PID Namespace (Phase 8)
│
├─ UTS Namespace (F1)
│  ├─ hostname = "container1"
│  ├─ domainname = "local"
│  └─ isolated from parent
│
├─ Network Namespace (F2)
│  ├─ veth0 = 10.0.0.2
│  ├─ eth0 = <virtual>
│  ├─ routing table (isolated)
│  └─ firewall rules (isolated)
│
├─ User Namespace (F3)
│  ├─ UID mapping: ns:0 -> host:1000
│  ├─ GID mapping: ns:0 -> host:1000
│  └─ capabilities (isolated)
│
└─ Security (Phase 8 + F4 eBPF + F6 Advanced)
   ├─ seccomp-BPF filter
   ├─ eBPF hooks
   └─ Allowed syscalls determined by BPF program
```

---

## Thread Safety

**All shared state protected by Arc<Mutex<>>**:
- `UtsNamespaceManager`
- `NetworkNamespace` (interfaces, routing, firewall)
- `UserNamespace` (UID/GID maps, capabilities)
- `BpfVm` (registers, stack, heap during execution)
- `BpfProgram` (caching)
- Extended controller state

---

## Testing Strategy

### Unit Tests
- Namespace creation/deletion
- UID/GID mapping validation
- BPF instruction execution
- Controller limit enforcement
- Syscall filtering logic

### Integration Tests
- Multi-namespace isolation
- Cross-namespace communication via bridge
- eBPF program loading and execution
- Syscall filtering with BPF
- Combined namespace + security scenarios

### Performance Tests
- Namespace create/switch latency
- BPF program execution overhead
- Controller limit enforcement overhead
- Memory usage benchmarks

---

## Performance Targets

| Operation | Target | Method |
|-----------|--------|--------|
| Namespace create | < 2ms | Atomic allocation |
| Namespace switch | < 1ms | Direct context swap |
| BPF execution | < 1μs | VM instruction cache |
| Controller overhead | < 0.1% | Lazy evaluation |
| Memory per namespace | < 100KB | Minimal state |

---

## Known Limitations & Future Work

**v0.9 Scope**:
- Full eBPF interpreter (no JIT)
- Virtual bridge (single subnet)
- Basic controller stats
- Static UID/GID maps

**v1.0+ Future**:
- eBPF JIT compilation
- Advanced bridging (VLAN, trunking)
- Per-namespace procfs
- Dynamic controller reconfiguration
- Network Quality of Service (QoS)
- Cgroup v3 unified hierarchy

