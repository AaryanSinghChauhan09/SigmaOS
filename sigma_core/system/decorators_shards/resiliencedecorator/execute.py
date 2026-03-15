from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import ResilienceDecorator

class ResilienceDecorator:
    def execute(self, action, *args, **kwargs):
        try:
            return super().execute(action, *args, **kwargs)
        except Exception as e:
            print(f"[ZENITH-FAULT] Exception in {self.metadata.get('name')}: {e}")
            return {'error': 'AUTO_REMEDY_ACTIVE'}