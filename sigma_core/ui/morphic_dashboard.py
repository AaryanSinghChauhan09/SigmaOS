"""
SigmaOS Morphic Dashboard (v1.0 Sovereign)
===========================================
USP: Real-time visual analytics hub with interactive system telemetry.
Integrates gamification progress and security status into a unified HUD.
"""
import os
import sys
import time
import random

# Robust System Path Injection
_p = os.path.abspath(__file__)
while _p and not os.path.exists(os.path.join(os.path.dirname(_p), "sigma_core")):
    _p = os.path.dirname(_p)
    if _p == os.path.dirname(_p): break
root = os.path.dirname(_p)
if root and root not in sys.path: sys.path.insert(0, root)

from sigma_core.system.interfaces import SigmaModuleBase

class MorphicDashboard(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.active_view = "SYS_HEALTH"
        self.refresh_rate = 1.0
        self.telemetry_history = []

    def get_analytic_summary(self):
        """USP: Recursive analytic pruning of system vibes."""
        if not self.kernel: 
            return {"error": "ORPHANED_DASHBOARD"}
            
        # Aggregated Telemetry from all shards
        summary = {
            "timestamp": time.time(),
            "cpu_vibe": self.kernel.hal.get_cpu_load(),
            "security_integrity": "STABLE" if self.kernel.security_warden else "DEGRADED",
            "xp_progress": self.kernel.registry.get("gamification").get_status() if self.kernel.registry.get("gamification") else "BETA",
            "active_swarms": self.kernel.registry.get("intelligence").stats.get("swarms_deployed", 0) if self.kernel.registry.get("intelligence") else 0
        }
        return summary

    def render_cli_hud(self):
        """USP: Minimalist Visualized HUD for Sovereign Researchers."""
        data = self.get_analytic_summary()
        hud = [
            "╔══════════════════════════════════════════════════════════════╗",
            f"║ SigmaOS Morphic Dashboard | v5.2.0 | {data['security_integrity']} ║",
            "╠══════════════════════════════════════════════════════════════╣",
            f"║ [CPU] {data['cpu_vibe']:<51} ║",
            f"║ [XP ] {str(data['xp_progress']):<51} ║",
            f"║ [SW ] Swarms Online: {data['active_swarms']:<41} ║",
            "╚══════════════════════════════════════════════════════════════╝"
        ]
        return "\n".join(hud)

    def health_check(self) -> str:
        return f"OK — Dashboard Active (View: {self.active_view})"
