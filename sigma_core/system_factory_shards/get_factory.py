from sigma_core.interfaces.base_sovereign import ISovereign
from sigma_core.system.decorators import LoggingDecorator, ResilienceDecorator, MetricsDecorator, PrivacyDecorator
import threading
from ..systemfactory._base import SystemFactory

def get_factory() -> SystemFactory:
    return SystemFactory()