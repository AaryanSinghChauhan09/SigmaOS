# Recovery & Forensics Tooling

To directly compete with tools like SystemRescue and CAINE, SigmaOS integrates recovery tools directly into the base OS.

## System Snapshotting
Powered by the Sovereign Recovery Suite, SigmaOS automatically takes base-level snapshots of the ZFS filesystem tree before every major configuration change or `OmniPkg` transaction. 

In the event of a critical failure or panic, users can instantly rollback via the bootloader to a known good Transaction Group (TXG).

## Forensic Audit Mode
SigmaOS can be booted into a specialized Forensic Mode. 
In this mode:
1. All physical disks are forcibly mounted as `READ-ONLY`.
2. The networking stack is disabled to prevent data exfiltration.
3. The system computes a cryptographic SHA256 hash tree of the entire file system for legal or compliance auditing.
