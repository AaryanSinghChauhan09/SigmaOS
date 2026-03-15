# Generated method: SigmaAppStore.update_all
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaAppStore:
    def update_all(self) -> List[str]:
        """Checks and installs updates for all installed apps."""
        results = []
        for app_id, app in self._installed.items():
            parts = app.version.split('.')
            parts[-1] = str(int(parts[-1]) + 1)
            new_ver = '.'.join(parts)
            old_ver = app.version
            app.version = new_ver
            app.checksum = self._compute_checksum(app.app_id, app.version)
            self._ledger_write(f'UPDATE | {app.name} {old_ver} → {new_ver}')
            results.append(f'✅ {app.name}: {old_ver} → {new_ver}')
        return results if results else ['All apps are up to date.']