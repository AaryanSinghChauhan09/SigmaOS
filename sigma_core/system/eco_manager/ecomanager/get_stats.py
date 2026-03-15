# Generated method: EcoManager.get_stats
import time
import random
from typing import Dict, Any

class EcoManager:
    def get_stats(self) -> Dict[str, Any]:
        return {'carbon_offset_est': f'{self.carbon_saved_kg:.4f} kg', 'efficiency_rating': 'A++', 'thermal_state': 'OPTIMAL' if self.system_temp < 70 else 'WARNING'}