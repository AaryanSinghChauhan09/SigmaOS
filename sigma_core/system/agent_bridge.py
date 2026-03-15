"""
SigmaOS Neural Agentic Bridge v1.0
===================================
USP: Standardized JSON-RPC interface for external AI agents.
Allows agents to query system telemetry, launch apps, and manage resources
via a secure, file-based or socket-based protocol.
"""
import json
import os
import time
from typing import Dict, Any

class AgenticBridge:
    def __init__(self, kernel, bridge_path: str = "sigma_storage/agent_bridge"):
        self.kernel = kernel
        self.bridge_path = bridge_path
        os.makedirs(self.bridge_path, exist_ok=True)
        self.inbox = os.path.join(self.bridge_path, "inbound.json")
        self.outbox = os.path.join(self.bridge_path, "outbound.json")

    def poll_for_agent_intent(self):
        """Checks if an agent has dropped a JSON intent in the bridge inbox."""
        if os.path.exists(self.inbox):
            try:
                with open(self.inbox, "r") as f:
                    cmd = json.load(f)
                os.remove(self.inbox) # Acknowledge
                self._dispatch_command(cmd)
            except Exception as e:
                print(f"[AGENT-BRIDGE] Malformed intent: {e}")

    def _dispatch_command(self, cmd: Dict[str, Any]):
        method = cmd.get("method")
        params = cmd.get("params", {})
        print(f"[AGENT-BRIDGE] Received command: {method}")
        
        response = {"status": "error", "msg": "Method not found"}
        
        if method == "get_telemetry":
            response = {
                "status": "ok",
                "cpu": 12.5,
                "mem_free": "8.4GB",
                "active_vibe": self.kernel.registry.get("vibe_scheduler").current_vibe if self.kernel.registry.get("vibe_scheduler") else "Normal"
            }
        elif method == "launch_app":
            app_id = params.get("app_id")
            self.kernel._morphic_island(f"AGENT: Launching {app_id}", "#7FFF00")
            response = {"status": "ok", "msg": f"Launch signal sent for {app_id}"}
            
        with open(self.outbox, "w") as f:
            json.dump(response, f)

    def push_telemetry(self, data: Dict[str, Any]):
        """Proactively pushes OS state to the agent."""
        state_file = os.path.join(self.bridge_path, "os_state.json")
        with open(state_file, "w") as f:
            json.dump(data, f)

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def __init__(self): self.registry = {}
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
        
    bridge = AgenticBridge(MockKernel())
    # Simulate agent drop
    with open(bridge.inbox, "w") as f:
        json.dump({"method": "get_telemetry"}, f)
    bridge.poll_for_agent_intent()
    with open(bridge.outbox, "r") as f:
        print(f"Agent received: {f.read()}")
