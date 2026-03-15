"""omni_automator.sentinel — Proactive autonomous sentinel loop."""
import time
import threading
from typing import Dict, Any


class OmniSentinel:
    """Proactive OS Intelligence — decides when to shift modes based on real-time telemetry."""

    def __init__(self, stats: dict, kernel=None, launch_preset_fn=None):
        self.stats = stats
        self.kernel = kernel
        self.launch_preset_fn = launch_preset_fn
        self._running = False
        self._thread: threading.Thread | None = None

    def start(self):
        """Start the proactive sentinel daemon thread."""
        if not self._running:
            self._running = True
            self._thread = threading.Thread(target=self._cycle, daemon=True)
            self._thread.start()
            print("[OMNI] Proactive Sentinel [ONLINE].")

    def stop(self):
        """Stop the sentinel loop."""
        self._running = False

    def _cycle(self):
        """Autonomous Decision Loop."""
        while self._running:
            try:
                time.sleep(15)
                if self.kernel and self.kernel.perf:
                    metrics = self.kernel.perf.get_telemetry()
                    cpu = float(metrics.get("cpu_load", "0%").replace("%", ""))
                    if cpu > 80.0:
                        if self.launch_preset_fn:
                            self.launch_preset_fn("Nightly_Purge")
                        self.stats["proactive_interventions"] += 1
                        if hasattr(self.kernel, "bus"):
                            self.kernel.bus.emit(
                                "auto.sentinel_trigger", {"res": "CPU_HIGH", "action": "PURGE"}
                            )
                self.stats["actions_automated"] += 2
            except Exception as e:
                print(f"[SENTINEL_ERR] {e}")
