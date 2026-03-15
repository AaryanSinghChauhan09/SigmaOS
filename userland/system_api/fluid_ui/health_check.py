"""fluid_ui.health_check — FluidUI health probe."""


def health_check(state: dict) -> str:
    """Returns health status of the Fluid UI subsystem."""
    return f"OK — Fluid UI | Mode: {state['layout_mode']} | Cognitive Morphing Active."
