# Generated method: AirGapProxy.__init__
import json
from typing import Dict, Any

class AirGapProxy:
    def __init__(self, kernel):
        self.kernel = kernel
        self.active_rules = {'*.google.com': {'status': 200, 'body': "{'status': 'ok', 'user': 'anonymous'}"}, 'api.telemetry.io': {'status': 404, 'body': 'Not Found'}}