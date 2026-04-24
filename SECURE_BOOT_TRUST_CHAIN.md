
# Secure Boot & Trust Chains



## Sovereign Integrity

SigmaOS integrates cryptographic verification at every stage of the boot sequence:
1. **Bootloader Verification:** Checking the OS image signature against a hardcoded Root of Trust.
2. **Module Verification:** Every loadable module is checked before being injected into user-space.
3. **Immutable Core:** The core kernel cannot be patched dynamically without passing strict cryptographic assertions.
