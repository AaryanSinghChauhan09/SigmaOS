# Sandbox-Hardening


This document outlines the security policies and hardening techniques applied to the SigmaOS Sovereign Sandbox.


The sandbox now utilizes `sandbox_policy.json` to declaratively define shard capabilities.



Syscalls are intercepted at the `sigma_syscall_gate` and matched against the shard's policy.





