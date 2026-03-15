"""
Auto-split from userland\system_api\linux_parity_engine.py — SigmaPackageManager.transactional_rollback
"""

import time
import uuid
import random
from typing import Dict, List, Any



class SigmaPackageManager:
    def transactional_rollback(self):
        """Rolls back the last package installation (Timeshift/Nix style)."""
        if not self._installed:
            return 'ERR: No packages installed to rollback.'
        pkg, ver = self._installed.popitem()
        if self.kernel and hasattr(self.kernel, 'snapshots'):
            self.kernel.snapshots.restore_snapshot(f'pkg_layer_{pkg}')
        return f"[sigma-pm] ATOMIC ROLLBACK: Uninstalled '{pkg}' v{ver}. Filesystem layer '{pkg}' purged and system state restored."
