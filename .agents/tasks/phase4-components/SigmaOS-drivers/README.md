# SigmaOS-drivers

## Overview

SigmaOS-drivers is the hardware driver management tool for SigmaOS. It automates the detection, installation, and updating of proprietary and third-party drivers — including GPU, Wi-Fi, Bluetooth, and printer drivers — providing a unified graphical and command-line interface. It draws from the approaches pioneered by mintdrivers and ubuntu-drivers-common.

## Features

- **Automatic hardware detection**: Scans PCI, USB, and DMI data to identify devices requiring non-free drivers.
- **Driver recommendations**: Ranks available drivers by stability (tested, recommended, latest) using the system device database.
- **One-click install**: Installs selected drivers via DKMS or pre-built packages without manual intervention.
- **Update notifications**: Monitors installed driver versions and alerts when newer packages are available.
- **Rollback support**: Keeps the previous driver version and allows single-command rollback.
- **CLI and GUI modes**: Full-featured terminal interface plus an optional GTK4 GUI overlay.

## Upstream References

- [mintdrivers](https://github.com/linuxmint/mintdrivers) — Linux Mint driver manager; reference for device detection and UI patterns.
- [ubuntu-drivers-common](https://github.com/tseliot/ubuntu-drivers-common) — Upstream library for device–driver matching logic.

## Building

```bash
pip install pylint
pylint src/ || true
python3 src/main.py --help
```

Requires Python 3.10+. Runtime dependencies: `python3-apt`, `python3-gi`, and kernel headers for DKMS builds.

## Contributing

1. Fork and create a feature branch.
2. Follow PEP 8; run `pylint src/` before committing.
3. Test against real hardware when adding new device IDs.
4. Document any new driver entries in `data/drivers.json`.
5. All contributions are released under the MIT License.
