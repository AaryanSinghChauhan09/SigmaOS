# AI Agent Attribute Definition Table Architecture (`docs/AGENTS_ATTRIBUTE_DEFINITIONS.md`)

This guide specifies technical architecture, schema definitions, and operational workflows for AI agents managing attribute definition tables in SigmaOS.

---

## 1. Attribute Table Architecture & Schemas

AI agents manage four primary attribute definition tables across the SigmaOS codebase:

### A. Sysfs Dynamic Hardware Attribute Table
- **Module:** `src/process/linux_sysfs.rs`
- **Struct:** `SysfsAttribute`
- **Schema:**
  - `path: String` (e.g. `/sys/class/power_supply/BAT0/capacity`)
  - `value: String`
  - `writable: bool`
- **Operations:** `read_attribute`, `write_attribute`

### B. Archive Extended PAX/xattr Header Table
- **Module:** `src/tools/archive.rs`
- **Struct:** `PaxTarHeader`
- **Schema:**
  - `uname: String`, `gname: String`
  - `mtime_nsec: u64`
  - `xattrs: HashMap<String, Vec<u8>>`

### C. Version Control xattr Property Table
- **Module:** `src/sigpkg/svntogit_repro.rs`
- **Struct:** `SvnXattrProperties`
- **Schema:**
  - `mime_type: Option<String>`
  - `ignore_patterns: Vec<String>`
  - `keywords: Vec<String>`

### D. Landlock v5 Security Access Attribute Vector
- **Module:** `src/distro/sovereign_nextgen_distro_leap.rs`
- **Struct:** `SovereignLandlockV5Guard`
- **Schema:**
  - `allowed_read_paths: Vec<String>`
  - `allowed_write_paths: Vec<String>`
  - `allowed_tcp_bind_ports: Vec<u16>`

---

## 2. Operational Workflows for AI Agents

1. **Schema Validation:** Verify that new attributes strictly conform to `#![no_std]` allocation requirements.
2. **Access Audit:** Verify read/write permission gating prior to mutating attribute table values.
3. **Automated Testing:** Verify attribute table integrity using `rustc --test src/process/linux_sysfs.rs`.
