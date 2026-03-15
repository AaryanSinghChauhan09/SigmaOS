# Generated method: SigmaMediaStudio._log_consent
import time
import os
import uuid

class SigmaMediaStudio:
    def _log_consent(self, action: str, details: str):
        """Immutable consent ledger for transparency."""
        entry = {'timestamp': time.time(), 'action': action, 'details': details, 'revoked': False}
        self.consent_ledger.append(entry)