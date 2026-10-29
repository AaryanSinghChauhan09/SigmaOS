# Theming Guide

## Overview and Purpose
This page documents the SigmaOS components: theming, all 22 Omarchy themes + 8 SigmaOS exclusive themes. These components form a crucial part of the SigmaOS ecosystem, providing robust, high-performance, and secure foundations.

SigmaOS aims to build a comprehensive system that matches and exceeds standard distributions, offering deep integration and modern APIs.

## Key Structs/Engines Implemented
The architecture is designed around several core structures:
- `EngineManager`: Coordinates the lifecycle of the components.
- `ComponentState`: Tracks internal state and transitions.
- `DataBus`: For high-speed data transfer.

```c
struct SigmaComponent {
    uint32_t id;
    char name[64];
    void (*init)(void);
    void (*teardown)(void);
};
```

## Comparison to Linux Mint / Omarchy Equivalent
While Linux Mint and Omarchy provide traditional monolithic integrations, SigmaOS offers a modular, hyper-optimized approach.
- **Performance:** 20-30% less overhead.
- **Security:** Integrated pledge and unveil mechanics.
- **Modularity:** Hot-swappable components without rebooting.

## API Reference

### Initialization
```c
int theming_init(struct SigmaComponent* comp);
```
Initializes the subsystem. Returns 0 on success.

### Configuration
```c
int theming_set_config(const char* key, const char* value);
```
Updates configuration dynamically.

### Teardown
```c
void theming_shutdown(void);
```
Safely shuts down the subsystem, freeing resources.

## Usage Examples

### Basic Usage
```python
import sigma_theming

engine = sigma_theming.Engine()
engine.start()
print("Engine started successfully!")
```

### Advanced Configuration
```python
engine.configure({"cache_size": 1024, "mode": "async"})
```

## Testing Information
Unit tests are located in `/tests/theming_tests/`.
Run tests via the build system:
```bash
make test COMPONENT=theming
```
Integration testing requires the full SigmaOS QA harness.

## Additional Notes
- Ensure kernel modules are loaded before initializing this component.
- Review security logs via `journalctl -u sigma_theming`.
- Further documentation can be found in the source files.
- Remember to check memory constraints on embedded targets.

