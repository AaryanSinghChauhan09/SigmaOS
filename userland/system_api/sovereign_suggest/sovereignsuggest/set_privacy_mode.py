# Generated method: SovereignSuggest.set_privacy_mode
import time
import json
import os

class SovereignSuggest:
    def set_privacy_mode(self, mode: str):
        """Modes: maximum (no history), balanced (local history), incognito."""
        self._user_prefs['privacy_level'] = mode