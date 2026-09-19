# Perf (Performance Events)

SigmaOS implements Perf (Performance Events), a performance monitoring and profiling mechanism inspired by Linux perf.

## Overview

Perf provides:

- **Performance event monitoring**: Track CPU cycles, instructions, cache events, and branches
- **Event counting**: Accurate count tracking with atomic operations
- **Sample collection**: Periodic sampling with IP and callchain information
- **Event configuration**: Flexible event type and sampling configuration
- **Enable/disable control**: Selective event monitoring
- **Atomic operations**: Lock-free synchronization

## Components

### PerfEventType

Performance event types:

```rust
pub enum PerfEventType {
    CpuCycles,                   // CPU cycles
    Instructions,                // Retired instructions
    CacheReferences,             // Cache references
    CacheMisses,                 // Cache misses
    BranchInstructions,          // Branch instructions
    BranchMisses,                // Branch misses
    BusCycles,                   // Bus cycles
    StalledCyclesFrontend,       // Stalled cycles frontend
    StalledCyclesBackend,        // Stalled cycles backend
    RefCpuCycles,                // Reference CPU cycles
}
```

### PerfEventConfig

Event configuration:

```rust
pub struct PerfEventConfig {
    pub event_type: PerfEventType,
    pub enabled: bool,
    pub sample_period: u64,
    pub sample_freq: u64,
    pub inherit: bool,
}
```

### PerfEventData

Event data with statistics:

```rust
pub struct PerfEventData {
    pub event_id: u64,
    pub config: PerfEventConfig,
    pub count: AtomicU64,
    pub time_enabled: AtomicU64,
    pub time_running: AtomicU64,
    pub read_format: u32,
}
```

### PerfSample

Performance sample:

```rust
pub struct PerfSample {
    pub sample_id: u64,
    pub event_id: u64,
    pub timestamp_ns: u64,
    pub ip: u64,  // Instruction pointer
    pub period: u64,
    pub callchain: Vec<u64>,
}
```

### PerfSubsystem

Central perf management:

```rust
pub struct PerfSubsystem {
    events: BTreeMap<u64, PerfEventData>,
    samples: Vec<PerfSample>,
    next_event_id: AtomicU64,
    next_sample_id: AtomicU64,
    max_samples: usize,
    enabled: AtomicU32,
}
```

## Usage

### Creating Events

```rust
let mut perf = PerfSubsystem::new(10000);

let config = PerfEventConfig {
    event_type: PerfEventType::CpuCycles,
    enabled: true,
    sample_period: 1000,
    sample_freq: 0,
    inherit: false,
};

let event_id = perf.create_event(config);
```

### Incrementing Event Count

```rust
perf.increment_event(event_id, 100).unwrap();
```

### Reading Event Count

```rust
let count = perf.read_event(event_id).unwrap();
println!("Event count: {}", count);
```

### Resetting Event Count

```rust
perf.reset_event(event_id).unwrap();
```

### Enabling/Disabling Events

```rust
perf.disable_event(event_id).unwrap();
perf.enable_event(event_id).unwrap();
```

### Adding Samples

```rust
perf.add_sample(event_id, current_timestamp_ns, instruction_pointer, 1000).unwrap();
```

### Getting Samples

```rust
let samples = perf.get_samples();
for sample in samples {
    println!("Sample at IP: 0x{:x}", sample.ip);
}
```

### Clearing Samples

```rust
perf.clear_samples();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Sample Buffer**: Configurable maximum sample count

## Comparison with Linux Perf

| Feature | Linux Perf | SigmaOS Perf |
|---------|------------|-------------|
| Event counting | Yes | Yes |
| Sampling | Yes | Yes |
| Event types | Yes | Yes |
| Callchain | Yes | Yes |
| Hardware PMU | Yes | No |
| Kernel tracing | Yes | No |
| Dynamic probes | Yes | No |

## Testing

Comprehensive test coverage includes:

- Event creation
- Event increment
- Event reset
- Enable/disable control
- Sample collection
- Sample clearing

## Future Enhancements

- Hardware PMU support
- Kernel tracing integration
- Dynamic probes (kprobes/uprobes)
- Callchain capture
- Perf record/replay
- Perf stat/report
- Per-CPU events
- Event groups

## References

- Linux Perf (Documentation/perf.txt)
- PAPI (Performance Application Programming Interface)
- FreeBSD pmc (sys/pmctest.h)
- DTrace (illumos/DTrace)
