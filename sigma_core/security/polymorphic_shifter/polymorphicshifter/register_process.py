# Generated method: PolymorphicShifter.register_process
import random
import time
import threading
from typing import Dict, List

class PolymorphicShifter:
    def register_process(self, process_id: str, real_name: str):
        """Registers a process for polymorphic protection."""
        with self._lock:
            self.active_processes[process_id] = {'real_name': real_name, 'current_alias': real_name, 'rotated_at': time.time(), 'history': [real_name]}
        print(f'[SHIFTER] Protected: {real_name} (ID: {process_id})')