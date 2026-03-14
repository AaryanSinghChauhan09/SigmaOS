"""
SigmaOS Profile Manager (v1.0 Apex)
====================================
USP: Sovereign User Profile Persistence & Preference Management.
Modularized from PersonalizationEngine to handle pure data lifecycle.
"""
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager(SigmaModuleBase):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.profile_path = "userland/system_api/user_profile.sigma"
        self.preferences: Dict[str, Any] = self._load_profile()

    def _load_profile(self) -> Dict[str, Any]:
        if os.path.exists(self.profile_path):
            try:
                with open(self.profile_path, "r") as f:
                    data = json.load(f)
                    return data if isinstance(data, dict) else {}
            except: pass
        return {"preferred_mode": "NEUTRAL", "vibe": "CYBERPUNK", "auto_stealth": True, "carbon_credits": 0.0}

    def save_profile(self, data: Dict[str, Any]):
        try:
            os.makedirs(os.path.dirname(self.profile_path), exist_ok=True)
            with open(self.profile_path, "w") as f:
                json.dump(data, f, indent=4)
            self.preferences = data
        except: pass

    def get_preference(self, key: str, default: Any = None) -> Any:
        return self.preferences.get(key, default)

    def set_preference(self, key: str, value: Any):
        self.preferences[key] = value
        self.save_profile(self.preferences)
