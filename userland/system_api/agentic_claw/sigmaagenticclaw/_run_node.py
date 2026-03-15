# Generated method: SigmaAgenticClaw._run_node
import time
import uuid
import threading
from typing import List, Dict, Any, Optional
from dataclasses import dataclass, field

class SigmaAgenticClaw:
    def _run_node(self, session_id: str, node: ActionNode) -> bool:
        """Executes a node by bridging to the relevant Sigma Subsystem (UAL, VFS, etc)."""
        attempt = 0
        while attempt < node.retry_policy:
            try:
                target = self.registry.get(node.action.split('.')[0].lower())
                if target and hasattr(target, 'handle_agent_action'):
                    target.handle_agent_action(node.action, node.params)
                elif self.bus:
                    self.bus.emit(f'agent.action.{node.action}', node.params)
                self.active_sessions[session_id]['log'].append(f'SUCCESS: {node.action}')
                return True
            except Exception as e:
                attempt += 1
                self._stats['self_heals'] += 1
                if self.bus:
                    self.bus.emit('claw.self_heal', {'session': session_id, 'err': str(e)})
        return False