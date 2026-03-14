"""
SigmaOS Adaptive Governor (v1.0 Apex)
=====================================
USP: Multi-stage orchestration of System Performance, Energy, and UX.
Integrates Predictive Scheduling, Mode Management, and Eco-Gamification.
"""
from typing import Dict, Any, List
try:
    from .interfaces import SigmaModuleBase # type: ignore
    from ..ui.fluid_design import FluidTheme # type: ignore
except ImportError:
    from sigma_core.system.interfaces import SigmaModuleBase # type: ignore
    from sigma_core.ui.fluid_design import FluidTheme # type: ignore

class AdaptiveGovernor(SigmaModuleBase):
    """
    The 'Brain' of the system subsystems. 
    Adjusts kernel tunables based on the current 'Sovereign Vibe'.
    """
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.state = {
            "adaptive_mode": "BALANCED",
            "performance_level": 1.0,
            "eco_priority": False,
            "drift_protection": True
        }
        self.current_vibe = "STANDARD"

    def switch_vibe(self, vibe_name: str):
        """USP: Atomic switch of system state and aesthetic alignment."""
        self.current_vibe = vibe_name
        
        # Mapping vibes to UI themes
        vibe_map = {
            "APEX": "APEX_GOLD",
            "RESOURCE_SAVING": "FOREST_ECO",
            "STANDARD": "DEEP_SPACE"
        }
        # Thread-safe theme update
        FluidTheme.set_vibe(vibe_map.get(vibe_name, "DEEP_SPACE"))
        
        if self.kernel and hasattr(self.kernel, "bus"):
            # Emit kernel event for other modules to react
            self.kernel.bus.emit("governor.vibe_switch", {"vibe": vibe_name})
            print(f"[AURA] Switched to {vibe_name} aesthetic.")

    def start_service(self) -> str:
        # Subscribe to mode changes to adjust system DNA
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.subscribe("mode.change", self._on_mode_change)
            self.kernel.bus.subscribe("eco.green_window", self._on_eco_window)
        return "Adaptive Governor: Orchestration Mesh Online."

    def _on_mode_change(self, payload: Dict[str, Any]):
        """USP: Dynamic DNA Shift. Recalibrates all shards when OS mode changes."""
        mode = payload.get("mode", "Standard").upper()
        self.log_event("dna_shift", {"target_mode": mode})

        if mode == "GAMING" or mode == "APEX":
            self._apply_profile(perf=2.0, eco=False, scheduler="QUANTUM")
            self.switch_vibe("APEX")
        elif mode == "RESOURCE_SAVING":
            self._apply_profile(perf=0.5, eco=True, scheduler="BATCH")
            self.switch_vibe("RESOURCE_SAVING")
        else:
            self._apply_profile(perf=1.0, eco=False, scheduler="NORMAL")
            self.switch_vibe("STANDARD")

    def _on_eco_window(self, payload: Dict[str, Any]):
        """Responds to high carbon intensity by throttling non-critical shards."""
        active = payload.get("active", False)
        if active:
            self.state["eco_priority"] = True
            if self.kernel.perf:
                self.kernel.perf.apply_tuning("Eco")
        else:
            self.state["eco_priority"] = False

    def _apply_profile(self, perf: float, eco: bool, scheduler: str):
        """Orchestrates across specialized system modules."""
        self.state["performance_level"] = perf
        self.state["eco_priority"] = eco
        
        # 1. Update Predictive Scheduler via the kernel attribute accessor
        if hasattr(self.kernel, "pbs"):
            self.kernel.pbs.set_policy(scheduler)
        
        # 2. Update Hardware Tuning (HAL/Boost)
        if hasattr(self.kernel, "perf"):
            intensity = "High" if perf > 1.2 else "Medium"
            self.kernel.perf.apply_tuning(intensity)

    def health_check(self) -> str:
        return f"OK — Profile: {self.state['adaptive_mode']} | Perf: {self.state['performance_level']}x"

if __name__ == "__main__":
    # Local verification
    gov = AdaptiveGovernor()
    print(gov.health_check())
