# Subsystem Dependencies and Initialization Directed Acyclic Graph (DAG)

## Initialization Sequence

```
[Firmware] -> [Bootloader] -> [Kernel Entry] -> [Arch Init]
                                                    |
                                                    v
                                            [Memory Init (Physical -> Paging -> Allocator)]
                                                    |
                                                    v
                                            [Interrupt Init (APIC/IDT -> Timers)]
                                                    |
                                                    v
                                            [Scheduler Init (CFS/EEVDF -> KThreads)]
                                                    |
                                                    v
                                            [VFS Init (Devfs -> Procfs -> Sysfs -> Rootfs)]
                                                    |
                                                    v
                                            [Driver Init (PCI -> Storage -> Input -> Net)]
                                                    |
                                                    v
                                            [Userspace Start (Init -> Shell Ready)]
```

## Failure Recovery Rules
1. Critical boot paths (Memory, Interrupts, Scheduler) panic with structured diagnostic reports upon invariant violation.
2. Non-critical driver probe failures log warnings and disable the failed device without halting boot.
3. Userspace crash events propagate signals or core dumps without corrupting kernel state.
