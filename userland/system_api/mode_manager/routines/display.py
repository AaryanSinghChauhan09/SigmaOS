"""mode_manager.routines.display — Display control routines."""


def monitor_cpu_temp(phase: str = "") -> str:
    """Simulates starting CPU temperature monitoring."""
    return "CPU temperature monitoring started."


def dim_display(phase: str = "") -> str:
    """Simulates dimming the display."""
    return "Display dimmed."


def restore_display(phase: str = "") -> str:
    """Simulates restoring display brightness."""
    return "Display brightness restored."


def calibrate_display(phase: str = "") -> str:
    """Simulates display calibration."""
    return "Display calibrated."


def reset_display_calibration(phase: str = "") -> str:
    """Simulates resetting display calibration."""
    return "Display calibration reset."
