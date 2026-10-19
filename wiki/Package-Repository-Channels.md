# SigmaOS Package Repository Channels

## Overview
SigmaOS package channel system providing edge/rc/stable channels, competitive with Arch pacman, Mint apt, and Omarchy package channels.

## Features
- **Three channels**: edge, rc, stable
- **Edge**: built from every PR, unsigned, for testing
- **RC**: release candidate, tested, signed
- **Stable**: production-ready, signed, long-term support
- **Channel promotion**: edge → rc → stable (no direct edge → stable)
- **GPG signing** for rc and stable channels
- **Repository configuration** with priority and GPG keys

## Channel System

| Channel | Description | Signature Required | Promotion From |
|---------|-------------|-------------------|----------------|
| **edge** | Bleeding edge, built from PRs | No | Any |
| **rc** | Release candidate, tested | Yes | edge |
| **stable** | Production-ready, LTS | Yes | rc |

## Structures

| Structure | Description |
|-----------|-------------|
| **Channel** | Package release channel (Edge, Rc, Stable) |
| **PackageMetadata** | Package metadata with channel info |
| **RepositoryConfig** | Repository configuration with URL, channel, GPG key |

## Status
- **Implementation**: Complete
- **Dependencies**: sigpkg core, GPG tools
- **Roadmap Phase**: Package Management (P1)

## Related Files
- `src/sigpkg/channel.rs` - Channel implementation
- `wiki/SigPkg-Universal-Package-Manager-Components.md` - Package manager wiki

## Principles Followed
PRACTICAL_LICENSING_AWARENESS, MERITOCRACY_AND_INCLUSIVITY, STABLE_APIS,
OPEN_STANDARD_AND_INTEROPERABILITY, COMMUNITY_ORIENTED_DEVELOPMENT,
VERSION_CONTROL_PRACTICES, SECURE_DEFAULTS, FAIL_FAST_SYSTEM