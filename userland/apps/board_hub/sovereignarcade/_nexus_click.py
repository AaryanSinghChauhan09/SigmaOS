"""
Auto-split from userland\apps\board_hub.py — SovereignArcade._nexus_click
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional



class SovereignArcade:
    def _nexus_click(self, e):
        x, y = (e.x, e.y)
        best_d = 25
        best_l = None
        for r in range(5):
            for c in range(5):
                if c < 4:
                    lx, ly = (70 + c * 100 + 50, 70 + r * 100)
                    d = ((x - lx) ** 2 + (y - ly) ** 2) ** 0.5
                    if d < best_d:
                        best_d, best_l = (d, ((c, r), (c + 1, r)))
                if r < 4:
                    lx, ly = (70 + c * 100, 70 + r * 100 + 50)
                    d = ((x - lx) ** 2 + (y - ly) ** 2) ** 0.5
                    if d < best_d:
                        best_d, best_l = (d, ((c, r), (c, r + 1)))
        if best_l and best_l not in self.nx_lines:
            p1, p2 = best_l
            color = PAL['p1'] if self.nexus_turn == 1 else PAL['p2']
            self.nx_lines[best_l] = self.nx_canv.create_line(70 + p1[0] * 100, 70 + p1[1] * 100, 70 + p2[0] * 100, 70 + p2[1] * 100, fill=color, width=5)
            found = False
            for r in range(4):
                for c in range(4):
                    if (c, r) not in self.nx_boxes:
                        edges = [((c, r), (c + 1, r)), ((c, r + 1), (c + 1, r + 1)), ((c, r), (c, r + 1)), ((c + 1, r), (c + 1, r + 1))]
                        if all((e in self.nx_lines for e in edges)):
                            self.nx_canv.create_rectangle(70 + c * 100 + 10, 70 + r * 100 + 10, 70 + (c + 1) * 100 - 10, 70 + (r + 1) * 100 - 10, fill=color, stipple='gray25', outline='')
                            self.nx_boxes.add((c, r))
                            if self.nexus_turn == 1:
                                self.nexus_p1 += 1
                            else:
                                self.nexus_p2 += 1
                            found = True
            if not found:
                self.nexus_turn = 3 - self.nexus_turn
                self.nx_lbl.config(text="RED'S TURN" if self.nexus_turn == 1 else "BLUE'S TURN", fg=PAL['p1'] if self.nexus_turn == 1 else PAL['p2'])
            self.nx_score.config(text=f'RED: {self.nexus_p1} | BLUE: {self.nexus_p2}')
