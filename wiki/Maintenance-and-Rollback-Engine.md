# 🔄 Maintenance, System State & Rollback Engine

SigmaOS provides a resilient transactional state management framework that combines **Copy-on-Write (CoW) atomic rollbacks** (< 1ms execution time), **NixOS-style declarative system generations**, **FreeBSD ZFS Boot Environments**, and **Fedora offline update staging**.

---

## ⚡ Sub-Millisecond Transactional Rollbacks (`sigpkg rollback`)

Unlike traditional package managers that install files directly over active rootfs files, `sigpkg` creates a Copy-on-Write snapshot before every transaction.

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                   ATOMIC TRANSACTION ROLLBACK                          │
   ├────────────────────────────────────────────────────────────────────────┤
   │ Snapshot #42 (Working System) ➔ Snapshot #43 (Broken Package Install)   │
   │                              │                                         │
   │                              └─── ➔ `sigpkg rollback 42` (< 1ms)       │
   │                                     Restores Snapshot #42 instantly   │
   └────────────────────────────────────────────────────────────────────────┘
```

### Performing Rollbacks:
```bash
# List system transaction history snapshots
sigpkg history

# Instant rollback to Snapshot #42
sigpkg rollback 42

# Verify active system snapshot
sigpkg snapshot status
```

---

## 🏛️ Declarative System Generations (`DeclarativeStateGraph`)

SigmaOS manages system configuration as immutable, versioned generation profiles (`src/system/state.rs`).

### Generation Profiles (`/sovereign/profiles/`):
Every configuration change, package installation, or kernel update produces a new numbered profile generation (e.g., `/sovereign/profiles/system- generation-14`).

```bash
# List system generations
sigctl generation list

# Switch to generation 12 without rebooting
sigctl generation switch 12

# Delete old generations older than 30 days (garbage collection)
sigctl generation gc --older-than 30d
```

---

## ⛵ FreeBSD ZFS Boot Environments (`bectl` Parity)

SigmaOS integrates ZFS Boot Environment management (`bectl`) into the bootloader menu:

```bash
# List available boot environments
bectl list

# Create new boot environment prior to major update
bectl create pre-update-2026

# Activate boot environment for next boot
bectl activate pre-update-2026
```

---

## 💤 Offline System Updates (`systemd-offline-update` Parity)

For critical system component updates, SigmaOS stage-updates packages into volatile rootfs overlays and executes the update cleanly during reboot (`FedoraOfflineUpdateEngine`):

```bash
# Stage system update for offline installation on reboot
sigpkg update --offline

# Trigger reboot and apply staged update
reboot
```
