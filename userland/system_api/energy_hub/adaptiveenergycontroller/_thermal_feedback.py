"""
Auto-split from userland\system_api\energy_hub.py — AdaptiveEnergyController._thermal_feedback
"""

import time
import random
import threading
from typing import Dict, Any



class AdaptiveEnergyController:
    def _thermal_feedback(self, status: str):
        """Closed-loop: automatically adjusts PerformanceBoost profile."""
        perf = self.kernel.registry.get('perf')
        if not perf:
            return
        mapping = {'COOL': 'Performance', 'OPTIMAL': 'Balanced', 'WARM': 'Stability', 'THROTTLE': 'Minimal', 'CRITICAL': 'Minimal'}
        target_profile = mapping.get(status, 'Balanced')
        if perf.active_profile != target_profile:
            perf.apply_tuning(target_profile)
            self.kernel.bus.emit('energy.profile_switched', {'thermal': status, 'profile': target_profile})
            if status == 'CRITICAL':
                wdog = self.kernel.registry.get('watchdog')
                if wdog:
                    wdog.record_failure('energy_hub', f'CRITICAL THERMAL EVENT: {self.temp_cpu:.1f}°C')
