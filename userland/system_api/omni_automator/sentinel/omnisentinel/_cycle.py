# Generated method: OmniSentinel._cycle
import time
import threading
from typing import Dict, Any

class OmniSentinel:
    def _cycle(self):
        """Autonomous Decision Loop."""
        while self._running:
            try:
                time.sleep(15)
                if self.kernel and self.kernel.perf:
                    metrics = self.kernel.perf.get_telemetry()
                    cpu = float(metrics.get('cpu_load', '0%').replace('%', ''))
                    if cpu > 80.0:
                        if self.launch_preset_fn:
                            self.launch_preset_fn('Nightly_Purge')
                        self.stats['proactive_interventions'] += 1
                        if hasattr(self.kernel, 'bus'):
                            self.kernel.bus.emit('auto.sentinel_trigger', {'res': 'CPU_HIGH', 'action': 'PURGE'})
                self.stats['actions_automated'] += 2
            except Exception as e:
                print(f'[SENTINEL_ERR] {e}')