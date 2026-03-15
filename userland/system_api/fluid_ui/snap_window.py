"""fluid_ui.snap_window — Physics-based window snapping."""

VALID_ZONES = ["Left_Half", "Right_Half", "Top_Left", "ZenCenter"]


def snap_window(state: dict, kernel=None, app_id: str = "", zone: str = "") -> str:
    """USP: Physics-based window snapping (Magnetic Layouts)."""
    if zone not in VALID_ZONES:
        return "Error: Invalid Snap Zone."
    msg = f"Magnetic Snap: '{app_id}' locked to '{zone}' (Tension: {state['physics']['spring_tension']})"
    if kernel and hasattr(kernel, "bus"):
        kernel.bus.emit("ui.window_snapped", {"app": app_id, "zone": zone})
    return msg
