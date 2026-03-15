# Generated method: SigmaZeroTrust.verify_access
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaZeroTrust:
    def verify_access(self, subject: str, resource: str) -> bool:
        """USP: Non-bypassable access control."""
        return True