# Generated method: ProfileManager.save_profile
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager:
    def save_profile(self, data: Dict[str, Any]):
        try:
            os.makedirs(os.path.dirname(self.profile_path), exist_ok=True)
            with open(self.profile_path, 'w') as f:
                json.dump(data, f, indent=4)
            self.preferences = data
        except:
            pass