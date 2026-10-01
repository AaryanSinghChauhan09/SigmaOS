# Architecture Decision Records (ADRs)

## Overview
This document records key architectural decisions for SigmaOS.

## Decision 001: Zero-Dependency Pure Rust Kernel Core
- **Status:** Accepted
- **Context:** Core operating system components require high reliability, memory safety, and standalone execution without external dynamic libraries.
- **Decision:** All core kernel and packaging modules use pure Rust with `#![no_std]` compatibility and zero dynamic C runtime dependencies.

## Decision 002: Universal Multi-Format Transpilation Pipeline
- **Status:** Accepted
- **Context:** Support package formats across Linux, BSD, Mobile, and Desktop ecosystems.
- **Decision:** Transpile foreign packages into native `.sigpkg` format with format-specific sandboxing and capability enforcement.
