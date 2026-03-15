"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._find_tile_pos_at
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _find_tile_pos_at(self, x, y):
        """Return the grid position (0-indexed) at canvas coords x, y."""
        n = self.grid_n
        ts = self.tile_size
        pad = 4
        board_px = n * (ts + pad) + pad
        ox = max(0, (self.canvas.winfo_width() - board_px) // 2)
        oy = max(0, (self.canvas.winfo_height() - board_px) // 2)
        col = (x - ox - pad) // (ts + pad)
        row = (y - oy - pad) // (ts + pad)
        if 0 <= row < n and 0 <= col < n:
            return row * n + col
        return None
