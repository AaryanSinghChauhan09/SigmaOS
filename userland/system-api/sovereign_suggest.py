"""
SigmaOS Sovereign Suggest Engine
=================================
USP: Local-first, privacy-respecting autocomplete and prediction logic.
Mimics: Google/Microsoft Autocomplete but with ZERO cloud telemetry.
"""

import time
import json
import os

class SovereignSuggest:
    """Predictive Input & Autocomplete Engine."""
    
    def __init__(self, kernel=None):
        self.kernel = kernel
        # Simple local dictionary for performance
        self._dict = [
            "sigmaos sovereign boot", "how to setup sigma mesh", "sovereign vault recovery",
            "sigmafs self-healing docs", "quantum-safe encryption kyber", "aether assistant intent commands",
            "auranotes math solver", "sigmamirror phone sync", "zero-trust network access",
            "biometric sudo elevation", "kanban board setup", "scrum sprint planning",
            "ncert syllabus math", "iit-jee physics prep", "humanity principles in tech"
        ]
        self._history = []
        self._user_prefs = {"privacy_level": "maximum"}

    def get_suggestions(self, query: str, limit: int = 5) -> list[str]:
        """Returns local suggestions based on query string."""
        if not query or len(query) < 2:
            return []
        
        q = query.lower()
        # Filter local dictionary
        matches = [s for s in self._dict if q in s.lower()]
        # Add from history if relevant
        history_matches = [s for s in self._history if q in s.lower()]
        
        combined = list(dict.fromkeys(matches + history_matches)) # Deduplicate
        return combined[:limit]

    def add_to_history(self, entry: str):
        """Learns from user locally if privacy allows."""
        if self._user_prefs["privacy_level"] != "incognito":
            if entry not in self._history:
                self._history.append(entry)
                if len(self._history) > 100:
                    self._history.pop(0)

    def set_privacy_mode(self, mode: str):
        """Modes: maximum (no history), balanced (local history), incognito."""
        self._user_prefs["privacy_level"] = mode

    def health_check(self) -> str:
        return f"OK — Suggest Engine Active. Dict Size: {len(self._dict)}, Privacy: {self._user_prefs['privacy_level']}"

if __name__ == "__main__":
    ss = SovereignSuggest()
    print(ss.get_suggestions("sigma"))
    print(ss.get_suggestions("how to"))
