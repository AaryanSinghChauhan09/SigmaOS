"""
Auto-split from sigma_core\system\app_prewarmer.py — SigmaAppPrewarmer._predict_and_warm
"""

import time
import uuid
import threading
from typing import Dict, List, Any, Optional



class SigmaAppPrewarmer:
    def _predict_and_warm(self, current_app: str):
        """Neural heuristic: if I launched X, I will probably launch Y."""
        predictions = self._transition_matrix.get(current_app.lower(), [])
        for p in predictions:
            self.prewarm(p)
