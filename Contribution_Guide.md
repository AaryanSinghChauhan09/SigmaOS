# Contribution Guide

> How to add new modules, tools, shards, and features to SigmaOS.

---

## Overview

This guide provides step-by-step instructions for contributing to SigmaOS. Whether you're adding a kernel primitive, hardware driver, security module, or CLI command, this guide will help you navigate the codebase structure and contribution process.

---

## Finding the Right Location

SigmaOS is organized into **600+ shards** grouped by domain. Each shard is identified by `S<NN>_<Name>` and lives in `suites/`.

### Shard Domains

| Suite Range | Domain |
|-------------|--------|
| S01–S10 | Kernel core (allocator, HAL, IPC, security) |
| S11–S20 | System services (VFS, networking, process mgmt) |
| S21–S30 | Performance (NUMA, cache, BPF, containers) |
| S31–S40 | Storage, immutability, ZKP, sovereign wire |
| S41–S50 | Boot, self-healing, caps, mesh |
| S51–S65 | Applications, AI, developer tools |

### Module Type Mapping

| Module Type | Location |
|-------------|----------|
| New kernel primitive | `suites/S01_Genesis/sigma_<name>.h` |
| New hardware driver | `suites/S04_HAL/sigma_<name>.hpp` |
| New security module | `suites/S08_Security/sigma_<name>.h` |
| New network feature | `suites/S37_SovereignWire/sigma_<name>.h` |
| New perf optimization | `suites/S28_PerformanceLattice/sigma_<name>.h` |
| New OOP driver class | `sigmaos/core/src/atomic_sigma_<name>_oop.cpp` |
| New CLI command | `orchestrator/main.cpp` (add ICommand subclass) |

---

## Step-by-Step Contribution Process

### 1. Choose Your Contribution Type

**Example: Adding a VPN Module**

```bash
touch suites/S37_SovereignWire/sigma_vpn.h
```

### 2. Follow the Atomic Module Contract

Every SigmaOS module must adhere to the Atomic Module Contract:

- **Single Responsibility**: Each module does one thing well
- **Clear Interface**: Well-defined public API
- **Testable**: Unit tests for all functionality
- **Documented**: Inline comments and external documentation
- **Capability-Gated**: Declares required security capabilities

### 3. Implement the Module

Create the header file with proper structure:

```c
// suites/S37_SovereignWire/sigma_vpn.h
#ifndef SIGMA_VPN_H
#define SIGMA_VPN_H

#include <sigma/types.h>
#include <sigma/net.h>

typedef struct sigma_vpn_config {
    char server_address[256];
    uint16_t server_port;
    char auth_token[128];
    uint8_t cipher_suite;
} sigma_vpn_config_t;

typedef struct sigma_vpn_state {
    int tunnel_fd;
    sigma_vpn_config_t config;
    uint8_t session_key[32];
    bool connected;
} sigma_vpn_state_t;

// Initialize VPN connection
int sigma_vpn_init(sigma_vpn_state_t *state, const sigma_vpn_config_t *config);

// Establish VPN tunnel
int sigma_vpn_connect(sigma_vpn_state_t *state);

// Send data through VPN
int sigma_vpn_send(sigma_vpn_state_t *state, const void *data, size_t len);

// Receive data from VPN
int sigma_vpn_recv(sigma_vpn_state_t *state, void *buf, size_t len);

// Close VPN connection
void sigma_vpn_shutdown(sigma_vpn_state_t *state);

#endif // SIGMA_VPN_H
```

### 4. Add CLI Command (if applicable)

Edit `orchestrator/main.cpp` to add a new command:

```cpp
// Add new ICommand subclass
class VPNCommand : public ICommand {
public:
    bool matches(const std::string& cmd) const override {
        return cmd == "vpn";
    }

    int execute(int argc, char** argv) const override {
        if (argc < 2) {
            std::cout << "Usage: vpn <connect|disconnect|status> [args]\n";
            return 1;
        }

        std::string action = argv[1];

        if (action == "connect") {
            // Parse connection parameters
            // Call sigma_vpn_init() and sigma_vpn_connect()
            std::cout << "[SigmaOS] VPN tunnel established via Sigma-Net.\n";
            return 0;
        } else if (action == "disconnect") {
            // Call sigma_vpn_shutdown()
            std::cout << "[SigmaOS] VPN tunnel disconnected.\n";
            return 0;
        } else if (action == "status") {
            // Display connection status
            std::cout << "[SigmaOS] VPN status: disconnected\n";
            return 0;
        }

        std::cout << "Unknown VPN command: " << action << "\n";
        return 1;
    }
};

// Register in CommandDispatcher constructor
CommandDispatcher::CommandDispatcher() {
    register_command(std::make_unique<VPNCommand>());
    // ... other commands
}
```

### 5. Add Subsystem Test

Edit `orchestrator/main.cpp`, find `TestCommand::run_subsystem_test()`:

```cpp
static void run_subsystem_test(const char* subsystem) {
    if (strcmp(subsystem, "networking") == 0) {
        // Test VPN module
        sigma_vpn_state_t vpn;
        sigma_vpn_config_t config = {
            .server_address = "vpn.sigmaos.net",
            .server_port = 1194,
            .cipher_suite = 1
        };
        
        if (sigma_vpn_init(&vpn, &config) == 0) {
            std::cout << "[✓] VPN module initialization passed\n";
        } else {
            std::cout << "[✗] VPN module initialization failed\n";
        }
    } else {
        std::cout << "[?] " << subsystem << " - All shards passed.\n";
    }
}
```

### 6. Build and Test

```bash
# Build the project
g++ -std=c++20 orchestrator/main.cpp -o s-cli

# Run subsystem test
./s-cli test --subsystem networking

# Run full test suite
./s-cli test
```

### 7. Commit and Push

```bash
git add suites/S37_SovereignWire/sigma_vpn.h orchestrator/main.cpp
git commit -m "feat: Add sigma_vpn atomic networking module"
git push origin feature/sigma-vpn
```

### 8. Open Pull Request

- Create a PR from your feature branch to `main`
- Include test results in the PR description
- Reference any related issues
- Request review from maintainers

---

## CI/CD Integration

CI will automatically run all workflows when you push:

- **Build Workflow**: Compiles all modules
- **Test Workflow**: Runs unit and integration tests
- **Security Workflow**: Scans for vulnerabilities
- **Documentation Workflow**: Validates documentation completeness

All workflows must pass before merging.

---

## Code Review Process

1. **Automated Checks**: CI must pass (build, test, security)
2. **Maintainer Review**: At least one maintainer approval required
3. **Security Review**: Required for security-related changes
4. **Documentation Review**: Wiki must be updated for user-facing features

---

## Common Contribution Types

### Adding a Kernel Primitive

1. Create file in appropriate `suites/S01_Genesis/`
2. Implement with `#![no_std]` if in kernel
3. Add unit tests in `kernel/tests/`
4. Document in wiki

### Adding a Hardware Driver

1. Use sigma-driver-porter or Driver SDK
2. Implement probe/init/shutdown lifecycle
3. Add capability declarations
4. Test with QEMU or real hardware
5. See [CONTRIBUTING_DRIVERS.md](CONTRIBUTING_DRIVERS.md)

### Adding a Security Module

1. Create file in `suites/S08_Security/`
2. Implement with Ada/SPARK for critical code
3. Run gnatprove for formal verification
4. Add security tests
5. Document threat model

### Adding a CLI Command

1. Add ICommand subclass in `orchestrator/main.cpp`
2. Implement execute() method
3. Add help text
4. Test with various arguments
5. Update CLI documentation

---

## Testing Requirements

### Unit Tests

- Every module must have unit tests
- Tests must cover happy path and error cases
- Use appropriate testing framework for language

### Integration Tests

- Test module integration with other components
- Test with real hardware when applicable
- Include performance benchmarks for critical paths

### Documentation Tests

- All examples in documentation must be tested
- Code snippets must compile and run
- API documentation must match implementation

---

## Getting Help

- **Documentation**: Check [wiki_repo/](wiki_repo/) for detailed guides
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions
- **Discord**: Join the SigmaOS Discord for real-time help

---

## Contribution Checklist

Before submitting your PR:

- [ ] Code follows [Coding-Standards.md](Coding-Standards.md)
- [ ] All tests pass locally
- [ ] New functionality is documented
- [ ] Wiki is updated for user-facing changes
- [ ] Commit messages follow conventional format
- [ ] PR description includes test results
- [ ] No hardcoded paths or magic numbers
- [ ] Security implications are documented
- [ ] Performance impact is measured (if applicable)

---

*See also: [CONTRIBUTING.md](CONTRIBUTING.md) · [Coding-Standards.md](Coding-Standards.md) · [CONTRIBUTING_DRIVERS.md](CONTRIBUTING_DRIVERS.md)*
