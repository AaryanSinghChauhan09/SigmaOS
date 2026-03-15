"""
Auto-split from userland\system_api\accessibility.py — SigmaAccessibilityHub.speak
"""

from dataclasses import dataclass
from enum import Enum, auto
import threading



class SigmaAccessibilityHub:
    def speak(self, text: str, interrupt: bool=True):
        """USP: Sovereign TTS - Reads text aloud for visually impaired users."""
        if not SPEECH_AVAILABLE or not self._active_features['screen_reader']:
            return False

        def _say():
            with self._speech_lock:
                try:
                    engine = pyttsx3.init()
                    engine.setProperty('rate', 150)
                    engine.say(text)
                    engine.runAndWait()
                    engine.stop()
                except Exception:
                    pass
        threading.Thread(target=_say, daemon=True).start()
        return True
