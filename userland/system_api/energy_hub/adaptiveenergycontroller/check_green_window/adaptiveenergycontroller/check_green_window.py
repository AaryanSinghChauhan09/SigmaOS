# Generated method: AdaptiveEnergyController.check_green_window
import time
import random
import threading
from typing import Dict, Any

class AdaptiveEnergyController:
    def check_green_window(self) -> Dict[str, Any]:
        """Returns whether it is currently a low-carbon window for heavy compute."""
        region = _CARBON_INTENSITY_REGIONS.get(self._region, {})
        carbon = region.get('avg_gco2_kwh', 500)
        hour = int(time.strftime('%H'))
        green_start = int(region.get('green_start', '22:00').split(':')[0])
        green_end = int(region.get('green_end', '06:00').split(':')[0])
        in_green = hour >= green_start or hour < green_end
        self._green_window_active = in_green
        return {'region': self._region, 'carbon_intensity': f'{carbon} gCO₂/kWh', 'green_window': in_green, 'recommended': 'AI Batch Training / System Updates' if in_green else 'Interactive workloads only'}