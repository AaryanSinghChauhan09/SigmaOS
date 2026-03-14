"""
SigmaOS Fluid Design System (v2.0 Apex — PREMIUM)
=================================================
USP: Centralized Aesthetic Intelligence & Design Tokens.
Enforces a consistent, premium visual language across the entire OS.
Reference: Sigma Vision Design Guidelines (Glassmorphism & Neon-Vibrancy).
"""
from typing import Dict, Any, Tuple

# Design Tokens: Palette (Atomic Colors - Enhanced High-Contrast)
# Design Tokens: Palettes (Chromatic Vibe Support)
THEMES = {
    "DEEP_SPACE": {
        "background": "#050505", "surface": "#0F1012", "primary": "#00D4FF",
        "secondary": "#FFD60A", "accent": "#FF375F", "text_primary": "#F2F2F7",
        "border": "#2C2C2E"
    },
    "APEX_GOLD": {
        "background": "#0A0905", "surface": "#1A1810", "primary": "#FFD700",
        "secondary": "#FFFFFF", "accent": "#FF8C00", "text_primary": "#FFFDEF",
        "border": "#3A3420"
    },
    "FOREST_ECO": {
        "background": "#050A05", "surface": "#101A10", "primary": "#32D74B",
        "secondary": "#8E8E93", "accent": "#FF9F0A", "text_primary": "#F0FFF0",
        "border": "#203A20"
    },
    "ZEN_FOCUS": {
        "background": "#05050A", "surface": "#0E0E1A", "primary": "#5E5CE6",
        "secondary": "#AF52DE", "accent": "#007AFF", "text_primary": "#E5E5EA",
        "border": "#1C1C2E"
    },
    "CINEMA_NIGHT": {
        "background": "#000000", "surface": "#120A05", "primary": "#FF9F0A",
        "secondary": "#FF3B30", "accent": "#FFD60A", "text_primary": "#FFDDA1",
        "border": "#2A1808"
    },
    "STUDY_MINT": {
        "background": "#050A08", "surface": "#101F18", "primary": "#66FFB2",
        "secondary": "#30D158", "accent": "#BF5AF2", "text_primary": "#E0FFE0",
        "border": "#153025"
    },
    "WORK_STEEL": {
        "background": "#08090A", "surface": "#14171A", "primary": "#64D2FF",
        "secondary": "#A2ADB9", "accent": "#0A84FF", "text_primary": "#E1E8ED",
        "border": "#2C343B"
    },
    "CRIMSON_ALIVE": {
        "background": "#0A0505", "surface": "#1F1010", "primary": "#FF3B30",
        "secondary": "#FF453A", "accent": "#FF9F0A", "text_primary": "#FFE0E0",
        "border": "#401515"
    },
    "VITAL_WARM": {
        "background": "#0A0805", "surface": "#1F1810", "primary": "#FF9500",
        "secondary": "#FFCC00", "accent": "#FF3B30", "text_primary": "#FFF4E0",
        "border": "#402515"
    },
    "TRAVEL_HORIZON": {
        "background": "#05080A", "surface": "#101820", "primary": "#5AC8FA",
        "secondary": "#007AFF", "accent": "#32D74B", "text_primary": "#E0F5FF",
        "border": "#152535"
    },
    "GAMING_NEON": {
        "background": "#05000A", "surface": "#110022", "primary": "#BF5AF2",
        "secondary": "#FF375F", "accent": "#00D4FF", "text_primary": "#F5E0FF",
        "border": "#330066"
    },
    "BATTERY_OLIVE": {
        "background": "#080805", "surface": "#12120A", "primary": "#AABB66",
        "secondary": "#8E8E93", "accent": "#FFCC00", "text_primary": "#DDEEAA",
        "border": "#222215"
    }
}

ACTIVE_VIBE = "DEEP_SPACE"
PALETTE = THEMES[ACTIVE_VIBE].copy()
# Re-add common tokens
PALETTE.update({
    "surface_variant": "#1A1C1E",
    "success": "#32D74B", "warning": "#FF9F0A", "error": "#FF453A",
    "text_secondary": "#8E8E93", "text_tertiary": "#48484A",
    "glass": "rgba(255, 255, 255, 0.05)",
    "neon_glow": "rgba(0, 212, 255, 0.4)"
})

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
    def set_vibe(vibe_name: str):
        """USP: Dynamic Aesthetic Transformation. Swaps the active design palette."""
        global ACTIVE_VIBE, PALETTE
        if vibe_name in THEMES:
            ACTIVE_VIBE = vibe_name
            PALETTE.update(THEMES[vibe_name])
            return True
        return False

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
