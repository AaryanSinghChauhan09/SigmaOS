# Generated method: SovereignArcade._bl_start
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _bl_start(self):
        if not self.bl_active:
            self.bl_active = True
            self.bricks = []
            for r in range(5):
                for c in range(8):
                    color = PAL['accent'] if r % 2 == 0 else '#32D74B'
                    b = self.bl_canv.create_rectangle(5 + c * 50, 40 + r * 20, 45 + c * 50, 55 + r * 20, fill=color, outline='')
                    self.bricks.append(b)
            self._bl_loop()