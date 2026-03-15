# Generated method: SettingsManager.load
import json
import os

class SettingsManager:
    @staticmethod
    def load():
        """Load user profile and settings."""
        if not os.path.exists(SETTINGS_PATH):
            SettingsManager.save(DEFAULT_SETTINGS)
            return DEFAULT_SETTINGS
        try:
            with open(SETTINGS_PATH, 'r') as f:
                return json.load(f)
        except:
            return DEFAULT_SETTINGS