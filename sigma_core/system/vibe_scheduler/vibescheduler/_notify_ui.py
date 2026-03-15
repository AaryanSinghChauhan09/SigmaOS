# Generated method: VibeScheduler._notify_ui
import time
from typing import Dict

class VibeScheduler:
    def _notify_ui(self):
        """Updates the Morphic Island and Aura color."""
        colors = {'DEEP_WORK': '#E94560', 'ZEN_STATE': '#0F3460', 'CASUAL_FLOW': '#00D2FC'}
        accent = colors.get(self.current_vibe, '#00D2FC')
        self.kernel._morphic_island(f'VIBE SHIFT: {self.current_vibe}', accent)