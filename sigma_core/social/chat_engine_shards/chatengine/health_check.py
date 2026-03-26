from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def health_check(self):
        return True