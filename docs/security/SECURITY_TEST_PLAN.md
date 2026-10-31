# SigmaOS Security Test Plan

## Overview

The Security Test Plan defines automated test requirements for validating kernel security boundaries.

## Test Categories

1. **Syscall Negative Boundary Tests**: Assert rejection of invalid pointers, negative lengths, and unauthorized syscall IDs.
2. **Capability Check Verification**: Assert `EPERM` when a process lacks required capability tokens.
3. **Fuzzing & Input Sanitization**: Fuzz syscall parameters, network packet headers, and ELF binary structures.
