# Generated method: SigmaDevForge.stop_container
import time
import uuid
import hashlib

class SigmaDevForge:
    def stop_container(self, c_id: str) -> str:
        if c_id in self.active_containers:
            del self.active_containers[c_id]
            return f"SigmaContainer '{c_id}' securely terminated. All memory wiped."
        return f"Error: Container '{c_id}' not found."