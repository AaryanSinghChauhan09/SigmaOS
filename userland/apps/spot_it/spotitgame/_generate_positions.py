"""
Auto-split from userland\apps\spot_it.py — SpotItGame._generate_positions
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _generate_positions(self, n, W, H, s):
        """Generate non-overlapping random positions."""
        positions, attempts = ([], 0)
        min_dist = s * 2 + 6
        while len(positions) < n and attempts < n * 50:
            x = random.randint(s + 10, W - s - 10)
            y = random.randint(s + 10, H - s - 10)
            if all((math.hypot(x - px, y - py) >= min_dist for px, py in positions)):
                positions.append((x, y))
            attempts += 1
        return positions
