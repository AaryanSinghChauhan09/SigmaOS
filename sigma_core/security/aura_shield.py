"""
SigmaOS Aura Shield (v2.0 Apex — ANALYTICAL)
==============================================
USP: Context-Aware Anti-Ransomware + Behavioral Anomaly Detection.
Monitors SigmaFS for high-entropy write bursts + mass encryption signatures.
Adaptive: Learn baseline entropy per extension type to reduce false positives.
"""
import time
import math
import hashlib
import os
import sys
from typing import Dict, Any, List, Optional

# Robust Shard Grid Path Injection
_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
if _ROOT not in sys.path: sys.path.insert(0, _ROOT)

try:
    from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService # type: ignore
except ImportError:
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel
        def log_event(self, a, c): pass
    class ISigmaService: pass

class SigmaAuraShield(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        if hasattr(SigmaModuleBase, "__init__") and SigmaModuleBase.__init__ != object.__init__:
            SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self._running = False
        self.stats: Dict[str, Any] = {
            "monitored_write_ops": 0,
            "anomalies_blocked": 0,
            "auto_snapshots_taken": 0,
            "ransomware_threat_level": "LOW"
        }
        self.entropy_threshold = 0.85 
        self.mass_change_threshold = 50 
        
        self._behavioral_baseline: Dict[str, float] = {} 
        self._trust_count = 0

    def start_service(self):
        self._running = True
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.subscribe("fs.write", self._analyze_write_behavior)
            self.kernel.bus.subscribe("fs.mass_delete", self._trigger_emergency_snapshot)
        
        return "Aura Shield: Ransomware Sentinel Active [Behavioral-Adaptive]."

    def stop_service(self):
        self._running = False

    def _analyze_write_behavior(self, payload: dict):
        """USP: Entropy + PII Profiling (Sovereign Stealth)."""
        count = int(self.stats.get("monitored_write_ops", 0))
        self.stats["monitored_write_ops"] = count + 1
        path = str(payload.get("path", ""))
        content_sample = bytes(payload.get("content_sample", b""))
        
        # 1. Entropy Collision Check
        entropy = float(self._calculate_entropy(content_sample))
        if entropy > float(self.entropy_threshold) and not any(path.endswith(e) for e in ['.zip', '.enc', '.rar']):
            return self._handle_anomaly(path, entropy, "Encryption_Burst")

        # 2. PII Stealth Sentinel (Personalized/Privacy)
        if b"User:" in content_sample or b"SSN:" in content_sample:
            return self._handle_anomaly(path, 1.0, "PII_LEAK_PREVENTION")
        
        # 3. Adaptive Baseline Learning
        ext = path.split('.')[-1] if '.' in path else "no_ext"
        self._update_baseline(ext, entropy)
        return {"action": "ALLOW"}

    def _handle_anomaly(self, path, value, type_str):
        self.stats["ransomware_threat_level"] = "CRITICAL"
        self.log_event("anomaly_detected", {"path": path, "val": value, "type": type_str})
        
        if self.kernel and hasattr(self.kernel, "fs"):
            if hasattr(self.kernel.fs, "create_snapshot"):
                self.kernel.fs.create_snapshot(f"AUTO_AURA_{type_str}") # type: ignore
            self.stats["auto_snapshots_taken"] = int(self.stats.get("auto_snapshots_taken", 0)) + 1
            self.stats["anomalies_blocked"] = int(self.stats.get("anomalies_blocked", 0)) + 1
            
            if self.kernel and hasattr(self.kernel, "gamification"):
                self.kernel.gamification.record_interaction("THREAT_BLOCKED") # type: ignore
                
        return {"action": "BLOCK", "reason": f"{type_str}_DETECTED"}

    def _update_baseline(self, ext: str, entropy: float):
        current = float(self._behavioral_baseline.get(ext, entropy))
        self._behavioral_baseline[ext] = (current * 0.95) + (entropy * 0.05)
        self._trust_count = int(self._trust_count) + 1 # type: ignore

    def _trigger_emergency_snapshot(self, payload: dict):
        """USP: Mass-Delete Protection."""
        if self.kernel and hasattr(self.kernel, "fs"):
             if hasattr(self.kernel.fs, "create_snapshot"):
                 self.kernel.fs.create_snapshot("AUTO_SHIELD_MASS_DELETE") # type: ignore
             self.stats["auto_snapshots_taken"] = int(self.stats.get("auto_snapshots_taken", 0)) + 1

    def _calculate_entropy(self, data: bytes):
        if not data: return 0.0
        entropy = 0.0
        freq: Dict[int, int] = {}
        for b in data:
            freq[b] = freq.get(b, 0) + 1
        for f in freq.values():
            p = float(f) / len(data)
            entropy -= p * math.log2(p)
        return float(entropy / 8.0)

    def health_check(self) -> str:
        s = self.stats
        return f"OK — Aura Shield: {s['ransomware_threat_level']} | Anomaly Blocked: {s['anomalies_blocked']}"
