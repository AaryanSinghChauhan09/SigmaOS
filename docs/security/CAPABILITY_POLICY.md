# SigmaOS Capability Security Policy

## Overview

Processes in SigmaOS do not inherit full root privileges. Privilege is governed by granular capability tokens.

## Capability Hierarchy

- `CAP_SYS_ADMIN`: Required for raw disk partition operations and mounting.
- `CAP_NET_ADMIN`: Required for raw packet socket binding and network route changes.
- `CAP_KILL`: Permission to send signals to other processes.
- `CAP_PTRACE`: Privilege to inspect memory of other processes.

## Enforcement Mechanism

Every syscall entry point checks process capability mask against required capabilities before executing kernel handlers.
