# SigmaOS-clock-global

## Overview

SigmaOS-clock-global is the world clock and timezone management tool for SigmaOS. It displays the current time across multiple user-configured timezones simultaneously, making it easy to coordinate across regions and track time globally.

## Features

- Display multiple world clocks in a clean, configurable panel
- Add, remove, and reorder timezone entries
- Automatic DST (daylight saving time) adjustments
- Integration with SigmaOS desktop notifications for alarms and reminders
- CLI mode for scripting and headless use cases
- Persistent timezone configuration stored in user preferences

## Upstream References

- [elsewhen](https://github.com/nicholasgasior/elsewhen) — lightweight world clock inspiration
- [gnome-clocks](https://gitlab.gnome.org/GNOME/gnome-clocks) — GNOME Clocks application for design patterns and timezone handling

## Building

Requires Python 3.10+.

```bash
# Install dependencies
pip install -r requirements.txt

# Run the application
python src/main.py

# Lint
pip install pylint && pylint src/
```

## Contributing

1. Fork the repository and create a feature branch.
2. Follow the PEP 8 style guide; lint with `pylint` before submitting.
3. Add or update tests where applicable.
4. Open a pull request against `main` with a clear description of the change.

All contributions are subject to the [MIT License](LICENSE).
