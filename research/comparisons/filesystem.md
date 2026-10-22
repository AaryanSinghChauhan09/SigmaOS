# Comparative Analysis: Filesystem Architecture

| Operating System | Mechanism | Primary Data Structure | Isolation & Recovery | SigmaOS Decision |
|---|---|---|---|---|
| xv6 | Inode FS | Buffer Cache & Log | None | Adopted 4-layer logging concept |
| Linux Kernel | Trait VFS | `dentry` / `inode` VTable | DAC + LSM Hooks | Adopted Trait VFS & Mount Namespaces |
| OpenBSD | VFS + unveil | Radix Path Tree | Unveil Path Restrictions | Adopted Unveil Path Trees |
| SigmaOS | Safe Rust VFS | `Arc<Spinlock<VNode>>` | Unveil Trees + Transaction Log | Native Implementation |

## Tradeoff Analysis
Trait-based VFS ensures memory safety in Rust while maintaining POSIX interface compatibility and OpenBSD unveil path isolation.
