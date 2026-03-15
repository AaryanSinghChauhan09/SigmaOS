# Generated method: FluidTheme.apply_to_widget
from typing import Dict, Any, Tuple

class FluidTheme:
    @staticmethod
    def apply_to_widget(widget, theme_type: str='surface'):
        """USP: Proactive UI Theming. Applies tokens to any supported widget class."""
        try:
            if theme_type == 'surface':
                widget.configure(bg=PALETTE['surface'], fg=PALETTE['text_primary'])
            elif theme_type == 'primary_btn':
                widget.configure(bg=PALETTE['primary'], fg=PALETTE['background'], activebackground=PALETTE['neon_glow'], font=TYPOGRAPHY['body_bold'])
            elif theme_type == 'glass_panel':
                widget.configure(bg=PALETTE['surface_variant'], highlightthickness=1, highlightbackground=PALETTE['border'])
        except Exception:
            pass