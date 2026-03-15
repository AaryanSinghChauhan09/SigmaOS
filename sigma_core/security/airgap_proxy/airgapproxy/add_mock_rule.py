# Generated method: AirGapProxy.add_mock_rule
import json
from typing import Dict, Any

class AirGapProxy:
    def add_mock_rule(self, domain: str, response_body: str, status_code: int=200):
        self.active_rules[domain] = {'status': status_code, 'body': response_body}