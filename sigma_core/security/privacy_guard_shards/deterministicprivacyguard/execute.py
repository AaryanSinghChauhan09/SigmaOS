from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IPrivacyGuard

from ._base import DeterministicPrivacyGuard

class DeterministicPrivacyGuard:
    def execute(self, action, tag=None, purpose=None):
        if action == 'AUTHORIZE':
            return self.authorize_access(tag, purpose)
        return None