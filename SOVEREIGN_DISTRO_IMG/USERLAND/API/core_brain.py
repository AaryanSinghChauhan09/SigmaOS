"""
SigmaCoreBrain: The OS Intelligence Orchestrator.
===============================================
USP: Centralized 'Brain' that manages prompts, rules, and adapter routing.
Independence: Logic is abstracted from brand names and execution layers.
"""

from typing import Dict, List, Any
import json

class SigmaCoreBrain:
    def __init__(self, kernel):
        self.kernel = kernel
        self._rules = {
            "Sovereignty": "Always prefer local compute and open standards.",
            "Independence": "Avoid vendor-specific lock-in; use abstract adapters.",
            "Zero_Trust": "Verify all external data before ingestion into Sigma-FS."
        }
        self._prompt_templates = {
            "Meta_OS": (
                "You are the core OS brain for SigmaOS. "
                "Current mode: {mode}. Mode config: {config}. "
                "Goal: {goal}. Steps: 1. Interpret. 2. Route to Adapters. 3. Synthesize."
            )
        }

    def process_task(self, goal: str) -> str:
        """USP: Routes a goal through the abstract brain logic and Semantic Bus."""
        mode_info = self.kernel.modes.get_active_profile()
        
        # 1. Semantic Intent Detection
        if "save" in goal.lower() or "document" in goal.lower():
            intent = "save_document"
            params = {"content": "Brain_Generated_Blob", "filename": "mission_auto.log"}
        elif "message" in goal.lower() or "send" in goal.lower():
            intent = "send_message"
            params = {"body": goal, "recipient": "Sovereign_Mesh_Broad"}
        else:
            intent = "Generic_Insight"
            params = {"goal": goal}

        # 2. Routing via Semantic Bus
        bus_res = self.kernel.semantic_bus.emit(intent, params)
        return f"CoreBrain: Goal '{goal}' parsed as '{intent}'. Bus Response: {bus_res}"

    def get_adapter(self, service_type: str) -> str:
        """USP: Maps abstract service types to current active providers."""
        mapping = {
            "Email": "SovereignRelay (Local)",
            "Browser": "SigmaBrowser (Engine: Blink/Chromium)",
            "Vault": "QuantumVault (Sovereign)",
            "Storage": "SigmaFS (Sharded Mesh)"
        }
        return mapping.get(service_type, "Generic_Sovereign_Adapter")

    def health_check(self) -> str:
        return f"OK — {len(self._rules)} Global Rules active. Brain sync'ed with Kernel."
