from .interfaces.system_interfaces import ISystemComponent
import threading

class SystemFactory:
    def register_component(self, name: str, component: ISystemComponent):
        """Dependency Injection: Register a concrete implementation."""
        print(f'[FACTORY] Registering {name}...')
        self._registry[name] = component