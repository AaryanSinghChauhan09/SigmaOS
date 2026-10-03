# 🛡️ BlackArch Linux Security Gap Closure & AI Agent Operational Roadmap

This document outlines the architectural guidelines, tool categorization, and AI agent execution workflows for achieving 100% feature and package parity between **SigmaOS** and **BlackArch Linux**.

---

## 1. Executive Overview

BlackArch Linux is an Arch Linux-based penetration testing distribution containing over 2,800+ security tools organized across 45+ specialized categories.

SigmaOS achieves complete BlackArch feature parity through `SovereignBlackArchMasterSuite` in `src/distro/sovereign_blackarch_gap_closure_engine.rs`, implementing:
1. **BlackArch 45+ Category Master Registry**: Enum and metadata mapping for all tool categories.
2. **`blackman` Source Build Pipeline Manager**: Source-compilation build runner with parallel thread concurrency and PKGBUILD mirror fetching.
3. **`strap.sh` Repository Setup Engine**: Automated keyring import, repository mirror configuration, and PQC signature verification.
4. **Sandboxed Tool Installer**: Landlock and pledge/unveil sandboxed execution wrappers for security tools.
5. **Live ISO Build Profiles**: Persistence overlays and RAM-boot loading for security live environments.

---

## 2. BlackArch Tool Category Matrix

| Category Enum | Group Name | Description | Key Tools Included |
| :--- | :--- | :--- | :--- |
| `Base` | `blackarch` | Monolithic base group containing all packages | Nmap, Wireshark, Metasploit, Ghidra |
| `Recon` | `blackarch-recon` | Information gathering and network reconnaissance | Nmap, Masscan, Amass, Maltego |
| `Webapp` | `blackarch-webapp` | Web application security assessment | Burp Suite, OWASP ZAP, Sqlmap, Nikto |
| `Fuzzer` | `blackarch-fuzzer` | Input generation and protocol fuzzers | AFL++, LibFuzzer, Radamsa, Sfuzz |
| `Reversing` | `blackarch-reversing` | Binary disassembly and reverse engineering | Ghidra, Radare2, Cutter, IDA Free |
| `Wireless` | `blackarch-wireless` | 802.11 / Wi-Fi / Bluetooth / NFC testing | Aircrack-ng, Kismet, Reaver, Wifite |
| `Malware` | `blackarch-malware` | Malware analysis and artifact analysis | Volatility, YARA, Cuckoo, Viper |
| `Firmware` | `blackarch-firmware` | Embedded device and firmware extraction | Binwalk, Firmwalker, FACT Core |
| `Hardware` | `blackarch-hardware` | JTAG, UART, and hardware glitching tools | OpenOCD, Bus Pirate, Flashrom |
| `Mobile` | `blackarch-mobile` | Android & iOS security analysis | MobSF, Apktool, Frida, Objection |
| `Forensics` | `blackarch-forensics` | Digital forensics and disk image analysis | Autopsy, Sleuth Kit, Guymager |
| `AntiForensic` | `blackarch-anti-forensic` | Data wiping and artifact obfuscation | BleachBit, Secure-Delete, Steghide |
| `Social` | `blackarch-social` | Social engineering assessment frameworks | SET (Social Engineer Toolkit), GoPhish |
| `Stego` | `blackarch-stego` | Steganography detection and extraction | Stegsolve, Zsteg, OutGuess |
| `Crypto` | `blackarch-crypto` | Cryptanalysis and cipher cracking | Hashcat, John the Ripper, RsaCftTool |
| `Proxy` | `blackarch-proxy` | Interception proxies and traffic relays | mitmproxy, Charles, Proxychains |
| `Pivoting` | `blackarch-pivoting` | Network tunneling and lateral movement | Chisel, Ligolo-ng, SSHuttle |
| `Backdoor` | `blackarch-backdoor` | Persistence and C2 agent stubs | Covenant, Sliver, Empire |
| `Dos` | `blackarch-dos` | Denial of service stress testing | Slowloris, LOIC, Hping3 |
| `Defensive` | `blackarch-defensive` | Blue-team hardening and detection rules | Wazuh, Snort, Suricata, Zeek |
| `Automation` | `blackarch-automation` | Automated vulnerability scanning pipelines | Nuclei, OpenVAS, Faraday |

---

## 3. `blackman` Source Build Pipeline

SigmaOS implements `BlackmanSourceBuildManager` to enable compiling security tools directly from source repositories:

```rust
use sigmaos::distro::BlackmanSourceBuildManager;

let manager = BlackmanSourceBuildManager::new();
assert!(manager.compile_tool_from_source("ghidra"));
```

Key capabilities:
- **Parallel Concurrency**: Multi-threaded compilation utilizing per-core job queues.
- **Git Source Caching**: Incremental source fetching stored under `/var/cache/blackman/sources`.
- **PKGBUILD Mirror Integration**: Native PKGBUILD parser compatibility for Arch and BlackArch package recipes.

---

## 4. `strap.sh` Repository Bootstrap Workflow

```rust
use sigmaos::distro::StrapShRepositoryInstaller;

let mut strap = StrapShRepositoryInstaller::new();
assert!(strap.execute_strap());
```

Workflow stages:
1. GPG & Post-Quantum Cryptographic Keyring Ingestion.
2. Mirror Latency Measurement & P2P Mirror Selection.
3. `/etc/pacman.conf` and `sigpkg` repository configuration update.

---

## 5. AI Agent Rules for BlackArch Integration

1. **Category Completeness**: Every added security tool MUST belong to `BlackArchCategory::Base` and at least one specialized category enum.
2. **Sandbox Enforcement**: Security tool execution MUST specify pledge promises (`stdio rpath wpath cpath inet dns proc exec`) and Landlock file path rules.
3. **PQC Attestation**: Binary packages and source tarballs MUST verify SHA256 checksums and PQC signatures before installation.

---
*Documentation source policy: Always edit documentation in `docs/`.*
