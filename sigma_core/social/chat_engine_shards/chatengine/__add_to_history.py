from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def __add_to_history(self, entry):
        """Private method for history management."""
        self.__history.append(entry)
        if len(self.__history) > self._max_history:
            self.__history.pop(0)