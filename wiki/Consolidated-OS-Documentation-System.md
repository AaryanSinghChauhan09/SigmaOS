# Consolidated OS Documentation & Handbook System

Inspired by the **Arch Linux Wiki** and **FreeBSD Handbook**, this single-source-of-truth handbook provides comprehensive guides for operating and administering SigmaOS.

## Handbook Articles Index

| Article Title | Category | Distro Inspiration |
| :--- | :--- | :--- |
| **SigmaOS Installation & Deployment Guide** | System Administration | Arch Linux Wiki & FreeBSD Handbook |
| **Kernel Compilation & Custom Module Build Handbook** | Kernel Development | Gentoo Kernel Handbook & Debian Kernel Guide |
| **Universal Package Management Handbook** | Software Management | Arch Linux pacman Wiki & FreeBSD pkg Handbook |
| **Security Hardening & Multi-Sandbox Policy Guide** | Security & Auditing | OpenBSD Security Handbook & HardenedBSD |

---

## Detailed Handbook Articles

# SigmaOS Installation & Deployment Guide

**Category:** System Administration | **Inspiration:** Arch Linux Wiki & FreeBSD Handbook

Step-by-step installation guide for bare-metal, QEMU/KVM, and cloud instances.

## Troubleshooting & FAQ
1. Check UEFI Secure Boot status if bootloader fails to handoff.
1. Verify ZFS/Btrfs subvolume mounting flags in /etc/fstab.


---

# Kernel Compilation & Custom Module Build Handbook

**Category:** Kernel Development | **Inspiration:** Gentoo Kernel Handbook & Debian Kernel Guide

Guide to compiling sovereign kernel images, configuring microarchitecture optimization flags (x86-64 v1..v4/ARM), and building out-of-tree drivers.

## Troubleshooting & FAQ
1. Ensure target ISA flags match hardware capability (`sigma-fetch --json`).
1. Check initramfs microcode splicing if early boot panics occur.


---

# Universal Package Management Handbook

**Category:** Software Management | **Inspiration:** Arch Linux pacman Wiki & FreeBSD pkg Handbook

Comprehensive guide to managing 100+ package formats via sigma-pkg and DPLL constraint resolution.

## Troubleshooting & FAQ
1. Run `sigma-pkg --verify-signatures` if PQC attestation fails.
1. Use `sigma-pkg --rollback` to revert to previous boot environment snapshot.


---

# Security Hardening & Multi-Sandbox Policy Guide

**Category:** Security & Auditing | **Inspiration:** OpenBSD Security Handbook & HardenedBSD

Complete guide to process isolation, pledge/unveil restrictions, FreeBSD Capsicum capabilities, and Linux Landlock v5 policies.

## Troubleshooting & FAQ
1. Inspect audit log via `sigma-ctl status security-audit` for pledge violation alerts.
1. Verify memory protection status with `sigma-ctl status kernel-hardening`.


---
