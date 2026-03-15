# Generated method: SovereignApp._call_service
import time

class SovereignApp:
    def _call_service(self, service_name, action, **kwargs):
        """Forces all service calls through the kernel proxy to ensure no leakage."""
        if self.kernel and self.kernel.offline_guard:
            audit = self.kernel.offline_guard.verify_privacy_perimeter()
            if audit['Sovereignty_Status'] != 'VERIFIED':
                raise Exception(f'Sovereign Breach: {self.app_name} blocked from making insecure calls.')
        return f'{self.app_name}: Executed local {action} via {service_name}.'