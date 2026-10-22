# ADR-006: Mandatory Access Control and Sandboxing Capabilities

## Component
Security

## Current gap
Syscall sandbox lacks OpenBSD pledge promises and unveil path tree restriction.

## Upstream study
OpenBSD `pledge(2)` and `unveil(2)` syscall filters.

## Improvement
Implement structured promise bitmasks and unveil path tree validation in Rust.

## Scope
- Pledge promise enforcement
- Unveil path tree restrictions
- Denial audit logging
