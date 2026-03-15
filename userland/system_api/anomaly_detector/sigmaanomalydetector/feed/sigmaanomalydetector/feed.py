# Generated method: SigmaAnomalyDetector.feed
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

class SigmaAnomalyDetector:
    def feed(self, module: str, metric: str, value: float) -> Optional[Dict]:
        with self._lock:
            if module not in self._baselines:
                self.register_module(module)
            bl = self._baselines[module].get(metric)
            if not bl:
                return None
            bl.update(value)
            z = bl.z_score(value)
            drift = bl.drift
            composite_risk = self._calculate_composite_risk(module)
            is_predictive = z > 2.0 and drift > 0.5 or composite_risk > 0.7
            is_anomaly = z > _SIGMA_THRESHOLD or is_predictive
            if is_anomaly:
                bl.anomaly_count = bl.anomaly_count + 1
                severity = 'CRITICAL' if z > _CRITICAL_SIGMA or composite_risk > 0.9 else 'PREDICTIVE' if is_predictive else 'WARNING'
                alert = {'ts': time.strftime('%Y-%m-%dT%H:%M:%S'), 'module': module, 'metric': metric, 'z_score': round(z, 2), 'drift': round(drift, 2), 'risk_composite': round(composite_risk, 2), 'severity': severity, 'forensic_code': f'SIGMA-{random.randint(100, 999)}'}
                self._alerts.append(alert)
                if self.kernel:
                    self.kernel.bus.emit('kad.anomaly', alert)
                    if severity in ['CRITICAL', 'PREDICTIVE']:
                        self.kernel.bus.emit('kad.pre_trip', alert)
                        if z > 5.0 and hasattr(self.kernel, 'repair_engine'):
                            self.kernel.repair_engine.repair(module, f'KAD High-Z: {z}')
                return alert
        return None