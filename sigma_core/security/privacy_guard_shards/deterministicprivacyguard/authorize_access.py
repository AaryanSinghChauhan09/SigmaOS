from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

from ._base import DeterministicPrivacyGuard

class DeterministicPrivacyGuard:
    def authorize_access(self, data_tag: str, requester_purpose: str) -> bool:
        """
            Determines if access is authorized based on deterministic contracts.
            """
        required = self._tag_registry.get(data_tag)
        if not required:
            print(f'[PRIVACY-FAIL] Untagged data access: {data_tag}')
            return False
        if required == requester_purpose:
            print(f'[PRIVACY-OK] Authorized access to {data_tag} for {requester_purpose}')
            return True
        print(f"[PRIVACY-DENIED] Contract Mismatch: {data_tag} requires '{required}', got '{requester_purpose}'")
        return False