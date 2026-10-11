# Package Management Comparison Matrix

| Project | Package Format | Dependency Resolution | Script Execution | Rollback Mechanism |
|---|---|---|---|---|
| **Arch** | `.pkg.tar.zst` (ALPM) | Topological DAG | Direct bash execution | Cache downgrade / Snapper |
| **Debian** | `.deb` (dpkg/apt) | SAT / Libapt solver | `preinst`/`postinst` maintainer scripts | Snapshot / Timeshift |
| **Fedora** | `.rpm` (rpm/dnf) | Libsolv | RPM scriptlets | OSTree / Greenboot rollback |
| **SigmaOS** | Schema metadata parser | DAG solver dry-run | Sandboxed execution | Atomic image generation rollback |
