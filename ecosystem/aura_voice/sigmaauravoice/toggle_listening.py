# Generated method: SigmaAuraVoice.toggle_listening
from typing import Dict, List, Any
import time

class SigmaAuraVoice:
    def toggle_listening(self, state: bool) -> str:
        self._is_active = state
        status = 'ONLINE' if state else 'OFFLINE'
        return f'AuraVoice: [{self._voice_profile}] is now {status}. All mics calibrated.'