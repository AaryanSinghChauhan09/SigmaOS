"""
Auto-split from sigma_core\security\integrity.py — IntegrityGuard.restore_shard
"""

import hashlib
import os
import sys
import json
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase



class IntegrityGuard:
    def restore_shard(self, rel_path: str) -> bool:
        """Restores a shard from its .bak backup."""
        import shutil
        abs_path = os.path.abspath(os.path.join(os.path.dirname(__file__), rel_path))
        bak_path = abs_path + '.bak'
        if os.path.exists(bak_path):
            try:
                shutil.copy2(bak_path, abs_path)
                return True
            except:
                return False
        return False
