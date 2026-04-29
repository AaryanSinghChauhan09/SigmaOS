# 🤝 Contributing to SigmaOS

> Welcome to the Sovereign Lattice. We're building the OS that surpasses Linux distros by combining industrial-grade security, AI-native UX, and bare-metal performance.

---

## Code of Conduct
- Be respectful, precise, and technically rigorous
- All code must pass CI before merging — no exceptions
- Document every shard with the standard header block

---

## What We Need

| Priority | Area | Example Tasks |
|----------|------|---------------|
| 🔴 Critical | Security hardening | Fix CWE-119 buffer patterns using `sigma_hardened_strcpy` |
| 🔴 Critical | CI/CD | Improve CodeQL coverage, add ARM cross-compile jobs |
| 🟠 High | New shards | Implement shards from the INFINITE_SHARDS roadmap |
| 🟠 High | Wiki | Architecture diagrams, developer tutorials |
| 🟡 Medium | Driver support | Implement silicon-native Wi-Fi, GPU drivers |
| 🟢 Low | UX polish | Theme engine tweaks, accessibility improvements |

---

## Coding Requirements

All contributions MUST follow the [Developer Guide](DEVELOPER_GUIDE.md). Key rules:

### ❌ Never Use
```cpp
#include <iostream>    // Use sigma_hal.h
#include <vector>      // Use custom arrays
#include <string>      // Use sigma_hardened_strcpy
strcpy(d, s)           // Use sigma_hardened_strcpy(d, s, MAX)
sprintf(b, fmt, ...)   // Use sigma_hardened_snprintf(b, MAX, fmt, ...)
```

### ✅ Always Use
```cpp
#include <sigma_types.h>
#include <sigma_hal.h>
#include <sigma_sechardener.h>

void my_api(const void* input, uint32_t size) {
    if (!input || size == 0) return;  // Input validation first
    sechardener_validate_buffer(input, size, CAPACITY);
    // ... logic
}
```

---

## Branch Strategy

```
main              ← Production-ready. Protected. CI must pass.
feat/<name>       ← New shards and features
fix/<cwe-id>      ← Security fixes (e.g., fix/cwe-119-buffer-hardening)
docs/<topic>      ← Wiki and documentation improvements
ci/<improvement>  ← CI/CD pipeline changes
```

---

## Pull Request Checklist

Before opening a PR, verify:

- [ ] `python3 scripts/lattice_coverage.py` — shard count increases
- [ ] `cppcheck --enable=all -Iinclude kernel/core/SovereignMyShared.cpp` — no errors
- [ ] No `<iostream>`, `<vector>`, `<string>` in kernel code
- [ ] No raw `strcpy`, `sprintf`, or `gets` calls
- [ ] All public APIs validate `NULL` inputs
- [ ] Implementation follows `Sovereign<Name>.cpp` naming
- [ ] Header follows `sigma_<name>.h` naming
- [ ] Documentation block present in implementation
- [ ] CI pipeline passes all 6 jobs ✅

---

## Issue Templates

When filing a bug or feature request, use these labels:

| Label | Use For |
|-------|---------|
| `security` | CWE violations, buffer issues, privilege escalation |
| `new-shard` | Proposing a new sovereign shard |
| `ci-cd` | CI/CD pipeline improvements |
| `wiki` | Documentation updates |
| `performance` | Scheduler, allocator, or throughput improvements |
| `ux` | User interface and experience improvements |

---

## Getting Help

- Open a GitHub Discussion for architectural questions
- File an Issue for bugs or feature proposals
- Check the [Architecture Overview](Architecture_Overview.md) for design context
- Check the [Developer Guide](DEVELOPER_GUIDE.md) for setup instructions

---

*"The Sovereign Lattice grows stronger with every contribution."*
