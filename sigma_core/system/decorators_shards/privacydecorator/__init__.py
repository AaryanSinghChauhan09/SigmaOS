from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import PrivacyDecorator

class PrivacyDecorator:
    def __init__(self, component, privacy_guard, required_tag):
        super().__init__(component)
        self._privacy_guard = privacy_guard
        self._required_tag = required_tag