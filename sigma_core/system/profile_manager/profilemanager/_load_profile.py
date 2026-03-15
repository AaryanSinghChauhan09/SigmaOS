# Generated method: ProfileManager._load_profile
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager:
    def _load_profile(self) -> Dict[str, Any]:
        if os.path.exists(self.profile_path):
            try:
                with open(self.profile_path, 'r') as f:
                    data = json.load(f)
                    return data if isinstance(data, dict) else {}
            except:
                pass
        return {'preferred_mode': 'NEUTRAL', 'vibe': 'CYBERPUNK', 'auto_stealth': True, 'carbon_credits': 0.0}