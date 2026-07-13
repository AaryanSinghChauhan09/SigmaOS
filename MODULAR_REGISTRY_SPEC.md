# MODULAR REGISTRY SPEC

> **Component**: `kernel/shards/registry/` | **Suite**: S10_Registry | **Status**: Active

The **Sovereign Registry** is the central orchestration nexus for the 33-suite lattice. It manages the lifecycle, attestation, and dispatching of all Sovereign Shards.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SOVEREIGN REGISTRY                       │
│                                                             │
│  ┌───────────────┐  ┌────────────────┐  ┌──────────────┐  │
│  │  Shard        │  │  Attestation   │  │  Lifecycle   │  │
│  │  Catalog      │  │  Engine        │  │  Manager     │  │
│  │  (name→slot)  │  │  (Dilithium5)  │  │  (init/term) │  │
│  └───────┬───────┘  └────────┬───────┘  └──────┬───────┘  │
│          │                   │                  │          │
│  ┌───────┴───────────────────┴──────────────────┴───────┐  │
│  │              SIGMA-BUS DISPATCH TABLE                │  │
│  │         (channel name → shard slot routing)          │  │
│  └──────────────────────────────────────────────────────┘  │
│          ↕ C-ABI                                           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              KERNEL BOOT SEQUENCE                    │  │
│  │    Stage 1: Genesis → Stage 2: HAL → Stage 3: Reg   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

All registries are located in: `kernel/suites/S10_Registry/shards/`

---

## Registration API

Shards must use the `SovereignRegistry_Register` API to bind to the lattice during stage-3 boot:

```rust
/// Register a shard with the Sovereign Registry
///
/// # Arguments
/// * `name` - Unique shard name (max 64 bytes, null-terminated)
/// * `category` - Shard category (core, security, driver, ui, optional)
/// * `capabilities` - Requested capabilities bitmap
/// * `bus_channels` - sigma-bus channels to subscribe to
/// * `init_fn` - Initialization function pointer
///
/// # Returns
/// * `Ok(ShardSlot)` - Assigned slot in the registry
/// * `Err(RegistryError)` - Registration failed
pub fn sovereign_registry_register(
    name: &[u8; 64],
    category: ShardCategory,
    capabilities: CapabilitySet,
    bus_channels: &[BusChannel],
    init_fn: unsafe extern "C" fn() -> i32,
) -> Result<ShardSlot, RegistryError> {
    // 1. Verify shard binary signature (Dilithium5)
    let sig_ok = attestation_engine::verify_caller_signature()?;
    if !sig_ok {
        return Err(RegistryError::AttestationFailed);
    }

    // 2. Check for name conflicts
    if CATALOG.contains(name) {
        return Err(RegistryError::NameConflict);
    }

    // 3. Validate requested capabilities against policy
    let policy_ok = capability_policy::check(category, capabilities)?;
    if !policy_ok {
        return Err(RegistryError::CapabilityDenied);
    }

    // 4. Allocate slot and register bus channels
    let slot = CATALOG.allocate_slot(name, category)?;
    for channel in bus_channels {
        sigma_bus::register_channel(slot, channel)?;
    }

    // 5. Call shard init function
    let init_result = unsafe { init_fn() };
    if init_result != 0 {
        CATALOG.free_slot(slot);
        return Err(RegistryError::InitFailed(init_result));
    }

    // 6. Register with profiler
    profiler::register_shard(slot.id());

    Ok(slot)
}
```

### C-ABI Equivalent

```c
// For C/C++ shards using the legacy interface
sigma_err_t SovereignRegistry_Register(
    const char* name,
    shard_category_t cat,
    uint64_t capabilities,
    const char** bus_channels,
    size_t channel_count,
    sigma_init_fn init_fn
);
```

---

## Shard Categories

```rust
#[repr(u8)]
pub enum ShardCategory {
    Core      = 0x01,  // Boot-critical (CoreLattice, Scheduler, Memory)
    Security  = 0x02,  // Security subsystem (MAC, Sandbox, Crypto)
    Driver    = 0x03,  // Hardware drivers (GPU, NIC, Storage)
    Network   = 0x04,  // Networking stack (TCP/IP, Firewall, VPN)
    UI        = 0x05,  // Desktop/GUI (Zenith, Window Manager)
    AI        = 0x06,  // Neural Core (Inference, Tuner)
    Optional  = 0x07,  // User-installed shards (community, marketplace)
    Debug     = 0x08,  // Development/debugging tools
}
```

---

## Capability System

Each shard declares required capabilities at registration. The registry enforces least-privilege:

| Capability | Bit | Description |
|---|---|---|
| `CAP_MEMORY_ALLOC` | 0x0001 | Can allocate kernel memory |
| `CAP_IPC_SEND` | 0x0002 | Can send sigma-bus messages |
| `CAP_IPC_RECV` | 0x0004 | Can receive sigma-bus messages |
| `CAP_NET_RAW` | 0x0008 | Raw network socket access |
| `CAP_DISK_WRITE` | 0x0010 | Direct block device write |
| `CAP_DISK_READ` | 0x0020 | Direct block device read |
| `CAP_SCHED_RT` | 0x0040 | Real-time scheduling priority |
| `CAP_CRYPTO_SIGN` | 0x0080 | Access to signing keys |
| `CAP_SHARD_SPAWN` | 0x0100 | Can spawn child shards |
| `CAP_SHARD_KILL` | 0x0200 | Can terminate other shards |
| `CAP_HARDWARE_DMA` | 0x0400 | DMA access to hardware |
| `CAP_USER_NS` | 0x0800 | Create user namespaces |

**Policy**: Core and Security shards get full capabilities. Optional shards are limited to `IPC_SEND | IPC_RECV | MEMORY_ALLOC`.

---

## Registry Catalog

The catalog maintains a fixed-size array of registered shards:

```rust
pub struct RegistryCatalog {
    entries: [RegistryEntry; MAX_SHARDS],  // 256 max
    count: u32,
}

pub struct RegistryEntry {
    name:         [u8; 64],
    category:     ShardCategory,
    capabilities: CapabilitySet,
    slot:         ShardSlot,
    state:        ShardState,        // from sovereign_profiler.rs
    boot_order:   u16,               // lower = earlier boot
    dependencies: [ShardSlot; 8],    // max 8 dependencies
    dep_count:    u8,
}
```

---

## Lifecycle Management

```
┌──────────┐    register()    ┌──────────────┐    init()    ┌─────────┐
│ UNKNOWN  │ ───────────────▶ │ REGISTERED   │ ──────────▶ │ RUNNING │
└──────────┘                  └──────────────┘              └────┬────┘
                                     ▲                          │
                                     │                     on_crash()
                                     │                          │
                              ┌──────┴──────┐              ┌────▼──────┐
                              │ RESTARTING  │ ◀────────── │ CRASHED   │
                              └─────────────┘              └───────────┘
                                                                │
                                                           5x crashes
                                                                │
                                                          ┌─────▼───────┐
                                                          │ QUARANTINED │
                                                          └─────────────┘
```

---

## Boot Sequence Integration

During kernel boot, the registry processes shards in dependency-topological order:

```
Stage 1 (Genesis):   CoreLattice, MemoryManager, SchedulerCore
Stage 2 (HAL):       HardwareAbstraction, DriverManager, ACPI
Stage 3 (Registry):  SovereignRegistry self-registers
Stage 4 (Security):  SovereignMAC, CryptoProvider, SecureBoot
Stage 5 (Network):   NetworkStack, SigmaShield, DNSResolver
Stage 6 (Storage):   VFSCore, NVMeDriver, FilesystemMount
Stage 7 (Telemetry): SovereignProfiler, AuditLog
Stage 8 (UI):        ZenithDesktop (if desktop profile)
Stage 9 (Optional):  User-installed shards from SHARDS.manifest
Stage 10 (Finality): SovereignFinality (system ready signal)
```

---

## Query API

```rust
// List all registered shards
pub fn registry_list() -> &[RegistryEntry];

// Find a shard by name
pub fn registry_find(name: &str) -> Option<&RegistryEntry>;

// Get shard health from profiler
pub fn registry_health(slot: ShardSlot) -> ShardMetrics;

// Unregister and terminate a shard
pub fn registry_unregister(slot: ShardSlot) -> Result<(), RegistryError>;
```

### CLI

```bash
# List all registered shards
sigma registry list
# NAME              CATEGORY   STATE      BOOT_ORDER  CAPS
# CoreLattice       core       RUNNING    1           0xFFFF
# SigmaScheduler    core       RUNNING    2           0x0043
# SovereignMAC      security   RUNNING    10          0x00CF
# SigmaShield       security   RUNNING    11          0x002E
# ZenithDesktop     ui         RUNNING    30          0x0006
# HelloWorld        optional   IDLE       99          0x0006

# Inspect a specific shard
sigma registry inspect SigmaShield

# Hot-reload a shard
sigma registry reload SigmaShield
```

---

## Roadmap

- [x] Shard catalog with fixed-size entries
- [x] C-ABI registration function
- [x] Dilithium5 attestation at registration
- [x] Boot-order topological sort
- [ ] Hot-reload via sigma-bus state transfer (Q3)
- [ ] Dependency graph cycle detection (Q3)
- [ ] Remote shard registration (mesh networking) (Q4)
- [ ] Marketplace shard auto-registration (Year 2)
