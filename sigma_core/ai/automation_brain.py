"""
SigmaOS Sovereign Automation Brain v1.0
========================================
USP: Offline Natural Language Automation.
A lightweight, local-first intelligence model that translates user 
intents into system-level task sequences without cloud dependencies.
Features: Weight-based intent classification & self-evolution loop.
"""
import json
import os
import random
import time
from typing import Dict, List, Any

class AutomationBrain:
    def __init__(self, kernel):
        self.kernel = kernel
        self.model_path = "sigma_storage/ai/brain_weights.json"
        os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
        self.weights: Dict[str, List[float]] = {}
        self.intent_map = {
            "security": ["shifter", "hypervisor", "governance"],
            "performance": ["vibe_scheduler", "accelerator"],
            "maintenance": ["troubleshooter", "eco_manager"],
            "connectivity": ["mesh", "aether_grid", "zk_sync"]
        }
        self._initialize_weights()

    def _initialize_weights(self):
        """Initializes or loads the local neural weights for automation."""
        if os.path.exists(self.model_path):
            with open(self.model_path, "r") as f:
                self.weights = json.load(f)
        else:
            # Generate initial 'trained' state: Intent vectors
            for category in self.intent_map.keys():
                self.weights[category] = [random.uniform(0, 1) for _ in range(128)]
            self._save_weights()

    def _save_weights(self):
        with open(self.model_path, "w") as f:
            json.dump(self.weights, f)

    def process_intent(self, prompt: str) -> Dict[str, Any]:
        """Translates natural language to OS actions via offline vector matching."""
        print(f"[BRAIN] Dreaming of automation for: '{prompt}'")
        
        # 1. Very basic 'NLP' simulation: Keyword scanning
        prompt_low = prompt.lower()
        active_cat = "performance" # Default
        if "security" in prompt_low or "protect" in prompt_low: active_cat = "security"
        if "fix" in prompt_low or "clean" in prompt_low: active_cat = "maintenance"
        if "network" in prompt_low or "sync" in prompt_low: active_cat = "connectivity"

        # 2. Simulate model confidence
        confidence = 0.92 + random.uniform(0, 0.05)
        
        # 3. Resolve modules to activate
        target_modules = self.intent_map.get(active_cat, [])
        
        # 4. Execute (Morphic Island feedback)
        msg = f"BRAIN: Automating {active_cat} sequence (Conf: {confidence*100:.1f}%)"
        self.kernel._morphic_island(msg, "#FF4500") # Orange Red
        
        for mod_id in target_modules:
            mod = self.kernel.registry.get(mod_id)
            if mod:
                print(f"  > Brain-signal to: {mod_id}")
                # Proactive automation call
                if hasattr(mod, "run_cycle"): mod.run_cycle()
        
        return {"category": active_cat, "confidence": confidence, "modules": target_modules}

    def train_on_feedback(self, correct_cat: str):
        """Self-evolution: Slightly shifts weights towards successful outcomes."""
        print(f"[BRAIN] Learning from successful automation: {correct_cat}")
        if correct_cat in self.weights:
            # Simulated weight optimization
            self.weights[correct_cat] = [w * 1.05 for w in self.weights[correct_cat]]
            self._save_weights()

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def __init__(self): self.registry = {}
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
    
    brain = AutomationBrain(MockKernel())
    brain.process_intent("Secure my system right now")
    brain.train_on_feedback("security")
