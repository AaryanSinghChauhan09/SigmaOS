from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

class DeterministicPrivacyGuard(SovereignModule, IPrivacyGuard):
    """
    Deterministic Privacy Guard.
    Enforces 'Purpose-of-Use' contracts on all data access shards.
    """
    def __init__(self):
        super().__init__("PRIVACY_GUARD")
        self._tag_registry = {} # Tag -> Required Purpose

    def register_tag(self, tag: str, required_purpose: str):
        self._tag_registry[tag] = required_purpose

    def authorize_access(self, data_tag: str, requester_purpose: str) -> bool:
        """
        Determines if access is authorized based on deterministic contracts.
        """
        required = self._tag_registry.get(data_tag)
        if not required:
            # Default Deny (Zero Trust)
            print(f"[PRIVACY-FAIL] Untagged data access: {data_tag}")
            return False
            
        if required == requester_purpose:
            print(f"[PRIVACY-OK] Authorized access to {data_tag} for {requester_purpose}")
            return True
        
        print(f"[PRIVACY-DENIED] Contract Mismatch: {data_tag} requires '{required}', got '{requester_purpose}'")
        return False

    def execute(self, action, tag=None, purpose=None):
        if action == "AUTHORIZE":
            return self.authorize_access(tag, purpose)
        return None

    def initialize(self):
        print("[PRIVACY] Zero-Waste Deterministic Privacy Engine Active.")

    def shutdown(self):
        self._tag_registry.clear()

    def health_check(self):
        return True
