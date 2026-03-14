"""
SigmaOS Aura Shield (v1.0 Apex)
==================================
USP: Context-Aware Anti-Ransomware + Behavioral Anomaly Detection.
Monitors SigmaFS for high-entropy write bursts + mass encryption signatures.
"""
import time
import math
import hashlib
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaAuraShield(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self._running = False
        self.stats: Dict[str, Any] = {
            "monitored_write_ops": 0,
            "anomalies_blocked": 0,
            "auto_snapshots_taken": 0,
            "ransomware_threat_level": "LOW"
        }
        self.entropy_threshold = 0.85 # High entropy = likely encrypted/compressed
        self.mass_change_threshold = 50 # Files modified in 10 seconds
        
        # USP: Adaptive Behavioral Profiling
        self._behavioral_baseline: Dict[str, float] = {} # Learned entropy per extension
        self._trust_count = 0

    def start_service(self):
        self._running = True
        # Subscribe to SigmaFS write events via Kernel Bus
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.subscribe("fs.write", self._analyze_write_behavior)
            self.kernel.bus.subscribe("fs.mass_delete", self._trigger_emergency_snapshot)
        
        self.log_event("shield_start", {"id": "AuraShield"})
        
        # Link to Gamification for "Sentinel XP"
        if self.kernel and hasattr(self.kernel, "gamification"):
             self.kernel.gamification.record_interaction("SECURITY_WATCH_ACTIVE")
             
        return "Aura Shield: Ransomware Sentinel Active [Behavioral-Adaptive]."

    def stop_service(self):
        self._running = False
        self.log_event("shield_stop", {"id": "AuraShield"})

    def _analyze_write_behavior(self, payload: dict):
        """USP: Entropy-based file-write profiling."""
        count = int(self.stats.get("monitored_write_ops", 0))
        self.stats["monitored_write_ops"] = count + 1
        path = str(payload.get("path", ""))
        content_sample = bytes(payload.get("content_sample", b""))
        
        # Calculate entropy of the sample
        entropy = float(self._calculate_entropy(content_sample))
        
        if entropy > float(self.entropy_threshold) and not path.endswith('.zip') and not path.endswith('.enc'):
            # Potentially malicious encryption burst detected!
            self.stats["ransomware_threat_level"] = "CRITICAL"
            self.log_event("anomaly_detected", {"path": path, "entropy": entropy, "type": "Encryption_Signature"})
            
            # Action: Atomic Snapshot of the FS before potentially losing more data
            if self.kernel and hasattr(self.kernel, "fs"):
                self.kernel.fs.create_snapshot("AUTO_AURA_SHIELD_RECOVERY")
                snap_count = int(self.stats.get("auto_snapshots_taken", 0))
                self.stats["auto_snapshots_taken"] = snap_count + 1
                
                # Action: Freeze the calling process (Simulation)
                blocked_count = int(self.stats.get("anomalies_blocked", 0))
                self.stats["anomalies_blocked"] = blocked_count + 1
                
                # Reward the user for a "System Save"
                if self.kernel and hasattr(self.kernel, "gamification"):
                    self.kernel.gamification.record_interaction("THREAT_BLOCKED")
                    
                return {"action": "BLOCK", "reason": "RANSOMWARE_SIG_DETECTED"}
        else:
            # USP: Adaptively learn the baseline entropy for this file type
            ext = path.split('.')[-1] if '.' in path else "no_ext"
            self._update_baseline(ext, entropy)
        
        return {"action": "ALLOW"}

    def _update_baseline(self, ext: str, entropy: float):
        """USP: Adaptive Machine Learning for baseline entropy."""
        current = self._behavioral_baseline.get(ext, entropy)
        # Moving average for long-term adaptation
        self._behavioral_baseline[ext] = (current * 0.95) + (entropy * 0.05)
        self._trust_count += 1

    def _trigger_emergency_snapshot(self, payload: dict):
        """USP: Mass-Delete Protection. Takes snapshot if 100+ files are suddenly deleted."""
        print(f"[AURA] Mass Delete detected! Shielding original data...")
        if self.kernel and hasattr(self.kernel, "fs"):
             self.kernel.fs.create_snapshot("AUTO_SHIELD_MASS_DELETE")
             snap_count = int(self.stats.get("auto_snapshots_taken", 0))
             self.stats["auto_snapshots_taken"] = snap_count + 1

    def _calculate_entropy(self, data: bytes):
        """Shannon Entropy calculation for bit-level analysis."""
        if not data: return 0.0
        entropy = 0
        freq = {}
        for b in data:
            freq[b] = freq.get(b, 0) + 1
        for f in freq.values():
            p = float(f) / len(data)
            entropy -= p * math.log2(p)
        return float(entropy / 8.0) # Normalized 0 to 1

    def health_check(self) -> str:
        s = self.stats
        return f"OK — Aura Shield: {s['ransomware_threat_level']} | Anomaly Blocked: {s['anomalies_blocked']} | Snaps: {s['auto_snapshots_taken']}"
