# Generated method: AetherAssistant._extract_entity
import re
import time
from typing import Dict, List, Any

class AetherAssistant:
    def _extract_entity(self, text: str, intent: str) -> str:
        """Extracts the target object/parameter from the user's prompt."""
        text_lower = text.lower()
        if intent == 'launch_app':
            userland_apps = ['lab', 'vanguard', 'forge', 'commerce', 'access', 'identity', 'brain', 'studio']
            for app in userland_apps:
                if app in text_lower:
                    return app
        if intent == 'sys_theme':
            if 'dark' in text_lower:
                return 'dark'
            if 'light' in text_lower:
                return 'light'
        if intent == 'agentic_task':
            match = re.search('(research|analyze|summarize)\\s+(.*)', text_lower)
            if match:
                return match.group(2)
        if intent == 'set_persona':
            for p in self._personas.keys():
                if p.lower() in text_lower:
                    return p
        return ''