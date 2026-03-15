# Generated file: forge_service
import os
from sigma_forge.forge_app import _write

def forge_service(name: str, output_dir: str='userland/apps') -> str:
    """Generate a new SigmaOS background service scaffold."""
    class_name = ''.join((x.capitalize() for x in name.replace('_', ' ').split()))
    content = f'"""\n{name} Background Service for SigmaOS\n"""\nfrom sigma_core.system.interfaces import SigmaModuleBase, ISigmaService\n\nclass {class_name}(SigmaModuleBase, ISigmaService):\n    def __init__(self, kernel=None):\n        super().__init__(kernel)\n        self._running = False\n\n    def start_service(self):\n        self._running = True\n        return f"{name} Service ONLINE"\n\n    def stop_service(self):\n        self._running = False\n        return f"{name} Service OFFLINE"\n\n    def health_check(self):\n        return "OK" if self._running else "INACTIVE"\n'
    return _write(name, output_dir, content, 'service')