# SigmaOS Power Manager

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-power-manager  
**Version:** 0.1.0  
**License:** MIT  

## Overview

Power efficiency and battery management for SigmaOS following OOPS principles.

## OOPS Principles Applied

- **State Pattern**: Power states (Performance, Balanced, PowerSaver)
- **Strategy Pattern**: Different power mode strategies
- **Observer Pattern**: Notify on power state changes

## Features

- Performance mode
- Balanced mode
- Power saver mode
- State change notifications

## Usage

```rust
use sigmaos_power_manager::{Component, PerformanceStrategy, LoggingObserver};

let mut component = Component::new();
component.init().unwrap();
component.add_observer(Box::new(LoggingObserver));
component.apply_strategy(Box::new(PerformanceStrategy))?;
```

## Testing

```bash
cargo test --offline
```

### Test Coverage

- **Unit tests (8)**: Test component lifecycle, strategy pattern, observer pattern
- **Integration tests (4)**: Test component integration
- **AAA pattern**: All tests follow Arrange-Act-Assert

## Component Contract

### Provides
- `power-management`: Power efficiency and battery management

### Depends On
- None

## Engineering Principles Applied

✅ STATE PATTERN  
✅ STRATEGY PATTERN  
✅ OBSERVER PATTERN  
✅ SINGLE RESPONSIBILITY  
✅ MODULARITY  
✅ TEST CODE USING CI/CD  
✅ ARRANGE-ACT-ASSERT  
✅ FAIL-FAST  

## License

MIT License
