# Generated method: SigmaCustomizer.morph_fonts
import json
import random
import os

class SigmaCustomizer:
    def morph_fonts(self, family: str, scale: float) -> str:
        """Morphs the entire OS typography system natively."""
        self._styles['font_family'] = family
        self._styles['font_scaling'] = scale
        return f'Customizer: Typography morphed to {family} at {scale}x scale.'