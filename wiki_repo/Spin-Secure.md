# SigmaOS Secure Spin — Security & Forensics Edition

The **SigmaOS Secure** spin is SigmaOS's answer to CAINE, Kali Linux, and SystemRescue — a hardened, forensics-ready, security-first environment for penetration testers, incident responders, and forensic analysts. Boots into a read-only forensic mode by default.

---

## 🔬 Digital Forensics

| Tool | Purpose |
|------|---------|
| Autopsy | GUI digital forensics platform |
| Sleuth Kit (TSK) | Command-line disk forensics |
| Volatility 3 | Memory forensics & malware analysis |
| Bulk Extractor | Extract artifacts from disk images |
| Foremost / Photorec | File carving & recovery |

## 💾 Disk & Recovery Tools

- **RescueZilla** — imaging & cloning (Clonezilla-compatible)
- **TestDisk** — partition recovery
- **ddrescue** — byte-level disk rescue
- **SystemRescue** equivalent — bootable repair environment
- **GParted** — partition editor
- **Extundelete / ext4magic** — Ext4 file undelete

## 🔐 Encryption & Integrity

- **VeraCrypt** — encrypted container creation & mounting
- **GnuPG** — asymmetric file signing & encryption
- **Hashcat** — GPU-accelerated password auditing (ethical use)
- **John the Ripper** — password hash analysis
- **sha256sum / md5sum** — evidence integrity verification

## 🌐 Network Security & Pen-Testing

| Tool | Category |
|------|---------|
| Nmap / Zenmap | Network scanning & discovery |
| Wireshark / tshark | Packet capture & analysis |
| Metasploit Framework | Exploitation framework |
| Burp Suite (Community) | Web application testing |
| Aircrack-ng | Wi-Fi security auditing |
| Hydra | Credential brute-forcing |
| Nikto | Web server vulnerability scanning |

## 🕵 OSINT & Threat Intelligence

- **Maltego CE** — graph-based OSINT
- **Shodan CLI** — internet-exposed device search
- **theHarvester** — email/domain OSINT
- **Spiderfoot** — automated OSINT collection

## 🛡 Hardening & Compliance

- **Lynis** — system security auditing & hardening
- **ClamAV** — open-source antivirus scanner
- **AIDE** — file integrity monitoring (Tripwire alternative)
- **OpenSCAP** — SCAP/CIS benchmark compliance checks
- **Fail2Ban** — automated brute-force mitigation

## 🧰 Live Forensic Mode

SigmaOS Secure boots in **RAM-only mode** by default:
- Root filesystem is **tmpfs** (write-blocked from evidence drives)
- All drives are **auto-mounted read-only** unless explicitly unlocked
- Network interfaces start **down** — analyst must explicitly enable
- Hash logging enabled for all accessed files

```bash
# Unlock a drive for write access (break-glass)
sigma-secure unlock /dev/sdb --reason "live imaging"
```

---

## 🚀 Installation

```bash
sigma-spin install secure
```

## 📚 See Also

- [Recovery & Forensics](Recovery-And-Forensics.md)
- [Sovereign Sandbox](Sovereign-Sandbox.md)
- [Post-Quantum Security](Post-Quantum-Security.md)
