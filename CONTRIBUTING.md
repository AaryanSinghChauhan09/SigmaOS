# Contributing to SigmaOS

We welcome contributors! As a meritocratic project, we value high-quality code, clear documentation, and a focus on sovereign computing.

## ðŸ›  Setup

1. Clone the repo.
2. Ensure you have a C++20 compatible compiler (GCC 12+, Clang 15+).
3. Follow the [Installation Guide](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki/blob/master/INSTALLATION_GUIDE.md).

## ðŸ§­ Branching Strategy

* `main`: Stable, production-ready code.
* `develop`: Integration branch for new features.
* `feature/*`: Individual feature shards.
* `fix/*`: Bug fixes.

## ðŸ“ Coding Standards

* **OOP Isolation**: All new shards must inherit from `SigmaObject`.
* **No Raw Pointers**: Use Sovereign smart pointers or reference-counted objects.
* **Documentation**: Every public method must be documented in the header.

## ðŸ—³ Sovereign Council

Major architectural changes require an RFC and approval from the [Sovereign Council](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki/blob/master/GOVERNANCE_CHARTER.md).
