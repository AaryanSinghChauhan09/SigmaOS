"""mode_manager.routines.log_mode_change — Mode-change logger."""
import time


def log_mode_change(current_mode: str = "", phase: str = "") -> str:
    """Logs the mode change event."""
    return f"System log: Mode change {phase} for {current_mode} at {time.time()}."
