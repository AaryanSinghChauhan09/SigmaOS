# SigmaOS Omega Method: TelemetryCollector.__init__
from sigma_core.security.resilience_guard import resilient_module # noqa
import time
import json

class TelemetryCollector:
    @resilient_module
    def __init__(self):
        self.logs = []