"""
SigmaOS Eco Gamification (v1.1 Apex)
=====================================
USP: Environmentally Aware User Incentives.
Optimizes system performance based on real-time carbon intensity telemetry.
"""
import random
import time
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase

class EcoGamification(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.carbon_stats = {
            "current_intensity": 0.0, # gCO2/kWh
            "saved_carbon_mg": 0.0,
            "eco_streak_days": 0
        }

    def start_service(self) -> str:
        # Initial carbon telemetry poll
        self.poll_grid_intensity()
        return "Eco-Gamification: Carbon-Aware Scheduler Hydrated."

    def poll_grid_intensity(self):
        """USP: Real-time Carbon Telemetry Poll."""
        # Simulation: In a real kernel, this would query APIs like WattTime or CarbonIntensity.org.uk
        # Values range from 50 (clean) to 600 (dirty)
        self.carbon_stats["current_intensity"] = float(random.randint(50, 450))
        
        # If intensity is low, notify the scheduler to prioritize heavy batch jobs
        if float(self.carbon_stats["current_intensity"]) < 150.0:
             self.log_event("green_window_open", {"intensity": self.carbon_stats["current_intensity"]})
             if self.kernel and hasattr(self.kernel, "bus"):
                  self.kernel.bus.emit("eco.green_window", {"active": True})

    def process_contribution(self, amount: float):
        """USP: Environmental Gamification Link. Projects credits to OS XP."""
        # Calculate saved carbon based on background task deferral during "dirty" grid hours
        saved = amount * (self.carbon_stats["current_intensity"] / 100.0)
        self.carbon_stats["saved_carbon_mg"] += float(saved)
        
        if self.kernel and hasattr(self.kernel, "gamification"):
             xp_gain = int(amount * 50) # Aggressive rewards for green behavior
             self.kernel.gamification.add_xp(xp_gain)
             # Update carbon karma in stats
             self.kernel.gamification.stats["carbon_karma"] = float(self.carbon_stats["saved_carbon_mg"])
             
        if hasattr(self.kernel, "bus"):
             self.kernel.bus.emit("eco.credits_awarded", {"amount": amount, "saved_mg": saved})

    def get_carbon_impact(self) -> Dict[str, Any]:
        """Heuristic analysis of user's environmental footprint."""
        impact = "NEUTRAL"
        if self.carbon_stats["saved_carbon_mg"] > 1000: impact = "CARBON_NEGATIVE_CHAMPION"
        elif self.carbon_stats["saved_carbon_mg"] > 100: impact = "ECO_CONSCIOUS"
        
        return {
            "status": impact,
            "saved_mg": self.carbon_stats["saved_carbon_mg"],
            "grid_intensity": self.carbon_stats["current_intensity"]
        }

    def health_check(self) -> str:
        return f"OK — Carbon Neutral Mode: {self.get_carbon_impact()['status']} | Saved: {self.carbon_stats['saved_carbon_mg']}mg"
