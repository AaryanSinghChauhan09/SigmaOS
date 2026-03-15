"""
Auto-split from userland\system_api\energy_hub.py — AdaptiveEnergyController.get_realtime_metrics
"""

import time
import random
import threading
from typing import Dict, Any



class AdaptiveEnergyController:
    def get_realtime_metrics(self) -> Dict[str, Any]:
        """Returns hardware-level metrics. Now drives PerformanceBoost automatically."""
        with self._feedback_lock:
            self.temp_cpu += random.uniform(-0.4, 1.2)
            self.temp_gpu += random.uniform(-0.3, 0.9)
            delta_bat = 0.04 if not self._is_charging else -0.07
            self.current_battery -= random.uniform(0, delta_bat)
            self.temp_cpu = max(28.0, min(95.0, self.temp_cpu))
            self.temp_gpu = max(28.0, min(90.0, self.temp_gpu))
            self.current_battery = max(0.0, min(100.0, self.current_battery))
        status = self._get_thermal_status()
        self._thermal_feedback(status)
        battery_warning = None
        if not self._is_charging and self.current_battery < 20:
            mins_left = self.current_battery / 0.05 * 0.5
            battery_warning = f'LOW BATTERY — estimated {mins_left:.0f} mins remaining'
            self.kernel.bus.emit('energy.battery_critical', {'pct': self.current_battery})
        return {'battery_pct': f'{self.current_battery:.1f}%', 'cpu_temp': f'{self.temp_cpu:.1f}°C', 'gpu_temp': f'{self.temp_gpu:.1f}°C', 'thermal_state': status, 'power_draw': f'{random.uniform(5.5, 12.0):.1f}W', 'fan_rpm': int(max(0, (self.temp_cpu - 48) * 90)), 'voltage_events': self._voltage_events, 'green_window': self._green_window_active, 'charging': self._is_charging, 'battery_warning': battery_warning}
