# Testing in SigmaOS

Comprehensive testing guide for unit testing, integration tests, and hardware emulation tests.

## Running Tests
To run all tests inside SigmaOS workspace:
```bash
./run_sigma_tests.sh
```

## Test Suites
- **Kernel Unit Tests (`tests/`)**: Test low-level scheduling, memory allocators, and math utilities.
- **Integration Tests (`tests/integration_test.rs`)**: End-to-end flow checks for package management, network stack, and virtual fs.
- **Hardware Parity Tests**: Validates PCI device discovery, GPU self-healing resets, and storage controller rings.\n