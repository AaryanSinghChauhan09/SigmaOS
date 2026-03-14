"""
SigmaOS Sensory Assets (v1.0 Apex)
===================================
USP: Centralized Glyph (Icon) and Soundscape Management.
Modularized from SovereignCustomizer to handle pure sensory data.
"""
from typing import Dict, Any, Optional

class SensoryAssets:
    def __init__(self, kernel=None):
        self.kernel = kernel
        
        # GLYPH SETS (Icon Packs)
        self.GLYPH_SETS = {
            "Sovereign": {"browser": "🌐", "explorer": "📁", "store": "📦", "brain": "🧠"},
            "Retro":     {"browser": "📟", "explorer": "💾", "store": "🏪", "brain": "🕹️"},
            "Fluency":   {"browser": "🌍", "explorer": "📂", "store": "🛍️", "brain": "💡"},
            "Monolith":  {"browser": "⬛", "explorer": "⬛", "store": "⬛", "brain": "⬛"}
        }
        
        # SOUNDSCAPES
        self.SOUNDSCAPES = {
            "Zen":      {"boot": "zen_hum.wav", "notify": "chime.wav"},
            "Cyber":    {"boot": "glitch_start.wav", "notify": "zap.wav"},
            "Silent":   {"boot": None, "notify": None}
        }

    def get_glyphs(self, set_name: str = "Sovereign") -> Dict[str, str]:
        return self.GLYPH_SETS.get(set_name, self.GLYPH_SETS["Sovereign"])

    def get_soundscape(self, set_name: str = "Zen") -> Dict[str, Optional[str]]:
        return self.SOUNDSCAPES.get(set_name, self.SOUNDSCAPES["Zen"])
