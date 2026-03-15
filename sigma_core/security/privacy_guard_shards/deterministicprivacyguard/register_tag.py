from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

from ._base import DeterministicPrivacyGuard

class DeterministicPrivacyGuard:
    def register_tag(self, tag: str, required_purpose: str):
        self._tag_registry[tag] = required_purpose