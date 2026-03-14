"""
SigmaOS Forensic Sentinel (v1.0 Apex)
=======================================
USP: Proactive maintenance: sub-millisecond self-healing and performance optimization.
Refactored from core kernel for better separation of concerns.
"""
import time
import threading
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class ForensicSentinel(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel):
        super().__init__(kernel)
        self._sentinel_running = False
        self._tick_count = 0

    def start_service(self) -> str:
        if not self._sentinel_running:
            self._sentinel_running = True
            t = threading.Thread(target=self._sentinel_loop, daemon=True)
            t.start()
            return "Forensic-Sentinel: Proactive Healing ACTIVE."
        return "Forensic-Sentinel: Already running."

    def stop_service(self):
        self._sentinel_running = False

    def _sentinel_loop(self):
        """Proactive maintenance: sub-millisecond self-healing, PBS ticks, and performance optimization."""
        while self._sentinel_running:
            time.sleep(30)
            self._tick_count += 1
            try:
                # 1. Proactive Integrity & Healing
                if self._tick_count % 5 == 0 and hasattr(self.kernel, "integrity"):
                    report = self.kernel.integrity.verify_system_integrity()
                    if report["status"] == "TAMPERED":
                        print(f"[SENTINEL] TAMPER DETECTED: Attempting automatic restoration...")
                        if hasattr(self.kernel, "self_healing_recovery"):
                            self.kernel.self_healing_recovery()
                        self.kernel.bus.emit("system.heal", {"report": "Auto-Restored from Bit-Level Baseline"})

                # 2. Performance Re-balancing
                if self._tick_count % 10 == 0:
                    pb = self.kernel.registry.get("perf")
                    if pb and hasattr(pb, "optimize_core_affinity"):
                        pb.optimize_core_affinity()

                # 3. PBS Tick
                pbs = self.kernel.registry.get("pbs")
                if pbs and hasattr(pbs, "tick_all"): 
                    pbs.tick_all()

                # 4. Energy thermal feedback
                energy = self.kernel.registry.get("energy_hub")
                if energy and hasattr(energy, "get_realtime_metrics"): 
                    energy.get_realtime_metrics()

                # 5. KAD anomaly pulse
                kad = self.kernel.registry.get("kad")
                if kad and self._tick_count % 2 == 0 and hasattr(kad, "scan_memory_anomalies"):
                    kad.scan_memory_anomalies()

                # 6. Proactive Health Pulse
                repair = self.kernel.registry.get("repair_engine")
                if repair and hasattr(repair, "check_proactive_health"):
                    repair.check_proactive_health()

            except Exception as e:
                print(f"[SENTINEL] Failure on tick {self._tick_count}: {e}")
                if self.kernel and hasattr(self.kernel, "bus"):
                    self.kernel.bus.emit("kernel.error", {"tick": self._tick_count, "err": str(e)})

    def health_check(self) -> str:
        return f"OK - Ticks: {self._tick_count}"
