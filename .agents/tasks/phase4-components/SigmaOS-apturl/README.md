# SigmaOS-apturl

## Overview

SigmaOS-apturl is a URL handler that intercepts `apt://` URIs and triggers the installation of the specified package(s) through the system package manager. When a user clicks an `apt://packagename` link in a browser or document, this tool presents a confirmation dialog and then invokes `apt-get install` (or `pkcon install` as a fallback) with appropriate privilege escalation.

## Features

- Handles `apt://` URI scheme as registered in the XDG MIME database
- Parses one or multiple package names from a single URI (`apt://pkg1,pkg2`)
- Displays a GTK confirmation dialog before installing anything
- Supports optional URI query parameters: `?section=`, `?channel=` for repository hints
- Polkit-based privilege escalation — no hard dependency on `sudo`
- Dry-run mode for scripting and testing (`--dry-run` flag)

## Upstream References

- [apturl](https://github.com/linuxmint/apturl) — Original Mint/Ubuntu apt:// URL handler this component is derived from

## Building

This component is written in Python. No compilation step is required.

```bash
# Install development dependencies
pip install pylint

# Lint the source
pylint src/

# Register the apt:// URI handler (development)
python3 src/main.py --register

# Test a dry-run installation
python3 src/main.py --dry-run apt://curl
```

Runtime requirements: `python3-gi`, `gir1.2-gtk-3.0`, `apt` or `packagekit`.

## Contributing

1. Fork this repository and create a feature branch.
2. Follow PEP 8 style guidelines; run `pylint src/` before submitting.
3. Security-sensitive changes (privilege escalation paths) require two reviewer approvals.
4. Open a pull request against `main` with a clear description of the change.
5. All contributions must be licensed under the MIT License.
