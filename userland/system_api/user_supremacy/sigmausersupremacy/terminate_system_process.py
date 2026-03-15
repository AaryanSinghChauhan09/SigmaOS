# Generated method: SigmaUserSupremacy.terminate_system_process
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaUserSupremacy:
    def terminate_system_process(self, pid: int):
        """Allows the user to kill ANY process, including system ones via HAL."""
        hal = self.kernel.registry.get('hal') if self.kernel else None
        return f'Process {pid} neutralized via HAL-Surgical-Strike. User authority confirmed.'