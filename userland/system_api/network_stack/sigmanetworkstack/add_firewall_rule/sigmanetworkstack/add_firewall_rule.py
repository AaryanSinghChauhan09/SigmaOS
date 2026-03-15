# Generated method: SigmaNetworkStack.add_firewall_rule
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaNetworkStack:
    def add_firewall_rule(self, chain: str, src: str, dst: str, action: str='DROP', comment: str='') -> dict:
        rule_id = f'rule-{str(uuid.uuid4())[:6]}'
        self._firewall_rules.append({'id': rule_id, 'chain': chain, 'src': src, 'dst': dst, 'action': action, 'comment': comment})
        return {'rule_id': rule_id, 'message': f'Firewall: Rule {rule_id} added [{chain}] {src}→{dst} {action}.'}