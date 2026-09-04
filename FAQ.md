# Frequently Asked Questions

## General

**Q: What is SigmaOS?**  
A: SigmaOS is a from-scratch operating system written in Rust. It uses a microkernel architecture with post-quantum cryptography, multi-distro compatibility, and AI integration.

**Q: Is SigmaOS ready for production use?**  
A: Not yet. SigmaOS is in active development (Beta stage). It is suitable for development, research, and testing.

**Q: What hardware does SigmaOS support?**  
A: Currently x86_64 (primary), with in-progress support for aarch64 and riscv64. QEMU/KVM is the primary testing platform.

**Q: Is SigmaOS based on Linux?**  
A: No. SigmaOS is a completely independent OS built from scratch in Rust. It provides compatibility with Linux software through adaptation layers.

## Technical

**Q: Why Rust?**  
A: Rust provides memory safety without garbage collection, eliminating entire classes of vulnerabilities (buffer overflows, use-after-free, race conditions) at compile time.

**Q: Why a microkernel?**  
A: Microkernels have a smaller trusted computing base, making security analysis and formal verification more tractable. Bugs in drivers cannot crash the entire kernel.

**Q: What's `#![no_std]`?**  
A: The kernel core uses `no_std` mode, meaning it doesn't depend on the Rust standard library. This allows it to run without an underlying OS.

**Q: How does SigmaOS run Debian/Arch/Fedora software?**  
A: Through the multi-distro compatibility layer (`src/compatibility/`), which translates package formats, system calls, and environment expectations.

**Q: What is the `unimplemented_features.rs` file?**  
A: It's a stub registry for planned features that aren't yet implemented. Functions there return `todo!()` and are tracked as development backlog.

## Contributing

**Q: How do I get started contributing?**  
A: See [[Contributing]] and [[Building-from-Source]]. Start with `cargo +nightly check` and look for issues labeled `good-first-issue`.

**Q: There are 296 compilation errors. Is the codebase broken?**  
A: The main branch has been substantially fixed (all branches merged). Run `cargo +nightly check` for current error count.

**Q: Where do I report security issues?**  
A: See [[Security-Policy]]. Do NOT use public GitHub issues for vulnerabilities.
