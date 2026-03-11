"""
SigmaOS Sovereign Theme Engine (Aura v1.0 Apex)
==============================================
USP: Dynamic OS-wide "Aura" skinning with zero-latency palette shifting.
Customizes UI, Terminal, and System Notifications based on active Mode.
"""

from typing import Dict, Any

try:
    from sigma_core.interfaces import SigmaModuleBase
except ImportError:
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel

class SigmaThemeEngine(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel # Explicit for linter
        self.current_aura = "DeepSpace"
        self.AURAS = {
            "DeepSpace":  {"bg": "#0D0D0D", "accent": "#00FFC2", "term": "Glassmorphism"},
            "SolarApex":  {"bg": "#FFFFFF", "accent": "#FF4D00", "term": "SleekLight"},
            "CyberPunk":  {"bg": "#1A0033", "accent": "#FF00FF", "term": "NeonRetro"},
            "MatrixOS":   {"bg": "#000000", "accent": "#00FF00", "term": "Monospace-Classic"},
            "Zodiac":     {"bg": "#121212", "accent": "#6C63FF", "term": "VelvetDark"}
        }
        
        if self.kernel and hasattr(self.kernel, "bus") and self.kernel.bus:
            self.kernel.bus.subscribe("mode.change", lambda p: self._auto_theme(p))

    def apply_aura(self, aura_name: str) -> bool:
        if aura_name not in self.AURAS: return False
        self.current_aura = aura_name
        
        # In a real OS, this would update CSS variables in the GUI or Registry keys
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit("aura.applied", self.AURAS[aura_name])
            
        print(f"[AURA] Switched to {aura_name} style.")
        return True

    def _auto_theme(self, payload: dict):
        """Automatically shift aura based on OS Mode."""
        mode = payload.get("preset", "Normal")
        if mode == "Gaming_Apex":
            self.apply_aura("CyberPunk")
        elif mode == "Nightly_Purge":
            self.apply_aura("DeepSpace")
        elif mode == "Work_Symmetry":
            self.apply_aura("Zodiac")

    def get_custom_palette(self) -> Dict[str, str]:
        return self.AURAS.get(self.current_aura, self.AURAS["DeepSpace"])

    def health_check(self) -> str:
        return f"OK — ThemeEngine Active | Aura: {self.current_aura} | Palettes: {len(self.AURAS)}"
