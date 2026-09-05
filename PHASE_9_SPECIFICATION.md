# SigmaOS Phase 9: Advanced Namespace & Compatibility Features

**Version**: Draft v0.9 Planning Document
**Status**: Ready for Approval
**Target Release**: v0.9 (8-12 weeks)
**Scope**: Extend Phase 8 with additional Linux/BSD comparable features

---

## Overview

Phase 9 extends the v0.8 feature set with advanced namespace types and Linux/BSD compatibility features that were not included in Phase 8 Tier 1.

### Phase 8 Summary (Completed)
- ✅ PID Namespaces
- ✅ IPC Namespaces  
- ✅ Mount Namespaces
- ✅ File Monitoring (inotify-like)
- ✅ Resource Limits (cgroups v2-like)
- ✅ Security Framework (seccomp-like)
- ✅ Event System (kqueue-like)

### Phase 9 Goals
- 🎯 UTS Namespace (hostname/domainname isolation)
- 🎯 Network Namespace (network stack isolation)
- 🎯 User Namespace (UID/GID mapping)
- 🎯 eBPF support (in-kernel programmable hooks)
- 🎯 Extended cgroups controllers (device, hugetlb, etc.)
- 🎯 Advanced syscall filtering (BPF-based)

---

## Feature Breakdown

### 9.1: UTS Namespace Implementation (20 hours)

**Purpose**: Isolate hostname, domainname, and other UTS properties per namespace

**Linux Parity**: 
- `CLONE_NEWUTS` flag
- `/proc/sys/kernel/hostname` per-namespace
- `sethostname()`/`gethostname()` isolation

**BSD Parity**:
- FreeBSD jail hostname isolation
- OpenBSD pledge `unveil` hostname masking

**Subtasks**:
- Create `src/kernel/uts_namespace.rs`
- Define `UtsNamespace` struct with hostname, domainname, nodename
- Implement hostname isolation per namespace
- Implement `sys_sethostname()` and `sys_gethostname()`
- Integration with PID namespace hierarchy
- Comprehensive tests

**Acceptance Criteria**:
- Processes in different UTS namespaces see different hostnames
- Hostname changes don't affect other namespaces
- CLONE_NEWUTS flag works
- 0 compilation errors
- 20+ tests passing

---

### 9.2: Network Namespace Implementation (25 hours)

**Purpose**: Isolate network stack (interfaces, routes, firewall rules) per namespace

**Linux Parity**:
- `CLONE_NEWNET` flag
- Per-namespace network interfaces
- Per-namespace routing tables
- Per-namespace firewall rules
- `ip netns` command compatibility

**BSD Parity**:
- FreeBSD jail network isolation
- OpenBSD `rtable` (routing table isolation)
- Network interface namespace

**Subtasks**:
- Create `src/net/network_namespace.rs`
- Define `NetworkNamespace` struct
- Per-namespace interface registry
- Per-namespace routing table
- Per-namespace firewall rules
- Network interface virtualization
- Namespace bridge network connectivity
- Integration with existing network stack
- Comprehensive tests

**Acceptance Criteria**:
- Network interfaces isolated per namespace
- Routing tables independent
- Firewall rules per-namespace
- Namespace communication via virtual bridge
- 0 compilation errors
- 25+ tests passing

---

### 9.3: User Namespace Implementation (20 hours)

**Purpose**: Map UID/GID ranges between host and namespace

**Linux Parity**:
- `CLONE_NEWUSER` flag
- UID/GID mapping files
- `subuid` and `subgid` support
- User namespace hierarchies
- Capability mapping

**BSD Parity**:
- FreeBSD jail UID mapping
- OpenBSD pledge user isolation

**Subtasks**:
- Create `src/security/user_namespace.rs`
- Define `UserNamespace` struct with UID/GID maps
- Implement UID/GID range mapping
- Implement `subuid`/`subgid` parsing
- Capability per-namespace management
- User namespace inheritance
- Integration with process management
- Comprehensive tests

**Acceptance Criteria**:
- UID/GID mapping works correctly
- Hierarchical user namespaces supported
- Capabilities mapped per-namespace
- 0 compilation errors
- 20+ tests passing

---

### 9.4: eBPF Support Implementation (30 hours)

**Purpose**: In-kernel programmable hooks for advanced filtering and monitoring

**Linux Parity**:
- eBPF VM for syscall filtering (seccomp-BPF)
- eBPF programs for network filtering
- eBPF hooks in event system
- BPF to BPF calls
- BPF helper functions

**BSD Parity**:
- BPF capture filters
- BPF packet filtering (OpenBSD pf)

**Subtasks**:
- Create `src/kernel/ebpf_vm.rs` — eBPF interpreter
- Define `BpfProgram`, `BpfVm` structs
- Implement BPF instruction set (subset for security)
- BPF register model and memory
- Helper function registration
- BPF program verification
- BPF program caching
- Integration with seccomp (BPF-based filtering)
- Integration with network stack
- Integration with event system
- BPF program loading syscall
- Comprehensive tests

**Acceptance Criteria**:
- BPF programs compile and load
- BPF instructions execute correctly
- Helper functions available
- Program verification works
- Filtering uses BPF backend
- 0 compilation errors
- 30+ tests passing

---

### 9.5: Extended Cgroups Controllers (20 hours)

**Purpose**: Add device, hugetlb, and other controllers beyond v2 base

**Linux Parity**:
- Device controller (allow/deny device access)
- Hugetlb controller (huge page allocation)
- Rdma controller (RDMA resource limits)
- Pids controller (process count limits)
- Net_cls controller (traffic classification)

**BSD Parity**:
- RCTL (resource control) limits
- Memory page size policies

**Subtasks**:
- Create `src/kernel/cgroup_controllers.rs`
- Define device controller
- Define hugetlb controller
- Define rdma controller
- Define pids controller
- Define net_cls controller
- Implement device access policy
- Implement allocation limits
- Cgroup controller registration
- Integration with hierarchy
- Comprehensive tests

**Acceptance Criteria**:
- All controllers defined
- Device access enforced
- Allocation limits respected
- 0 compilation errors
- 25+ tests passing

---

### 9.6: Advanced Syscall Filtering with BPF (20 hours)

**Purpose**: BPF-based syscall filtering with complex rules

**Linux Parity**:
- seccomp-BPF with complex conditions
- Syscall argument inspection
- Return value assignment
- BPF program chaining

**BSD Parity**:
- OpenBSD pledge syscall filtering
- Filter condition chains

**Subtasks**:
- Extend `src/security/seccomp.rs` for BPF
- BPF program loading for syscall filtering
- Argument inspection helpers
- Condition building API
- Return value assignment
- Program chaining
- Integration with security manager
- Comprehensive tests

**Acceptance Criteria**:
- BPF-based filtering works
- Complex conditions supported
- Argument inspection works
- Return values assigned
- 0 compilation errors
- 20+ tests passing

---

## Implementation Order (DAG)

```
Phase 9 START
├── 9.1: UTS Namespace (20 hours)
│   └── Depends on: Phase 8.1 (Namespaces)
│
├── 9.2: Network Namespace (25 hours)
│   └── Depends on: Phase 8.1 + 9.1
│
├── 9.3: User Namespace (20 hours)
│   └── Depends on: Phase 8.4 (Security)
│
├── 9.4: eBPF Support (30 hours)
│   └── Depends on: Phase 8.4 + 8.5 (Security + Events)
│
├── 9.5: Extended cgroups (20 hours)
│   └── Depends on: Phase 8.3 (Resource Limits)
│
└── 9.6: Advanced Syscall Filtering (20 hours)
    └── Depends on: 9.4 (eBPF) + Phase 8.4
```

**Sequential path**: 9.1 → 9.2 → 9.3 → 9.4 → 9.5 → 9.6
**Total effort**: 155 hours
**Estimated duration**: 8-12 weeks (assuming parallel team work)

---

## Success Criteria

### Code Quality
- ✅ Zero compilation errors
- ✅ 200+ new tests (all passing)
- ✅ Thread-safe implementations (Arc/Mutex)
- ✅ Memory-safe (100% Rust)
- ✅ >80% code coverage

### Linux/BSD Parity
- ✅ UTS namespace: 100% Linux/BSD compatible
- ✅ Network namespace: 100% Linux/BSD compatible  
- ✅ User namespace: 100% Linux/BSD compatible
- ✅ eBPF: Full compatibility with Linux eBPF subset
- ✅ Cgroups: v2 extended controllers support
- ✅ Syscall filtering: BPF backend operational

### Documentation
- ✅ Wiki pages for each feature
- ✅ API reference documentation
- ✅ Code examples for each feature
- ✅ Release notes for v0.9

### Repository
- ✅ All changes pushed to GitHub
- ✅ Clean main branch
- ✅ No redundant branches
- ✅ v0.9 release tag created

---

## Deliverables

### Code (165+ hours → ~6,600+ LOC)
```
src/kernel/
├── uts_namespace.rs        (1,200+ LOC)
├── ebpf_vm.rs              (2,000+ LOC)
└── cgroup_controllers.rs   (1,400+ LOC)

src/net/
└── network_namespace.rs    (1,500+ LOC)

src/security/
├── user_namespace.rs       (1,200+ LOC)
└── seccomp_ebpf.rs         (800+ LOC)

tests/
├── uts_namespace_tests.rs
├── network_namespace_tests.rs
├── user_namespace_tests.rs
├── ebpf_tests.rs
├── cgroup_tests.rs
└── phase9_integration_tests.rs
```

### Documentation
```
wiki_content/
├── UTS-Namespace.md
├── Network-Namespace.md
├── User-Namespace.md
├── eBPF.md
├── Advanced-Cgroups.md
└── Advanced-Syscall-Filtering.md

RELEASE_NOTES_v0.9.md
API_DOCUMENTATION_v0.9.md
```

---

## Decision Points

**For User Review**:

1. **Scope**: Should Phase 9 include all 6 features, or prioritize subset?
   - Option A: All 6 features (155 hours, 8-12 weeks)
   - Option B: Core 3 (UTS, Network, User) first (65 hours, 4-6 weeks), then eBPF/Cgroups in Phase 10

2. **eBPF Approach**: Full interpreter or simplified subset?
   - Option A: Full eBPF interpreter (complex, complete)
   - Option B: Simplified subset for security use cases only (simpler, faster)

3. **Network Namespace**: Virtual bridge implementation?
   - Option A: Full virtual bridge (complex)
   - Option B: Simple veth pair model (simpler)

---

## Next Steps

Once you approve this specification:
1. Create detailed spec files (requirements.md, design.md, tasks.md)
2. Begin implementation following DAG order
3. Maintain the same quality standards as Phase 8
4. Regular progress reporting

---

**Status**: Ready for your approval
**Questions**: See Decision Points section above

