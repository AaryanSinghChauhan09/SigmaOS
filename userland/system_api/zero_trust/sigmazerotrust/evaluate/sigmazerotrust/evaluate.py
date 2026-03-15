# Generated method: SigmaZeroTrust.evaluate
import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaZeroTrust:
    def evaluate(self, identity_id: str, resource: ResourceType, action: str, context: dict | None=None) -> dict:
        """
                ABAC evaluation: checks trust level + matching policy rules.
                Returns ALLOW / DENY with detailed reasoning.
                """
        identity = self._identities.get(identity_id)
        if identity is None:
            self._threat_count += 1
            return {'decision': 'DENY', 'reason': 'Unknown identity.', 'message': f"ZeroTrust: DENY — identity '{identity_id}' not registered."}
        context = context or {}
        decisions = []
        for rule in self._policies.values():
            if rule.resource != resource:
                continue
            if rule.action not in ('*', action):
                continue
            if 'trust_min' in rule.conditions:
                required = TrustLevel[rule.conditions['trust_min']]
                if identity.trust.value < required.value:
                    decisions.append(('DENY', f'Insufficient trust: {identity.trust.name} < {required.name}'))
                    continue
            decisions.append((rule.effect.upper(), f'rule {rule.rule_id}'))
        final = 'DENY'
        if any((d[0] == 'ALLOW' for d in decisions)):
            final = 'ALLOW'
        if any((d[0] == 'DENY' for d in decisions)):
            final = 'DENY'
        if not decisions:
            final = 'DENY'
        self._record_permission(identity_id, resource.value, action, final)
        return {'decision': final, 'identity': identity.subject, 'resource': resource.value, 'action': action, 'reasoning': decisions, 'message': f"PolicyEngine: {final} — '{identity.subject}' {action} on {resource.value}. ({len(decisions)} rules evaluated)"}