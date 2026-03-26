from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def _handle_send(self, message):
        timestamp = time.time()
        entry = {'time': timestamp, 'msg': message, 'sender': 'SovereignUser'}
        self.__add_to_history(entry)
        print(f'[CHAT] Message Sent at {timestamp}')
        return True