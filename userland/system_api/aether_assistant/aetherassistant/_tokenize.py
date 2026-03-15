"""
Auto-split from userland\system_api\aether_assistant.py — AetherAssistant._tokenize
"""

import re
import time
from typing import Dict, List, Any



class AetherAssistant:
    def _tokenize(self, text: str) -> list:
        """Lexical analysis and normalization."""
        words = re.findall('\\b\\w+\\b', text.lower())
        return [w for w in words if w not in self._stop_words]
