# 🛠️ Core Built-in Apps: Reference & Architecture

This specification documents the architecture, capabilities, and command-line interfaces of SigmaOS's four core built-in system applications: **SigmaPkg**, **SigmaFS Snapshot Tool**, **Zenith Control Center**, and **SigmaShield**.

---

## 📦 1. SigmaPkg (Sovereign Package Manager)

`SigmaPkg` manages system software using content-addressed storage (CAS) and transactional atomic updates.

### Architecture
```
┌───────────────────────────────────────────┐
│              sigma-pkg CLI                │
└─────────────────────┬─────────────────────┘
                      ▼
┌───────────────────────────────────────────┐
│     Zero-Allocation Dependency Solver     │
│   (Arch-inspired SAT Solver with cycles)  │
└─────────────────────┬─────────────────────┘
                      ▼
┌───────────────────────────────────────────┐
│       Dilithium-5 Signature Verifier      │
└─────────────────────┬─────────────────────┘
                      ▼
┌───────────────────────────────────────────┐
│         Content-Addressed Storage         │
│     (/sigma/store/sha256-hash/...)        │
└───────────────────────────────────────────┘
```

### CLI Command Reference
```bash
# Install a package with atomic validation
sigpkg install <package-name>

# Verify integrity of all installed package recipes
sigpkg verify --all

# List active repository package versions
sigpkg list
```

---

## 💾 2. SigmaFS Snapshot Tool

The snapshot tool coordinates with the content-addressed block allocator to take sub-millisecond, read-only system snapshots.

### Core Features
- **Zero-Copy Generation:** Instant snapshot creation by recording the root generation node and incrementing metadata references.
- **Atomic Rollback:** Revert system state instantly by swapping the active generation pointer to a previous snapshot hash.
- **Deduplication:** Blocks that do not change between snapshots share physical sectors.

### CLI Command Reference
```bash
# Create a system snapshot
snapshot create --name "Before_Major_Update"

# List all local snapshots and timestamps
snapshot list

# Revert to a previous snapshot generation
snapshot restore <snapshot-id>
```

---

## 🖥️ 3. Zenith Desktop Control Center

The central hub to configure and adapt display layouts, active compositor profiles, and accessibility services.

### CLI Command Reference
```bash
# Switch to a specialized user experience profile
profile switch developer

# Arrange multiple display screens
display set --primary DP-1 --arrange right

# Toggle screen readers and magnification
accessibility screen-reader --enable true
```

---

## 🛡️ 4. SigmaShield (Firewall & Anomaly Detection)

`SigmaShield` runs as an active security monitor, filtering packet traffic and scanning processes for anomalous execution profiles.

### Technical Primitives
- **YARA Process Scanner:** Scans process memory against YARA-style malware signatures.
- **Network Rule Enforcement:** Inspects all outbound/inbound network frames against active firewall tables.
- **Log Integrity Guard:** Stream security audits to tamper-evident cryptographic ledgers.

### CLI Command Reference
```bash
# Start background threat scanning
shield scan --target memory

# Apply firewall rule
shield firewall add-rule --allow --port 443

# Check system security status
shield status
```
