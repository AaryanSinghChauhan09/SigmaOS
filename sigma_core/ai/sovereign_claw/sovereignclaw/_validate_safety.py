# Generated method: SovereignClaw._validate_safety
from typing import List, Dict, Any, Optional
import os
import time

class SovereignClaw:
    def _validate_safety(self, intents: List[Dict]) -> bool:
        """Consults PrivacyShield before moving any file or reading sensitive data."""
        for intent in intents:
            if 'target' in intent and any((x in intent['target'] for x in ['.env', 'private', 'key'])):
                return False
        return True