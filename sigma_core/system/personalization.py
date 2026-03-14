"""
SigmaOS Personalization Engine (v4.0 Apex)
===========================================
USP: Cognitive Intent Mapping & Neural Pre-Loading.
Modular Architecture: Delegating to ProfileManager, IntentEngine, and EcoGamification.
"""
from typing import Dict, Any, List
from .profile_manager import ProfileManager
from .intent_engine import IntentEngine
from .eco_gamification import EcoGamification

class SigmaModuleBase:
    def __init__(self, kernel): self.kernel = kernel
    def log_event(self, a, c):
        if self.kernel and hasattr(self.kernel, "bus"):
             self.kernel.bus.emit(f"personalization.{a}", c)

class PersonalizationEngine(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.profiles = ProfileManager(kernel)
        self.intent = IntentEngine(kernel)
        self.eco = EcoGamification(kernel)
        
    def record_interaction(self, action: str):
        """Unified interaction logging via modular delegation."""
        self.intent.record_action(action)
        prediction = self.intent.predict_intent()
        
        if prediction["confidence"] > 0.8:
            self.log_event("predictive_warmup", {"target": prediction["intent"]})

    def award_carbon_credits(self, amount: float):
        """Sovereign eco-incentives via modular delegation."""
        current = float(self.profiles.get_preference("carbon_credits", 0.0))
        self.profiles.set_preference("carbon_credits", current + amount)
        self.eco.process_contribution(amount)
        self.log_event("eco_boost", {"credits": amount})

    def health_check(self) -> str:
        mode = self.profiles.get_preference("preferred_mode", "NEUTRAL")
        return f"OK — Profile: {mode} | Modular Intelligence: ACTIVE"
