# Architecture Overview

1

1

1

graph TD
    L1[Layer 1: Bare-Metal Silicon & TPM]
    L2[Layer 2: HAL & Quantum Watchdog]
    L3[Layer 3: Sovereign Core Kernel]
    L4[Layer 4: Shard Orchestrator & Sandbox]
    L5[Layer 5: Sovereign Package Manager & AI Daemon]
    L6[Layer 6: Zenith Wayland Compositor]
    L7[Layer 7: Sovereign Applications & ML Hub]

    L1 --> L2
    L2 --> L3
    L3 --> L4
    L4 --> L5
    L5 --> L6
    L6 --> L7

1

1

Communication between layers is enforced by the **Sovereign IPC Bus**. No driver can directly access the kernel space without an encrypted capability token.

1

1

The `SovereignHAL` provides a strict interface for hardware interaction. Direct I/O port mapping is prohibited unless verified by the Hardware Attestation TPM driver during boot.

