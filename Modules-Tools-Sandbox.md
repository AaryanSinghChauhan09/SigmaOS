# Σ tools/sandbox — Sovereign Testing Sandbox

A **safe, instrumented execution environment** for running untrusted code,
fuzz targets, and integration tests without risking the production kernel state.

## Use Cases

| Use Case | Description |
|---|---|
| **Fuzz testing** | Execute AFL++/libFuzzer harnesses in an isolated shard |
| **CI integration tests** | Run full kernel subsystem tests in a QEMU-backed sandbox |
| **Third-party capsules** | Test unverified plugins before signing |
| **Kernel regression tests** | Snapshot state → run test → verify → rollback |

## Architecture

```
Test Runner
   └─ sandbox_create(config)
         └─ Isolated shard (separate memory domain, IOMMU)
               ├─ Fake hardware (QEMU device model via virtio)
               ├─ Snapshot of kernel state (SovereignFS CoW)
               └─ Instrumented syscall interceptor (for coverage)
```

## API Interface

```c
typedef struct {
    const char *name;
    uint64_t   memory_limit_mb;
    bool       network_enabled;
    bool       fs_writable;
    const char *rootfs_snapshot;    // SovereignFS snapshot tag
} sandbox_config_t;

// Create a testing sandbox
sandbox_t *sandbox_create(const sandbox_config_t *cfg);

// Execute a binary inside the sandbox, return exit code
int sandbox_run(sandbox_t *sb, const char *binary, char *const argv[]);

// Capture all syscall events from the sandbox
int sandbox_trace_syscalls(sandbox_t *sb, syscall_trace_cb_t cb);

// Reset sandbox to its initial snapshot (for repeated test runs)
int sandbox_reset(sandbox_t *sb);

// Destroy the sandbox and free all resources
void sandbox_destroy(sandbox_t *sb);

// Initialise the sandbox subsystem
void init_tools_sandbox(void);
```

## Fuzz Integration

```bash

# Build a fuzz harness

sigma build --fuzz target/fuzz_net_parser

# Run under AFL++

sigma fuzz --target fuzz_net_parser --timeout 3600

# Triage a crash

sigma sandbox run --repro crash-001.bin --target fuzz_net_parser
```

## Roadmap

- [ ] Basic shard isolation (`sandbox_create` / `sandbox_destroy`)

- [ ] SovereignFS snapshot-based reset (`sandbox_reset`)

- [ ] Syscall trace interceptor (`sandbox_trace_syscalls`)

- [ ] AFL++ / libFuzzer integration harness

- [ ] QEMU-backed hardware simulation (virtio-blk, virtio-net)

- [ ] Coverage-guided fuzzing via KCOV equivalent

- [ ] Distributed sandbox pool for CI parallelism

## Related Modules

- [`modules/security/isolation`](../../security/isolation/README.md) — Production isolation

- [`modules/tools/diag`](../diag/README.md) — Syscall tracing

- [`modules/perf/bench`](../../perf/bench/README.md) — Performance regression testing
