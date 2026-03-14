"""
SigmaOS Settings & Personalization Manager (v1.0)
===================================================
USP: State-Persistent User Profiling & UI Customization.
100% Native JSON storage.
"""
import json
import os

SETTINGS_PATH = "userland/system_api/user_config.json"

DEFAULT_SETTINGS = {
    "user_name": "Sigma Researcher",
    "theme": "Dark Matter",
    "accent_color": "#6C63FF",
    "automation_level": 5,
    "last_session": "N/A"
}

class SettingsManager:
    @staticmethod
    def load():
        """Load user profile and settings."""
        if not os.path.exists(SETTINGS_PATH):
            SettingsManager.save(DEFAULT_SETTINGS)
            return DEFAULT_SETTINGS
        
        try:
            with open(SETTINGS_PATH, "r") as f:
                return json.load(f)
        except:
            return DEFAULT_SETTINGS

    @staticmethod
    def save(settings):
        """Persist settings to disk."""
        try:
            with open(SETTINGS_PATH, "w") as f:
                json.dump(settings, f, indent=4)
            return True
        except:
            return False

    @staticmethod
    def update_key(key, value):
        """Atomic update of a single preference."""
        s = SettingsManager.load()
        if isinstance(s, dict):
            # Safe assignment for personalization
            s[key] = value
            SettingsManager.save(s)
            return True
        return False
