from .interfaces.system_interfaces import ISystemComponent
import threading

class SystemFactory:
    def get_component(self, name: str) -> ISystemComponent:
        """Retrieves a registered singleton component."""
        return self._registry.get(name)