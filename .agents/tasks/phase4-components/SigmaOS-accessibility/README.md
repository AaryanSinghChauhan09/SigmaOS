# SigmaOS-accessibility

## Overview

SigmaOS-accessibility provides the accessibility tools and AT-SPI (Assistive Technology Service Provider Interface) bridge for the SigmaOS desktop. It ensures that all SigmaOS components are accessible to users who rely on assistive technologies such as screen readers, magnifiers, and switch access devices.

## Features

- AT-SPI2 bridge integration for full assistive technology support
- Screen reader compatibility via the Orca screen reader backend
- Keyboard navigation enhancements for all desktop components
- High-contrast and large-text theme hooks
- Accessibility event logging and debugging via accerciser integration
- Configurable accessibility profiles stored per user session

## Upstream References

- [accerciser](https://gitlab.gnome.org/GNOME/accerciser) — Interactive Python accessibility explorer for GNOME
- [orca](https://gitlab.gnome.org/GNOME/orca) — Scriptable screen reader for GNOME-based desktops

## Building

This component is written in Python. No compilation step is required.

```bash
# Install development dependencies
pip install pylint

# Lint the source
pylint src/

# Run the module directly (requires AT-SPI2 runtime)
python3 src/main.py
```

Runtime requirements: `python3-pyatspi`, `at-spi2-core`, `orca` (optional).

## Contributing

1. Fork this repository and create a feature branch.
2. Follow PEP 8 style guidelines; run `pylint src/` before submitting.
3. Add or update docstrings for any new public functions or classes.
4. Open a pull request against `main` with a clear description of the change.
5. All contributions must be licensed under the MIT License.
