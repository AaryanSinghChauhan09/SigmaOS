# Generated method: ProfileManager.__init__
import os
import json
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ProfileManager:
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.profile_path = 'userland/system_api/user_profile.sigma'
        self.preferences: Dict[str, Any] = self._load_profile()