# SigmaOS Shell Maintenance & Operating Guide

This document outlines the maintenance standards, execution lifecycle, safety policies, and conformance guidelines for the SigmaOS Shell subsystem.

## Key Maintenance Rules
- **Deterministic Script Execution**: Scripts run deterministically with predictable environment defaults.
- **Explicit Exit Status**: Commands return standard POSIX exit status codes (0 for success, 2 for parse errors, 127 for command not found, 128+N for signals).
- **No Partial Execution on Parse Error**: Parsing failures halt execution prior to running side-effects.
- **Quoting & Expansion Safety**: Quoting rules and expansion order sequence are strictly maintained.
