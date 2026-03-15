# Generated method: SovereignStrategist._draw_board
import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys

class SovereignStrategist:
    def _draw_board(self):
        b = self.engine.board
        lm = cast(Optional[Tuple[Tuple[int, int], Tuple[int, int]]], self.engine.last_move)
        for r in range(8):
            for c in range(8):
                p = b[r][c]
                sym = PIECES.get(cast(str, p), '')
                fg = PAL['white'] if p and cast(str, p).startswith('W') else PAL['black']
                bg = PAL['sq_light'] if (r + c) % 2 == 0 else PAL['sq_dark']
                if lm and (r, c) in lm:
                    bg = '#3A3A22'
                if self._selected == (r, c):
                    bg = PAL['accent']
                    fg = 'white'
                if (r, c) in self._hints:
                    bg = '#1A4A1A'
                self.cells[r][c].config(text=sym, fg=fg, bg=bg)