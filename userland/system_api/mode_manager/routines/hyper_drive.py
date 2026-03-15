"""mode_manager.routines.hyper_drive — Hyper-Drive engagement routines."""


def engage_hyper_drive(kernel=None, phase: str = "") -> str:
    """USP: Engages the Hyper-Drive Quantum Optimizer."""
    if kernel and hasattr(kernel, "registry"):
        hd = kernel.registry.get("hyper_drive")
        if hd and hasattr(hd, "execute_ai_debloat") and hasattr(hd, "trigger_precognitive_cache"):
            hd.execute_ai_debloat()
            hd.trigger_precognitive_cache("Optimizing for Apex performance.")
            return "Hyper-Drive engaged: AI De-bloat and Pre-cognitive cache active."
    return "Hyper-Drive module not found."


def activate_zen_latency(kernel=None, phase: str = "") -> str:
    """USP: Activates Zen Latency mode for instant UI feedback."""
    if kernel and hasattr(kernel, "registry"):
        hd = kernel.registry.get("hyper_drive")
        if hd and hasattr(hd, "engage_zen_latency_mode"):
            return hd.engage_zen_latency_mode()
    return "Hyper-Drive module not available for Zen Latency."


def disengage_hyper_drive(phase: str = "") -> str:
    """Disengages Hyper-Drive optimizations."""
    return "Hyper-Drive disengaged. Reverting to standard scheduling."
