from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def on_event(self, event_type, data):
        """Observer implementation."""
        if event_type == 'INCOMING_CHAT':
            self.__add_to_history({'time': time.time(), 'msg': data.get('msg'), 'sender': data.get('sender')})
            print(f"[CHAT] Message Received from {data.get('sender')}")