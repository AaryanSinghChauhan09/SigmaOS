# OSS Absorption: Systemd Feature Absorption

> **Status**: 🔄 Active | **Source Project**: systemd v255 | **Target Shard**: `SigmaOS Init + Service Layer`

---

## 1. Executive Summary

While SigmaOS uses its own `sigma-init` (a Rust-native, capability-secure init system), several systemd subsystems provide important functionality worth absorbing: `systemd-networkd` for declarative networking, `systemd-resolved` for DNS-over-TLS, `systemd-homed` for portable home directories, `journald` for structured logging, and `systemd-boot` for UEFI boot management.

SigmaOS reimplements these as standalone Rust modules under the `sigma-init` umbrella, remaining free of the systemd monolith while capturing its most valuable innovations.

> [!NOTE]
> SigmaOS does NOT adopt the PID 1 systemd daemon. Only specific subsystems are re-implemented in Rust with the same functionality.

---

## 2. Absorbed Subsystems

### 2.1 `sigma-networkd` — Declarative Networking (from `systemd-networkd`)

```toml
# /etc/sigma/network.d/eth0.toml
[match]
name = "eth*"

[network]
dhcp = "ipv4"
dns  = ["1.1.1.1", "8.8.8.8"]
domains = ["local"]

[dhcp]
use_hostname  = true
use_routes    = true
send_hostname = true

[ipv6]
accept_ra = true
privacy_ext = true   # RFC 4941 IPv6 privacy extensions

[bridge]
name = "sigma-br0"   # Optional — join a bridge for VM networking
```

```bash
$ sigma net status
Σ [INFO] Network Status:
  eth0      UP     192.168.1.100/24  gateway 192.168.1.1  DNS 1.1.1.1
  wlan0     UP     192.168.1.101/24  SSID: HomeNetwork
  docker0   UP     172.17.0.1/16     (container bridge)
  lo        UP     127.0.0.1/8

$ sigma net restart eth0
$ sigma net set-dns 1.1.1.1 9.9.9.9
```

### 2.2 `sigma-resolved` — Encrypted DNS (from `systemd-resolved`)

```rust
// kernel/net/resolved.rs
// SPDX-License-Identifier: MIT

pub struct SigmaResolved {
    config:   ResolvedConfig,
    cache:    DnsCache,        // TTL-respecting LRU cache
    upstream: Vec<DnsServer>,
}

pub struct DnsServer {
    pub addr:     SocketAddr,
    pub protocol: DnsProtocol, // Classic, TLS, or HTTPS
    pub pin:      Option<SpkiPin>, // Certificate pinning for DoT/DoH
}

impl SigmaResolved {
    pub async fn resolve(&mut self, query: &DnsQuery) -> DnsResponse {
        // Check local cache first
        if let Some(cached) = self.cache.get(query) {
            return cached;
        }

        // LLMNR/mDNS for .local domains
        if query.name.ends_with(".local") {
            return self.mdns_resolve(query).await;
        }

        // Forward to upstream — prefer DoH, fallback to DoT, then UDP
        let resp = self.upstream_resolve(query).await?;
        self.cache.insert(query.clone(), resp.clone(), resp.ttl);
        resp
    }
}
```

```bash
$ sigma resolve google.com
Σ [DNS] google.com → 142.250.80.46 (DoH via 1.1.1.1, cached, TTL 298s)

$ sigma resolve --dnssec sigmaos.io
Σ [DNS] sigmaos.io → 45.33.20.10
  DNSSEC: ✅ Verified (RRSIG valid, chain of trust complete)
```

### 2.3 `sigma-homed` — Portable Home Directories (from `systemd-homed`)

Portable home directories: a user's home is a LUKS2-encrypted, content-addressed directory image. Move it between machines by copying one file.

```bash
# Create a portable home for user "alice"
$ sigma homed create alice --size 50G
Σ [HOMED] Creating portable home for alice...
  Encrypted image: /var/lib/sigma/homes/alice.home
  Encryption: LUKS2 (Argon2id, AES-256-GCM)
  Size: 50GB (sparse — uses only actual data)

# Activate (mount + unlock) the home
$ sigma homed activate alice
Σ [HOMED] Unlocking alice's home (enter password):
  Passphrase: ****
  Mounted at: /home/alice

# Export home for use on another SigmaOS machine
$ sigma homed export alice alice.sigma-home
$ scp alice.sigma-home other-machine:~/
```

### 2.4 `sigma-journal` — Structured Binary Logging (from `journald`)

```rust
// kernel/log/journal.rs
pub struct JournalEntry {
    pub timestamp: SystemTime,
    pub priority:  LogPriority,    // 0=emergency .. 7=debug
    pub unit:      String,
    pub message:   String,
    pub fields:    HashMap<String, String>,  // Structured key=value fields
}

impl SigmaJournal {
    pub fn log(&self, entry: JournalEntry) {
        // Write to binary journal with O(log n) indexed lookup
        self.writer.append(&entry.to_binary())?;
    }

    pub fn query(&self, filter: JournalFilter) -> impl Iterator<Item=JournalEntry> {
        self.index.seek(filter.since)
            .filter(|e| filter.matches(e))
    }
}
```

```bash
# View logs (journalctl-compatible)
$ sigma logs                          # Latest logs
$ sigma logs -u sigma-networking      # Logs from a specific shard
$ sigma logs --since "1 hour ago"     # Time-based filter
$ sigma logs -p err                   # Only errors and above
$ sigma logs -f                       # Follow (tail -f equivalent)
$ sigma logs --export journal.json    # Export as structured JSON
```

### 2.5 `sigma-boot` — UEFI Boot Manager (from `systemd-boot`)

```bash
# List boot entries
$ sigma boot list
Σ [BOOT] UEFI Boot Entries:
  * SigmaOS 1.0 (current)    kernel/sigma-1.0.0    [default]
    SigmaOS 1.0 (snapshot 3) kernel/sigma-1.0.0 @snap3
    SigmaOS 0.9 (fallback)   kernel/sigma-0.9.5
    UEFI Firmware Setup

# Set default boot entry
$ sigma boot set-default "SigmaOS 0.9 (fallback)"
```

---

## 3. What SigmaOS Explicitly DOES NOT Absorb from systemd

| systemd Component | Reason Excluded |
|:-----------------|:----------------|
| PID 1 (main daemon) | Replaced by `sigma-init` (Rust, capability-secure) |
| `D-Bus` dependency | Replaced by `sigma-ipc` (cap-rights IPC) |
| `polkit` | Replaced by `sigma-doas` (simpler, auditable) |
| `udev` | Replaced by `sigma-hal` (Rust device enumeration) |
| `cgroups v1` | Only cgroups v2 used natively |

---

## 4. References & Standards

- systemd — `systemd.io` (LGPL-2.1)
- RFC 8484 — DNS over HTTPS
- RFC 7858 — DNS over TLS
- LUKS2 specification — `gitlab.com/cryptsetup`
