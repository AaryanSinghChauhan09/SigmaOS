# Generated method: WatchTheCup._cup_at
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _cup_at(self, x, y):
        """Return cup index under click, or None."""
        for i, cx in enumerate(self.cup_xs):
            cy = self.CUP_Y
            if cx - 45 <= x <= cx + 45 and cy <= y <= cy + self.CUP_H:
                return i
        return None