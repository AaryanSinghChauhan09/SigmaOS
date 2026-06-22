# SigmaOS Developer Ecosystem (DevEx)

SigmaOS features a natively integrated developer ecosystem, entirely eliminating the need for bloated, legacy Unix tools (GNU Make, CMake, APT). 

## 1. sigma-pkg (v2)
Our Sovereign package manager. Instead of central repositories, `.spkg` packages are distributed in a decentralized manner.
- **Dependency Resolution**: Uses a native SAT solver algorithm.
- **Post-Quantum Security**: Every package signature is verified using Kyber-1024 cryptography natively before installation.

## 2. sigma-build
A declarative build system designed natively for Sovereign environments. Eliminates CMake and Autotools.

## 3. Developer Hub (LSP & CI/CD)
SigmaOS provides a native Language Server Protocol (LSP) bridge. You can edit code in standard IDEs (VSCode, Neovim) and receive native AST telemetry straight from the SigmaOS API.
