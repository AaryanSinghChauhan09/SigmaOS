# Generated method: SigmaAppStore.list_userland_apps
import time
from typing import List, Dict
from app_sandbox import SigmaAppSandbox

class SigmaAppStore:
    def list_userland_apps(self, category: str=None) -> Dict:
        """Returns the current catalog (can filter by category)."""
        self._load_catalog()
        if category:
            return {category: self.catalog.get(category, [])}
        return self.catalog