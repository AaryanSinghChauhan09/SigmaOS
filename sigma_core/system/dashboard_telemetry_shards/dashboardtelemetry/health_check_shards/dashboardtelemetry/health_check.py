from abc import ABC, abstractmethod
from .base_sovereign import SigmaModule
import json

class DashboardTelemetry:
    def health_check(self):
        return True