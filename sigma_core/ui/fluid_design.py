"""
SigmaOS Fluid Design System (v2.0 Apex — PREMIUM)
=================================================
USP: Centralized Aesthetic Intelligence & Design Tokens.
Enforces a consistent, premium visual language across the entire OS.
Reference: Sigma Vision Design Guidelines (Glassmorphism & Neon-Vibrancy).
"""
from typing import Dict, Any, Tuple

# Design Tokens: Palette (Atomic Colors - Enhanced High-Contrast)
PALETTE = {
    "background": "#050505",    # Deep Infinite Black
    "surface": "#0F1012",       # Frosted Steel
    "surface_variant": "#1A1C1E",
    "primary": "#00D4FF",       # Sovereign Blue (Electric)
    "secondary": "#FFD60A",     # Advocate Gold
    "accent": "#FF375F",        # Stealth Red (Laser)
    "success": "#32D74B",       # Resilient Green (Matrix)
    "warning": "#FF9F0A",       # Warning Orange
    "error": "#FF453A",         # Critical Red
    "text_primary": "#F2F2F7",
    "text_secondary": "#8E8E93",
    "text_tertiary": "#48484A",
    "border": "#2C2C2E",        # Sub-pixel Border
    "glass": "rgba(255, 255, 255, 0.05)",
    "neon_glow": "rgba(0, 212, 255, 0.4)"
}

# Design Tokens: Typography (Modern & Clean)
TYPOGRAPHY = {
    "h1": ("Outfit", 32, "bold"),
    "h2": ("Outfit", 24, "bold"),
    "h3": ("Inter", 18, "bold"),
    "body": ("Inter", 10),
    "body_bold": ("Inter", 10, "bold"),
    "caption": ("Inter", 8, "bold"),
    "mono": ("JetBrains Mono", 9)
}

# Design Tokens: Animation & Transitions (Ultra-Smooth)
ANIMATION = {
    "fast": 150,      # Elastic In
    "standard": 350,  # Cubic Bezier
    "slow": 600       # Smooth Fade
}

class FluidTheme:
    """Orchestrates themed assets and logic for Sigma shards with Aesthetic Intelligence."""
    @staticmethod
    def get_color(token: str) -> str:
        return PALETTE.get(token, "#FF00FF")

    @staticmethod
    def get_font(token: str) -> Tuple[str, int, str]:
        """Provides type-safe font tuples derived from design tokens."""
        font_data = TYPOGRAPHY.get(token, ("Arial", 10, "normal"))
        
        # Robust parsing for 2 or 3 element font tuples
        family = str(font_data[0])
        size   = int(font_data[1])
        weight = "normal"
        
        # Use list conversion and safe access to satisfy strict index checking
        font_list = list(font_data)
        if len(font_list) > 2:
            weight = str(font_list[2])
        else:
            weight = "normal"
            
        return (family, size, weight)

    @staticmethod
    def apply_to_widget(widget, theme_type: str = "surface"):
        """USP: Proactive UI Theming. Applies tokens to any supported widget class."""
        try:
            if theme_type == "surface":
                widget.configure(bg=PALETTE["surface"], fg=PALETTE["text_primary"])
            elif theme_type == "primary_btn":
                widget.configure(bg=PALETTE["primary"], fg=PALETTE["background"], 
                                 activebackground=PALETTE["neon_glow"], 
                                 font=TYPOGRAPHY["body_bold"])
            elif theme_type == "glass_panel":
                widget.configure(bg=PALETTE["surface_variant"], highlightthickness=1, 
                                 highlightbackground=PALETTE["border"])
        except Exception:
            pass # Graceful degradation for non-standard widgets
