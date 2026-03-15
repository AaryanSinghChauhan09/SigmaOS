# Generated method: ProfileManager.set_preference
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager:
    def set_preference(self, key: str, value: Any):
        self.preferences[key] = value
        self.save_profile(self.preferences)