"""
SigmaOS Kernel Anomaly Detector (KAD) — v3.0 "FORENSIC ORACLE"
=============================================================
USP: Drift-Prediction AI + Multivariate Synergistic Correlation.
Predicts system failure before thresholds are breached.
"""

import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

try:
    from sigma_core.interfaces import SigmaModuleBase
except ImportError:
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel

_SIGMA_THRESHOLD = 2.5
_CRITICAL_SIGMA  = 4.5
_MIN_SAMPLES     = 10

class ModuleBaseline:
    def __init__(self, name: str):
        self.name    = name
        self.n       = 0
        self._mean   = 0.0
        self._m2     = 0.0
        self.history = collections.deque(maxlen=1000)
        self.z_history = collections.deque(maxlen=100)
        self.last    = 0.0
        self.drift   = 0.0 
        self.anomaly_count = 0

    def update(self, value: float):
        self.n = self.n + 1
        self.last = value
        self.history.append(value)
        delta = value - self._mean
        self._mean = self._mean + (delta / self.n)
        delta2 = value - self._mean
        self._m2 = self._m2 + (delta * delta2)
        
        z = self.z_score(value)
        if len(self.z_history) > 0:
            self.drift = z - self.z_history[-1]
        self.z_history.append(z)

    @property
    def mean(self) -> float: return self._mean

    @property
    def stddev(self) -> float:
        if self.n < 2: return 0.01
        return math.sqrt(max(0, self._m2 / (self.n - 1)))

    def z_score(self, value: float) -> float:
        s = self.stddev
        if s < 1e-9: return 0.0
        return abs(value - self.mean) / s

class SigmaAnomalyDetector(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel # Explicit for linter
        self._baselines: Dict[str, Dict[str, ModuleBaseline]] = {}
        self._alerts: List[Dict] = []
        self._lock = threading.Lock()
        self._running = False
        self._scan_interval = 20

    def register_module(self, name: str):
        with self._lock:
            if name not in self._baselines:
                self._baselines[name] = {
                    "latency_ms": ModuleBaseline(name),
                    "event_rate": ModuleBaseline(name),
                    "error_rate": ModuleBaseline(name),
                    "mem_usage_mb": ModuleBaseline(name),
                    "cpu_pressure": ModuleBaseline(name),
                }

    def feed(self, module: str, metric: str, value: float) -> Optional[Dict]:
        with self._lock:
            if module not in self._baselines: self.register_module(module)
            bl = self._baselines[module].get(metric)
            if not bl: return None

            bl.update(value)
            z = bl.z_score(value)
            drift = bl.drift
            
            # Multivariate Synergistic Correlation (Apex v3.0)
            # If multiple metrics in the same module are drifting, escalate
            composite_risk = self._calculate_composite_risk(module)
            
            is_predictive = (z > 2.0 and drift > 0.5) or composite_risk > 0.7
            is_anomaly = (z > _SIGMA_THRESHOLD) or is_predictive

            if is_anomaly:
                bl.anomaly_count = bl.anomaly_count + 1
                severity = "CRITICAL" if (z > _CRITICAL_SIGMA or composite_risk > 0.9) else "PREDICTIVE" if is_predictive else "WARNING"
                
                alert = {
                    "ts": time.strftime("%Y-%m-%dT%H:%M:%S"),
                    "module": module,
                    "metric": metric,
                    "z_score": round(z, 2),
                    "drift": round(drift, 2),
                    "risk_composite": round(composite_risk, 2),
                    "severity": severity,
                    "forensic_code": f"SIGMA-{random.randint(100, 999)}"
                }
                self._alerts.append(alert)
                
                if self.kernel:
                    self.kernel.bus.emit("kad.anomaly", alert)
                    if severity in ["CRITICAL", "PREDICTIVE"]:
                         self.kernel.bus.emit("kad.pre_trip", alert)
                         # Trigger Auto-Repair if z is extreme
                         if z > 5.0 and hasattr(self.kernel, "repair_engine"):
                             self.kernel.repair_engine.repair(module, f"KAD High-Z: {z}")
                
                return alert
        return None

    def _calculate_composite_risk(self, module: str) -> float:
        """USP: Multivariate Correlation. Calculates if module is failing across multiple dimensions."""
        metrics = self._baselines.get(module, {})
        if not metrics: return 0.0
        
        # High Z-scores across multiple metrics = high risk
        z_scores = [b.z_score(b.last) for b in metrics.values() if b.n > _MIN_SAMPLES]
        if not z_scores: return 0.0
        
        # Sigmoid-style normalization of average Z
        avg_z = sum(z_scores) / len(z_scores)
        return 1.0 / (1.0 + math.exp(- (avg_z - 2.0)))

    def health_check(self) -> str:
        s = self.get_realtime_metrics()
        driftiest = max(s.items(), key=lambda x: x[1].get("drift", 0), default=("NONE", {}))
        return f"OK — KAD v3.0 Oracle | Modules: {len(self._baselines)} | Alerts: {len(self._alerts)} | Peak Drift: {driftiest[0]}"

    def get_realtime_metrics(self) -> Dict:
        with self._lock:
            res = {}
            for mod, met_dict in self._baselines.items():
                max_z = max((b.z_score(b.last) for b in met_dict.values()), default=0.0)
                max_drift = max((b.drift for b in met_dict.values()), default=0.0)
                res[mod] = {"z_max": round(max_z, 1), "drift": round(max_drift, 2)}
            return res
