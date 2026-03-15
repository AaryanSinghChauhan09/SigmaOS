# Generated method: PolymorphicShifter.shift_all
import random
import time
import threading
from typing import Dict, List

class PolymorphicShifter:
    def shift_all(self):
        """Rotates the identity of all protected processes."""
        with self._lock:
            for pid, info in self.active_processes.items():
                new_alias = random.choice(self.common_fake_names) + '_' + str(random.randint(100, 999))
                info['current_alias'] = new_alias
                info['rotated_at'] = time.time()
                info['history'].append(new_alias)
                msg = f"POLYMORPH: '{info['real_name']}' is now masking as '{new_alias}'"
                self.kernel._morphic_island(msg, '#8A2BE2')