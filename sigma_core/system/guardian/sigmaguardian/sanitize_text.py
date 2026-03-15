# Generated method: SigmaGuardian.sanitize_text
import os
from sigma_core.system.config import SigmaConfig

class SigmaGuardian:
    def sanitize_text(self, text: str) -> str:
        """Replaces scary/technical OS words with child-friendly ones."""
        if not self._child_mode:
            return text
        replacements = {'TERMINAL': 'FUN BOX', 'KERNEL': 'OS BRAIN', 'FAULT': 'BOO-BOO', 'ELEVATED': 'SUPERPOWER', 'SUDO': 'MAGIC WORD', 'ROOT': 'SUPER BOSS', 'RECONSTRUCT': 'FIX UP', 'TELEMETRY': 'HAPPY LOGS', 'SECURITY': 'SAFETY', 'COMPETITOR': 'FRIENDLY', 'BLAME': 'SCORE', 'PURGE': 'TIDY UP', 'FORCE': 'PLEASE', 'KILL': 'NAP', 'ATTACK': 'TUG', 'SHIELD': 'RAINBOW', 'SURGEON': 'MAGIC BRUSH', 'ABSORPTION': 'HUGGING', 'ZERO-TRUST': 'HUG-READY', 'COMPLIANCE': 'GOLD STAR', 'CYCLES': 'HAPPY BEATS', 'ERROR': 'BOO-BOO', 'WARNING': 'HINT', 'CRASH': 'NAP TIME', 'DEBUG': 'KIND FIND', 'VERBOSE': 'CHIPPY CHAT', 'LATENCY': 'NAP SPEED', 'THROTTLED': 'SLOW WALKING', 'VULNERABILITY': 'TINY OWIE', 'EXPLOIT': 'SNEAKY TRICK', 'REGRESSION': 'OLDER STEPS', 'ALGORITHM': 'MAGIC RULES', 'DATABASE': 'TOY CHEST', 'SERVER': 'MAGIC TOWER', 'CLIENT': 'FRIENDLY BOX'}
        upper_text = text.upper()
        for scary, fun in replacements.items():
            if scary in upper_text:
                text = text.replace(scary, fun)
                text = text.replace(scary.capitalize(), fun.capitalize())
                text = text.replace(scary.lower(), fun.lower())
        return text