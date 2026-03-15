"""
Auto-split from ecosystem\sigma_app_store.py — SigmaAppStore.launch_app
"""

import time
from typing import List, Dict
from app_sandbox import SigmaAppSandbox



class SigmaAppStore:
    def launch_app(self, app_id: str) -> str:
        silo_id = self.installed_userland_apps.get(app_id)
        if not silo_id:
            return f"Error: '{app_id}' is not installed."
        msg = self.sandbox.execute_in_silo(silo_id, 'start')
        return f'{app_id} Silo [ACTIVE]: {msg}'
