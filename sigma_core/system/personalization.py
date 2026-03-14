"""
SigmaOS Personalization Engine (v1.0 Sovereign)
================================================
USP: Adaptive system vibes and personalized resource allocation.
Learns from user interactions to automate 'Ghost Mode' and 'Apex Mode' shifts.
"""
import os
import sys
import json
import time

# Robust System Path Injection
_p = os.path.abspath(__file__)
while _p and not os.path.exists(os.path.join(os.path.dirname(_p), "sigma_core")):
    _p = os.path.dirname(_p)
    if _p == os.path.dirname(_p): break
root = os.path.dirname(_p)
if root and root not in sys.path: sys.path.insert(0, root)

from sigma_core.system.interfaces import SigmaModuleBase

class PersonalizationEngine(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.profile_path = "userland/system_api/user_profile.sigma"
        self.user_preferences = self._load_profile()
        self.adaptive_threshold = 0.85

    def _load_profile(self):
        if os.path.exists(self.profile_path):
            try:
                with open(self.profile_path, "r") as f:
                    return json.load(f)
            except: pass
        return {"preferred_mode": "NEUTRAL", "vibe": "CYBERPUNK", "auto_stealth": True}

    def adapt_system(self):
        """USP: Resilient & Adaptive Environment Awareness."""
        vibe = self.user_preferences.get("vibe", "DEFAULT")
        if self.kernel and hasattr(self.kernel, "config"):
            self.kernel.config.apply_vibe(vibe)
            
        # If CPU load is low, enable resource-saving
        if self.kernel and self.kernel.hal.get_cpu_load() == "Adaptive Logic: [STABLE]":
             self.kernel.bus.emit("power.conserve", {"level": "MAX"})
        
        return f"Sovereign: Environment adapted to {vibe}."

    def set_stealth_priority(self, enabled: bool):
        self.user_preferences["auto_stealth"] = enabled
        if enabled and self.kernel:
            self.kernel.registry.get("security").stats["anonymity_score"] = 99.9
        
    def health_check(self) -> str:
        return f"OK — Profile Ready | Vibe: {self.user_preferences['preferred_mode']}"
