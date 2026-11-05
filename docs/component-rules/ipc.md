# IPC Component Rules

## Component owner
- Owner: ipc-agent

## Upstream references
- Upstream: POSIX IPC, Linux pipes, D-Bus, Android Binder

## Purpose
Formalized design and architectural boundaries for the IPC subsystem in SigmaOS.

## Mandatory rules
- All code changes must provide verified proofs and automated tests.
- Zero-dependency `#![no_std]` core where required.

## Prohibited
- Direct commits without branch reviews.
- Unverified capability claims.

## Required evidence
- Unit test logs and integration test outputs.
- Runtime execution benchmarks.

## Promotion criteria
- Must fulfill ADR, testing, and security review gates according to promotion level.
