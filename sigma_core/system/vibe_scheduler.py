"""
SigmaOS Vibe-Driven Scheduler v1.0
===================================
USP: Emotional and Professional context-aware resource scheduling.
Transitions between 'Deep Work', 'Casual Flow', and 'Zen State' automatically.
"""
import time
from typing import Dict

class VibeScheduler:
    def __init__(self, kernel):
        self.kernel = kernel
        self.current_vibe = "NOMINAL"
        self.activity_level = 0.0 # 0.0 to 1.0
        self._last_shift = time.time()

    def update_vibe(self, keystrokes_per_min: int, cpu_usage: float) -> str:
        """Determines the OS vibe and adjusts scheduling profiles."""
        old_vibe = self.current_vibe
        
        # 🟢 Logic: High typing + High CPU = Deep Work
        if keystrokes_per_min > 40 and cpu_usage > 0.3:
            self.current_vibe = "DEEP_WORK"
        # 🟡 Logic: Low typing + Low CPU = Zen State / Idle
        elif keystrokes_per_min < 5 and cpu_usage < 0.1:
            self.current_vibe = "ZEN_STATE"
        # 🔵 Logic: Moderate activity = Casual Flow
        else:
            self.current_vibe = "CASUAL_FLOW"

        if self.current_vibe != old_vibe:
            self._apply_vibe_profile()
            self._notify_ui()
            
        return self.current_vibe

    def _apply_vibe_profile(self):
        """Adjusts kernel resource governor based on vibe."""
        if not hasattr(self.kernel, "resource_governor"):
            return

        if self.current_vibe == "DEEP_WORK":
            # Maximize foreground performance, silence background telemetry
            self.kernel.resource_governor.boost_foreground(1.5)
            self.kernel.resource_governor.throttle_background(0.2)
        elif self.current_vibe == "ZEN_STATE":
            # Minimize clock speeds, maximal power saving
            self.kernel.resource_governor.boost_foreground(0.5)
            self.kernel.resource_governor.throttle_background(0.1)
        else:
            # Balanced profile
            self.kernel.resource_governor.boost_foreground(1.0)
            self.kernel.resource_governor.throttle_background(0.5)

    def _notify_ui(self):
        """Updates the Morphic Island and Aura color."""
        colors = {
            "DEEP_WORK": "#E94560", # Fire Red
            "ZEN_STATE": "#0F3460", # Night Blue
            "CASUAL_FLOW": "#00D2FC" # Sky Cyan
        }
        accent = colors.get(self.current_vibe, "#00D2FC")
        self.kernel._morphic_island(f"VIBE SHIFT: {self.current_vibe}", accent)

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
        class ResourceGov:
            def boost_foreground(self, v): print(f"FG Boost: {v}")
            def throttle_background(self, v): print(f"BG Throttle: {v}")
        resource_governor = ResourceGov()
        
    vibe = VibeScheduler(MockKernel())
    print(f"Update 1: {vibe.update_vibe(60, 0.4)}") # Deep Work
    print(f"Update 2: {vibe.update_vibe(0, 0.05)}") # Zen State
