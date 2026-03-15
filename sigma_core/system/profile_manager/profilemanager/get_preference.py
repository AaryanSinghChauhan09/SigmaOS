# Generated method: ProfileManager.get_preference
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager:
    def get_preference(self, key: str, default: Any=None) -> Any:
        return self.preferences.get(key, default)