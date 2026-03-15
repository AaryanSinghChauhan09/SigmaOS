# Generated method: SigmaAppStore.uninstall
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaAppStore:
    def uninstall(self, app_id: str) -> str:
        if app_id not in self._installed:
            return f"Error: '{app_id}' is not installed."
        app = self._installed.pop(app_id)
        app.installed = False
        app.install_path = None
        self._ledger_write(f'UNINSTALL | {app.name} v{app.version}')
        return f"✅ '{app.name}' uninstalled. Sandbox cleaned. Sovereign state restored."