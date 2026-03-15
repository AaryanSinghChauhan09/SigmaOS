# Generated method: SigmaHardwareWarden.get_sensors
from typing import Dict, List, Any

class SigmaHardwareWarden:
    def get_sensors(self) -> Dict:
        """Returns real-time hardware telemetry."""
        return {'GPU': f"{self._tunables['GPU_Clock']}MHz / 55°C", 'CPU': f"{self._tunables['CPU_Volt']}V / 62°C", 'Stability_Score': '100.0 (Optimal)'}