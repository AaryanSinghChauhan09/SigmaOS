# Generated method: SystemMonitor.get_health_report
import os
import sys
import time
from userland.system_api.sigma_std import SigmaSys
from sigma_core.hal.kernel_hal import SovereignHAL

class SystemMonitor:
    @staticmethod
    def get_health_report():
        """Aggregates system vitals for transparency."""
        cpu = SigmaSys.cpu_usage()
        ram = SigmaSys.ram_usage()
        eco_mode = 'ECO-ACTIVE' if cpu < 20 else 'PERFORMANCE'
        throttle = SystemMonitor.STATIC_THROTTLE_MS if eco_mode == 'ECO-ACTIVE' else SystemMonitor.ECO_THROTTLE_MS
        return {'CPU': f'{cpu}%', 'RAM': f'{ram}%', 'PowerState': eco_mode, 'Integrity': 'VERIFIED', 'Uptime': f'{int(time.process_time())}s', 'Arch': sys.platform.upper(), 'Throttle': throttle}