"""
Auto-split from userland\system_api\aether_assistant.py — AetherAssistant._apply_persona
"""

import re
import time
from typing import Dict, List, Any



class AetherAssistant:
    def _apply_persona(self, raw_response: str) -> str:
        """USP: Modifies dialogue output based on the active structural persona."""
        if self.active_persona == 'Sovereign':
            return raw_response
        if self.active_persona == 'Maverick':
            if 'Optimal' in raw_response:
                return "We're flying fast. RAM is tight but holding."
            return f"[MAVERICK] {raw_response} — Done. What's next?"
        if self.active_persona == 'Scholar':
            return f'According to system telemetry: {raw_response}. This indicates nominal operation.'
        return raw_response
