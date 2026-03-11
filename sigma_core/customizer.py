"""
SigmaOS Sovereign Customizer (v2.0 Apex)
=========================================
USP: Deep Morphological Personalization & Aesthetic Intelligence.
Handles system-wide themes, soundscapes, and visual DNA.
"""

import os
import json
from .interfaces import SigmaModuleBase

class SovereignCustomizer(SigmaModuleBase):
    """
    Sovereign Customizer manages the 'Vibe' and 'Aura' of SigmaOS.
    It provides high-fidelity aesthetic presets and morphological controls.
    """

    def __init__(self, kernel):
        super().__init__(kernel)
        self.active_vibe = "Minimalist"
        self.glass_opacity = 0.85
        self.blur_strength = 10
        self.border_radius = 12
        
        # GLYPH SETS (Icon Packs)
        self.GLYPH_SETS = {
            "Sovereign": {"browser": "🌐", "explorer": "📁", "store": "📦", "brain": "🧠"},
            "Retro":     {"browser": "📟", "explorer": "💾", "store": "🏪", "brain": "🕹️"},
            "Fluency":   {"browser": "🌍", "explorer": "📂", "store": "🛍️", "brain": "💡"},
            "Monolith":  {"browser": "⬛", "explorer": "⬛", "store": "⬛", "brain": "⬛"}
        }
        
        # SOUNDSCAPES (Placeholders for audio engine)
        self.SOUNDSCAPES = {
            "Zen":      {"boot": "zen_hum.wav", "notify": "chime.wav"},
            "Cyber":    {"boot": "glitch_start.wav", "notify": "zap.wav"},
            "Silent":   {"boot": None, "notify": None}
        }

    def apply_morphic_preset(self, preset_name: str):
        """Applies a morphological 'Vibe' preset."""
        presets = {
            "Brutalist": {"radius": 0, "opacity": 1.0, "blur": 0},
            "Glass":     {"radius": 16, "opacity": 0.6, "blur": 25},
            "Classic":   {"radius": 8, "opacity": 0.95, "blur": 5},
            "Aura":      {"radius": 24, "opacity": 0.75, "blur": 40}
        }
        
        if preset_name in presets:
            p = presets[preset_name]
            self.border_radius = p["radius"]
            self.glass_opacity = p["opacity"]
            self.blur_strength = p["blur"]
            return {"status": "SUCCESS", "preset": preset_name}
        return {"status": "ERROR", "msg": "Preset not found"}

    def generate_ai_theme(self, prompt: str):
        """USP: Neural Aesthetic Synthesis. Generates harmonized palettes via color theory logic."""
        prompt = prompt.lower()
        
        # 1. Base Color Generation (Deterministic Seed)
        seed = sum(ord(c) for c in prompt) % 360
        base_h = seed
        base_s = 70
        base_l = 20 if "dark" in prompt or "cyber" in prompt else 85
        
        # 2. Harmonic Calculation (Analogous/Complimentary)
        accent_h = (base_h + 180) % 360  # Complimentary
        text_l = 95 if base_l < 50 else 5
        
        def _hsl_to_hex(h, s, l):
            # Extremely simplified HSL->Hex for zero-dependency
            return f"hsl({h}, {s}%, {l}%)"

        styles = {
            "accent": _hsl_to_hex(accent_h, 85, 60),
            "background": _hsl_to_hex(base_h, 30, base_l),
            "text": _hsl_to_hex(base_h, 10, text_l),
            "glow": _hsl_to_hex(accent_h, 90, 70)
        }
        
        msg = f"Neural Fabric woven for: '{prompt}'. Harmony identified at {base_h}° Hue."
        if self.kernel:
             self.kernel.bus.emit("theme.ai_gen", {"prompt": prompt, "base_hue": base_h})

        return {
            "message": msg,
            "applied_styles": styles,
            "meta": {"seed": seed, "harmony": "Complimentary"}
        }

    def get_glyphs(self, set_name="Sovereign"):
        return self.GLYPH_SETS.get(set_name, self.GLYPH_SETS["Sovereign"])

    def health_check(self):
        return "OK"
