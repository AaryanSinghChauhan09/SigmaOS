from .interfaces.system_interfaces import ISystemComponent
import threading

class SystemFactory:
    def list_components(self):
        return list(self._registry.keys())