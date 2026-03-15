"""
Auto-split from ecosystem\sigma_app_store.py — SigmaAppStore.install_app
"""

import time
from typing import List, Dict
from app_sandbox import SigmaAppSandbox



class SigmaAppStore:
    def install_app(self, app_id: str) -> str:
        """One-click 'hydration' install for apps and games."""
        app_name = 'Unknown App'
        found = False
        self._load_catalog()
        for cat in self.catalog.values():
            for a in cat:
                if a['id'] == app_id:
                    app_name = a['name']
                    found = True
                    break
        if not found:
            return f"Error: App '{app_id}' not found."
        if app_id in self.installed_userland_apps:
            return f"'{app_name}' is already installed."
        if app_id.startswith('G'):
            if hasattr(self.kernel, 'games'):
                res = self.kernel.games.install_game(app_id)
                if res['status'] == 'success':
                    self.installed_userland_apps[app_id] = 'GAME_SILO'
                    return res['message']
        silo_id = self.sandbox.create_silo(app_id)
        self.installed_userland_apps[app_id] = silo_id
        if hasattr(self.kernel, 'fs'):
            self.kernel.fs.create_snapshot(f'app-install-{app_id}')
        return f"Successfully hydrated '{app_name}' and siloted in {silo_id}."
