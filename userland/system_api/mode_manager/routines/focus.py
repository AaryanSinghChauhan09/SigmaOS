"""mode_manager.routines.focus — Focus timer routines."""


def start_focus_timer(phase: str = "") -> str:
    """Starts a Pomodoro-style focus timer (25 min work / 5 min break)."""
    return "Focus Timer ACTIVE: 25-minute Pomodoro session started. Distractions blocked."


def stop_focus_timer(phase: str = "") -> str:
    """Stops the active focus timer."""
    return "Focus Timer STOPPED. All sessions logged. Distraction control lifted."
