# Generated method: SigmaAnomalyDetector._calculate_composite_risk
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaAnomalyDetector:
    def _calculate_composite_risk(self, module: str) -> float:
        """USP: Multivariate Correlation. Calculates if module is failing across multiple dimensions."""
        metrics = self._baselines.get(module, {})
        if not metrics:
            return 0.0
        z_scores = [b.z_score(b.last) for b in metrics.values() if b.n > _MIN_SAMPLES]
        if not z_scores:
            return 0.0
        avg_z = sum(z_scores) / len(z_scores)
        return 1.0 / (1.0 + math.exp(-(avg_z - 2.0)))