# SovereignGSTCalculator Specification

## Regulatory Compliance

Designed specifically for **GST Act / Income Tax Act Compliance**.

## Architecture

Operates in an isolated Shard (Ring-3) with zero high-level dependencies.
Memory is allocated via `sigma_malloc` direct hardware paging to ensure secure, cryptographically attested execution.
