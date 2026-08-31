# SigmaOS Testing

## Overview

SigmaOS provides a comprehensive testing framework for kernel, user-space, and integration testing. The testing infrastructure is designed to ensure system reliability, security, and performance across all components.

**Implementation:** `scripts/test_runner.rs`, `tests/kernel/`, `tests/fs/`

## Test Runner

### Overview

The SigmaOS test runner provides a unified framework for executing and managing test suites across the entire system.

**Implementation:** `scripts/test_runner.rs`

### Test Result Types

```rust
pub enum TestResult {
    Pass = 0,
    Fail = 1,
    Skip = 2,
    Error = 3,
}
```

### Test Suite Structure

```rust
pub struct TestSuite {
    pub name: [u8; 64],
    pub test_count: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub errors: u32,
    pub total_duration_us: u64,
}
```

### Test Runner API

**Initialization:**
```c
int test_runner_init(void);
```

**Suite Management:**
```c
int test_runner_add_suite(const uint8_t *name);
int test_runner_get_suite_count(void);
int test_runner_get_suite(uint32_t suite_index, TestSuite *suite);
```

**Test Execution:**
```c
int run_all(void);
int run_suite(uint32_t suite_index);
```

**Result Recording:**
```c
int test_runner_record_result(
    uint32_t suite_index,
    const uint8_t *test_name,
    TestResult result,
    uint64_t duration_us,
    const uint8_t *message
);
```

**Summary Retrieval:**
```c
int test_runner_get_summary(
    uint32_t *total_tests,
    uint32_t *passed,
    uint32_t *failed,
    uint32_t *skipped,
    uint32_t *errors,
    uint64_t *duration_us
);
```

**Verification Functions:**
```c
int check(void);
int verify_build(void);
int verify_suites(void);
int verify_manifests(void);
int verify_hal(void);
```

## Kernel Tests

### Scheduler Tests

**Implementation:** `tests/kernel/test_scheduler.rs`

**Test Coverage:**
- Thread creation and addition to runqueue
- Priority-based scheduling
- Context switch simulation
- Thread state transitions (Ready, Running, Blocked)
- CPU time accounting
- Runqueue overflow protection

**Mock Thread Structure:**
```rust
pub struct MockThread {
    pub id: u32,
    pub priority: u32,
    pub state: u32,
    pub cpu_time: u64,
}
```

**Test Functions:**
```rust
fn test_thread_creation() -> TestResult;
fn test_priority_scheduling() -> TestResult;
fn test_context_switch() -> TestResult;
fn test_thread_states() -> TestResult;
fn test_cpu_time_accounting() -> TestResult;
fn test_runqueue_overflow() -> TestResult;
```

**Usage:**
```bash
# Run scheduler tests
sigma-test --suite=scheduler

# Run specific test
sigma-test --test=test_thread_creation
```

### Syscall Dispatch Tests

**Implementation:** `tests/kernel/test_syscall_dispatch.rs`

**Test Coverage:**
- Read syscall dispatch
- Write syscall dispatch
- Open syscall dispatch
- Close syscall dispatch
- Mmap syscall dispatch
- Unknown syscall handling
- Register preservation
- Error handling

**Mock Register Structure:**
```rust
pub struct MockRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}
```

**Test Functions:**
```rust
fn test_read_syscall() -> TestResult;
fn test_write_syscall() -> TestResult;
fn test_open_syscall() -> TestResult;
fn test_close_syscall() -> TestResult;
fn test_mmap_syscall() -> TestResult;
fn test_unknown_syscall() -> TestResult;
fn test_register_preservation() -> TestResult;
fn test_error_handling() -> TestResult;
```

**Usage:**
```bash
# Run syscall dispatch tests
sigma-test --suite=syscall

# Run with verbose output
sigma-test --suite=syscall --verbose
```

## Filesystem Tests

### VFS Tests

**Implementation:** `tests/fs/test_vfs.rs`

**Test Coverage:**
- Mount TmpFS
- Open file
- Write to file
- Read from file
- Write and read data match
- Close file
- File descriptor exhaustion
- Invalid file descriptor handling
- File offset tracking
- Inode allocation

**Mock File Structure:**
```rust
pub struct MockFile {
    pub fd: i32,
    pub offset: u64,
    pub flags: u32,
    pub mode: u32,
}
```

**Mock Inode Structure:**
```rust
pub struct MockInode {
    pub inode_num: u64,
    pub size: u64,
    pub mode: u32,
    pub nlink: u32,
}
```

**Test Functions:**
```rust
fn test_mount_tmpfs() -> TestResult;
fn test_open_file() -> TestResult;
fn test_write_file() -> TestResult;
fn test_read_file() -> TestResult;
fn test_write_read_match() -> TestResult;
fn test_close_file() -> TestResult;
fn test_fd_exhaustion() -> TestResult;
fn test_invalid_fd() -> TestResult;
fn test_offset_tracking() -> TestResult;
fn test_inode_allocation() -> TestResult;
```

**Usage:**
```bash
# Run VFS tests
sigma-test --suite=vfs

# Run with coverage
sigma-test --suite=vfs --coverage
```

## Driver Testing Framework

### Overview

The driver testing framework provides OOP-based testing for hardware drivers using traits.

**Features:**
- Device trait for driver abstraction
- Mock hardware simulation
- Driver lifecycle testing
- Performance benchmarking
- Error injection

**Usage:**
```bash
# Run driver tests
sigma-test --suite=drivers

# Test specific driver
sigma-test --driver=nvidia
```

## Integration Tests

### Package Manager Integration Tests

**Implementation:** Integration tests for sigma-pkg

**Test Coverage:**
- Dependency resolver
- Conflict detection
- Package installation
- Package removal
- Rollback functionality
- Signature verification

**Usage:**
```bash
# Run integration tests
sigma-integration-test

# Run specific integration test
sigma-integration-test --test=package_manager
```

## Security Testing

### Security Audit System

**Implementation:** `security/kernel_audit.rs`

### Overview

The SigmaOS security audit system provides comprehensive static analysis for kernel code to detect security vulnerabilities.

### Vulnerability Types

```rust
pub enum VulnType {
    BufferOverflow,
    UseAfterFree,
    DoubleFree,
    IntegerOverflow,
    FormatString,
    RaceCondition,
    NullPointerDereference,
    MemoryLeak,
    InformationLeak,
    PrivilegeEscalation,
    DenialOfService,
    CodeInjection,
    Xss,
    SqlInjection,
    Csrf,
    Other,
}
```

### Severity Levels

```rust
pub enum Severity {
    Info = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}
```

### Static Analysis Rules

**Buffer Overflow Detection:**
- Detects unsafe array access patterns
- Identifies memcpy without size validation
- Flags strcpy/strcat usage

**Use-After-Free Detection:**
- Tracks freed variables
- Detects use of freed memory

**Double-Free Detection:**
- Tracks free operations
- Detects duplicate frees

**Integer Overflow Detection:**
- Identifies unchecked arithmetic operations
- Flags potential overflow conditions

**Null Pointer Dereference:**
- Detects pointer dereference without null check
- Identifies unsafe pointer operations

**Race Condition Detection:**
- Identifies shared state access without locking
- Flags unsafe static mutable access

**Memory Leak Detection:**
- Tracks allocations and frees
- Identifies unfreed memory

**Information Leak Detection:**
- Detects sensitive information in debug output
- Flags potential data exposure

**Privilege Escalation Detection:**
- Identifies privilege escalation functions
- Flags unsafe privilege changes

**Unsafe Function Usage:**
- Detects usage of unsafe C functions
- Flags deprecated functions

**Hardcoded Credentials:**
- Detects hardcoded passwords/keys
- Flags insecure credential storage

**Cryptographic Weakness:**
- Identifies weak cryptographic algorithms
- Flags deprecated crypto implementations

### Audit Report

**Report Structure:**
```rust
pub struct AuditReport {
    pub findings: Vec<AuditFinding>,
    pub total_findings: u32,
    pub critical_count: u32,
    pub high_count: u32,
    pub medium_count: u32,
    pub low_count: u32,
    pub info_count: u32,
    pub scan_duration_ms: u64,
    pub files_scanned: u32,
    pub lines_scanned: u32,
}
```

**Audit Finding:**
```rust
pub struct AuditFinding {
    pub id: String,
    pub vuln_type: VulnType,
    pub severity: Severity,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub description: String,
    pub recommendation: String,
    pub cwe_id: Option<u32>,
    pub cvss_score: Option<f32>,
}
```

### Usage

**Initialize Auditor:**
```rust
let mut auditor = KernelAuditor::new();
auditor.init();
```

**Scan File:**
```rust
let findings = auditor.scan_file("kernel/core/sigma_sched.rs", &content);
```

**Enable/Disable Rules:**
```rust
auditor.enable_rule("Buffer Overflow Detection");
auditor.disable_rule("Memory Leak Detection");
```

**Get Report:**
```rust
let report = auditor.get_report();
println!("{}", report.get_summary());
```

**Export Report:**
```rust
let json = auditor.export_report_json();
let csv = auditor.export_report_csv();
```

**Command-Line Usage:**
```bash
# Run security audit
sigma-audit kernel/

# Generate report
sigma-audit --report security-report.md kernel/

# Check specific vulnerability
sigma-audit --check=buffer-overflow kernel/

# Export to JSON
sigma-audit --format=json --output=audit.json kernel/
```

## Performance Testing

### Benchmarking Tools

**CPU Benchmarks:**
```bash
sigma-bench cpu --iterations=1000
sigma-bench scheduler --tasks=64
sigma-bench context-switch --iterations=100000
```

**Memory Benchmarks:**
```bash
sigma-bench memory --bandwidth
sigma-bench memory --latency
sigma-bench cache
```

**I/O Benchmarks:**
```bash
sigma-bench disk --device=/dev/nvme0n1
sigma-bench network --target=192.168.1.100
sigma-bench filesystem --path=/mnt/sigmafs
```

**System Benchmarks:**
```bash
sigma-bench system --all
sigma-bench boot
sigma-bench startup
```

## Fuzz Testing

**Usage:**
```bash
# Fuzz test syscall handler
sigma-fuzz --target=syscall --input=syscalls.txt

# Fuzz test filesystem
sigma-fuzz --target=filesystem --input=operations.txt

# Fuzz test network stack
sigma-fuzz --target=network --input=packets.bin
```

## Continuous Integration

### GitHub Actions Integration

SigmaOS uses GitHub Actions for automated testing on every push and pull request.

**Test Workflow:**
```yaml
name: SigmaOS Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run kernel tests
        run: sigma-test --suite=kernel
      - name: Run security audit
        run: sigma-audit kernel/
      - name: Run integration tests
        run: sigma-integration-test
```

## Test Configuration

### Test Configuration File

```toml
[test]
verbose = true
parallel = true
timeout = 300

[suites]
kernel = true
filesystem = true
network = true
drivers = true

[security]
audit = true
severity_threshold = "medium"
```

### Environment Variables

```bash
export SIGMAOS_TEST_VERBOSE=1
export SIGMAOS_TEST_PARALLEL=1
export SIGMAOS_TEST_TIMEOUT=300
export SIGMAOS_AUDIT_SEVERITY=medium
```

## Troubleshooting

### Test Failures

**Check Test Logs:**
```bash
sigma-test --suite=scheduler --verbose
```

**Debug with GDB:**
```bash
gdb --args sigma-test --suite=scheduler
```

**Check for Memory Leaks:**
```bash
valgrind --leak-check=full sigma-test
```

### Security Audit Issues

**False Positives:**
```bash
# Disable specific rule for file
sigma-audit --disable="Buffer Overflow Detection" kernel/core/specific.rs
```

**Custom Rules:**
```bash
# Add custom rule
sigma-audit --add-rule=custom-rule.yaml
```

## Best Practices

### Writing Tests

1. **Test Isolation:** Each test should be independent and not depend on other tests
2. **Clear Naming:** Use descriptive test names that explain what is being tested
3. **Arrange-Act-Assert:** Structure tests with clear setup, execution, and verification phases
4. **Mock External Dependencies:** Use mocks for hardware, network, and filesystem operations
5. **Test Edge Cases:** Include tests for boundary conditions and error cases

### Security Testing

1. **Regular Audits:** Run security audits regularly, especially before releases
2. **Fix Critical Issues First:** Prioritize critical and high severity findings
3. **Track Findings:** Maintain a list of security findings and their resolution status
4. **Update Rules:** Keep analysis rules updated with new vulnerability patterns
5. **Manual Review:** Combine automated analysis with manual code review

## References

- [Kernel Architecture](Kernel-Architecture.md)
- [Security Documentation](Security.md)
- [Developer Tools](Developer-Tools.md)
- [Driver Development Guide](Driver-Development-Guide.md)

## License

All SigmaOS testing components are licensed under MIT License. See [LICENSE](../LICENSE) for details.
