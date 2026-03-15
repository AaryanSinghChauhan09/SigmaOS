# Generated method: SigmaGatewayAgent.handle_system_alert
import time
import json
import uuid
from typing import List, Dict, Any, Optional
from datetime import datetime

class SigmaGatewayAgent:
    def handle_system_alert(self, payload: Dict[str, Any]):
        """USP: Proactive Outbound Alerts (Clawdbot parity)."""
        msg = f"⚠️ SYSTEM ALERT: {payload.get('msg', 'Anomaly detected')}"
        if self.bus:
            self.bus.emit('chat.outgoing', {'user': 'APEX_MASTER', 'message': msg})