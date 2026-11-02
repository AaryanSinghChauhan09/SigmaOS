"""SigmaOS-configurator: System configurator and first-run setup wizard.

Guides the user through initial system configuration steps including locale,
keyboard layout, timezone, and user account settings.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from dataclasses import dataclass, field

CONFIG_PATH = os.path.expanduser("~/.config/sigmaos/configurator-state.json")

# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------


@dataclass
class ConfigState:
    """Tracks which wizard steps have been completed."""

    locale_done: bool = False
    keyboard_done: bool = False
    timezone_done: bool = False
    account_done: bool = False
    extras_done: bool = False

    chosen_locale: str = "en_US.UTF-8"
    chosen_keyboard: str = "us"
    chosen_timezone: str = "UTC"
    chosen_username: str = ""
    install_extras: bool = False
    extra_packages: list[str] = field(default_factory=list)


# ---------------------------------------------------------------------------
# Persistence
# ---------------------------------------------------------------------------


def load_state() -> ConfigState:
    """Load wizard state from disk if available."""
    if os.path.exists(CONFIG_PATH):
        try:
            with open(CONFIG_PATH, "r", encoding="utf-8") as fh:
                data = json.load(fh)
            state = ConfigState()
            for key, value in data.items():
                if hasattr(state, key):
                    setattr(state, key, value)
            return state
        except (json.JSONDecodeError, OSError):
            pass
    return ConfigState()


def save_state(state: ConfigState) -> None:
    """Persist wizard state to disk."""
    os.makedirs(os.path.dirname(CONFIG_PATH), exist_ok=True)
    data = {
        "locale_done": state.locale_done,
        "keyboard_done": state.keyboard_done,
        "timezone_done": state.timezone_done,
        "account_done": state.account_done,
        "extras_done": state.extras_done,
        "chosen_locale": state.chosen_locale,
        "chosen_keyboard": state.chosen_keyboard,
        "chosen_timezone": state.chosen_timezone,
        "chosen_username": state.chosen_username,
        "install_extras": state.install_extras,
        "extra_packages": state.extra_packages,
    }
    with open(CONFIG_PATH, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2)


# ---------------------------------------------------------------------------
# Wizard steps
# ---------------------------------------------------------------------------


def step_locale(state: ConfigState) -> None:
    """Wizard step: configure system locale."""
    print("\n=== Step 1/5: Locale ===")
    print(f"Current locale: {state.chosen_locale}")
    locale = input("Enter locale (e.g. en_US.UTF-8) [Enter to keep]: ").strip()
    if locale:
        state.chosen_locale = locale
    state.locale_done = True
    print(f"Locale set to: {state.chosen_locale}")


def step_keyboard(state: ConfigState) -> None:
    """Wizard step: configure keyboard layout."""
    print("\n=== Step 2/5: Keyboard Layout ===")
    print(f"Current layout: {state.chosen_keyboard}")
    layout = input("Enter keyboard layout (e.g. us, gb, de) [Enter to keep]: ").strip()
    if layout:
        state.chosen_keyboard = layout
    state.keyboard_done = True
    print(f"Keyboard layout set to: {state.chosen_keyboard}")


def step_timezone(state: ConfigState) -> None:
    """Wizard step: configure timezone."""
    print("\n=== Step 3/5: Timezone ===")
    print(f"Current timezone: {state.chosen_timezone}")
    timezone = input("Enter timezone (IANA name, e.g. America/New_York) [Enter to keep]: ").strip()
    if timezone:
        state.chosen_timezone = timezone
    state.timezone_done = True
    print(f"Timezone set to: {state.chosen_timezone}")


def step_account(state: ConfigState) -> None:
    """Wizard step: configure user account."""
    print("\n=== Step 4/5: User Account ===")
    username = input("Enter username for the primary account: ").strip()
    if username:
        state.chosen_username = username
    state.account_done = True
    print(f"Username set to: {state.chosen_username}")


def step_extras(state: ConfigState) -> None:
    """Wizard step: optional extra package installation."""
    print("\n=== Step 5/5: Optional Extras ===")
    answer = input("Install recommended extras (codecs, fonts)? [y/N]: ").strip().lower()
    state.install_extras = answer == "y"
    if state.install_extras:
        state.extra_packages = ["ubuntu-restricted-extras", "fonts-noto"]
        print("Extras will be installed.")
    else:
        print("Skipping extras.")
    state.extras_done = True


def apply_configuration(state: ConfigState) -> None:
    """Apply the collected configuration to the system (dry-run in non-root mode)."""
    print("\n=== Applying Configuration ===")
    print(f"  Locale:   {state.chosen_locale}")
    print(f"  Keyboard: {state.chosen_keyboard}")
    print(f"  Timezone: {state.chosen_timezone}")
    print(f"  Username: {state.chosen_username}")
    if state.install_extras:
        print(f"  Extras:   {', '.join(state.extra_packages)}")

    if os.geteuid() == 0:
        # Real system commands when running as root
        subprocess.run(["timedatectl", "set-timezone", state.chosen_timezone], check=False)
        subprocess.run(
            ["localectl", "set-keymap", state.chosen_keyboard], check=False
        )
        if state.install_extras and state.extra_packages:
            subprocess.run(
                ["apt-get", "install", "-y"] + state.extra_packages, check=False
            )
    else:
        print("  (Running as non-root: system commands skipped; configuration saved.)")

    print("\nFirst-run configuration complete.")


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def run_wizard(state: ConfigState) -> None:
    """Run the full first-run wizard."""
    print("Welcome to SigmaOS First-Run Configuration Wizard")
    print("=" * 50)

    steps = [
        ("locale", step_locale),
        ("keyboard", step_keyboard),
        ("timezone", step_timezone),
        ("account", step_account),
        ("extras", step_extras),
    ]

    for step_name, step_fn in steps:
        done_attr = f"{step_name}_done"
        if not getattr(state, done_attr, False):
            step_fn(state)
            save_state(state)
        else:
            print(f"\n[Skipping {step_name} — already configured]")

    apply_configuration(state)


def main(argv: list[str] | None = None) -> int:
    """Entry point for SigmaOS-configurator."""
    if argv is None:
        argv = sys.argv[1:]

    state = load_state()

    if "--reset" in argv:
        state = ConfigState()
        save_state(state)
        print("Configuration state reset.")

    if "--status" in argv:
        print(json.dumps(state.__dict__, indent=2, default=str))
        return 0

    try:
        run_wizard(state)
    except KeyboardInterrupt:
        print("\nWizard cancelled. Progress saved.")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
