# Generated method: SigmaGatewayAgent.__init__
import time
import json
import uuid
from typing import List, Dict, Any, Optional
from datetime import datetime

class SigmaGatewayAgent:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.bus = getattr(kernel, 'bus', None)
        self.registry = getattr(kernel, 'registry', {})
        self.context_memory = []
        self._stats = {'messages_bridged': 0, 'proactive_briefs_sent': 0, 'cli_commands_proxied': 0}
        if self.bus:
            self.bus.subscribe('chat.incoming', self.handle_incoming_chat_event)
            self.bus.subscribe('system.alert', self.handle_system_alert)