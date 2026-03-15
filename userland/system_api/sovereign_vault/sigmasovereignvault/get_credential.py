# Generated method: SigmaSovereignVault.get_credential
from typing import Dict, List, Any
import random

class SigmaSovereignVault:
    def get_credential(self, site: str) -> str:
        """USP: Auto-fills site credentials into Sovereign Browser."""
        if self._vault_status != 'UNLOCKED':
            return 'Auth Required: Unlock Vault.'
        return self._credentials.get(site, 'Credential not found.')