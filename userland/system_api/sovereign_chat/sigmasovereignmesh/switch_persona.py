"""
Auto-split from userland\system_api\sovereign_chat.py — SigmaSovereignMesh.switch_persona
"""

from dataclasses import dataclass, field
from enum import Enum
import time
import hashlib
import json
import random



class SigmaSovereignMesh:
    def switch_persona(self, new_alias: str) -> dict:
        """The Bitchat Killer: Instantly flip between verified public and ephemeral anonymous modes."""
        if new_alias not in self._identities:
            return {'error': 'Identity not found. Create it first.'}
        self._active_alias = new_alias
        persona = self._identities[new_alias].persona_type.value
        return {'status': 'Switched', 'active': new_alias, 'mode': persona, 'message': f"AuraMesh: Seamlessly shifted context to '{new_alias}' ({persona})."}
