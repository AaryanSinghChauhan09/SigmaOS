# SigmaOS Phase 9 Core - Task List

**Status**: READY FOR EXECUTION
**Total Tasks**: 12 high-priority tasks
**Estimated Total Effort**: 35 hours
**Current Progress**: 25 hours complete (Phases 9.1-9.2.3)

---

## Task Execution Model

Each task is:
- **Standalone**: Can be executed independently once dependencies met
- **Testable**: Includes specific acceptance criteria
- **Actionable**: Specifies files, LOC targets, and test requirements
- **Verifiable**: Build check and test verification required

---

## PRIORITY BATCH 1 (Next 5 hours)

### Task 1: 9.2.4 - Network NS Integration Tests
- **id**: 9.2.4-network-ns-integration-tests
- **status**: ready
- **effort**: 5 hours
- **description**: Comprehensive end-to-end integration tests for Network Namespace
- **subtasks**:
  - End-to-end namespace isolation (3 namespaces, verify isolation)
  - Bridge connectivity tests (veth pairs, forwarding)
  - Multi-namespace communication (cross-NS socket tests)
  - Performance benchmarks (< 100µs operations)
  - Edge cases (invalid interfaces, conflicts, state violations)
- **files_to_create**:
  - tests/network_ns_integration_tests.rs (400+ LOC)
- **test_requirements**:
  - 15+ integration tests
  - 100% pass rate
  - Performance validation
  - Edge case coverage
- **acceptance_criteria**:
  - All 15+ tests passing
  - 0 compilation errors
  - Cross-namespace isolation verified
  - Bridge forwarding functional
  - Performance < 100µs per operation
- **depends_on**: [Phase 9.2.3 complete]

---

## PRIORITY BATCH 2 (Next 20 hours)

### Task 2: 9.3.1 - User Namespace Core
- **id**: 9.3.1-user-ns-core
- **status**: queued
- **effort**: 5 hours
- **description**: User namespace core structures and namespace management
- **subtasks**:
  - UserNamespace struct (id, uid_map, gid_map, capabilities, owner_uid)
  - CapabilitySet enum (CAP_CHOWN, CAP_DAC_OVERRIDE, etc.)
  - UserNamespaceManager (create, get, delete namespaces)
  - Namespace creation and registry
  - Unit tests (5+)
- **files_to_create**:
  - src/security/user_namespace.rs (300+ LOC)
- **test_requirements**:
  - 5+ unit tests
  - Namespace creation working
  - Registry functional
- **acceptance_criteria**:
  - Structs compile
  - Namespace creation works
  - Registry tracks namespaces
  - 0 errors
  - 5+ tests passing
- **depends_on**: [Task 1 complete]

### Task 3: 9.3.2 - UID/GID Mapping
- **id**: 9.3.2-uid-gid-mapping
- **status**: queued
- **effort**: 6 hours
- **description**: UID/GID range mapping between host and namespace
- **subtasks**:
  - UidGidMapping struct (container_id, host_id, count)
  - set_uid_map() and set_gid_map() functions
  - map_uid_host_to_ns() translation
  - map_uid_ns_to_host() translation
  - validate_mapping() (overlap/range checks)
  - Unit tests (10+)
- **files_to_modify**:
  - src/security/user_namespace.rs
- **test_requirements**:
  - 10+ unit tests
  - Mapping validation
  - Translation accuracy
  - Edge cases (overlaps, invalid ranges)
- **acceptance_criteria**:
  - Mapping works correctly
  - Validation prevents invalid ranges
  - Translations accurate both directions
  - 0 errors
  - 10+ tests passing
- **depends_on**: [Task 2 complete]

### Task 4: 9.3.3 - subuid/subgid Support
- **id**: 9.3.3-subuid-subgid
- **status**: queued
- **effort**: 5 hours
- **description**: /etc/subuid and /etc/subgid file parsing and support
- **subtasks**:
  - SubuidEntry struct
  - SubgidEntry struct
  - parse_subuid_file() parsing
  - parse_subgid_file() parsing
  - allocate_subuid_range() with conflict checking
  - allocate_subgid_range() with conflict checking
  - SubuidAllocationTracker
  - Unit tests (10+)
- **files_to_modify**:
  - src/security/user_namespace.rs
- **test_requirements**:
  - 10+ unit tests
  - File parsing working
  - Allocation tracking
  - Conflict detection
- **acceptance_criteria**:
  - Files parse correctly
  - Allocations tracked
  - Range enforcement works
  - 0 errors
  - 10+ tests passing
- **depends_on**: [Task 3 complete]

### Task 5: 9.3.4 - User Namespace Syscalls
- **id**: 9.3.4-user-ns-syscalls
- **status**: queued
- **effort**: 4 hours
- **description**: User namespace syscall integration (setuid, setgid, etc.)
- **subtasks**:
  - sys_setuid() with namespace mapping
  - sys_setgid() with namespace mapping
  - sys_getuid() returning namespace-mapped UID
  - sys_getgid() returning namespace-mapped GID
  - sys_setgroups() for supplementary groups
  - CLONE_NEWUSER flag support
  - User context tracking per-namespace
  - Capability enforcement
  - Integration tests (10+)
- **files_to_create**:
  - src/syscall/user_syscalls.rs (300+ LOC)
- **files_to_modify**:
  - src/security/user_namespace.rs (add syscall hooks)
- **test_requirements**:
  - 10+ integration tests
  - Full syscall lifecycle
  - Mapping verification
  - Capability enforcement
- **acceptance_criteria**:
  - Syscalls work correctly
  - CLONE_NEWUSER supported
  - Proper error codes returned
  - 0 errors
  - 10+ tests passing
- **depends_on**: [Task 4 complete]

---

## PRIORITY BATCH 3 (Next 10 hours - Part 1 of eBPF VM)

### Task 6: 9.4.1 - BPF Instruction Set
- **id**: 9.4.1-bpf-instruction-set
- **status**: queued
- **effort**: 6 hours
- **description**: Complete eBPF instruction set definition and validation
- **subtasks**:
  - BpfInstruction enum (20+ instruction types)
  - Load/Store instructions (LoadImm64, LoadReg64, StoreReg64, etc.)
  - Arithmetic instructions (Add, Sub, Mul, Div, Mod, Neg)
  - Bitwise instructions (And, Or, Xor, Lsh, Rsh, Arsh)
  - Jump instructions (Ja, Jeq, Jne, Jgt, Jge, Jlt, Jle, etc.)
  - Function call instructions (Call, Return)
  - Memory access (LoadAbs, LoadInd)
  - validate_instruction() function
  - is_valid_register() and is_valid_immediate()
  - Unit tests (10+)
- **files_to_create**:
  - src/kernel/ebpf_vm.rs (600+ LOC - instruction set portion)
- **test_requirements**:
  - 10+ unit tests
  - All instruction types creatable
  - Validation working
  - Register/immediate bounds checked
- **acceptance_criteria**:
  - All instructions defined
  - Instruction format correct
  - Validation works
  - 0 errors
  - 10+ tests passing
- **depends_on**: [Task 5 complete]

### Task 7: 9.4.2 - BPF VM Core Engine
- **id**: 9.4.2-bpf-vm-core
- **status**: queued
- **effort**: 4 hours
- **description**: eBPF virtual machine core execution engine
- **subtasks**:
  - BpfVm struct (registers R0-R10, PC, stack, heap, memory)
  - new() initializer
  - load_program() for loading instructions
  - execute_instruction() for single instruction
  - Basic arithmetic execution (Add, Sub, Mul, Div, Mod)
  - Bitwise operations (And, Or, Xor, shifts)
  - Register get/set
  - Stack push/pop
  - Jump handling (set PC)
  - Unit tests (10+)
- **files_to_modify**:
  - src/kernel/ebpf_vm.rs (add execution engine)
- **test_requirements**:
  - 10+ unit tests
  - All basic instructions execute
  - Registers work correctly
  - Stack operations valid
  - PC updates correct
- **acceptance_criteria**:
  - VM executes all basic instructions
  - Registers/memory/stack function
  - 0 errors
  - 10+ tests passing
  - Foundation ready for Part 2 (helpers/verification)
- **depends_on**: [Task 6 complete]

---

## BATCH SUMMARY

**Batch 1**: Network NS Integration Tests (5h) → Commit → Push
**Batch 2**: User Namespace Complete (20h total: 5+6+5+4) → Commit → Push
**Batch 3**: eBPF VM Foundation (10h: 6+4) → Commit → Push

**Total**: 35 hours of implementation
**Output**: 2,400+ LOC, 100+ tests, 0 errors
**Result**: Phase 9 at 80%+ feature completion

---

## Execution Requirements

### Quality Gates (All Tasks)
✅ Compile: `cargo build --lib` succeeds
✅ Tests: 100% pass rate
✅ Safety: Arc<Mutex<>> or Arc<RwLock<>>
✅ Errors: Result<T, String> with meaningful messages
✅ Documentation: Comments on public functions

### After Each Task
1. Commit with descriptive message
2. Push to GitHub: `git push origin main`
3. Verify: `cargo build --lib 2>&1 | tail -3`
4. Report: LOC added, tests added, time spent

### After Each Batch
1. Create batch summary (LOC, tests, commits)
2. Push all commits
3. Verify final build
4. Ready for next batch

---

## Success Criteria

✅ **All 7 Tasks Complete**:
- Phase 9.2.4: Integration tests (15+ tests)
- Phase 9.3: User Namespace (35+ tests total)
- Phase 9.4: eBPF foundation (20+ tests)

✅ **Total Deliverables**:
- 2,400+ LOC new code
- 100+ new tests
- 0 compilation errors
- 100% test pass rate
- GitHub synchronized

✅ **Production Status**:
- Phases 9.1-9.2.4: PRODUCTION READY
- Phases 9.3: PRODUCTION READY
- Phases 9.4: FOUNDATION READY (Part 2 next)

---

## Next Phases (After Batch 3)

**Batch 4**: eBPF VM Completion (Part 2 & 3)
- BPF helpers (bpf_probe_read, bpf_ktime_get_ns, etc.)
- Program verification
- Syscall integration

**Batch 5**: Extended Cgroups (20h)
- Device controller
- Hugetlb, RDMA, Pids, Net_cls controllers
- Per-cgroup enforcement

**Batch 6**: Advanced Filtering & Release (32h)
- BPF-based seccomp integration
- Filter loading/execution
- Syscall argument inspection
- End-to-end integration
- Performance benchmarking
- v0.9 release tag

**Total Remaining After Batch 3**: ~72 hours to v0.9 release

