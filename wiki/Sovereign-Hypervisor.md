# Sovereign Bare-Metal Hypervisor

A Type-1 hypervisor built directly into SigmaOS to run isolated guest VMs
without depending on Linux KVM.

## Design

- **VT-x / AMD-V** on x86_64; **EL2** on AArch64

- Each guest VM is a capability-gated shard — the hypervisor is just another
  kernel module, not a privileged monolith

- Live migration via SovereignFS snapshot deltas

## Roadmap

- [x] VMCS/VMCB setup (x86_64)
- [x] Guest memory isolation (EPT / NPT)
- [x] Virtio-net / Virtio-blk para-virtual devices
- [x] Live migration prototype
- [ ] ARM64 EL2 virtualization support
- [ ] Nested virtualization
- [ ] GPU passthrough
- [ ] SR-IOV support
- [ ] Hypervisor management API
- [ ] VM snapshot and restore
