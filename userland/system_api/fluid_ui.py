"""
fluid_ui — SigmaOS Fluid UI (v3 Apex)
======================================
Backward-compat shim.  Real implementation lives in fluid_ui/ package.
"""
import time
from typing import Dict, Any, List, Optional

from userland.system_api.fluid_ui.render_taskbar import render_taskbar_extension
from userland.system_api.fluid_ui.universal_search import launch_universal_search
from userland.system_api.fluid_ui.window_transparency import apply_window_transparency
from userland.system_api.fluid_ui.snap_window import snap_window
from userland.system_api.fluid_ui.configure_widgets import configure_widgets
from userland.system_api.fluid_ui.cognitive_metamorphosis import instant_cognitive_metamorphosis
from userland.system_api.fluid_ui.adaptive_theme import set_adaptive_theme
from userland.system_api.fluid_ui.health_check import health_check
from userland.system_api.fluid_ui.accessibility_suite import get_accessibility_suite


class SigmaFluidUI:
    """
    Sovereign Fluid UI (v3 Apex)
    ============================
    USP: Replaces clunky, static Desktop Environments with a physics-based,
    AI-adaptive UI layer. Class is a thin facade over the modular function package.
    """

    def __init__(self, kernel=None, user_name="Sovereign-User"):
        self.kernel = kernel
        self.user = user_name
        self._state = {
            "layout_mode": "Dynamic_Glass",
            "active_widgets": ["cpu_glance", "mission_control"],
            "physics": {"friction": 0.85, "spring_tension": 300},
            "transparency_alpha": 0.9,
        }

    # ── Delegating facade methods ────────────────────────────
    def render_taskbar_extension(self) -> str:
        return render_taskbar_extension(self.kernel)

    def launch_universal_search(self) -> str:
        return launch_universal_search()

    def apply_window_transparency(self, alpha: float = 0.9) -> str:
        return apply_window_transparency(self._state, self.kernel, alpha)

    def snap_window(self, app_id: str, zone: str) -> str:
        return snap_window(self._state, self.kernel, app_id, zone)

    def configure_widgets(self, add=None, remove=None):
        return configure_widgets(self._state, add, remove)

    def instant_cognitive_metamorphosis(self, stress_level: float, task_type: str) -> str:
        return instant_cognitive_metamorphosis(self._state, stress_level, task_type)

    def set_adaptive_theme(self, context: str) -> str:
        return set_adaptive_theme(self._state, context)

    def health_check(self) -> str:
        return health_check(self._state)

    @staticmethod
    def get_accessibility_suite() -> Dict[str, str]:
        return get_accessibility_suite()

    # ── State property pass-throughs ──────────────────────────
    @property
    def layout_mode(self) -> str:
        return self._state["layout_mode"]

    @property
    def transparency_alpha(self) -> float:
        return self._state["transparency_alpha"]

    @property
    def active_widgets(self) -> List[str]:
        return self._state["active_widgets"]
