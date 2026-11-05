# Kernel Component Rules

## Component owner
- Owner: kernel-agent

## Upstream references
- Upstream: Linux kernel, BSD kernel, xv6

## Purpose
Formalized design and architectural boundaries for the Kernel subsystem in SigmaOS.

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
