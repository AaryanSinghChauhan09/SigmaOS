# SigmaOS-dots-configuration

## Overview

SigmaOS-dots-configuration is the official dotfiles and configuration management system for SigmaOS. It provides a reproducible, version-controlled approach to managing desktop environment setups, user preferences, and system configuration files. Inspired by tools like chezmoi and omadots, it allows users to bootstrap a fully configured SigmaOS desktop from a fresh install in minutes.

## Features

- **Reproducible setups**: Declarative configuration files that can be applied to any SigmaOS installation.
- **Profile support**: Maintain multiple configuration profiles (minimal, full, developer) and switch between them.
- **Secrets management**: Integrates with system keyrings to keep sensitive values out of version control.
- **Templating**: Support for per-machine variable substitution in config templates.
- **Symlink management**: Automatic creation and updating of dotfile symlinks to their target locations.
- **Backup and restore**: Backs up existing configuration before applying new dotfiles.

## Upstream References

- [chezmoi](https://www.chezmoi.io/) — Cross-platform dotfile manager with templating support.
- [omadots](https://github.com/omadots/omadots) — Lightweight dotfile organiser used as a reference for SigmaOS conventions.

## Building

```bash
bash src/main.sh
```

No compilation step is required. The script is self-contained and relies only on standard POSIX shell utilities available in every SigmaOS installation.

## Contributing

1. Fork this repository and create a feature branch.
2. Follow the SigmaOS shell scripting style guide (POSIX-compatible where possible).
3. Test your changes against a clean SigmaOS VM snapshot before submitting.
4. Open a pull request against `main` and fill in the PR template.
5. All contributions are released under the MIT License.
