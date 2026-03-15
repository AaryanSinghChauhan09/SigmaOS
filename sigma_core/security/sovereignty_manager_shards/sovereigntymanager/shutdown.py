from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.security_interfaces import ISecurityGuard
import hashlib

from ._base import SovereigntyManager

class SovereigntyManager:
    def shutdown(self):
        print('[SECURITY] Sovereignty Manager Offline.')