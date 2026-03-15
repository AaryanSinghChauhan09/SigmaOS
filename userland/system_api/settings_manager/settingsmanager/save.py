# Generated method: SettingsManager.save
import json
import os

class SettingsManager:
    @staticmethod
    def save(settings):
        """Persist settings to disk."""
        try:
            with open(SETTINGS_PATH, 'w') as f:
                json.dump(settings, f, indent=4)
            return True
        except:
            return False