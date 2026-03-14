"""
SigmaOS Sovereign Ludo Engine (v1.0 Apex Core)
=============================================
Pure logic handler for Ludo piece movement, dice orchestration, and P2P-ready state management.
Decoupled for zero-latency multi-device mesh play.
"""
import random
from typing import Dict, Any, List, Optional, Tuple

class LudoEngine:
    def __init__(self):
        self.turn = "RED"
        self.dice_val = 1
        # Path: Start (0) to Home Entry (51), then 5 steps to Home (56)
        # Coordinates for path (Conceptual mapping)
        self.piece_states = {
            "RED": [0, 0, 0, 0],    # Position on path (0 = yard)
            "GREEN": [0, 0, 0, 0],
            "BLUE": [0, 0, 0, 0],
            "YELLOW": [0, 0, 0, 0]
        }
        self.yard_coords = {
            "RED": [(60, 60), (60, 160), (160, 60), (160, 160)],
            "GREEN": [(440, 60), (440, 160), (540, 60), (540, 160)],
            "BLUE": [(60, 440), (60, 540), (160, 440), (160, 540)],
            "YELLOW": [(440, 440), (440, 540), (540, 440), (540, 540)]
        }
        self.history = ["SYSTEM: GRID INITIALIZED", "SYSTEM: ADVERSARIAL LOGIC LOADED"]

    def roll_dice(self) -> int:
        self.dice_val = random.randint(1, 6)
        return self.dice_val

    def move_piece(self, color: str, piece_idx: int) -> bool:
        """USP: Atomic piece movement via path-indexed quantization."""
        if color != self.turn: return False
        
        pos = self.piece_states[color][piece_idx]
        v = self.dice_val

        if pos == 0: # In Yard
            if v == 6:
                self.piece_states[color][piece_idx] = 1 # Enter board
                self.history.append(f"{color} #{piece_idx}: DEPLOYED TO GRID")
                return True
            return False
        
        new_pos = pos + v
        if new_pos > 56: # Must land exactly on Home (56) or similar
            return False
            
        self.piece_states[color][piece_idx] = new_pos
        self.history.append(f"{color} #{piece_idx}: ADVANCED TO {new_pos}")

        # Collision Check (Casualty Logic)
        for other_color, states in self.piece_states.items():
            if other_color == color: continue
            for i, other_pos in enumerate(states):
                if other_pos == new_pos and new_pos != 0:
                    # Casualty! Send back to yard
                    self.piece_states[other_color][i] = 0
                    self.history.append(f"CRITICAL: {color} ELIMINATED {other_color} AT {new_pos}")

        if v != 6:
            colors = ["RED", "GREEN", "BLUE", "YELLOW"]
            idx = colors.index(self.turn)
            self.turn = colors[(idx + 1) % 4]
            
        return True

    def get_piece_coord(self, color: str, idx: int) -> Tuple[int, int]:
        pos = self.piece_states[color][idx]
        if pos == 0:
            return self.yard_coords[color][idx]
        
        # Simple simulated path mapping for UI visualization
        offsets = {"RED": 0, "GREEN": 13, "BLUE": 26, "YELLOW": 39}
        base = 300 # Center bias
        angle = (pos + offsets[color]) * (360/52)
        import math
        r = 200 # Radius
        x = base + r * math.cos(math.radians(angle))
        y = base + r * math.sin(math.radians(angle))
        return (int(x), int(y))

    def reset(self):
        self.__init__()
