# SigmaOS Shell Maintenance Guide

## Overview
This document outlines the maintenance standards and operating procedures for the `sigma-sh` userspace shell engine in SigmaOS.

## Maintenance Procedures
1. **Parser and Lexer Integrity**:
   - Ensure all grammar changes maintain LL(1) deterministic parsing guarantees.
   - Avoid infinite loops or recursion overflows on malformed inputs.
2. **Signal Handling**:
   - `SIGINT` (Ctrl+C) must interrupt current foreground execution without killing the shell session.
   - `EOF` (Ctrl+D) on an empty prompt gracefully exits interactive shell loop with code 0.
3. **Execution Safety**:
   - Strict validation of environment variables and expansion orders.
   - Quoting integrity must prevent arbitrary code execution during pathname expansion.
4. **Testing Suite**:
   - All shell features must be verified using `tests/shell_lifecycle_tests.rs`.
