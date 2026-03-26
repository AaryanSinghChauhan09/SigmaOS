from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.security_interfaces import ISecurityGuard
import hashlib

from ._base import SovereigntyManager

class SovereigntyManager:
    def initialize(self):
        print('[SECURITY] Sovereignty Manager Online.')