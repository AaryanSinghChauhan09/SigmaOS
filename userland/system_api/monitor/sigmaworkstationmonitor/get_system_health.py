"""
Auto-split from userland\system_api\monitor.py — SigmaWorkstationMonitor.get_system_health
"""

import os
import random
import time



class SigmaWorkstationMonitor:
    def get_system_health(self) -> dict:
        """
            Kernel Watchdog Interface (required by watchdog_monitor).
            Returns a health dict including load_avg and memory pressure.
            """
        telemetry = self.get_realtime_telemetry()
        cpu_str = telemetry.get('CPU_Load', '0%')
        try:
            load_avg = float(cpu_str.split('%')[0].split()[-1])
        except (ValueError, IndexError):
            load_avg = 0.0
        return {'load_avg': load_avg, 'telemetry': telemetry, 'forensics': self.forensic_scan(), 'thermal': self.hardware_thermal_guard(), 'healing_status': self.predictive_self_healing(), 'health_score': self.health_score, 'status': 'NOMINAL' if load_avg < 85 else 'DEGRADED'}
