# Sandbox-Hardening

1

This document outlines the security policies and hardening techniques applied to the SigmaOS Sovereign Sandbox.

1

The sandbox now utilizes `sandbox_policy.json` to declaratively define shard capabilities.

1

1

Syscalls are intercepted at the `sigma_syscall_gate` and matched against the shard's policy.

1

1

1

1

1
