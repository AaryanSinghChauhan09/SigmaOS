from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IChaosResilience
import random

class ChaosMonkey(SovereignModule, IChaosResilience):
    """
    Sovereign Chaos Monkey.
    Deliberately perturbs the system to ensure resilience.
    Demonstrates the Strategy for failure testing.
    """
    def __init__(self, target_factory):
        super().__init__("CHAOS_MONKEY")
        self._factory = target_factory
        self._perturbation_count = 0

    def perturb_system(self):
        """Standard Chaos Resilience action."""
        self._perturbation_count += 1
        print(f"[CHAOS] Perturbation Seq #{self._perturbation_count} Initialized.")
        
        # Simulate a memory/logic pressure on a random component
        event_types = ["SIMULATED_MEMORY_LEAK", "LOGIC_JITTER", "RPC_TIMEOUT"]
        event = random.choice(event_types)
        print(f"[CHAOS] Injecting Chaos Event: {event}")
        return event

    def execute(self, action=None):
        if action == "TICK":
            return self.perturb_system()
        return f"PERTURBATIONS_{self._perturbation_count}"

    def initialize(self):
        print("[CHAOS] Chaos Monkey Released into the Mesh.")

    def shutdown(self):
        print("[CHAOS] Chaos Monkey Recalled.")

    def health_check(self):
        return True
