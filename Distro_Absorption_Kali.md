# Distro Absorption: Kali Linux

> **Status**: 📋 Planned | **Source Paradigm**: Kali Linux (Offensive Security) | **Target Shard**: `SigmaOS Offensive / Cybernetics Profile`

---

## 1. Executive Summary

Kali Linux is the industry-standard distribution for penetration testing, digital forensics, and reverse engineering. Its core value is not a novel kernel architecture, but rather an immense, meticulously curated repository of thousands of security tools combined with a kernel patched for wireless packet injection.

SigmaOS absorbs the **Offensive Profile**, providing a sandboxed, ephemeral toolkit environment (`sigma-kali`) alongside a kernel capable of all advanced radio frequency and network packet manipulation required by security researchers.

---

## 2. Key Features to Absorb

### 2.1 The `sigma-kali` Metapackage Shard

Rather than dual-booting or running a VM, SigmaOS users can invoke an ephemeral OCI container or overlay environment packed with Kali's toolset.

```bash
$ sigma env --profile offensive
Σ [ENV] Dropping into Offensive Security Profile...
  Mounting toolset (Metasploit, Nmap, Wireshark, Aircrack-ng, Burp Suite)
  Configuring network shard for raw socket access
```

### 2.2 Kernel Patches for Wireless Injection

Standard Linux kernels often restrict WiFi drivers. SigmaOS's kernel (`sigma-kernel`) is pre-patched with mac80211 injection support for all supported wireless chipsets (Atheros, Realtek, Ralink).

```bash
# Enable monitor mode and injection
$ sigma net wifi monitor wlan0 enable
Σ [NET] wlan0 is now in monitor mode (mon0). Packet injection enabled.

# Test injection
$ aireplay-ng --test mon0
12:34:56  Trying broadcast probe requests...
12:34:56  Injection is working!
```

### 2.3 Undercover Mode

Kali Linux features an "Undercover Mode" that themes the desktop to look exactly like Windows 10/11, preventing suspicion in public places or physical engagements. 

SigmaOS achieves this via a single command that completely swaps the Zenith compositor theme, icon set, and behavior.

```bash
$ sigma ui set-theme win-undercover
Σ [UI] Applying Windows 11 camouflage...
  Taskbar moved to bottom
  Window borders and icons swapped
  (To revert, run: sigma ui set-theme default)
```

---

## 3. References & Standards

- Kali Linux — `kali.org` (GPL-2.0 / various)
- mac80211 injection patches
