from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

from ._base import DeterministicPrivacyGuard

class DeterministicPrivacyGuard:
    def __init__(self):
        super().__init__('PRIVACY_GUARD')
        self._tag_registry = {}