"""
SigmaOS Autonomic Healer (v4.0 Apex)
=====================================
USP: Self-healing architecture with proactive anomaly suppression.
Modular Architecture: Delegating to IntegrityScanner and RecoveryEngine.
"""
import threading
import time
from typing import Dict, Any, Optional
from .integrity_scanner import IntegrityScanner
from .recovery_engine import RecoveryEngine

class SigmaModuleBase:
    def __init__(self, kernel): self.kernel = kernel
    def log_event(self, a, c):
        if self.kernel and hasattr(self.kernel, "bus"):
             self.kernel.bus.emit(f"healer.{a}", c)

class AutonomicHealer(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.scanner = IntegrityScanner(kernel)
        self.recovery = RecoveryEngine(kernel)
        self._running = False
        self._thread: Optional[threading.Thread] = None
        self.stats = {"heals": 0}

    def start_service(self) -> str:
        self._running = True
        t = threading.Thread(target=self._loop, daemon=True)
        self._thread = t
        t.start()
        return "Autonomic Healer v4: Self-Healing Active."

    def stop_service(self):
        self._running = False

    def _loop(self):
        while self._running:
            report = self.scanner.scan_shards()
            if report.get("fault_detected"):
                 if self.recovery.execute_restoration():
                      self.stats["heals"] += 1
                      self.log_event("self_heal", {"method": "SNAPSHOT_ROLLBACK"})
            time.sleep(10)

    def health_check(self) -> str:
        return f"OK — Modular Sentinel Active | Heals: {self.stats['heals']}"
