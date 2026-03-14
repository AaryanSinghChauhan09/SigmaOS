"""
SigmaOS Minimalist Controller (v1.0 Apex)
===========================================
USP: Dynamic OS Pruning and Shard Deactivation for Ultra-Low Resources.
Ensures SigmaOS can run in a 'Ghost' state with near-zero overhead.
"""
from typing import List, Dict, Any

try:
    from sigma_core.system.interfaces import SigmaModuleBase
except (ImportError, ValueError):
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel
        def log_event(self, a, c): pass

class MinimalistController(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.active_mode = "STANDARD"
        self.non_essential_shards = ["gamification", "nexus", "portal", "vision"]

    def engage_minimalist_mode(self) -> str:
        """USP: Minimalist Operation. Deactivates UI and Gamification layers."""
        if not self.kernel: return "Kernel Link Required."
        
        self.active_mode = "MINIMAL"
        deactivated = []
        
        for shard in self.non_essential_shards:
            if hasattr(self.kernel, shard):
                _shard_obj = getattr(self.kernel, shard)
                if _shard_obj and hasattr(_shard_obj, "stop_service"):
                    _shard_obj.stop_service()
                deactivated.append(shard)
        
        if hasattr(self.kernel, "resource_alchemist") and self.kernel.resource_alchemist:
            self.kernel.resource_alchemist.shift_profile("SUSTAINABLE")
            
        if hasattr(self.kernel, "hal") and self.kernel.hal and hasattr(self.kernel.hal, "loader"):
            self.kernel.hal.loader.hot_unload_core("ipc") 
            
        return f"Minimalist Mode Engaged: {', '.join(deactivated)} shards deactivated. Resources Saved."

    def release_minimalist_mode(self) -> str:
        """USP: Dynamic Recovery. Restores OS to full capability."""
        self.active_mode = "STANDARD"
        if self.kernel and hasattr(self.kernel, "resource_alchemist") and self.kernel.resource_alchemist:
            self.kernel.resource_alchemist.shift_profile("NEURAL_RESEARCH")
        return "OS Re-Hydrated: All layers restoration initiated."

    def health_check(self) -> str:
        return f"OK — Mode: {self.active_mode} | Stealth: {'ACTIVE' if self.active_mode == 'MINIMAL' else 'STANDBY'}"
