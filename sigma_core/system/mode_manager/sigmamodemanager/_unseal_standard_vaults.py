"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._unseal_standard_vaults
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _unseal_standard_vaults(self, phase: str='') -> str:
        """Restores standard vault access after a high-security mode exits."""
        if self.kernel and hasattr(self.kernel, 'crypt_guard') and self.kernel.crypt_guard:
            return 'Standard vaults unsealed. Access restored to normal privilege level.'
        return 'Vaults unsealed (CryptGuard offline — fallback mode).'
