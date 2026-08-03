# SigmaOS Sovereign Wiki

Welcome to the SigmaOS development wiki. This directory contains detailed architectural resources and specifications for our sovereign operating system.

## Recent Major Updates (Unreleased)
- **Comprehensive Branch Merge**: Successfully merged 6 new feature branches
- **Network Enhancements**: Wireshark-inspired traffic analyzer, NFS/rsync/samba/SSH/tcpdump compatibility
- **Kernel Improvements**: Enhanced kernel modules with Linux-inspired capabilities, process lifecycle management
- **Security Enhancements**: AppArmor with advanced modes, glob path matching, capabilities, network restrictions
- **Build System**: Linux-kernel inspired build system with configuration files
- **Package Management**: Universal package system improvements with PAM stack integration
- **Compatibility**: Comprehensive network compatibility tools for enterprise environments

## Table of Contents
1. [Core Microkernel Specification](../FUTURE-DEVELOPMENT-ROADMAP.md)
2. [Driver Subsystem and Hardware Integration](../FUTURE-DEVELOPMENT-ROADMAP.md)
3. [Zenith Desktop and Compositor Architecture](../FUTURE-DEVELOPMENT-ROADMAP.md)
4. [Accessibility and Localization Guide](./ACCESSIBILITY_LOCALIZATION_GUIDE.md)
5. [Release Governance and CI/CD](./RELEASE_GOVERNANCE_CI_CD.md)
6. [Universal Driver Support Plan](../UNIVERSAL_DRIVER_SUPPORT_PLAN.md)
7. [OS Structure Upgrade Plan](../OS_STRUCTURE_UPGRADE_PLAN.md)

## System Architecture Overview

SigmaOS decomposes the traditional monolithic kernel into specialized, isolated shards:

- **S-MM**: Sovereign Memory Manager (Buddy Allocator)
- **S-SCHED**: Predictive Multi-Priority Scheduler (MLFQ + CFS + EDF)
- **S-FS**: Sovereign Distributed Filesystem (VFS + SigmaFS)
- **S-SEC**: Security Framework (PQC + MAC + Sandbox)
- **S-AI**: AI Task Orchestrator (Local LLM routing)
- **S-COMPAT**: Cross-Platform Compatibility Layer (Windows NT, Linux, macOS, BSD)