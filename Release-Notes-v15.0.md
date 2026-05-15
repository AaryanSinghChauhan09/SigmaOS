# SigmaOS Zenith v15.0 Release Notes

## Overview

SigmaOS Zenith v15.0 represents a major milestone in stabilizing the microkernel infrastructure, enhancing security, and resolving systemic compilation and dependency vulnerabilities.

## Security Fixes & Vulnerability Remediation

- **Code Scanning Resolution**: Addressed potential unsafe memory handling, unchecked input validation, and race conditions in kernel/shard orchestration by enforcing CodeQL scanning across all branches (`main`, `release/app`, `release/browser`, `release/dual-boot`, `release/standalone`).
- **Dependency Updates (Dependabot)**: Fixed dependency CVEs including prototype pollution, SSRF, command injection, and weak cryptographic defaults.
- **Bootloader Isolation**: Audited the bootloader code in the `release/dual-boot` branch to ensure strict partition isolation and validate persistence policies.
- **Workflow Enhancements**: Configured automated GitHub Actions workflows for CodeQL and dependabot to maintain continuous security auditing.

## Architecture & Performance Improvements

- **Microkernel Orchestration**: Modularized the kernel and optimized shard orchestration routines.
- **Relative Path Refactoring**: Executed a systemic refactoring of include hierarchies to use depth-aware relative paths (`../../../include/SigmaOOP.hpp`), fully resolving compilation debt.
- **SigmaSingleton Stabilization**: Resolved template specialization issues preventing `getInstance()` resolution across driver subsystems.
- **App & Browser Optimization**: Patched SSL/TLS libraries, benchmarked rendering engines, and optimized API calls to enhance UI responsiveness and WASM runtime security.

## Summary

These changes ensure SigmaOS adapts securely and efficiently across monolithic, microkernel, distributed, real-time, and cloud-virtualized deployments.
