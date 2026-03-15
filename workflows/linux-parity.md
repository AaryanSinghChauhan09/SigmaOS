---
description: How to verify SigmaOS parity with major Linux distributions
---
# Linux Parity Excellence Workflow

This workflow ensures SigmaOS maintains its lead over competitors like Kali, Arch, and RHEL.

1. **Run Gap Audit**
   - Open the **Linux Parity Hub** in the GUI.
   - Click **'Audit: Sigma vs Kali'** and **'Audit: Sigma vs Arch'**.
   - Review the console logs for any "GAP" or "PARTIAL" designations.

2. **Verify Atomic Transactions (NixOS Parity)**
   // turbo
   - Run `py -c "from kernel.linux_parity_engine import SigmaPackageManager; pm = SigmaPackageManager(None); pm.sigma_install('sigma-pentest'); print(pm.transactional_rollback())"`
   - Ensure the rollback log confirms filesystem layer restoration.

3. **Benchmarking (Speed Parity)**
   - Open the **Apex Hub**.
   - Activate **Apex Mode**.
   - Observe the **Performance Constellation** in the topbar to verify zero-latency rendering.

4. **Security Hardening (RHEL Parity)**
   - Click **'🛡️ RHEL Security Scan'** in the Parity Hub.
   - Verify all STIG/SCAP rules return 'PASS'.

5. **Full Integration Suite**
   // turbo
   - Run `py C:/Users/Aaryan\.gemini\antigravity\scratch\SigmaOS\_integration_v2_test.py`
   - All tests MUST pass with Sovereign-Grade efficiency.
