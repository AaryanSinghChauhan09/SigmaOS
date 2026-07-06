# Unified Package Manager (`sigpkg`)

`sigpkg` is SigmaOS's bespoke package manager, built entirely in `no_std` Rust. It absorbs the best features of apt, dnf, and pacman while operating without dynamic memory allocation (no heap).

## Features

1. **Static DB:** Maintains installed package states in a static array, avoiding fragmented database corruptions.
2. **SAT-Lite Resolution:** Uses a zero-allocation ring buffer queue to resolve and topological-sort dependencies before installation.
3. **PQC Signatures (WIP):** All packages `.spkg` are structurally validated via Ed25519 (and planned Dilithium/Kyber Post-Quantum) signatures against trusted repo public keys.
4. **Mirrors:** Automatically rotates through redundant mirrors on latency timeout.

## Core Modules

* `sigma_pkg_core.rs`: Main install, search, and remove entrypoints.
* `sigma_pkg_dep.rs`: SAT-lite dependency solver.
* `sigma_pkg_repo.rs`: Repository configuration and signature validation.
