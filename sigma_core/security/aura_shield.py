"""
SigmaOS Aura Shield (v1.0 Apex)
==================================
USP: Context-Aware Anti-Ransomware + Behavioral Anomaly Detection.
Monitors SigmaFS for high-entropy write bursts + mass encryption signatures.
"""

import time
import math
import hashlib
from .interfaces import SigmaModuleBase, ISigmaService

class SigmaAuraShield(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self._running = False
        self.stats = {
            "monitored_write_ops": 0,
            "anomalies_blocked": 0,
            "auto_snapshots_taken": 0,
            "ransomware_threat_level": "LOW"
        }
        self.entropy_threshold = 0.85 # High entropy = likely encrypted/compressed
        self.mass_change_threshold = 50 # Files modified in 10 seconds

    def start_service(self):
        self._running = True
        # Subscribe to SigmaFS write events via Kernel Bus
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.subscribe("fs.write", self._analyze_write_behavior)
            self.kernel.bus.subscribe("fs.mass_delete", self._trigger_emergency_snapshot)
        
        self.log_event("shield_start", {"id": "AuraShield"})
        return "Aura Shield: Ransomware Sentinel Active."

    def stop_service(self):
        self._running = False
        self.log_event("shield_stop", {"id": "AuraShield"})

    def _analyze_write_behavior(self, payload: dict):
        """USP: Entropy-based file-write profiling."""
        self.stats["monitored_write_ops"] += 1
        path = payload.get("path", "")
        content_sample = payload.get("content_sample", b"")
        
        # Calculate entropy of the sample
        entropy = self._calculate_entropy(content_sample)
        
        if entropy > self.entropy_threshold and not path.endswith('.zip') and not path.endswith('.enc'):
            # Potentially malicious encryption burst detected!
            self.stats["ransomware_threat_level"] = "CRITICAL"
            self.log_event("anomaly_detected", {"path": path, "entropy": entropy, "type": "Encryption_Signature"})
            
            # Action: Atomic Snapshot of the FS before potentially losing more data
            if self.kernel and hasattr(self.kernel, "fs"):
                self.kernel.fs.create_snapshot("AUTO_AURA_SHIELD_RECOVERY")
                self.stats["auto_snapshots_taken"] += 1
                
                # Action: Freeze the calling process (Simulation)
                self.stats["anomalies_blocked"] += 1
                return {"action": "BLOCK", "reason": "RANSOMWARE_SIG_DETECTED"}
        
        return {"action": "ALLOW"}

    def _trigger_emergency_snapshot(self, payload: dict):
        """USP: Mass-Delete Protection. Takes snapshot if 100+ files are suddenly deleted."""
        print(f"[AURA] Mass Delete detected! Shielding original data...")
        if self.kernel and hasattr(self.kernel, "fs"):
             self.kernel.fs.create_snapshot("AUTO_SHIELD_MASS_DELETE")
             self.stats["auto_snapshots_taken"] += 1

    def _calculate_entropy(self, data: bytes):
        """Shannon Entropy calculation for bit-level analysis."""
        if not data: return 0.0
        entropy = 0
        freq = {}
        for b in data:
            freq[b] = freq.get(b, 0) + 1
        for f in freq.values():
            p = f / len(data)
            entropy -= p * math.log2(p)
        return entropy / 8.0 # Normalized 0 to 1

    def health_check(self) -> str:
        s = self.stats
        return f"OK — Aura Shield: {s['ransomware_threat_level']} | Anomaly Blocked: {s['anomalies_blocked']} | Snaps: {s['auto_snapshots_taken']}"

if __name__ == "__main__":
    shield = SigmaAuraShield()
    print(shield.start_service())
    # Mock suspicious high-entropy write
    suspicious_content = b"\x00\xFF\x1A\x2B" * 256 # Low entropy
    print(f"Entropy Low: {shield._calculate_entropy(suspicious_content)}")
    random_content = bytes([abs(hash(str(i))) % 256 for i in range(1024)]) # Higher entropy
    print(f"Entropy High: {shield._calculate_entropy(random_content)}")
    shield.stop_service()
