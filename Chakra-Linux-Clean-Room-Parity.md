# Chakra Linux Clean-Room Parity & Half-Rolling Architecture in SigmaOS

## Overview

SigmaOS incorporates the **half-rolling release model** and pure Qt/KDE application isolation principles pioneered by **Chakra Linux**.

---

## Key Modules

- [`src/compatibility/chakra.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/compatibility/chakra.rs): Chakra package bundle system, half-rolling repository manager, and Akabe package tool.
- [`src/sigpkg/mod.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/mod.rs): Integrated half-rolling dependency resolution.

---

## Core Principles

1. **Half-Rolling Core**:
   - Base system (Kernel, Drivers, Core klib) remains on ultra-tested stable releases.
   - Desktop environments and userland applications follow rolling updates.
2. **Bundle Isolation**:
   - Legacy GTK or external non-native applications execute inside self-contained bundles (`.bundle`) with private dependencies to prevent root filesystem contamination.
