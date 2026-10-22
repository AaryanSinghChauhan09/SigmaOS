# Comparative Analysis: Packaging and Distribution

| Metric / Feature | Arch | Debian | Fedora | SigmaOS Target |
|---|---|---|---|---|
| Package Format | `.pkg.tar.zst` | `.deb` | `.rpm` | `.sigpkg` |
| Verification | GPG / pacman-key | GPG / apt-key | GPG / rpm-key | Dilithium5 PQC Signatures |
| Dependency Solver | ALPM / libalpm | APT libapt-pkg | DNF / libsolv | Sovereign SAT solver |
