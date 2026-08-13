# Linux Distro Inspirations

> Full catalog: [docs/LINUX_DISTRO_INSPIRATIONS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/LINUX_DISTRO_INSPIRATIONS.md)

SigmaOS absorbs the best ideas from 25+ Linux distributions, all reimplemented natively in safe Rust.

## Summary Table

| Distro | Key Concept Adopted | SigmaOS File |
|---|---|---|
| **Arch Linux** | Topological dependency resolution | `src/distro/linux_ideas.rs::NativeDependencyResolver` |
| **NixOS** | Hash-addressed immutable store | `src/distro/linux_ideas.rs::NixStyleStore` |
| **Fedora/RHEL** | A/B atomic update state machine | `src/distro/linux_ideas.rs::AtomicUpdateManager` |
| **Debian** | APT priority pinning | `src/distro/linux_ideas.rs::AptPinStore` |
| **Alpine Linux** | Slab pool / musl minimal footprint | `src/distro/linux_ideas.rs::SlabPool` |
| **Gentoo** | USE flags compile-time feature gating | `src/distro/linux_ideas.rs::UseFlags` |
| **openSUSE** | YaST-style config manager | `src/distro/linux_ideas.rs::YastConfigStore` |
| **Void Linux** | runit service supervisor | `src/distro/linux_ideas.rs::RunitSupervisor` |
| **Clear Linux** | CPU-topology thread affinity | `src/distro/linux_ideas.rs::CpuTopology` |
| **SteamOS** | GPU self-healing + thermal | `src/driver/device.rs` |
| **Qubes OS** | VM-domain isolation | `src/security/qubes_isolation.rs` |
| **CachyOS** | BORE/EEVDF kernel scheduler | `src/compatibility/cachy_os.rs` |
| **Garuda Linux** | Zen kernel + Dr460nized UI | `src/compatibility/garuda_zen.rs` |
| **Bodhi Linux** | Moksha/EFL desktop canvas | `src/compatibility/bodhi_moksha.rs` |
| **Mint Linux** | Update stability tiers | `src/compatibility/mint_linux.rs` |
| **Parrot OS** | AnonSurf anonymization | `src/security/parrot_parity.rs` |
| **Kali Linux** | Penetration testing toolkit | `src/compatibility/penetration_assistant.rs` |
| **EndeavourOS** | Welcome wizard layer | `src/compatibility/endeavour.rs` |
| **Fedora (SSSD)** | Offline credential caching | `src/compatibility/sssd.rs` |

## Implementation Highlights

### Arch: Rolling Dependency Resolution
Kahn's topological sort with cycle detection, entirely native (no std::collections):
```rust
let mut r = NativeDependencyResolver::new();
r.add_package("libssl".into(), vec![]);
r.add_package("curl".into(), vec!["libssl".into()]);
let order = r.resolve_order().unwrap(); // ["libssl", "curl"]
```

### NixOS: Content-Addressed Store
FNV-inspired hash for deduplication without external crates:
```rust
let mut store = NixStyleStore::new();
let idx = store.intern("pkg".into(), "1.0".into(), b"content");
// Same content → same index (deduplication)
```

### Gentoo: USE Flags
Type-safe feature flags as bitmask:
```rust
let flags = UseFlags::NONE
    .enable(UseFlags::IPV6)
    .enable(UseFlags::HARDENED)
    .enable(UseFlags::PQC);
assert!(flags.has(UseFlags::HARDENED));
```

### Fedora: Atomic A/B Updates
Full state machine for transactional OS updates:
```
Idle → Downloading → Staging → ReadyToApply → Applying → Applied
                                                              ↓
                                                     (health check fails)
                                                              ↓
                                                         RollingBack
```

## Native String Utilities (Reduce stdlib Dependency)

`src/distro/linux_ideas.rs::NativeStr` provides:
- `starts_with_bytes` / `ends_with_bytes`
- `trim_ascii`
- `split_on`
- `eq_ignore_ascii_case`
- `parse_u64` (no `FromStr` trait required)

All implemented without `std::str` or `std::string`.
