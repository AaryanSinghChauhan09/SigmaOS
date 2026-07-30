# Distro Absorption: Kali Linux — Penetration Testing OS

> **Status**: 📋 Planned | **Source Paradigm**: Kali Linux (Offensive Security) | **Target Shard**: `SigmaOS Security Audit Suite`

---

## 1. Executive Summary

Kali Linux is the industry standard penetration testing distribution from Offensive Security. It ships with 600+ security tools pre-installed, a forensics mode that prevents disk writes, and specialized kernels with wireless injection patches.

SigmaOS absorbs Kali's **security audit tooling integration** and **forensics mode** into `sigma-sec-audit`, providing security professionals with a native, capability-enforced penetration testing environment that doesn't require a separate boot.

---

## 2. Key Features to Absorb

### 2.1 sigma-sec-audit Tooling Bundle

A curated collection of security tools installed as a `sigma-pkg` group, sandboxed with fine-grained capabilities so they can only operate within explicitly authorized network ranges.

```bash
$ sigma-pkg group install sec-audit
Σ [PKG] Installing sigma-sec-audit group (42 tools):
  nmap, rustscan, masscan      — Network scanning
  sigma-burp, mitmproxy        — HTTP interception
  hashcat, john                — Password auditing
  sigma-forensic               — Disk forensics
  sigma-exploit-kit            — CVE exploit PoCs (sandboxed)

$ sigma-sec capability-grant --tool nmap --allow "10.0.0.0/8" --deny internet
Σ [SEC] nmap granted: scan 10.0.0.0/8 only
```

### 2.2 Forensics Mode

In forensics mode, `sigma-init` mounts all local disks read-only. No swap is activated. No automount occurs. The system is ready for evidence collection without contaminating the target media.

```bash
$ sigma boot --mode forensics /dev/sda
Σ [BOOT] Forensics Mode:
  /dev/sda  → mounted READ-ONLY (no writes)
  Swap:     → DISABLED
  Journal:  → RAM only
  Evidence hash: blake3:a1b2c3d4... (of /dev/sda at mount time)
```

---

## 3. References & Standards

- Kali Linux — `kali.org` (Debian-based, mixed licenses)
- Offensive Security — `offsec.com`
