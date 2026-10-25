# SigmaOS Device Driver Development and Maintenance Plan

## Architectural Overview and Baseline
SigmaOS lists drivers for serial, RTC, display, input, storage, USB, audio, and networking, but cautions that a driver module or passing unit tests do not prove runtime or hardware support. The inventory gives concrete next steps for several drivers: VirtIO block needs QEMU PCI discovery, VirtIO network needs Ethernet integration, NVMe and AHCI need real DMA/device testing, and display needs a real GPU/page-flip path. The wiki's status vocabulary—Proposed, Prototype, Integrated, Supported—should govern all driver claims.

The sequence below prioritizes driver infrastructure and one virtual device end to end before broad physical-hardware claims. Linux and FreeBSD both emphasize managed device/resource lifecycles; PCI probing, DMA setup, interrupts, and teardown must be treated as one connected correctness and safety problem.

## Driver Rules for Every PR
- **Separate device discovery from driver logic**: Define a bus/device model with typed IDs and resource descriptions. Let bus enumeration match devices to drivers; do not have each driver scan raw hardware independently. ACPI and PCI enumeration illustrate why firmware-described devices and bus-discoverable devices need distinct paths.
- **Use explicit probe/attach and remove/detach lifecycles**: A failed probe releases everything acquired; detach quiesces the device and releases resources in a documented order. Linux's driver model expects failed probes to free allocated resources and remove callbacks to release driver-owned state.
- **Never treat a DMA address as a CPU physical or virtual address**: Add a platform DMA API for mapping, unmapping, direction, masks, cache synchronization, and errors. DMA addresses may be translated by an IOMMU or host bridge; map/unmap failures must not be ignored.
- **Define interrupt context**: Interrupt handlers must be short, non-blocking, and limited to operations safe in interrupt context; defer lengthy or sleeping work to a worker/task mechanism.
- **Specify concurrency and memory ordering**: For each queue, descriptor, and MMIO access, document ownership, locking, barriers, and ordering. Coherent DMA memory still may require barriers before notifying a device.
- **Quiesce before freeing**: Disable device interrupts, stop DMA, drain/cancel work, synchronize with in-flight handlers, then free buffers and MMIO/resources. Linux PCI guidance warns that freeing DMA buffers before the device has stopped can cause corruption or crashes.
- **Use safe Rust boundaries**: Keep volatile MMIO, raw pointers, DMA, and architecture-specific operations behind narrow unsafe abstractions with documented safety contracts; driver-level code should use safe subsystem APIs where available.
- **Test the real path**: Distinguish parser/model/unit tests from QEMU integration and physical-device tests. Linux separates internal unit tests from userspace/selftests and end-to-end testing; SigmaOS should make the same evidence distinction.
- **Don't claim broad support from one test machine**: Document exact device IDs, firmware, bus, architecture, QEMU machine/device configuration, and operations tested.
- **Use secure contribution controls**: Require review and successful checks for driver and DMA changes; protect the default branch. GitHub supports required PR reviews, status checks, and restrictions on force pushes.

## Proposed PR Sequence

| PR | Proposed Title | Main Work | Acceptance Criteria |
|---|---|---|---|
| 1 | docs(drivers): audit driver status and define support evidence | Reconcile driver source, tests, component wiki, FEATURE_STATUS.toml, and advertised hardware. Add a per-driver table for bus, device IDs, architecture, tested operations, failure behavior, and limitations. | Every driver is explicitly Proposed, Prototype, Integrated, or Supported; compile/unit/QEMU/hardware evidence are separated; unsupported devices are not implied. |
| 2 | feat(drivers): define device, bus, and driver lifecycle interfaces | Add typed device descriptors, bus enumeration interfaces, ID matching, driver registration, per-device state, and probe/remove callback semantics. | A mock bus can enumerate, match, probe, reject, and remove devices; duplicate ownership and failed-probe cleanup are tested. |
| 3 | feat(drivers): add managed resource acquisition and rollback | Create RAII/resource-guard patterns for MMIO mappings, IRQ registrations, DMA mappings, queues, and subsystem registrations. | Every acquisition has one cleanup path; injected failure at every probe stage leaves no leaked resource or registered callback. |
| 4 | feat(mmio): add validated register access and ordering APIs | Add typed MMIO mapping and width-aware volatile reads/writes; define alignment, bounds, endianness, and posted-write behavior. | Drivers cannot construct MMIO access without an owned mapping; out-of-range accesses are rejected in tests; ordering/flush requirements are documented. |
| 5 | feat(dma): add device DMA mapping API with masks and ownership | Add coherent and streaming mapping abstractions, address-width masks, directions, sync operations, and mapping failure results. | CPU virtual, CPU physical, and device DMA addresses are distinct types; failed mapping blocks device start; map/unmap/sync pairing is tested. |
| 6 | feat(irq): define interrupt registration and deferred work contract | Add IRQ registration/removal, handler context rules, safe shared-device handling, and a deferred work queue. | Handler cannot sleep or allocate unbounded resources; detach waits for in-flight handlers; tests cover spurious interrupts and teardown races. |
| 7 | feat(pci): add QEMU PCI enumeration and resource ownership | Enumerate PCI config space, validate IDs/BARs, allocate resource ranges, and expose a device tree for driver matching. | QEMU PCI devices are enumerated once; malformed/unavailable resources fail safely; overlapping BAR ownership is rejected; no driver probes by raw global scan. |
| 8 | feat(drivers): integrate VirtIO block through PCI discovery | Connect the existing VirtIO block module to PCI discovery, feature negotiation, queue setup, DMA, interrupts/polling, and VFS block interface. | QEMU can identify a documented VirtIO block device, read/write/flush test sectors, handle device reset/errors, and shut down cleanly. The current inventory calls PCI discovery the next milestone. |
| 9 | feat(drivers): connect VirtIO network queues to Ethernet stack | Integrate the partial VirtIO network driver with interface lifecycle, TX/RX buffers, DMA, and Ethernet receive/transmit paths. | QEMU NIC sends/receives frames through the real stack; queue reset, malformed descriptors, and link state are tested. Inventory currently marks this integration incomplete. |
| 10 | feat(drivers): add device diagnostics and health reporting | Add standard per-device identity, bound driver, state, last error, counters, and bounded diagnostic snapshots; add reset/recovery hooks where safe. | Diagnostics are read-only by default and avoid secrets; a failed device is distinguishable from a missing driver; recovery is opt-in and tested. Linux’s devlink health model is a useful reference for diagnosis and controlled recovery. |
| 11 | feat(drivers): implement suspend resume and shutdown contracts | Define power-state transitions and driver callbacks; stop DMA and interrupts before suspend/reboot; restore state on resume. | QEMU or mocks test suspend/resume state order, partial failure, shutdown, and wakeup policy; unsupported power management is reported honestly. |
| 12 | test(drivers): add fault-injection and lifecycle test harness | Add alloc/map/IRQ/timeout/device-reset failure injection; test probe rollback, disconnect during I/O, and repeated bind/unbind. | Each failure point has a regression test; resource counts return to baseline; no hangs, leaks, UAFs, or unbounded retries. Linux has fault-injection facilities for allocation and I/O failures that provide a useful pattern. |
| 13 | test(drivers): add QEMU hardware matrix and boot smoke checks | Automate supported QEMU machines and devices for PCI, VirtIO, serial, storage, and input; preserve logs and configuration. | CI verifies discovery, bind, basic I/O, error path, and teardown on named configurations; external dependencies are avoided or isolated. |
| 14 | security(drivers): add driver threat model and DMA isolation policy | Document untrusted-device assumptions, DMA confinement/IOMMU plans, firmware policy, privileged register access, and attack surfaces. | Every driver states trust assumptions and bounds; DMA cannot target arbitrary memory by default where an IOMMU/architecture policy exists; missing isolation is a release limitation. |
| 15 | docs(drivers): publish device support matrix and recovery guide | Document known hardware and QEMU devices, firmware requirements, validated features, known issues, and recovery/troubleshooting. | “Supported” entries have repeatable evidence and exact configuration; unsupported hardware and limitations are visible to users. |

## Recommended Implementation Order
Start with PRs 1–7, then choose VirtIO block as the first end-to-end vertical slice because the inventory already identifies PCI discovery as a specific blocker and the result provides a concrete storage path for boot/test images. Follow with VirtIO networking, serial/input, then physical NVMe/AHCI/display. Do not expand to multiple physical devices before resource cleanup, DMA abstractions, IRQ teardown, and QEMU testing are reliable.

## Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring PR/Task | Required Evidence |
|---|---|---|
| Every PR | ci(drivers): run checks for changed driver layers | Format/build, unit tests, static checks, and relevant bus/DMA/IRQ/integration tests; skipped checks explained. |
| Weekly | maint(drivers): triage hardware and QEMU regressions | Failures are tied to a reproducer, supported configuration, owner, and issue/PR; stale “supported” claims are flagged. |
| Weekly | test(drivers): review fuzz and malformed-device findings | Parser/descriptor crashes minimized; fixed issues converted to regression tests; device inputs and fuzz bounds documented. |
| Monthly | audit(drivers): verify probe rollback and detach cleanup | Resource acquisition and reverse-order cleanup reviewed for touched drivers; fault-injection results recorded. |
| Monthly | audit(dma): check mapping ownership and synchronization | Every mapping has direction, lifetime, error handling, and paired unmap/sync behavior; descriptor barriers are reviewed. |
| Monthly | test(drivers): run QEMU discovery and basic I/O matrix | Named machine/device configurations, boot logs, data integrity results, errors, and teardown results. |
| Quarterly | docs(drivers): review device IDs, firmware and compatibility claims | Device-ID source, firmware version/license, architecture support, and known limitations are revalidated. |
| Quarterly | security(drivers): review DMA, MMIO, and device trust boundaries | Threat model, address bounds, DMA confinement, firmware trust, privileged actions, and recovery reviewed. |
| Each release candidate | release(drivers): certify supported hardware matrix | Exact tested QEMU and physical devices, firmware revisions, operations, failures, and unresolved limitations. |
| After each incident | fix(drivers): prevent recurrence of `<failure>` | Root cause, affected devices, regression test, impact, and backport/release decision. |

Maintenance cadence should be a reviewed task queue, not automatic code churn. Avoid claiming a physical-device regression is fixed solely because a mock test passes; record the exact test tier.

## Component-Specific PR Rules
- **PCI and bus enumeration**: Validate IDs and BAR sizes; reserve resources before mapping; avoid overlapping MMIO; handle absent devices and hotplug/remove if supported. Driver matching should be bus-mediated and type-safe.
- **ACPI and firmware discovery**: Separate firmware-described devices from natively enumerable bus devices; validate table checksums and resource ranges; reject inconsistent or unsafe descriptions. ACPI enumeration shows that platform/serial-bus devices and PCI-discoverable devices require different representation.
- **DMA and queues**: Define descriptor ownership transitions, alignment, maximum length, address width, mapping direction, barriers, cache coherency, and error cleanup. Never assume DMA addresses equal CPU physical addresses.
- **Interrupt handlers**: Acknowledge/identify the source promptly, avoid blocking operations, and defer work. Detach must prevent new interrupts and synchronize with handlers already running. FreeBSD’s PCI handbook explicitly warns to quiesce the interrupt stream and remove/synchronize the handler during detach.
- **Storage**: Validate command fields and completion lengths; bound retries/timeouts; flush and error behavior must be defined; data-integrity tests should cover partial I/O and reset.
- **Networking**: Validate descriptors and packet lengths before passing buffers to the network stack; ensure RX/TX ownership, reset, and link lifecycle are tested.
- **USB/input/audio/display**: Keep bus/core responsibilities separate from device-specific drivers; validate event and transfer lengths; document DMA and interrupt paths; use subsystem APIs instead of custom one-off user interfaces.
- **Power and hotplug**: Device state transitions must be explicit. Suspend/shutdown/removal should stop new I/O, quiesce hardware, drain work, and release resources in a deterministic order. Linux’s PCI lifecycle lists disable interrupts, stop DMA, release buffers, unregister subsystems, then release mappings/resources.
- **Firmware**: Specify firmware source, version compatibility, integrity/authenticity expectations, license and redistribution status, and failure behavior. Do not silently load an incompatible firmware image.
- **Rust safety**: Place unsafe only in small MMIO/DMA/interrupt abstractions with safety comments and tests; keep leaf drivers on safe APIs. This mirrors Rust-for-Linux guidance to encapsulate unsafe bindings behind reviewed abstractions.
- **Diagnostics**: Include device identity, driver name, state, and actionable error codes; rate-limit repetitive logs; do not dump unbounded device-controlled data.

## PR Review Template and Merge Gates
Use this template for driver changes:

```text
## Summary
What device capability, bug, or integration gap does this PR address?

## Device and scope
Bus, vendor/device or compatible IDs, architecture, subsystem, and exact operations.

## Lifecycle
Probe/attach steps, resources acquired, failure rollback, remove/detach, reset,
suspend/resume, and shutdown behavior.

## DMA and concurrency
DMA mask/direction, buffer ownership, sync/barriers, interrupt context,
locking, and deferred work.

## Security and failure handling
Untrusted inputs, MMIO bounds, timeouts, retry limits, firmware assumptions,
DMA isolation, and recovery behavior.

## Tests and evidence
Unit/mock, fault injection, QEMU, and physical-device tests.
List exact device/machine/firmware configuration and anything not tested.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated support matrix, component page, and test evidence.

## Checklist
- [ ] Probe failure releases every acquired resource
- [ ] Detach quiesces IRQs and DMA before freeing memory
- [ ] DMA addresses are distinct from CPU addresses
- [ ] Interrupt handler is bounded and non-blocking
- [ ] Malformed device input and timeout paths are tested
- [ ] Documentation and maturity status match evidence
- [ ] Required CI passes; review conversations are resolved
```

Require one maintainer approval for isolated documentation or mock tests; require two relevant reviewers for DMA, interrupt teardown, unsafe Rust, bus resource allocation, firmware loading, or security-sensitive drivers. Require passing checks and protect the default branch against unreviewed direct changes. Linux’s driver submission process emphasizes subsystem review, while FreeBSD’s committer guide requires non-trivial changes to be reviewed and asks kernel changes to compile across relevant configurations and architectures.

## Industry References and Evidence Bar
Use Linux’s PCI, DMA, driver-model, and kernel-testing documentation as design references; use FreeBSD’s Newbus/PCI and bus-resource model as an alternative BSD reference. These are not promises of API or driver compatibility. Linux’s DMA guide specifically explains that a device’s bus address can differ from the CPU physical address and requires mapping/error-handling discipline; Linux’s PCI guide gives an explicit shutdown order; FreeBSD likewise documents bus-managed resources and interrupt teardown.

For testing, combine isolated logic tests with fault injection and real QEMU/device tests. Linux distinguishes KUnit’s in-kernel white-box tests from kselftest’s userspace feature tests, and recommends using both where appropriate; apply that separation in SigmaOS, rather than treating all tests as equivalent evidence.

Track any proposed “missing component” as a PR with a concrete user need and dependencies. High-priority foundational gaps include a unified device/bus model, managed resource cleanup, a real DMA API, interrupt/deferred-work contracts, hardware/firmware inventory, and repeatable QEMU integration. Do not add a large driver catalogue ahead of those foundations.

### Exit Criterion
- Mark a driver **Integrated** only after the documented boot target enumerates and binds the device, exercises its real subsystem path, and handles at least one failure safely.
- Mark a driver **Supported** only after repeatable tests cover device operations, lifecycle/teardown, error recovery, and exact hardware/firmware scope. This matches the repository’s own warning that module tests and driver names do not prove hardware support.
