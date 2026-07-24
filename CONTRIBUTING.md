# Contributing to SigmaOS

Welcome. SigmaOS is built in the open — every kernel subsystem, driver, and doc is community-authored. Here's how to get involved.

---

## Quick Start

```bash
# 1. Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# 2. Set up upstream remote
git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git

# 3. Create a feature branch
git checkout -b feat/my-contribution

# 4. Make changes, build, test
make all -j$(nproc)
make test

# 5. Commit (conventional commits format)
git commit -m "feat(kernel): add buddy allocator initial implementation"

# 6. Push and open a PR
git push origin feat/my-contribution
```

---

## What to Work On

### Highest Impact (Phase G — blocks everything)
- `kernel/core/sigma_sched.cpp` — round-robin scheduler
- `kernel/core/sigma_mm.cpp` — buddy + slab allocator
- `kernel/mm/sigma_vmm.cpp` — x86_64 page table walker
- `kernel/core/sigma_irq.cpp` — APIC + PIC init
- `kernel/core/sigma_syscall_dispatch.cpp` — 30 syscalls
- `sigma-boot/sigma_boot.c` — UEFI loader (sigma-boot.efi)

### Good First Issues
- Look for `good first issue` label on GitHub Issues
- Fix markdown lint issues in `wiki_repo/`
- Add tests in `tests/unit/`
- Improve error messages in CLI tools

### Driver Contributions
See [Driver-Development](Driver-Development) for the SDF template.
Priority: VESA framebuffer → VirtIO-GPU → Intel i915 → iwlwifi

---

## Commit Convention

```
<type>(<scope>): <short description>

[optional body]
[optional footer: Fixes #123]
```

**Types:** `feat` · `fix` · `docs` · `test` · `refactor` · `perf` · `build` · `ci`

**Scopes:** `kernel` · `driver` · `net` · `fs` · `security` · `crypto` · `ui` · `pkg` · `docs`

**Examples:**
```
feat(kernel): add round-robin scheduler for 64 tasks
fix(net): resolve TCP SYN retransmit race condition
docs(wiki): add kernel internals page
test(security): add fuzz test for Kyber-1024 KEM
```

---

## PR Guidelines

1. **One concern per PR** — don't mix driver work with kernel changes
2. **CI must be green** — `sigma_ci.yml` runs on every push
3. **Update manifests**: if fixing a bug, mark it resolved in `CURRENT_PROBLEMS_MANIFEST.md`
4. **New subsystem** = new wiki page in `wiki_repo/`
5. **Kernel changes** = include QEMU boot log or smoke test output in PR description
6. **No force-push** to `main` — new commits only

---

## Code Standards

- **Language**: C++ (kernel, drivers, UI), C (low-level kernel paths), Rust (safe modules in `lib/`), Python/Shell (scripts)
- **Style**: clang-format enforced (see `.clang-format`); run `make format` before committing
- **Warnings**: `-Werror` enabled — zero warnings in CI
- **Headers**: place in `include/sigma_<subsystem>.h`; use `#pragma once`
- **Memory**: no `new`/`delete` in kernel code — use `kmalloc`/`kfree`
- **No global state**: use `SovereignEngine` singleton pattern (see existing subsystems)

---

## Testing

```bash
# Unit tests
make test

# Run a specific subsystem's tests
make test SUITE=kernel
make test SUITE=security
make test SUITE=net

# Fuzz testing
make fuzz          # all fuzz targets
make fuzz TARGET=pqc   # PQC crypto only

# QEMU smoke test
make qemu-test

# Static analysis
make lint
make clang-tidy
```

---

## Governance

See [GOVERNANCE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/GOVERNANCE.md) for roles, RFC process, and decision-making policy.

---

*See also: [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) · [Building-from-Source](Building-from-Source) · [Branch-Guide](Branch-Guide)*
