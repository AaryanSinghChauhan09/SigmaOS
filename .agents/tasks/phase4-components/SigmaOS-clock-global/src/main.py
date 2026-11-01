"""SigmaOS-clock-global: World clock and timezone management tool.

Displays the current time across multiple configured timezones and provides
a simple API for adding, removing, and listing timezone entries.
"""

from __future__ import annotations

import json
import os
import sys
from datetime import datetime
from zoneinfo import ZoneInfo, ZoneInfoNotFoundError

CONFIG_PATH = os.path.expanduser("~/.config/sigmaos/clock-global.json")

DEFAULT_TIMEZONES: list[str] = [
    "UTC",
    "America/New_York",
    "Europe/London",
    "Asia/Tokyo",
]


def load_config() -> list[str]:
    """Load saved timezone list from disk, falling back to defaults."""
    if os.path.exists(CONFIG_PATH):
        try:
            with open(CONFIG_PATH, "r", encoding="utf-8") as fh:
                data = json.load(fh)
            if isinstance(data, list):
                return data
        except (json.JSONDecodeError, OSError):
            pass
    return list(DEFAULT_TIMEZONES)


def save_config(timezones: list[str]) -> None:
    """Persist the timezone list to disk."""
    os.makedirs(os.path.dirname(CONFIG_PATH), exist_ok=True)
    with open(CONFIG_PATH, "w", encoding="utf-8") as fh:
        json.dump(timezones, fh, indent=2)


def resolve_zone(name: str) -> ZoneInfo | None:
    """Return a ZoneInfo object for *name*, or None if unknown."""
    try:
        return ZoneInfo(name)
    except ZoneInfoNotFoundError:
        return None


def display_clocks(timezones: list[str]) -> None:
    """Print the current time for each timezone in *timezones*."""
    now_utc = datetime.now(ZoneInfo("UTC"))
    col_width = max(len(tz) for tz in timezones) + 2
    print(f"\n{'Timezone':<{col_width}}  {'Local Time':<25}  UTC Offset")
    print("-" * (col_width + 40))
    for tz_name in timezones:
        zone = resolve_zone(tz_name)
        if zone is None:
            print(f"{tz_name:<{col_width}}  {'(unknown timezone)':<25}")
            continue
        local_time = now_utc.astimezone(zone)
        offset = local_time.strftime("%z")
        formatted = local_time.strftime("%Y-%m-%d %H:%M:%S")
        print(f"{tz_name:<{col_width}}  {formatted:<25}  {offset}")
    print()


def add_timezone(timezones: list[str], name: str) -> list[str]:
    """Add *name* to *timezones* if valid and not already present."""
    if resolve_zone(name) is None:
        print(f"Error: '{name}' is not a recognised IANA timezone.", file=sys.stderr)
        return timezones
    if name in timezones:
        print(f"'{name}' is already in the list.")
        return timezones
    timezones.append(name)
    print(f"Added '{name}'.")
    return timezones


def remove_timezone(timezones: list[str], name: str) -> list[str]:
    """Remove *name* from *timezones* if present."""
    if name not in timezones:
        print(f"'{name}' not found in the list.", file=sys.stderr)
        return timezones
    timezones.remove(name)
    print(f"Removed '{name}'.")
    return timezones


def print_usage() -> None:
    """Print CLI usage information."""
    print(
        "Usage:\n"
        "  main.py                  Show all configured world clocks\n"
        "  main.py add <timezone>   Add a timezone (IANA name)\n"
        "  main.py remove <tz>      Remove a timezone\n"
        "  main.py list             List configured timezones\n"
        "  main.py reset            Reset to default timezones\n"
    )


def main(argv: list[str] | None = None) -> int:
    """Entry point for the SigmaOS world clock tool."""
    if argv is None:
        argv = sys.argv[1:]

    timezones = load_config()

    if not argv:
        display_clocks(timezones)
        return 0

    command = argv[0].lower()

    if command == "add" and len(argv) == 2:
        timezones = add_timezone(timezones, argv[1])
        save_config(timezones)
    elif command == "remove" and len(argv) == 2:
        timezones = remove_timezone(timezones, argv[1])
        save_config(timezones)
    elif command == "list":
        for tz in timezones:
            print(tz)
    elif command == "reset":
        timezones = list(DEFAULT_TIMEZONES)
        save_config(timezones)
        print("Reset to default timezones.")
    else:
        print_usage()
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
