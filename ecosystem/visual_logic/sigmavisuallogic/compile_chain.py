# Generated method: SigmaVisualLogic.compile_chain
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

class SigmaVisualLogic:
    def compile_chain(self, start_block_id: str) -> dict:
        """Translates a block-chain into a SigmaRoutine compatible JSON."""
        if start_block_id not in self.active_canvas:
            return {'status': 'error', 'msg': 'Invalid start block.'}
        chain = []
        current_id = start_block_id
        while current_id:
            block = self.active_canvas[current_id]
            chain.append({'name': block.name, 'type': block.type, 'inputs': block.inputs})
            current_id = block.next_block_id
        self._stats['chains_compiled'] += 1
        return {'status': 'compiled', 'routine_name': f'Visual_{start_block_id}', 'execution_steps': chain, 'complexity': len(chain)}