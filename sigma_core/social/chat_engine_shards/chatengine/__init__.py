from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

from ._base import ChatEngine

class ChatEngine:
    def __init__(self):
        super().__init__('CHAT_ENGINE')
        self.__history = []
        self._max_history = 500
        self.privacy_tag = 'SOCIAL_MESH'