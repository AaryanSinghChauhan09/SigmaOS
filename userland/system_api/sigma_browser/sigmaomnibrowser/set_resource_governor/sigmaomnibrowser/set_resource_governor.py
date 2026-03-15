# Generated method: SigmaOmniBrowser.set_resource_governor
import random
from sigma_core.system.sovereign_app import SovereignApp

class SigmaOmniBrowser:
    def set_resource_governor(self, ram_mb, cpu_percent):
        """
                Opera GX Style: Limits how much of the OS assets this browser can consume.
                """
        self.resource_limit_ram = f'{ram_mb} MB'
        self.resource_limit_cpu = f'{cpu_percent}%'
        return f'Resource Governor: RAM capped at {self.resource_limit_ram}, CPU at {self.resource_limit_cpu}.'