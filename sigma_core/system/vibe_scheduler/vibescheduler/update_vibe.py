# Generated method: VibeScheduler.update_vibe
import time
from typing import Dict

class VibeScheduler:
    def update_vibe(self, keystrokes_per_min: int, cpu_usage: float) -> str:
        """Determines the OS vibe and adjusts scheduling profiles."""
        old_vibe = self.current_vibe
        if keystrokes_per_min > 40 and cpu_usage > 0.3:
            self.current_vibe = 'DEEP_WORK'
        elif keystrokes_per_min < 5 and cpu_usage < 0.1:
            self.current_vibe = 'ZEN_STATE'
        else:
            self.current_vibe = 'CASUAL_FLOW'
        if self.current_vibe != old_vibe:
            self._apply_vibe_profile()
            self._notify_ui()
        return self.current_vibe