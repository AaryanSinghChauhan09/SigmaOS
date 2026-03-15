from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.security_interfaces import ISecurityGuard
import hashlib

from ._base import SovereigntyManager

class SovereigntyManager:
    def authorize(self, actor: str, resource: str) -> bool:
        """
            Implements ISecurityGuard interface.
            """
        print(f'[SECURITY] Authorizing {actor} for {resource}')
        return True