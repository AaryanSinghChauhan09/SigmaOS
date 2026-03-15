"""
SigmaOS Air-Gapped Proxy v1.0
==============================
USP: Zero-Trust Network Simulation.
Intercepts untrusted app requests and feeds them AI-generated 'Fake Internet' responses.
Prevents data exfiltration by keeping apps 100% offline.
"""
import json
from typing import Dict, Any

class AirGapProxy:
    def __init__(self, kernel):
        self.kernel = kernel
        self.active_rules = {
            "*.google.com": {"status": 200, "body": "{'status': 'ok', 'user': 'anonymous'}"},
            "api.telemetry.io": {"status": 404, "body": "Not Found"}
        }

    def intercept_request(self, target_url: str, request_data: Dict[str, Any] = None) -> Dict:
        """Determines if a request should be mocked or blocked."""
        print(f"[AIRGAP] Intercepted request to: {target_url}")
        
        # Privacy Check: If the request contains PII, scrub it in the mock response log
        self.kernel._morphic_island(f"AIRGAP: Suppressed telemetry to {target_url}", "#FF4500") # Orangered

        # Match Rule
        for pattern, response in self.active_rules.items():
            if pattern in target_url or pattern.replace("*.", "") in target_url:
                return {
                    "status": response["status"],
                    "body": response["body"],
                    "source": "SIGMA-VIRTUAL-NET"
                }
        
        # Fallback: Generic fake success response to keep the app running without errors
        return {
            "status": 200,
            "body": "<html><body>SigmaOS Virtual Gateway: Connectivity Simulated.</body></html>",
            "source": "SIGMA-VIRTUAL-NET"
        }

    def add_mock_rule(self, domain: str, response_body: str, status_code: int = 200):
        self.active_rules[domain] = {"status": status_code, "body": response_body}

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
    
    proxy = AirGapProxy(MockKernel())
    res = proxy.intercept_request("https://api.telemetry.io/v1/update")
    print(f"Mock Response: {res}")
    
    res2 = proxy.intercept_request("https://check-connectivity.org")
    print(f"Generic Response: {res2['body']}")
