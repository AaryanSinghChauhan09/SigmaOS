# Generated method: SovereignCompositor.compose_frame
from dataclasses import dataclass, field
from typing import List

class SovereignCompositor:
    def compose_frame(self):
        """USP: Central Compositor Loop (Double-Buffered)."""
        sorted_wins = sorted(self.windows, key=lambda w: w.z_order)
        composite_log = []
        for win in sorted_wins:
            composite_log.append(f'Blitting Win {win.id} @ {win.x},{win.y} Alpha={win.opacity}')
        composite_log.append('Rendering Mouse Cursor (Layer 999)')
        return composite_log