"""
SigmaOS Antigravity Engine
==========================
Verlet Integration based physics handler for Zero-G window interaction.
Focuses on low-mass, high-efficiency momentum and buoyancy in the OS UI.
"""

import time
import math
from typing import Dict, List, Any

class SigmaAntigravityEngine:
    def __init__(self, kernel):
        self.kernel = kernel
        self.is_active = False
        self.drift_y = -0.05 # Default upward drift
        self.elasticity = 0.8
        self.mass_map = {
            "browser": 1.5,
            "explorer": 1.0,
            "store": 2.5, # Heavier apps drift slower
            "ai": 0.5    # Aether is lightweight
        }
        self.entities = {} # {page_id: {"pos": [x,y], "vel": [vx,vy], "mass": m}}
        self.bounds = [1920, 1080] # Simulation target bounds

    def toggle_drift(self, state: bool):
        self.is_active = state
        if self.is_active:
             self.kernel.bus.emit("ag.drift.enabled", {"status": "ZERO-G ACTIVE"})
        else:
             self.kernel.bus.emit("ag.drift.disabled", {"status": "GRAVITY ENGAGED"})

    def register_entity(self, page_id: str, x: float, y: float, mass: float = 1.0):
        self.entities[page_id] = {
            "pos": [x, y],
            "prev_pos": [x, y],
            "acc": [0.0, self.drift_y],
            "mass": self.mass_map.get(page_id, mass)
        }

    def update_simulation(self, dt: float = 0.016):
        """Verlet Integration: Cheap, high-fidelity physics for UI."""
        if not self.is_active: return {}
        
        updates = {}
        for eid, e in self.entities.items():
            # Verlet Integration
            temp_x, temp_y = e["pos"][0], e["pos"][1]
            
            # x_new = 2x - x_prev + a * dt^2
            vx = (e["pos"][0] - e["prev_pos"][0])
            vy = (e["pos"][1] - e["prev_pos"][1])
            
            e["pos"][0] = e["pos"][0] + vx + e["acc"][0] * (dt * dt)
            e["pos"][1] = e["pos"][1] + vy + e["acc"][1] * (dt * dt)
            
            e["prev_pos"] = [temp_x, temp_y]
            
            # Collision with bounds (simulated)
            if e["pos"][1] < 0:
                e["pos"][1] = 0
                e["prev_pos"][1] = e["pos"][1] + vy * self.elasticity
            elif e["pos"][1] > self.bounds[1]:
                e["pos"][1] = self.bounds[1]
                e["prev_pos"][1] = e["pos"][1] + vy * self.elasticity
                
            if e["pos"][0] < 0 or e["pos"][0] > self.bounds[0]:
                vx = -vx * self.elasticity
                e["prev_pos"][0] = e["pos"][0] + vx
            
            updates[eid] = e["pos"]
        
        return updates

    def apply_impulse(self, eid: str, fx: float, fy: float):
        """Mouse impulse: Clicking provides a push."""
        if eid in self.entities:
            # Apply force directly to position for immediate drift
            self.entities[eid]["pos"][0] += fx
            self.entities[eid]["pos"][1] += fy

    def gather_all(self):
        """The Gather Command: Pull all drift entities back to center."""
        for eid in self.entities:
            self.entities[eid]["pos"] = [self.bounds[0]/2, self.bounds[1]/2]
            self.entities[eid]["prev_pos"] = [self.bounds[0]/2, self.bounds[1]/2]
            self.entities[eid]["acc"] = [0.0, 0.0]
        self.kernel.bus.emit("ag.gather", {"status": "CENTERED"})
