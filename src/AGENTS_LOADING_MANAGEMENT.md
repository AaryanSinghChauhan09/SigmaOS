# Kernel, Bootloader & Driver Loading Rules for AI Agents

## Multi-Stage Boot Pipeline
- Respect the 4-phase boot sequence: Bootloader -> Kernel Initialization -> Dynamic Driver Loading -> Userland Supervisor
- Implement secure boot verification
- Use measured boot with TPM PCR measurements

## Driver Management
- Implement dynamic driver loading with version compatibility
- Use DKMS-style rebuild system for out-of-tree drivers
- Implement driver signature verification

## Design Patterns
- **Chain of Responsibility**: Boot sequence pipeline
- **Factory**: Driver instantiation
- **Observer**: Boot stage notifications
