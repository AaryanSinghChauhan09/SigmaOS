# Generated method: FluidTheme.get_font
from typing import Dict, Any, Tuple

class FluidTheme:
    @staticmethod
    def get_font(token: str) -> Tuple[str, int, str]:
        """Provides type-safe font tuples derived from design tokens."""
        font_data = TYPOGRAPHY.get(token, ('Arial', 10, 'normal'))
        family = str(font_data[0])
        size = int(font_data[1])
        weight = 'normal'
        font_list = list(font_data)
        if len(font_list) > 2:
            weight = str(font_list[2])
        else:
            weight = 'normal'
        return (family, size, weight)