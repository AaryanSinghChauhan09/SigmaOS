# SigmaOS Sovereign Architecture Dependency Flow

```text
                ┌───────────────┐
                │   UI Module   │
                │ (Window mgr,  │
                │ toolkit, UX)  │
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │ Plugin System │
                │ (WASM shards, │
                │ extensions)   │
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │   Services    │
                │ (logging,     │
                │ monitoring,   │
                │ updates)      │
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │ Networking    │
                │ (protocols,   │
                │ VPN, services)│
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │ Storage       │
                │ (FS, cache,   │
                │ persistence)  │
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │ Drivers       │
                │ (I/O, display,│
                │ input, network)│
                └───────▲───────┘
                        │
                        │
                ┌───────────────┐
                │ Core Kernel   │
                │ (scheduler,   │
                │ memory, sec.) │
                └───────────────┘
```

## Key Principles
*   **Bottom-up dependency**: The Core Kernel is the foundation; everything else builds on it.
*   **Hardware Abstraction**: Drivers depend on the kernel but expose standardized hardware functionality upward (e.g., via `SovereignDriver_t`).
*   **Services Layering**: Storage & Networking sit above drivers, providing abstracted services. Services (logging, monitoring, updates) depend on storage/networking.
*   **Isolation**: Plugins extend services and UI without touching the kernel directly, minimizing risk and surface area.
*   **Top-level Interaction**: UI is the zenith layer, interacting directly with users and indirectly relying on everything below.

## Benefits
*   Clear separation of responsibilities.
*   Easy to swap or extend modules/shards (e.g., replace networking stack without touching UI).
*   Plugins/shards remain isolated via WASM JIT, reducing risk of breaking core functionality.
*   Testing is highly modular, layered bottom-to-top.
