# Generated method: SigmaAdaptiveKernel.start_autonomous_observer
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def start_autonomous_observer(self, sample_interval_s: float=5.0):
        """
                Starts a background thread that polls process signals and re-tunes
                the kernel every `sample_interval_s` seconds autonomously.
                """
        if self._running:
            return 'Observer already running.'
        self._running = True

        def _loop():
            while self._running:
                self.auto_tune(self._sensors)
                time.sleep(sample_interval_s)
        self._observer_thread = threading.Thread(target=_loop, daemon=True)
        self._observer_thread.start()
        return f'AdaptiveKernel: Autonomous observer started (interval={sample_interval_s}s, thread=daemon).'