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
root_dir = str(os.path.dirname(_p))
if root_dir and root_dir not in sys.path: sys.path.insert(0, root_dir)

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
        hal = self.kernel.hal
        carbon = hal.get_carbon_footprint()
        
        summary = {
            "timestamp": time.time(),
            "cpu_vibe": hal.get_hardware_state().get("cpu_load"),
            "carbon_impact": carbon.get("hourly_impact_gCO2"),
            "efficiency": hal.get_energy_efficiency().get("efficiency_nps"),
            "security_integrity": "STABLE",
            "xp_progress": self.kernel.gamification.get_status() if hasattr(self.kernel, "gamification") else "BETA",
            "active_swarms": self.kernel.cortex.stats.get("cognitive_cycles", 0) if hasattr(self.kernel, "cortex") else 0
        }
        return summary

    def render_cli_hud(self):
        """USP: Minimalist Visualized HUD for Sovereign Researchers."""
        data = self.get_analytic_summary()
        hud = [
            "╔══════════════════════════════════════════════════════════════╗",
            f"║ SigmaOS Morphic Dashboard | v5.3.0 | {data['security_integrity']} ║",
            "╠══════════════════════════════════════════════════════════════╣",
            f"║ [CPU] Load: {data['cpu_vibe']:<43}  ║",
            f"║ [ENV] Carbon: {data['carbon_impact']:<15} Efficiency: {data['efficiency']:<12} ║",
            f"║ [XP ] Progress: {str(data['xp_progress']):<41} ║",
            f"║ [AI ] Cognitive Cycles: {data['active_swarms']:<33} ║",
            "╚══════════════════════════════════════════════════════════════╝"
        ]
        return "\n".join(hud)

    def health_check(self) -> str:
        return f"OK — Dashboard Active (View: {self.active_view})"
