"""
SigmaOS Fluid Design System (v1.0 Apex)
========================================
USP: Centralized Aesthetic Intelligence & Design Tokens.
Enforces a consistent, premium visual language across the entire OS.
Reference: Sigma Vision Design Guidelines.
"""
from typing import Dict, Any

# Design Tokens: Palette (Atomic Colors)
PALETTE = {
    "background": "#050505",
    "surface": "#0F1012",
    "surface_variant": "#1A1C1E",
    "primary": "#00D4FF",      # Sovereign Blue
    "secondary": "#FFD60A",    # Advocate Gold
    "accent": "#FF375F",       # Stealth Red
    "success": "#32D74B",      # Resilient Green
    "warning": "#FF9F0A",      # Warning Orange
    "error": "#FF453A",        # Critical Red
    "text_primary": "#F2F2F7",
    "text_secondary": "#8E8E93",
    "text_tertiary": "#48484A",
    "border": "#2C2C2E",
    "glass": "rgba(255, 255, 255, 0.05)"
}

# Design Tokens: Typography
TYPOGRAPHY = {
    "h1": ("Outfit", 32, "bold"),
    "h2": ("Outfit", 24, "bold"),
    "h3": ("Inter", 18, "bold"),
    "body": ("Inter", 10),
    "body_bold": ("Inter", 10, "bold"),
    "caption": ("Inter", 8, "bold"),
    "mono": ("JetBrains Mono", 9)
}

# Design Tokens: Animation & Transitions
ANIMATION = {
    "fast": 200,      # ms
    "standard": 400,  # ms
    "slow": 700       # ms
}

class FluidTheme:
    """Orchestrates themed assets and logic for Sigma shards."""
    @staticmethod
    def get_color(token: str) -> str:
        return PALETTE.get(token, "#FF00FF")

    @staticmethod
    def get_font(token: str) -> tuple:
        return TYPOGRAPHY.get(token, ("Arial", 10))

    @staticmethod
    def apply_to_widget(widget, theme_type: str = "surface"):
        """Applies theme tokens to a Tkinter/Fluid widget."""
        if theme_type == "surface":
            widget.configure(bg=PALETTE["surface"])
        elif theme_type == "primary_btn":
            widget.configure(bg=PALETTE["primary"], fg=PALETTE["background"])
        # Extended logic for specialized widgets...
