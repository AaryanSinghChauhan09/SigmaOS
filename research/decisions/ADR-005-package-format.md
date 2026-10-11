# ADR-005: Package Metadata Schema and Dependency Graph Resolver

## Status
Accepted

## Upstream study
Arch Linux `PKGBUILD` / ALPM database format and Debian `control` stanzas.

## SigmaOS decision
Implement a zero-dependency TOML/DEB822 package metadata parser and DAG dependency resolver supporting dry-run installations.

## Scope
- Package manifest parsing (`name`, `version`, `depends`, `provides`, `conflicts`)
- Topological sort dependency resolution with cycle detection
- Dry-run installation plan output

## Non-goals
- Live binary downloads from untrusted mirrors
- Host execution of package maintainer scripts
- P2P package distribution

## Validation
- Package metadata parsing unit tests
- Dependency cycle resolution tests
