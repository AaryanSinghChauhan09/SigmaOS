from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver
import time

class ChatEngine(SovereignModule, IEventObserver):
    """
    Sovereign Chat Engine.
    Implements Encapsulation for history and Abstraction for messaging.
    """
    def __init__(self):
        super().__init__("CHAT_ENGINE")
        self.__history = [] # Private Encapsulation
        self._max_history = 500
        self.privacy_tag = "SOCIAL_MESH" # Used by PrivacyDecorator

    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        payload = kwargs.get('payload')
        if action == "SEND_MESSAGE":
            return self._handle_send(payload)
        elif action == "GET_HISTORY":
            return self.__history[-50:]
        return f"CHAT_ENGINE_READY"

    def _handle_send(self, message):
        timestamp = time.time()
        entry = {"time": timestamp, "msg": message, "sender": "SovereignUser"}
        self.__add_to_history(entry)
        print(f"[CHAT] Message Sent at {timestamp}")
        return True

    def __add_to_history(self, entry):
        """Private method for history management."""
        self.__history.append(entry)
        if len(self.__history) > self._max_history:
            self.__history.pop(0)

    def on_event(self, event_type, data):
        """Observer implementation."""
        if event_type == "INCOMING_CHAT":
            self.__add_to_history({"time": time.time(), "msg": data.get("msg"), "sender": data.get("sender")})
            print(f"[CHAT] Message Received from {data.get('sender')}")

    def initialize(self):
        print("[CHAT] Sigma Social Engine Online.")

    def shutdown(self):
        self.__history.clear()
        print("[CHAT] Sigma Social Engine Offline.")

    def health_check(self):
        return True
