"""
Auto-split from userland\system_api\aether_assistant.py — AetherAssistant.process_prompt
"""

import re
import time
from typing import Dict, List, Any



class AetherAssistant:
    def process_prompt(self, prompt: str) -> dict:
        """Main entry point for the Assistant. End-to-end processing pipeline."""
        start_t = time.perf_counter()
        tokens = self._tokenize(prompt)
        intent = self._classify_intent(tokens)
        entity = self._extract_entity(prompt, intent)
        response = self._execute_intent(intent, entity, prompt)
        self._context_buffer.append({'u': prompt, 'intent': intent, 'entity': entity})
        if len(self._context_buffer) > 10:
            self._context_buffer.pop(0)
        ms_taken = (time.perf_counter() - start_t) * 1000
        return {'intent': intent, 'entity': entity, 'persona': self.active_persona, 'response': self._apply_persona(response), 'latency_ms': float(f'{ms_taken:.2f}'), 'status': 'SUCCESS' if intent != 'unknown' else 'UNRECOGNIZED'}
