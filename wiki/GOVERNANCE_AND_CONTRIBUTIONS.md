# SigmaOS Subsystem Governance and Contribution Model

To ensure structured development across kernel, security, and utility boundaries, SigmaOS adopts a modular subsystem maintainer system inspired by the Linux Kernel development guidelines.

---

## 1. Subsystem Tree & Maintainer Structure

SigmaOS is organized into distinct sub-trees, each overseen by designated maintainers responsible for code reviews, architectural consistency, and pull request sign-offs:

```
                          [ Kernel Core Tree ]
                         (Maintainer: Core Team)
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
[ Subsystem: Drivers ]    [ Subsystem: Network ]     [ Subsystem: Security ]
 (Maintainer: Dev-A)       (Maintainer: Dev-B)        (Maintainer: Dev-C)
```

- **Core Tree (`src/kernel/`, `src/klib/`)**: Oversees memory allocations, scheduling loops, and IPC.
- **Drivers Subtree (`src/driver/`, `src/drivers/`)**: Covers GPU, USB, audio, and device registration.
- **Network Subtree (`src/net/`, `src/network/`)**: Handles socket states, protocol headers, and TLS validation.
- **Security Subtree (`src/security/`, `src/container/`)**: Enforces LSMs, namespaces, pledge/unveil, and crypto filters.

---

## 2. Developer Certificate of Origin (DCO)

All code contributions must be certified under the Developer Certificate of Origin by appending a `Signed-off-by` tag to git commit messages:

```
Signed-off-by: Jane Doe <jane.doe@example.com>
```

By signing off, contributors assert that they have the right to submit the patch under the repository's open-source licensing terms.

---

## 3. Pull Request Guidelines

1. **Self-Sufficiency**: Submissions must not add dependencies on external crates or predefined dynamic link libraries (`libc`).
2. **Safety Documentation**: Any `unsafe` blocks introduced must contain a safety comment outlining the boundary checks in place.
3. **Regression Tests**: Changes to core architectures must include corresponding integration test blocks.
