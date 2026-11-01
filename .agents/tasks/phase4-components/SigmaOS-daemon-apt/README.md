# SigmaOS-daemon-apt

## Overview

SigmaOS-daemon-apt is a privilege-separated APT package management daemon. It exposes a D-Bus interface that allows unprivileged desktop components to request package installs, removals, and updates while keeping all APT operations isolated in a root-owned daemon process.

## Features

- D-Bus service interface for APT install, remove, upgrade, and search operations
- PolicyKit-based authorisation for all privileged operations
- Transaction progress reporting via D-Bus signals
- Debconf integration for interactive package configuration prompts
- Automatic lock management to prevent concurrent APT operations
- Structured error reporting with APT error codes exposed over D-Bus
- Logging of all transactions for auditability

## Upstream References

- [aptdaemon](https://launchpad.net/aptdaemon) — Python APT daemon providing the D-Bus interface design and transaction model this component is based on

## Building

Requires Python 3.10+ and `python3-apt`.

```bash
# Install dependencies
pip install -r requirements.txt

# Run the daemon (requires root for APT operations)
sudo python src/main.py

# Lint
pip install pylint && pylint src/
```

## Contributing

1. Fork the repository and create a feature branch.
2. Follow PEP 8 and run `pylint` before opening a pull request.
3. Security-sensitive changes (auth, PolicyKit) require two approvals.
4. Open a pull request against `main` with a clear description of the change.

All contributions are subject to the [MIT License](LICENSE).
