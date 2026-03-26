from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        payload = kwargs.get('payload')
        if action == 'SEND_MESSAGE':
            return self._handle_send(payload)
        elif action == 'GET_HISTORY':
            return self.__history[-50:]
        return f'CHAT_ENGINE_READY'