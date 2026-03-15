# Generated method: PolymorphicShifter.start_rotation_thread
import random
import time
import threading
from typing import Dict, List

class PolymorphicShifter:
    def start_rotation_thread(self, interval_sec: int=300):
        """Starts a background thread to automatically shift identities."""
        self.is_running = True

        def loop():
            while self.is_running:
                time.sleep(interval_sec)
                self.shift_all()
        t = threading.Thread(target=loop, daemon=True)
        t.start()
        print(f'[SHIFTER] Dynamic Rotation Active (Interval: {interval_sec}s)')