"""sigma_forge.forge_service — Background service scaffold generator."""
import os
from sigma_forge.forge_app import _write


def forge_service(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS background service scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f'''"""
{name} Background Service for SigmaOS
"""
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class {class_name}(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self._running = False

    def start_service(self):
        self._running = True
        return f"{name} Service ONLINE"

    def stop_service(self):
        self._running = False
        return f"{name} Service OFFLINE"

    def health_check(self):
        return "OK" if self._running else "INACTIVE"
'''
    return _write(name, output_dir, content, "service")
