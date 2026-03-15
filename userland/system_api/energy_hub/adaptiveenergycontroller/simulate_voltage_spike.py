"""
Auto-split from userland\system_api\energy_hub.py — AdaptiveEnergyController.simulate_voltage_spike
"""

import time
import random
import threading
from typing import Dict, Any



class AdaptiveEnergyController:
    def simulate_voltage_spike(self) -> str:
        """USP: SigmaOS absorbs power spikes via capacitor-model soft clamp."""
        self._voltage_events += 1
        spike_mv = random.randint(150, 600)
        if spike_mv > 400:
            perf = self.kernel.registry.get('perf')
            if perf:
                perf.apply_tuning('Minimal')
            self.kernel.bus.emit('energy.voltage_spike', {'mv': spike_mv, 'action': 'freq_clamped'})
            return f'⚡ VOLTAGE SPIKE +{spike_mv}mV absorbed. CPU frequency clamped (hardware protected).'
        return f'⚡ Minor spike +{spike_mv}mV — within tolerance. No action needed.'
