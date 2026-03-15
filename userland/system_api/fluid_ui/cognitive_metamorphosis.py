"""fluid_ui.cognitive_metamorphosis — Instant Cognitive UI morphing."""


def instant_cognitive_metamorphosis(state: dict, stress_level: float, task_type: str) -> str:
    """USP: Phase 3 Singularity - Instant Cognitive UI based on biometric/task inputs."""
    from userland.system_api.fluid_ui.configure_widgets import configure_widgets
    from userland.system_api.fluid_ui.window_transparency import apply_window_transparency

    if stress_level > 7.5:
        state["layout_mode"] = "Absolute_Minimalism (High-Stress Override)"
        configure_widgets(state, remove=list(state["active_widgets"]))
        apply_window_transparency(state, alpha=1.0)
        return (
            f"COGNITIVE-UI: Stress spike detected ({stress_level}). "
            "Liquidating distractions. Shell morphing into pure text-focus layout."
        )
    if "creative" in task_type.lower():
        state["layout_mode"] = "Holographic_Canvas"
        apply_window_transparency(state, alpha=0.4)
        return "COGNITIVE-UI: Creative intent sensed. Shell borders dissolved. Glassmorphism maximized to 60%."
    return "COGNITIVE-UI: Biological steady-state. Standard morphological boundaries maintained."
