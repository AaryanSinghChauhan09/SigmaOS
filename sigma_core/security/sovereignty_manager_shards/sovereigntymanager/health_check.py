from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.security_interfaces import ISecurityGuard
import hashlib

from ._base import SovereigntyManager

class SovereigntyManager:
    def health_check(self) -> bool:
        """
            Required by ISystemComponent.
            """
        return self._status == 'READY'