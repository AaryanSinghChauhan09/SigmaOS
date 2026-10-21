# Σ SigmaOS Zenith — Professional GitHub Wiki Index
## Sovereign Lattice Desktop v15.0

![SigmaOS Banner](wiki/zenith_desktop.css)

---

## 🏛️ Overview

**SigmaOS** is a sovereign, AI-native desktop operating system built for modularity, security, and long-term sustainability. This wiki documents the complete source of inspiration, engineering canon adherence, and continuous improvement pipeline derived from Omarchy 4.0.5/4.0.6 and Linux Mint 22.3 Cinnamon HWE inspirations.

**Current Status:**
- ✅ **Main branch only** — all redundant branches merged and deleted
- ✅ **Clean main** — zero tracked `.md`/`.txt` files on `main`
- ✅ **GitHub synced** — `origin/main` up to date with all improvements
- ✅ **Wiki consolidated** — professional index and documentation established
- ✅ **Omarchy 4.0.6 inspired** — kernel linux-t2 naming, package sets, bootloader config
- ✅ **Linux Mint 22.3 Cinnamon HWE inspired** — desktop parity and UX patterns

---

## 🛠️ Engineering Canon Principles (Applied Throughout)

All SigmaOS development follows the complete engineering canon documented across the GitHub wiki and repo:

| Principle | Application |
|-----------|-------------|
| **Abstraction** | Clear separation of mechanism & policy, component isolation |
| **Architecture** | Layered design with sovereignty enforcement |
| **Command-Query Separation** | Read/write operations distinctly handled |
| **Single Responsibility** | Each subsystem has one well-defined purpose |
| **Maximise Cohesion** | High internal coherence within subsystems |
| **Minimise Coupling** | Loose coupling between subsystems and dependencies |
| **Open/Closed** | Extensible without modification (Open/Closed Principle) |
| **Liskov Substitution** | Types interchangeable without altering program correctness |
| **Interface Segregation** | Clients dependent only on needed interfaces |
| **Dependency Inversion** | Depend on abstractions, not concretions |
| **Fail-Fast System** | Defensively check assumptions, surface errors immediately |
| **Secure Defaults** | Safe by default, privilege minimization |
| **Cache Conscious Design** | Locality of reference optimized |
| **Deterministic Testing** | Repeatable, timely validation |
| **Zero-One-Infinity** | Scalable pattern enforcement |
| **Don't Repeat Yourself** | DRY — single source of truth |
| **Rule of Three** | Abstract after three similar implementations |
| **Booth Method** | Structured design and verification |
| **Gall's Law** | Complex systems evolve from simple working bases |
| **Worse Is Better** | Practical simplicity over completeness |
| **Zen of Python** | Aesthetic, explicit, readable code |
| **Manual Memory Control** | Discipline and safety in resource management |
| **Process Isolation** | Robust separation between subsystems |
| **Race-Free Design** | Concurrent safety guarantees |
| **Demeter Law** | Limited knowledge, minimal dependencies |
| **Direct Device Control** | Low-level hardware interaction where needed |
| **Power of Ten** | Code density and reliability guidelines |
| **Rule of Three** | Abstract after three similar patterns |

---

## 🌟 Omarchy 4.0.6 Inspiration

SigmaOS draws significant design inspiration from **Omarchy 4.0.6** (`/home/aaryansinghchauhan/Downloads/omarchy-4.0.6.iso`):

| Feature | SigmaOS Implementation |
|---------|----------------------|
| **Kernel Naming** | `vmlinuz-linux-t2` (matches Omarchy linux-t2 convention) |
| **Keyring Package** | `sigmaos-keyring` (modeled on `omarchy-keyring 20251027-1`) |
| **Settings Package** | `sigmaos-settings` (modeled on `omarchy-settings 4.0.6-1`) |
| **Bootloader** | limine configuration with linux-t2 entries + speakup accessibility |
| **Package List** | 482 packages including `omarchy-keyring`, `omarchy-settings`, `gum`, `plymouth` |
| **Initramfs** | archinstall-guided with accessibility and recovery options |
| **Theme** | Plymouth theme similar styling and boot splash approach |

---

## 💎 Linux Mint 22.3 Cinnamon HWE Inspiration

SigmaOS also incorporates design patterns from **Linux Mint 22.3 cinnamon-64bit hwe-6.17.iso**:

| Feature | SigmaOS Implementation |
|---------|----------------------|
| **Desktop Parity** | Hyprland WM + Waybar + Wofi mimicking Cinnamon workflow patterns |
| **Package Parity** | Core desktop packages for out-of-the-box usability |
| **Kernel Stability** | Linux HWE series integration (6.17.x series concepts) |
| **Driver Support** | Broad hardware compatibility philosophy |
| **User Experience** | Familiar desktop flow for smooth transition |
| **Stability Focus** | Long-term support and reliability patterns |

---

## 📦 Key SigmaOS Improvements (Latest Commit)

```
d91f780312 feat: Omarchy 4.0.6 inspired improvements - kernel linux-t2 naming,
sigmaos-keyring, sigmaos-settings, archinstall support, accessibility,
cloud-init, TPM tools, recovery tools (clonezilla, drbl, testdisk),
sdparm, openbsd-netcat, xorg-xprop; improved limine.conf with linux-t2 entries;
updated build_iso.sh with linux-t2 kernel staging; added gum accessibility and
linux-t2 in grub.cfg; updated packages with Omarchy 4.0.6 style packages;
sigmaos-settings and sigmaos-keyring PKGBUILDs; version 4.0.6
```

### Technical Highlights:
- **linux-t2 kernel** throughout bootloader and ISO builder
- **sigmaos-keyring** and **sigmaos-settings** PKGBUILDs (Omarchy 4.0.6 style)
- **archinstall** guided installer support
- **speakup** screen reader accessibility
- **cloud-init** first-boot provisioning
- **TPM 2.0** security support (tpm2-tools, tpm2-tss)
- **Recovery tools**: clonezilla, drbl, testdisk
- ** enhanced limine.conf** with 9 linux-t2 entries including accessibility mode
- ** build_iso.sh** stages `vmlinuz-linux-t2` kernel
- ** grub.cfg** uses `linux /boot/vmlinuz-linux-t2`
- ** Version**: 4.0.6

---

## 📁 Repository Structure

```
SigmaOS/                          # Main repository
├── bin/                          # 23 runtime executables
├── etc/                          # 24 system config files
├── default/                      # 25 desktop configs (Hyprland, Waybar, Wofi)
├── shell/                        # QML desktop shell and plugins
├── install/                      # Provisioning and configuration scripts
├── boot/                         # limine bootloader config
├── pkgbuild/                     # Package definitions and hooks
├── tests/acceptance/             # base, session, apps, all test suites
├── iso_root/                     # ISO root (installer mount point)
├── scripts/                      # build_iso.sh, ISO builder
└── main branch only ✅
```

---

## 📚 Wiki Documentation Structure

The SigmaOS wiki contains comprehensive documentation across these domains:

| Category | Key Files |
|----------|-----------|
| **Engineering Principles** | `Sovereign-Engineering-Principles.md`, `PROCESS_RULES.toml`, `SCHEDULER_RULES.toml` |
| **Omarchy Integration** | `Omarchy-4.0.5-Complete-Integration-Catalog.md`, `Omarchy-Audio-Tuning.md`, `Omarchy-File-Layout-and-Mental-Model.md` |
| **Linux Mint Parity** | `LinuxMint-Parity.md`, `Linux-Universal-Distro-Parity.md` |
| **Subsystem Docs** | `src/`, `shell/`, `install/`, `boot/`, `pkgbuild/` documentation |
| **Test Suites** | `tests/acceptance/` (base, session, apps, all), `run_sigma_tests.sh` |
| **Build & ISO** | `scripts/build_iso.sh`, `sovereign_merge_all.sh`, `sync_github.sh` |
| **Governance** | `GOVERNANCE.md`, `COMPONENT_RULES.toml`, `FEATURE_STATUS.toml` |
| **Compatibility** | `distro_inspirations.rs`, `linuxmint_inspirations.rs`, `omarchy-4.0.6.iso` comparisons |

---

## 🔄 Continuous Improvement Pipeline

SigmaOS follows a periodic improvement cycle inspired by the GitHub wiki processes:

1. **Research** — Identify opportunities from Omarchy 4.0.6, Linux Mint 22.3 inspirations
2. **Implement** — Develop improvements following engineering canon
3. **Test** — Run acceptance tests (`tests/acceptance/`) and validation suites
4. **Merge** — Integrate into `main` via PR process
5. **Sync** — Push to `origin/main`, consolidate wiki
6. **Review** — Periodic PR/component rules evaluation
7. **Iterate** — Continuous improvement long-term sustainability

---

## 📥 Getting Started

### From the Main Repository:
```bash
# Clone the main SigmaOS repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Verify main branch is clean
git ls-files | grep -E '\.(md|txt)$'  # Should return nothing

# View current state
git branch -a  # Only main exists
git log --oneline -3  # Recent commits

# Key improvements are already integrated
```

### From the Wiki:
- Start at the [SigmaOS Zenith HTML interface](index.html)
- Explore [Sovereign Engineering Principles](GOVERNANCE.md)
- Review [Omarchy Integration Catalog](Omarchy-4.0.5-Complete-Integration-Catalog.md)
- Check [Linux Mint Parity status](LinuxMint-Parity.md)
- Run acceptance tests: `bash tests/acceptance/all/`

---

## 📈 Roadmap & Next Steps

- ✅ Main branch clean and consolidated
- ✅ All PRs merged, redundant branches removed
- ✅ Wiki professional index established
- ✅ Omarchy 4.0.6 inspired improvements integrated
- ✅ Linux Mint 22.3 HWE parity patterns applied
- � Ongoing: Periodic improvement pipeline
- � Ongoing: Wiki content expansion
- � Ongoing: Subsystem integration and testing

---

*This wiki was last updated following the consolidation of all SigmaOS branches into main, with Omarchy 4.0.6 and Linux Mint 22.3 Cinnamon HWE inspirations professionally documented. For the latest changes, view the Git history or open issues on the GitHub repository.*

