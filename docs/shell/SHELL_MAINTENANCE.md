# SigmaOS Shell Maintenance Specification

## Overview

The SigmaOS interactive shell is governed by strict lifecycle standards defined in `SHELL_RULES.toml`.
Every shell modification must preserve POSIX conformance, memory safety (`#![no_std]`), and deterministic execution.
