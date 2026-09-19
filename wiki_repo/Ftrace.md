# Ftrace (Function Tracer)

SigmaOS implements Ftrace (Function Tracer), a function tracing and instrumentation mechanism inspired by Linux ftrace.

## Overview

Ftrace provides:

- **Function entry/exit tracing**: Track function call entry and exit points
- **Performance profiling**: Measure function execution time
- **Call stack tracking**: Monitor nested function calls with depth
- **Statistics collection**: Call count, total duration, average duration
- **Selective tracing**: Enable/disable tracing per function
- **Atomic operations**: Lock-free synchronization

## Components

### FtraceEntry

Function trace entry:

```rust
pub struct FtraceEntry {
    pub id: u64,
    pub function_name: String,
    pub entry_time_ns: u64,
    pub exit_time_ns: u64,
    pub duration_ns: u64,
    pub depth: u32,
    pub parent_id: Option<u64>,
}
```

### FtraceFunctionType

Function type classification:

```rust
pub enum FtraceFunctionType {
    Kernel,    // Kernel functions
    Driver,    // Driver functions
    Syscall,   // System call functions
    Scheduler, // Scheduler functions
    Memory,    // Memory management functions
    Network,   // Network functions
}
```

### FtraceFunction

Function descriptor with statistics:

```rust
pub struct FtraceFunction {
    pub name: String,
    pub function_type: FtraceFunctionType,
    pub enabled: bool,
    pub call_count: AtomicU64,
    pub total_duration_ns: AtomicU64,
}
```

### FtraceSubsystem

Central ftrace management:

```rust
pub struct FtraceSubsystem {
    functions: BTreeMap<String, FtraceFunction>,
    entries: Vec<FtraceEntry>,
    next_entry_id: AtomicU64,
    current_depth: AtomicU32,
    max_entries: usize,
    enabled: AtomicU32,
}
```

## Usage

### Registering Functions

```rust
let mut ftrace = FtraceSubsystem::new(10000);

ftrace.register_function("my_function".to_string(), FtraceFunctionType::Kernel);
```

### Tracing Function Calls

```rust
// Function entry
let id = ftrace.function_entry("my_function", current_timestamp_ns).unwrap();

// ... function executes ...

// Function exit
ftrace.function_exit(id, current_timestamp_ns).unwrap();
```

### Enabling/Disabling Tracing

```rust
// Disable specific function
ftrace.disable_function("my_function").unwrap();

// Enable specific function
ftrace.enable_function("my_function").unwrap();

// Disable all tracing
ftrace.disable_all();

// Enable all tracing
ftrace.enable_all();
```

### Getting Statistics

```rust
let stats = ftrace.get_function_stats("my_function").unwrap();
println!("Call count: {}", stats.0);
println!("Total duration: {} ns", stats.1);
println!("Average duration: {} ns", stats.2);
```

### Clearing Entries

```rust
ftrace.clear_entries();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Call Depth Tracking**: Monitors nested function call depth

## Comparison with Linux Ftrace

| Feature | Linux Ftrace | SigmaOS Ftrace |
|---------|-------------|---------------|
| Function tracing | Yes | Yes |
| Performance profiling | Yes | Yes |
| Call stack | Yes | Yes |
| Statistics | Yes | Yes |
| Selective tracing | Yes | Yes |
| Dynamic probes | Yes | No |
| Graph tracer | Yes | No |
| Kprobes integration | Yes | No |

## Testing

Comprehensive test coverage includes:

- Function registration
- Function entry/exit tracing
- Function statistics
- Enable/disable control
- Nested function calls
- Entry clearing

## Future Enhancements

- Dynamic probes (kprobes/uprobes)
- Graph tracer
- Function graph filtering
- Tracepoint integration
- Ring buffer for entries
- Per-CPU tracing
- Latency tracer
- Wakeup tracer

## References

- Linux Ftrace (Documentation/trace/ftrace.txt)
- DTrace (illumos/DTrace)
- SystemTap (Linux)
- FreeBSD ktrace (sys/ktrace.h)
