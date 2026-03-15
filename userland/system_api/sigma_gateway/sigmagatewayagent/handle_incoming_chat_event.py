# Generated method: SigmaGatewayAgent.handle_incoming_chat_event
import time
import json
import uuid
from typing import List, Dict, Any, Optional
from datetime import datetime

class SigmaGatewayAgent:
    def handle_incoming_chat_event(self, payload: Dict[str, Any]):
        """Logic for processing events from the bus wrapper."""
        platform = payload.get('platform', 'Unknown')
        user = payload.get('user')
        message = payload.get('message')
        response = self.handle_incoming_chat(platform, user, message)
        if self.bus:
            self.bus.emit('chat.outgoing', {'user': user, 'message': response})