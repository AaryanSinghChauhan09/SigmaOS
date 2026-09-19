# Interrupt Subsystem

SigmaOS implements an Interrupt subsystem for interrupt handling and management, inspired by Linux IRQ subsystem.

## Overview

The Interrupt subsystem provides:

- **Interrupt request management**: Request and free IRQ lines
- **Interrupt handlers**: Register and execute interrupt handlers
- **Interrupt controllers**: Hardware interrupt controller management
- **Interrupt flags**: Control interrupt behavior (disabled, shared, level-triggered)
- **Global control**: Enable/disable all interrupts globally
- **Statistics**: Track interrupt counts and spurious interrupts
- **Atomic operations**: Lock-free synchronization

## Components

### IrqNumber

Interrupt number:

```rust
pub type IrqNumber = u32;
```

### IrqVector

Interrupt vector:

```rust
pub type IrqVector = u32;
```

### IrqHandler

Interrupt handler function:

```rust
pub type IrqHandler = fn(IrqNumber) -> Result<(), &'static str>;
```

### IrqFlags

Interrupt flags:

```rust
pub struct IrqFlags {
    pub disabled: bool,
    pub shared: bool,
    pub level_triggered: bool,
}
```

### IrqDescriptor

Interrupt descriptor:

```rust
pub struct IrqDescriptor {
    pub irq: IrqNumber,
    pub vector: IrqVector,
    pub handler: Option<IrqHandler>,
    pub flags: IrqFlags,
    pub name: String,
    pub count: AtomicU64,
    pub spurious_count: AtomicU32,
}
```

### InterruptController

Hardware interrupt controller:

```rust
pub struct InterruptController {
    pub name: String,
    pub irq_base: IrqNumber,
    pub irq_count: u32,
    pub enabled: AtomicU32,
}
```

### InterruptSubsystem

Central interrupt management:

```rust
pub struct InterruptSubsystem {
    irqs: BTreeMap<IrqNumber, IrqDescriptor>,
    controllers: BTreeMap<String, InterruptController>,
    next_vector: AtomicU32,
    global_enable: AtomicU32,
}
```

## Usage

### Registering Controllers

```rust
let mut subsystem = InterruptSubsystem::new();

let controller = InterruptController::new("PIC".to_string(), 0, 16);
subsystem.register_controller(controller);
```

### Requesting IRQs

```rust
let flags = IrqFlags::new(false, false, true);
let vector = subsystem.request_irq(1, "test".to_string(), flags).unwrap();
```

### Setting Handlers

```rust
let handler: IrqHandler = |irq| {
    println!("Handling IRQ {}", irq);
    Ok(())
};

subsystem.set_irq_handler(1, handler).unwrap();
```

### Enabling/Disabling IRQs

```rust
subsystem.disable_irq(1).unwrap();
subsystem.enable_irq(1).unwrap();
```

### Handling Interrupts

```rust
subsystem.handle_interrupt(1).unwrap();
```

### Global Control

```rust
subsystem.disable_all();
subsystem.enable_all();
```

### Getting Statistics

```rust
let stats = subsystem.get_irq_stats(1).unwrap();
println!("Count: {}, Spurious: {}", stats.0, stats.1);
```

### Freeing IRQs

```rust
subsystem.free_irq(1).unwrap();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Vector Allocation**: Automatic vector number assignment

## Comparison with Linux IRQ

| Feature | Linux IRQ | SigmaOS Interrupt |
|---------|-----------|-------------------|
| IRQ request/free | Yes | Yes |
| Interrupt handlers | Yes | Yes |
| Shared IRQs | Yes | Yes |
| IRQ affinity | Yes | No |
| IRQ balancing | Yes | No |
| IRQ threading | Yes | No |
| Hardware controllers | Yes | Yes |

## Testing

Comprehensive test coverage includes:

- Controller registration
- IRQ request
- IRQ handler
- Enable/disable control
- Interrupt handling
- Global enable/disable

## Future Enhancements

- IRQ affinity (CPU binding)
- IRQ balancing
- IRQ threading (threaded IRQs)
- IRQ masking
- IRQ priority levels
- IRQ domain support
- Hardware-specific controller drivers
- Interrupt latency profiling

## References

- Linux IRQ (Documentation/IRQ/irq-affinity.txt)
- FreeBSD interrupt (sys/interrupt.h)
- OpenBSD intr (sys/intr.h)
- x86 APIC (Advanced Programmable Interrupt Controller)
