## Description
<!--- Describe your changes in detail -->

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Driver / Hardware support implementation
- [ ] Refactoring / Architecture enhancement
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation / CI / Tooling update

## Architectural Compliance Checklist
- [ ] **no_std Enforcement:** Verified no unallowed `std::` usages exist in kernel/driver code (`./scripts/no_std_check.sh`).
- [ ] **Capability-Based Security:** Verified capability token (`CapabilityToken`) validation is present on all syscall entrypoints or IPC endpoints touched.
- [ ] **Type Annotations & Memory Safety:** Explicit type annotations are used on public APIs and memory copies use bounds-clamped/checked calls.
- [ ] **Driver Lifecycle Test (if applicable):** Drivers follow IoManager / DriverObject / DeviceObject / DeviceExtension patterns and include unit tests for DriverObject creation, attach, detach, and destroy.
- [ ] **Paged/NonPaged Memory Pools:** Verified pool allocations adhere to non-paged vs paged requirements.

## Testing & Verification
- [ ] Executed standalone module tests via `./scripts/changed_files_rustc_tests.sh`.
- [ ] Executed `./run_sigma_tests.sh` or relevant integration smoke tests.
- [ ] All new and existing tests passed cleanly.

## Unsafe Code Justification
<!--- If this PR introduces or modifies `unsafe` blocks, state the precise memory safety invariants enforced and why `unsafe` is necessary. -->
- [ ] N/A (No unsafe code modified)
- [ ] Invariants documented and verified in code.
