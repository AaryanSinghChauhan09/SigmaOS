from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

from ._base import DeterministicPrivacyGuard

class DeterministicPrivacyGuard:
    def health_check(self):
        return True