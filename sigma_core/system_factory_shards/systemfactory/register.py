from sigma_core.interfaces.base_sovereign import ISovereign
from sigma_core.system.decorators import LoggingDecorator, ResilienceDecorator, MetricsDecorator, PrivacyDecorator
import threading

from ._base import SystemFactory

class SystemFactory:
    def register(self, name: str, component: ISovereign, resilient: bool=True, logged: bool=True, metrics: bool=True):
        """
            Registers and provides a decorated Sovereign Unit.
            """
        wrapped = component
        if resilient:
            wrapped = ResilienceDecorator(wrapped)
        if metrics:
            wrapped = MetricsDecorator(wrapped)
        if logged:
            wrapped = LoggingDecorator(wrapped)
        privacy_guard = self._registry.get('PrivacyGuard')
        if privacy_guard and name != 'PrivacyGuard':
            tag = getattr(component, 'privacy_tag', name)
            wrapped = PrivacyDecorator(wrapped, privacy_guard, tag)
        print(f'[FACTORY] Assembling Sovereign Unit: {name}')
        self._registry[name] = wrapped