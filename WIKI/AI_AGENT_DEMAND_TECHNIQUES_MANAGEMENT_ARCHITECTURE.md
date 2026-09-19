# AI Agent Various Demand Techniques Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       AI Demand Techniques Manager                              |
|   (DemandTechniquesGovernor, DemandPagingEngine, OnDemandDriverHotplugger)      |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                  Demand Page Fault Interrupt Handler (#PF)                      |
|       (CR2 Fault Address, PML4 Self-Ref Map, PTE_COW & PTE_LAZY Check)          |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Lazy Zero-Page Fill   |   | Copy-on-Write Duplicate|  | Demand ELF Code Mmap  |
| (Zero 4KB PMM Frame)  |   | (PTE_RW Promotion)    |   | (Page Cache Fault In) |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                    On-Demand Driver & Socket Service Activator                  |
|     (Devd Hardware Hotplug, Modprobe Aliases, Socket-Activated Services)        |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Demand Page Fault Handler (#PF)**:
   - Reads CR2 register fault addresses upon page fault interrupts.
   - Differentiates demand zero-fill, Copy-on-Write (COW), and file-backed demand mmap faults.

2. **Copy-on-Write & Executable Segment Loader**:
   - Manages shared copy-on-write physical pages for process forks.
   - Demand-loads ELF/APE binary text/data segments from page cache into process address spaces on demand.

3. **Dynamic Library PLT/GOT Relocator**:
   - Intercepts PLT trampolines for dynamic symbol resolution on first function call.

4. **On-Demand Driver & Socket Activator**:
   - `BsdDevdHardwareEventDispatcher` resolves modprobe module aliases and loads drivers on demand when hardware is plugged.
   - Socket activation spawns background daemons upon receiving incoming network socket payloads.

5. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
