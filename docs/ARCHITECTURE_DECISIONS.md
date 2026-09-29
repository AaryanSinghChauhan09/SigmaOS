# SigmaOS Architecture Decisions Record (ADR)

## Overview
This document records key architectural decisions for the SigmaOS operating system.

## ADR 001: Pure Rust Memory Safety Userland & Kernel
- **Status**: Accepted
- **Context**: Operational stability and self-sufficiency requirement.
- **Decision**: All core userland utilities, desktop environments, and kernel components are developed in safe memory-managed Rust.

## ADR 002: Universal Subsystem Absorption
- **Status**: Accepted
- **Context**: Interoperability across Linux, BSD, and Windows workflows.
- **Decision**: Native adapters and universal package bridge engines support multi-distro package formats and POSIX/BSD CLI dialects.
