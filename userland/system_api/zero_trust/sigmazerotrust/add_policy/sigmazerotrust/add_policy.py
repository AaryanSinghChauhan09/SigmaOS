# Generated method: SigmaZeroTrust.add_policy
import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaZeroTrust:
    def add_policy(self, subject: str, resource: ResourceType, action: str, effect: str, conditions: dict | None=None) -> dict:
        rule_id = f'pol-{str(uuid.uuid4())[:6]}'
        rule = PolicyRule(rule_id, subject, resource, action, conditions or {}, effect)
        self._policies[rule_id] = rule
        return {'rule_id': rule_id, 'effect': effect, 'message': f'PolicyEngine: Rule {rule_id} added [{resource.value}/{action}={effect}].'}