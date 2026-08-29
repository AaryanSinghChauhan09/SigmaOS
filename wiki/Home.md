# SigmaOS Wiki

Welcome to the SigmaOS Wiki! SigmaOS is the world's most advanced sovereign, bare-metal operating system for the next generation of silicon sovereignty.

## Project Overview and Mission
SigmaOS aims to provide a secure, modular, and highly performant operating system written entirely in Rust. It borrows the best concepts from various Linux and BSD distributions while enforcing strict memory safety and capability-based security.

## Quick Links
- [Components Table](wiki/Components-Table.md)
- [Architecture](wiki/Architecture.md)
- [Linux Distro Inspirations](wiki/Linux-Distro-Inspirations.md)
- [Getting Started](wiki/Getting-Started.md)
- [Security Model](wiki/Security-Model.md)
- [Package Management](wiki/Package-Management.md)
- [Roadmap](wiki/Roadmap.md)

## Key Features
| Feature | Description |
|---|---|
| Rust-native | Built entirely in Rust for memory safety |
| Microkernel | Highly modular microkernel architecture |
| sigpkg | Advanced multiformat package management |
| Sentinel | Strict capability-based security subsystem |
| OCI Integration | Native container runtime support |

## Architecture Overview
SigmaOS uses a capability-based microkernel design. It isolates drivers, filesystems, and networking stacks into separate user-space components, improving system stability and security.

## Getting Started
Head over to the [Getting Started](wiki/Getting-Started.md) guide to build and run SigmaOS in QEMU!
