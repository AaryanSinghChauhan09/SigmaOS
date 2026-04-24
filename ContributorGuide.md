
# SigmaOS Contributor Guide


The mission of SigmaOS is to achieve absolute architectural parity with global ecosystems while maintaining pure silicon sovereignty.


## Coding Standards



### Pure ASM & Freestanding C

- No standard library usage (`-nostdlib`).
- Every function must be documented with its "Sovereign Purpose".
- Use `uintptr_t` for hardware addresses.


### Shard Naming Convention

- Format: `S[SuiteID]_[ModuleName]`
- Example: `S04_HAL_KeyboardDriver`


## Testing Requirements

All new lattice shards must include an atomic test file named `test_[module].c` in their respective suite directory. 
The CI/CD pipeline will automatically verify these tests across Linux, macOS, and Windows.


## Review Process

1. **Lattice Parity**: Does the change maintain or improve the 33-suite balance?
2. **Silicon Efficiency**: Is there any unnecessary abstraction?
3. **Observability**: Is the change reflected in the Zenith Dashboard or API Bridge?


## Pull Requests

- Ensure `build_sovereign.sh` passes.
- Include a description of how "sovereignty" is maintained in the module.
