from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import PrivacyDecorator

class PrivacyDecorator:
    def execute(self, action, *args, **kwargs):
        """
            Overridden execute to check privacy contract.
            """
        if not self._privacy_guard:
            return super().execute(action, *args, **kwargs)
        purpose = kwargs.get('purpose')
        if self._privacy_guard.authorize_access(self._required_tag, purpose):
            return super().execute(action, *args, **kwargs)
        print(f"[PRIVACY-VETO] Blocked execution of {self.metadata.get('name')} - Purpose '{purpose}' unauthorized for tag '{self._required_tag}'")
        return {'error': 'PRIVACY_VIOLATION', 'required_tag': self._required_tag}