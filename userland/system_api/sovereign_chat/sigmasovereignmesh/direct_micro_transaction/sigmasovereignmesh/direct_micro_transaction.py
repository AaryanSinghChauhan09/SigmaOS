# Generated method: SigmaSovereignMesh.direct_micro_transaction
from dataclasses import dataclass, field
from enum import Enum
import time
import hashlib
import json
import random

class SigmaSovereignMesh:
    def direct_micro_transaction(self, target_alias: str, amount: float) -> dict:
        """The Substack/Patreon Killer: Native zero-fee economy tipping."""
        if not self._active_alias:
            return {'error': 'No active identity.'}
        sender = self._identities[self._active_alias]
        if sender.token_balance < amount:
            return {'error': 'Insufficient native OS tokens.'}
        sender.token_balance -= amount
        self._stats['transactions'] += 1
        return {'from': self._active_alias, 'to': target_alias, 'amount': amount, 'fee': '0.00%', 'message': f'AuraMesh: Sent {amount} Σ-Tokens to {target_alias} instantly via OS-native ledger. No middleman fees.'}