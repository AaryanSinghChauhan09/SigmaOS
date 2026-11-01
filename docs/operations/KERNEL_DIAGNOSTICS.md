# SigmaOS Kernel Diagnostics Protocol

## Overview

The Kernel Diagnostics Protocol governs boot initialization logging, structured panic diagnostic reporting, register context serialization, and greppable log formats.

## Diagnostic Principles

1. **Structured Formatting**: Diagnostic events use key-value structured tags (e.g. `[INIT] subsystem=paging status=ok`).
2. **Panic Context**: Panics record component, file location, failure message, and register frame snapshot.
3. **No Silent Failures**: Unrecoverable boot-critical errors emit diagnostic output prior to CPU halt.
