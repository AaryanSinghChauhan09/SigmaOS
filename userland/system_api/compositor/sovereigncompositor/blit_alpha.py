# Generated method: SovereignCompositor.blit_alpha
from dataclasses import dataclass, field
from typing import List

class SovereignCompositor:
    def blit_alpha(self, src_color: tuple, dest_color: tuple, alpha: int) -> tuple:
        """USP: Standard Alpha Blending Formula."""
        r = (src_color[0] * alpha + dest_color[0] * (255 - alpha)) // 255
        g = (src_color[1] * alpha + dest_color[1] * (255 - alpha)) // 255
        b = (src_color[2] * alpha + dest_color[2] * (255 - alpha)) // 255
        return (r, g, b)