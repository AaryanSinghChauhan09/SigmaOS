"""
Auto-split from ecosystem\sigma_ai_nexus.py — SigmaAINexus.list_models
"""

import time
import random
from typing import Dict, List, Any



class SigmaAINexus:
    def list_models(self) -> Dict:
        if self.kernel and hasattr(self.kernel, 'cfg') and getattr(self.kernel.cfg, 'LOCAL_ONLY_MODE', False):
            return {k: v for k, v in self._available_models.items() if v['region'] == 'Local'}
        return self._available_models
