# Drivers Component Rules

## Component owner
- Owner: driver-agent

## Upstream references
- Upstream: VirtIO, ACPI, NVMe, USB, PCI

## Purpose
Formalized design and architectural boundaries for the Drivers subsystem in SigmaOS.

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
