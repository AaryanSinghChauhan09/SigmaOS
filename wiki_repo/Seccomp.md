# Seccomp (Secure Computing Mode)

SigmaOS implements Seccomp (Secure Computing Mode), a system call filtering and security sandboxing mechanism inspired by Linux seccomp.

## Overview

Seccomp provides:

- **System call filtering**: Fine-grained control over which syscalls can be executed
- **Argument filtering**: Filter syscalls based on argument values
- **Security sandboxing**: Restrict process capabilities
- **Multiple actions**: Allow, kill, trap, return error, trace, log
- **Comparison operators**: Equal, less than, greater than, masked equal
- **Atomic operations**: Lock-free synchronization

## Components

### SeccompAction

Actions for rule violations:

```rust
pub enum SeccompAction {
    Allow,           // Allow the syscall
    KillProcess,     // Kill the entire process
    KillThread,      // Kill the calling thread
    Trap,            // Send SIGTRAP
    Errno(u16),      // Return specified errno
    Trace,           // Notify tracer
    Log,             // Log the violation
}
```

### SeccompCompareOp

Comparison operators for argument filtering:

```rust
pub enum SeccompCompareOp {
    NotEqual,               // arg != value
    LessThan,               // arg < value
    LessThanOrEqual,        // arg <= value
    Equal,                  // arg == value
    GreaterThanOrEqual,     // arg >= value
    GreaterThan,            // arg > value
    MaskedEqual(u64),       // (arg & mask) == (value & mask)
}
```

### SeccompRule

System call rule:

```rust
pub struct SeccompRule {
    pub syscall: SyscallNumber,
    pub action: SeccompAction,
    pub args: Vec<(u32, SeccompCompareOp, u64)>,
}
```

### SeccompFilter

Filter containing multiple rules:

```rust
pub struct SeccompFilter {
    pub id: u32,
    pub rules: Vec<SeccompRule>,
    pub default_action: SeccompAction,
    pub enabled: AtomicU32,
}
```

### SeccompSubsystem

Central seccomp management:

```rust
pub struct SeccompSubsystem {
    filters: BTreeMap<u32, SeccompFilter>,
    next_filter_id: AtomicU32,
}
```

## Usage

### Creating Filters

```rust
let mut subsystem = SeccompSubsystem::new();

let filter_id = subsystem.create_filter(SeccompAction::KillProcess);
```

### Adding Rules

```rust
let filter = subsystem.get_filter_mut(filter_id).unwrap();

// Allow open syscall with specific flags
let mut rule = SeccompRule::new(2, SeccompAction::Allow); // open syscall
rule.add_arg(1, SeccompCompareOp::Equal, 0o644); // mode argument
filter.add_rule(rule);
```

### Evaluating Syscalls

```rust
let action = subsystem.evaluate_syscall(syscall_number, &args);

match action {
    SeccompAction::Allow => {
        // Execute syscall
    }
    SeccompAction::KillProcess => {
        // Kill process
    }
    SeccompAction::Errno(errno) => {
        // Return error
    }
    _ => {
        // Handle other actions
    }
}
```

### Enabling/Disabling Filters

```rust
let filter = subsystem.get_filter(filter_id).unwrap();
filter.disable();
filter.enable();
```

### Deleting Filters

```rust
subsystem.delete_filter(filter_id).unwrap();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Rule Evaluation**: Efficient argument matching

## Comparison with Linux Seccomp

| Feature | Linux Seccomp | SigmaOS Seccomp |
|---------|---------------|-----------------|
| Syscall filtering | Yes | Yes |
| Argument filtering | Yes | Yes |
| Multiple actions | Yes | Yes |
| BPF filters | Yes | No |
| TSYNC | Yes | No |
| Landlock | Yes | No |

## Testing

Comprehensive test coverage includes:

- Filter creation
- Rule matching
- Rule with arguments
- Filter evaluation
- Enable/disable control
- Comparison operators

## Future Enhancements

- BPF filter support
- TSYNC (thread synchronization)
- Landlock integration
- Seccomp notify
- User-space notifications
- Filter export/import
- Performance profiling

## References

- Linux Seccomp (Documentation/prctl/seccomp_filter.txt)
- OpenBSD pledge (man pledge)
- FreeBSD Capsicum (sys/capsicum.h)
- AppArmor (Linux)
