# Generated method: SigmaAnomalyDetector.get_realtime_metrics
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

class SigmaAnomalyDetector:
    def get_realtime_metrics(self) -> Dict:
        with self._lock:
            res = {}
            for mod, met_dict in self._baselines.items():
                max_z = max((b.z_score(b.last) for b in met_dict.values()), default=0.0)
                max_drift = max((b.drift for b in met_dict.values()), default=0.0)
                res[mod] = {'z_max': round(max_z, 1), 'drift': round(max_drift, 2)}
            return res