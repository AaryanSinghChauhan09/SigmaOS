# Generated method: SettingsManager.update_key
import json
import os

class SettingsManager:
    @staticmethod
    def update_key(key, value):
        """Atomic update of a single preference."""
        s = SettingsManager.load()
        if isinstance(s, dict):
            s[key] = value
            SettingsManager.save(s)
            return True
        return False