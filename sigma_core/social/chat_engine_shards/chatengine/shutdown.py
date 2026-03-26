from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def shutdown(self):
        self.__history.clear()
        print('[CHAT] Sigma Social Engine Offline.')