# rust-vmm Crates Integration

## Overview

[rust-vmm](https://github.com/rust-vmm) is a collection of Apache-2.0 / MIT Rust crates for building Virtual Machine Monitors (VMMs). SigmaOS uses these crates to build its internal VMM layer for the microkernel and cloud profiles.

---

## Crates Used

| Crate | Version | Purpose |
|---|---|---|
| `vm-memory` | 0.14.1 | Guest physical memory abstractions (`GuestMemoryMmap`) |
| `vmm-sys-util` | 0.12.1 | Linux KVM/eventfd/epoll wrappers |
| `virtio-bindings` | 0.2.2 | Low-level virtio C struct bindings |
| `linux-loader` | 0.11.0 | Load a Linux bzImage / ELF kernel into guest memory |
| `kvm-ioctls` | 0.16.0 | Safe Rust wrappers around `/dev/kvm` ioctls |
| `event-manager` | 0.4.0 | epoll-based I/O event dispatch |

---

## Vendoring

All crates are vendored into `virtualization/rust-vmm-crates/` using `cargo vendor`:

```
cargo vendor --manifest-path virtualization/Cargo.toml virtualization/rust-vmm-crates
```

Add to `.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "virtualization/rust-vmm-crates"
```

---

## Cargo.toml Snippet (Pinned Versions)

```toml
[dependencies]
vm-memory        = { version = "=0.14.1", features = ["backend-mmap"] }
vmm-sys-util     = "=0.12.1"
virtio-bindings  = "=0.2.2"
linux-loader     = "=0.11.0"
kvm-ioctls       = "=0.16.0"
event-manager    = "=0.4.0"
```

---

## Example: Using `vm-memory` GuestMemoryMmap

```rust
use vm_memory::{
    GuestAddress, GuestMemoryMmap, GuestMemory, Bytes, MmapRegion,
};

const GUEST_MEM_START: u64 = 0x0000_0000;
const GUEST_MEM_SIZE:  usize = 128 * 1024 * 1024; // 128 MiB

fn create_guest_memory() -> GuestMemoryMmap<()> {
    let regions = vec![
        (GuestAddress(GUEST_MEM_START), GUEST_MEM_SIZE),
    ];
    GuestMemoryMmap::from_ranges(&regions)
        .expect("Failed to create guest memory")
}

fn load_kernel(mem: &GuestMemoryMmap<()>, kernel_path: &str) -> u64 {
    use linux_loader::loader::{self, KernelLoader};
    let mut kernel_file = std::fs::File::open(kernel_path).unwrap();
    let entry = loader::bzfile::BzImage::load(
        mem,
        None,
        &mut kernel_file,
        Some(GuestAddress(0x0100_0000)), // load at 1 MiB
    )
    .unwrap();
    entry.kernel_load.0
}

fn main() {
    let guest_mem = create_guest_memory();
    let entry_point = load_kernel(&guest_mem, "/boot/sigma-vmlinux.bin");
    println!("Kernel entry point: {:#x}", entry_point);

    // Next: configure vCPUs, set sregs/regs, start KVM run loop
}
```

---

## Minimal VMM Prototype Boot Sequence

1. Open `/dev/kvm`, create VM and vCPU with `kvm-ioctls`
2. Allocate guest memory with `vm-memory::GuestMemoryMmap`
3. Load Linux bzImage via `linux-loader`
4. Set up initial CPU registers (CR0, CR4, EFER, CS, SS)
5. Configure PIT/APIC via `vmm-sys-util` eventfd
6. Enter KVM run loop; handle `KVM_EXIT_IO` (serial console)

---

## Exit Criteria

- Minimal Rust VMM prototype (`virtualization/sigma-vmm/`) boots a Linux bzImage in QEMU-KVM mode.
- Guest prints kernel boot messages to serial; VMM exits cleanly on `poweroff`.
- `cargo test -p sigma-vmm` passes with mock KVM backend.
