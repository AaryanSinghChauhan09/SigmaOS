"""
SigmaOS Personalization Engine (v2.0 Apex)
================================================
USP: Adaptive system vibes and personalized resource allocation.
Learns from user interactions to automate 'Ghost Mode' and 'Apex Mode' shifts.
"""
import os
import sys
import json
import time
from typing import Dict, Any, Optional

try:
    from sigma_core.system.interfaces import SigmaModuleBase
except ImportError:
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel
        def log_event(self, a, c): pass

class PersonalizationEngine(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.profile_path = "userland/system_api/user_profile.sigma"
        self.user_preferences: Dict[str, Any] = self._load_profile()
        self.adaptive_threshold = 0.85

    def _load_profile(self) -> Dict[str, Any]:
        if os.path.exists(self.profile_path):
            try:
                with open(self.profile_path, "r") as f:
                    data = json.load(f)
                    return data if isinstance(data, dict) else {}
            except: pass
        return {"preferred_mode": "NEUTRAL", "vibe": "CYBERPUNK", "auto_stealth": True}

    def learn_vibe_pattern(self, event_type: str, intensity: float):
        """USP: Automated Interaction Learning. Maps user pulses to OS vibes."""
        if event_type == "KEYBOARD_HIGH_VELOCITY" and intensity > 0.8:
            self.user_preferences["preferred_mode"] = "APEX_GAMING"
            self.log_event("cognitive_learn", {"shift": "APEX_FOCUS"})
        elif event_type == "LOW_ACTIVITY" and intensity < 0.2:
            self.user_preferences["preferred_mode"] = "SUSTAINABLE"
            
        self._save_profile(self.user_preferences)

    def _save_profile(self, data: Dict[str, Any]):
        try:
            os.makedirs(os.path.dirname(self.profile_path), exist_ok=True)
            with open(self.profile_path, "w") as f:
                json.dump(data, f, indent=4)
        except: pass

    def adapt_system(self) -> str:
        """USP: Resilient & Adaptive Environment Awareness."""
        mode = self.user_preferences.get("preferred_mode", "NEUTRAL")
        
        # Proactively inform the Resource Alchemist
        if self.kernel and hasattr(self.kernel, "resource_alchemist"):
            self.kernel.resource_alchemist.shift_profile(str(mode))
            
        return f"Sovereign: Environment automated for {mode} mode."

    def health_check(self) -> str:
        mode = self.user_preferences.get("preferred_mode", "NEUTRAL")
        return f"OK — Mode: {mode} | Adaptive: ACTIVE"
