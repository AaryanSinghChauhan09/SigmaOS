# SigmaOS Phase 9 Core - Design

**Architecture**: Advanced Linux/BSD Namespaces & Security
**Design Pattern**: Per-namespace managers with Arc<RwLock<>>/Arc<Mutex<>>
**Test Strategy**: Comprehensive unit + integration + performance tests
**Integration**: Builds on Phase 8 (v0.8) foundation

---

## System Architecture

### Namespace Managers (Thread-Safe Global Singletons)

```
UtsNamespaceManager
├── HashMap<UtsNamespaceId, Arc<Mutex<UtsNamespace>>>
└── Arc<RwLock<>> for manager lock

NetworkNamespaceManager
├── HashMap<NetworkNamespaceId, Arc<Mutex<NetworkNamespace>>>
├── Virtual bridge registry
└── Arc<RwLock<>> for manager lock

UserNamespaceManager (TO IMPLEMENT)
├── HashMap<UserNamespaceId, Arc<Mutex<UserNamespace>>>
└── Arc<RwLock<>> for manager lock
```

### Module Organization

```
src/
├── kernel/
│   ├── uts_namespace.rs ✅
│   ├── ebpf_vm.rs (TODO)
│   └── cgroup_controllers.rs (TODO)
├── net/
│   ├── network_namespace.rs ✅
│   ├── virtual_bridge.rs ✅
│   ├── network_syscalls.rs ✅
│   └── advanced_routing.rs (TODO)
├── security/
│   ├── user_namespace.rs (TODO)
│   └── seccomp_ebpf.rs (TODO)
└── syscall/
    ├── uts_syscalls.rs ✅
    ├── user_syscalls.rs (TODO)
    ├── bpf_syscalls.rs (TODO)
    └── cgroup_syscalls.rs (TODO)
```

---

## Phase 9.2.4: Network NS Integration Tests Design

### Test Categories

1. **Isolation Verification Tests**
   - Multiple namespace creation
   - Interface isolation
   - Route isolation
   - Firewall rule isolation

2. **Bridge Connectivity Tests**
   - Veth pair creation
   - Bridge forwarding
   - Cross-namespace communication
   - State management

3. **Multi-Namespace Scenarios**
   - 3+ namespaces interacting
   - Complex routing topologies
   - Firewall rule enforcement
   - Socket communication

4. **Performance Tests**
   - Operation latency < 100µs
   - Throughput benchmarks
   - Memory usage tracking

### Test File Structure

```
tests/network_ns_integration_tests.rs
├── integration_tests module
├── 15+ test functions
└── Helper functions for setup/teardown
```

---

## Phase 9.3: User Namespace Design

### Core Structures

```rust
UserNamespace {
    id: UserNamespaceId,
    uid_map: Vec<UidGidMapping>,
    gid_map: Vec<UidGidMapping>,
    capabilities: CapabilitySet,
    owner_uid: u32,
}

UidGidMapping {
    container_id: u32,  // Start in namespace
    host_id: u32,       // Start on host
    count: u32,         // Range size
}

SubuidEntry {
    user: String,
    start_uid: u32,
    count: u32,
}
```

### Mapping Model

```
Host UID 0-65535
    ↓
User Namespace (via mapping)
    ↓
NS UID 0-65535

Example:
Host UID 100000-165535 → NS UID 0-65535
Host UID 200000 → NS UID 100000
```

### Capabilities (Linux CAP_* constants)

- CAP_CHOWN (0)
- CAP_DAC_OVERRIDE (1)
- CAP_DAC_READ_SEARCH (2)
- CAP_FOWNER (3)
- CAP_FSETID (4)
- ... (up to CAP_BLOCK_SUSPEND, CAP_AUDIT_READ, etc.)

---

## Phase 9.4: eBPF VM Design

### Instruction Set Architecture

```
64-bit Registers (R0-R10):
- R0: Return value
- R1-R5: Function arguments
- R6-R9: Callee-saved
- R10: Stack frame pointer

Instruction Format (64-bit):
[opcode(8)] [dst(4)] [src(4)] [offset(16)] [imm(32)]

Memory:
- Stack: 512 bytes (R10-based)
- Heap: Dynamic
- Memory-mapped I/O: Custom regions
```

### Instruction Categories

1. **ALU Instructions**: Add, Sub, Mul, Div, Mod, And, Or, Xor, Shift
2. **Load/Store**: LdImm64, LdReg, StReg, StImm
3. **Jumps**: Ja, Jeq, Jne, Jgt, Jlt (20+ variants)
4. **Function Calls**: Call (helper functions), Return
5. **Special**: Nop, Exit

### VM Execution Model

```
Load program → Validate → Initialize → Execute loop → Return R0

Execute loop:
  1. Fetch instruction at PC
  2. Decode instruction
  3. Execute (modify registers/memory)
  4. Increment PC
  5. Check for Return or error
  6. Repeat
```

---

## Thread Safety Pattern

All shared state uses:
- **Arc<RwLock<>>** for read-heavy workloads (namespace managers)
- **Arc<Mutex<>>** for write-heavy workloads (namespace state)

```rust
Arc<RwLock<HashMap<NamespaceId, Arc<Mutex<NamespaceStruct>>>>>
       ↑                                    ↑
    Manager lock                      Namespace lock
```

---

## Error Handling Pattern

All fallible operations return `Result<T, String>`:

```rust
pub fn operation() -> Result<T, String> {
    // ... 
    if error_condition {
        return Err("Descriptive error message".to_string());
    }
    Ok(value)
}
```

---

## Testing Strategy

### Unit Tests
- Individual function correctness
- State transitions
- Edge cases
- Error conditions

### Integration Tests
- Multi-component interactions
- End-to-end scenarios
- Cross-namespace operations
- Bridge forwarding

### Performance Tests
- Operation latency
- Memory usage
- Throughput
- Scalability

### Property-Based Tests (Where Applicable)
- Isolation properties
- Mapping consistency
- State machine validity

---

## Implementation Order

1. **Phase 9.2.4**: Integration tests (leverages existing NS infrastructure)
2. **Phase 9.3**: User namespace (simpler than eBPF, builds isolation concepts)
3. **Phase 9.4**: eBPF VM (complex, but independent subsystem)
4. **Phase 9.5-9.7**: Extensions and integration

---

## Key Design Decisions

✅ **Decision**: Per-namespace managers instead of global state
- Rationale: Easier testing, clearer semantics, better scalability

✅ **Decision**: Arc<Mutex<>> instead of lock-free
- Rationale: Rust conventions, acceptable performance, memory safety

✅ **Decision**: POSIX compatibility
- Rationale: Enterprise readiness, Linux/BSD parity

✅ **Decision**: Result<T, String> error model
- Rationale: Simple, idiomatic Rust, clear error propagation

---

## Production Readiness Criteria

1. All tests passing (100% pass rate)
2. 0 compilation errors
3. Memory and thread-safe implementation
4. Error handling comprehensive
5. Performance acceptable (< 100µs operations)
6. Documentation complete
7. Code reviewed and approved

