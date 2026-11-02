# SigmaOS-configurator

## Overview

SigmaOS-configurator is the system configurator and first-run setup wizard for SigmaOS. It guides new users through initial system configuration — locale, keyboard layout, timezone, user account settings, and optional component installation — so the desktop is personalised and ready to use immediately after the first boot.

## Features

- Step-by-step first-run wizard with progress tracking
- Locale, language, and keyboard layout selection
- Timezone and system clock setup
- User account creation and password configuration
- Optional feature and codec installation during setup
- Idempotent re-run support for reconfiguring an existing installation
- Integration with SigmaOS-control-center for post-install adjustments

## Upstream References

- [mintconfiguration](https://github.com/linuxmint/mintconfiguration) — Linux Mint configuration tool providing design patterns for system-level configuration
- [gnome-initial-setup](https://gitlab.gnome.org/GNOME/gnome-initial-setup) — GNOME first-run wizard, referenced for UX flow and locale handling

## Building

Requires Python 3.10+.

```bash
# Install dependencies
pip install -r requirements.txt

# Run the wizard
python src/main.py

# Lint
pip install pylint && pylint src/
```

## Contributing

1. Fork the repository and create a feature branch.
2. Follow PEP 8; run `pylint` before opening a pull request.
3. Test changes with a fresh virtual machine to verify wizard flow.
4. Open a pull request against `main` with a clear description of the change.

All contributions are subject to the [MIT License](LICENSE).
