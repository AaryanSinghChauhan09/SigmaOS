"""mode_manager.routines.console — Console switching routines."""


def switch_to_text_console(phase: str = "") -> str:
    """Simulates switching to a text-only console."""
    return "Switched to text console."


def kill_gui_processes(phase: str = "") -> str:
    """Simulates killing GUI-related processes."""
    return "GUI processes terminated."


def start_gui_processes(phase: str = "") -> str:
    """Simulates starting GUI-related processes."""
    return "GUI processes started."


def switch_to_graphical_console(phase: str = "") -> str:
    """Simulates switching to a graphical console."""
    return "Switched to graphical console."
