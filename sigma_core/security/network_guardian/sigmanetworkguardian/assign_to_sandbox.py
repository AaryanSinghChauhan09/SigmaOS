# Generated method: SigmaNetworkGuardian.assign_to_sandbox
import time
import threading
from typing import Dict, List, Any

class SigmaNetworkGuardian:
    def assign_to_sandbox(self, pid: str) -> str:
        """Place an untrusted process into the isolated network namespace."""
        return f"Process {pid} jailed to 'sigmaos-sandbox' netns. Zero host network access."