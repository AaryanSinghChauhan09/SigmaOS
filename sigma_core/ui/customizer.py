"""
SigmaOS Sovereign Customizer (v3.0 Apex)
=========================================
USP: Deep Morphological Personalization & Aesthetic Intelligence.
Modular Architecture: Delegating to MorphicEngine, SensoryAssets, and NeuralThemer.
"""
from typing import Dict, Any, List
from .morphic_engine import MorphicEngine
from .sensory_assets import SensoryAssets
from .neural_themer import NeuralThemer

class SigmaModuleBase:
    def __init__(self, kernel=None): self.kernel = kernel

class SovereignCustomizer(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.morphic = MorphicEngine(kernel)
        self.sensory = SensoryAssets(kernel)
        self.themer = NeuralThemer(kernel)
        
    def apply_morphic_preset(self, preset_name: str) -> Dict[str, Any]:
        """Sovereign morphological re-tuning via modular delegation."""
        res = self.morphic.apply_preset(preset_name)
        if res["status"] == "SUCCESS" and self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit("ui.morphic_shift", {"preset": preset_name})
        return res

    def generate_ai_theme(self, prompt: str) -> Dict[str, Any]:
        """Sovereign neural synthesis via modular delegation."""
        theme_data = self.themer.generate_theme(prompt)
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit("theme.ai_gen", theme_data)
        return theme_data

    def get_glyphs(self, set_name: str = "Sovereign") -> Dict[str, str]:
        return self.sensory.get_glyphs(set_name)

    def health_check(self) -> str:
        return "OK — Customizer Modularized (Morphic + Sensory + Neural)"
