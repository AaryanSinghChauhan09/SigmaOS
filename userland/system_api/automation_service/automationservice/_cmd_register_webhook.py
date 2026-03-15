# Generated method: AutomationService._cmd_register_webhook
import json
import time
from typing import Dict, Any, Optional

class AutomationService:
    def _cmd_register_webhook(self, data: Any) -> Dict[str, Any]:
        """USP: External IoT devices can subscribe to SigmaOS events via webhooks."""
        if not isinstance(data, dict):
            return {'error': 'Payload must be JSON dict.'}
        event = data.get('event', 'global')
        url = data.get('url', '')
        if event and url:
            self.active_webhooks[event] = url
            return {'status': 'REGISTERED', 'event': event, 'url': url}
        return {'error': 'Missing event or url.'}