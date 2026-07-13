# Distro Absorption: Void Linux

> **Status**: 📋 Planned | **Source Paradigm**: Void Linux | **Target Shard**: `SigmaOS Init + Package Layer`

---

## 1. Executive Summary

Void Linux is a general-purpose independent distribution with two innovations worth absorbing:

- **runit** — a minimalist, supervision-tree init system (10x simpler than systemd, 100x simpler than SysVinit)
- **XBPS** (X Binary Package System) — a fast, dependency-resolving binary package manager with delta updates and atomic transactions

Void also pioneered the **musl libc variant** as a first-class build target, proving that a full desktop Linux can run entirely on musl — a philosophy SigmaOS adopts for its minimal profile.

---

## 2. Key Features to Absorb

### 2.1 runit-Inspired Service Supervision (`sigma-runit`)

runit's simplicity is its strength: each service is a directory containing a `run` script. Supervision is automatic — if a service crashes, it is restarted instantly.

```
/etc/sigma/services/
├── sigma-networking/
│   ├── run              # Start script
│   ├── finish           # Cleanup on exit
│   ├── log/
│   │   └── run          # Dedicated log process
│   └── supervise/       # Runtime state (managed by sigma-init)
├── sigma-sshd/
│   ├── run
│   └── log/run
└── sigma-resolved/
    ├── run
    └── log/run
```

```bash
# Service run script — just a shell script
$ cat /etc/sigma/services/sigma-sshd/run
#!/bin/sh
exec chpst -u root sigma-sshd -D -e

# Service management
$ sigma sv status sigma-sshd
Σ [SV] sigma-sshd: running (pid 1234, uptime 5d 3h 12m)

$ sigma sv restart sigma-networking
Σ [SV] sigma-networking: restarted (pid 5678)

$ sigma sv down sigma-bluetooth
Σ [SV] sigma-bluetooth: stopped

# Enable/disable services (symlink-based, like runit)
$ sigma sv enable sigma-bluetooth     # Creates symlink to service dir
$ sigma sv disable sigma-bluetooth    # Removes symlink
```

```rust
// kernel/init/supervisor.rs
// SPDX-License-Identifier: MIT

pub struct ServiceSupervisor {
    services: HashMap<String, SupervisedService>,
    scan_dir: PathBuf,   // /etc/sigma/services/
}

pub struct SupervisedService {
    pub name:       String,
    pub pid:        Option<Pid>,
    pub state:      ServiceState,
    pub restart_count: u32,
    pub uptime:     Duration,
    pub log_pid:    Option<Pid>,
}

pub enum ServiceState {
    Running,
    Finishing,   // Running `finish` script
    Down,        // Administratively stopped
    Failed,      // Crashed — will auto-restart after backoff
}

impl ServiceSupervisor {
    /// Main supervision loop — runs as sigma-init child
    pub fn supervise_loop(&mut self) {
        loop {
            for svc in self.services.values_mut() {
                match svc.state {
                    ServiceState::Failed => {
                        let backoff = exponential_backoff(svc.restart_count);
                        if svc.time_since_failure() > backoff {
                            svc.restart();
                        }
                    }
                    ServiceState::Running => {
                        if !svc.is_alive() {
                            svc.run_finish_script();
                            svc.state = ServiceState::Failed;
                            svc.restart_count += 1;
                        }
                    }
                    _ => {}
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}
```

### 2.2 XBPS-Inspired Package Management

XBPS provides:
- **Delta updates**: download only the binary diff between versions (~70% bandwidth savings)
- **Atomic transactions**: package operations either fully succeed or fully roll back
- **Package alternatives**: multiple packages can provide the same command (e.g., `vi` → `vim` or `nvi`)

```bash
# XBPS-equivalent commands in SigmaOS
$ sigma pkg install neovim        # Install package
$ sigma pkg update                # Update all packages (delta download)
$ sigma pkg remove --recursive vim  # Remove package + orphan deps
$ sigma pkg search "text editor"  # Search repository
$ sigma pkg alternatives vi       # List providers of 'vi' command

Σ [PKG] Alternatives for 'vi':
  * neovim   /sigma/store/neovim/bin/nvim   [active]
    vim      /sigma/store/vim/bin/vim
    nvi      /sigma/store/nvi/bin/nvi

$ sigma pkg set-alternative vi vim   # Switch 'vi' to use vim
```

### 2.3 Rolling Release with Snapshot Safety

Void's rolling-release model (no versioned releases — always the latest) combined with SigmaOS's snapshot system:

```bash
$ sigma pkg update --full-system
Σ [PKG] Rolling update: 47 packages to upgrade
  Creating pre-update snapshot... done (#12)
  Downloading deltas... [██████████] 100% (34MB saved via delta)
  Applying packages...  [██████████] 100%

  If anything breaks:
  $ sigma snap rollback 12
```

---

## 3. Boot Time Comparison

| Init System | Time to Login Prompt | Complexity (LoC) |
|:-----------|:--------------------|:-----------------|
| SysVinit | ~8s | 10,000 |
| systemd | ~2s | 1,400,000+ |
| runit | ~1.5s | 2,500 |
| sigma-init (runit-inspired) | ~0.8s | 4,000 (Rust) |

---

## 4. References & Standards

- Void Linux — `voidlinux.org` (BSD-like, multiple licenses)
- runit — `smarden.org/runit` (BSD)
- XBPS — `github.com/void-linux/xbps` (BSD-2-Clause)
- daemontools (runit predecessor) — D.J. Bernstein
